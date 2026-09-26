import { setStatus } from "../shared/db";
import {
  type CommandFrame,
  decodeFrame,
  encodeFrame,
  errorFrame,
  helloFrame,
  PROTOCOL_VERSION,
  resultFrame
} from "../shared/protocol";
import type { ExecutorKind, ExecutorMode } from "../shared/types";

// MV3 evicts the background service worker after ~30 s idle: a pending
// setTimeout is lost with it, so a cold start could stall until an unrelated
// tab event happened to revive the worker. A chrome.alarms keepalive survives
// eviction and re-wakes the worker itself; 0.5 min is the shortest period
// Chrome honors. The alarm is cleared once the socket is `ready` and re-armed
// on every drop.
const KEEPALIVE_ALARM = "keepalive";
const KEEPALIVE_PERIOD_MIN = 0.5;
const HEARTBEAT_MS = 25_000;

export interface BridgeClientConfig {
  url: string;
  token: string;
  /** Stable per-install id (routing target). */
  id?: string;
  /** Human-friendly label for this browser. */
  label?: string;
  /** "chrome" | "firefox" | … */
  browser?: string;
}

export interface BridgeClientDeps {
  /** Latest connection config (re-read on every (re)connect). */
  getConfig: () => Promise<BridgeClientConfig>;
  /** Execute one command and resolve with its result data. */
  onCommand: (frame: CommandFrame) => Promise<unknown>;
  /**
   * The broker abandoned an in-flight command (agent abort / timeout / agent
   * gone) — tear down any UI/work for that command id. No result is sent.
   */
  onCancel?: (id: string) => void;
  /** Executor kind to publish in the status row, for the UI. */
  executorKind: () => ExecutorKind;
  /**
   * Called when the server advertises an executor preference in `ready`. The
   * operator's plugin-side `executor` option takes precedence over the
   * dashboard choice on each connect.
   */
  onServerPreference?: (executor: ExecutorMode) => void | Promise<void>;
  /**
   * Called when the link drops (manual disconnect or the server going away) so
   * the executor can release control — e.g. detach the CDP debugger so the
   * browser isn't left with the "being debugged" banner after the agent stops.
   */
  onDisconnected?: () => void | Promise<void>;
  /**
   * Called on a server `release` frame — let go of control (detach the
   * debugger) but stay connected, so a later command can re-attach.
   */
  onRelease?: () => void | Promise<void>;
  /** Descriptor sent in the hello frame (e.g. "chrome/Linux"). */
  clientName: string;
}

/**
 * Outbound WebSocket to the plugin's bridge. The extension is always the
 * dialer (extensions can't host servers). Reconnects on a chrome.alarms
 * keepalive (which survives MV3 worker eviction) and runs a lightweight ping
 * heartbeat.
 */
export class BridgeClient {
  private ws: WebSocket | null = null;
  private heartbeatTimer: ReturnType<typeof setInterval> | null = null;
  private stopped = true;
  /**
   * Set when the broker rejects our handshake (bad token). We don't go dormant —
   * the rejection is often the *host's* fault (a stale/rotated token, or an old
   * host still squatting the port), fixed host-side without touching the
   * extension. So the keepalive keeps retrying, and the link self-heals when a
   * good host returns. Cleared on a successful `ready` (and on a fresh
   * `connect()`/`reconnect()`).
   */
  private handshakeRejected = false;

  constructor(private readonly deps: BridgeClientDeps) {}

  async connect(): Promise<void> {
    this.stopped = false;
    this.handshakeRejected = false;
    this.clearReconnect();
    await this.openSocket();
  }

  disconnect(): void {
    this.stopped = true;
    this.clearReconnect();
    this.stopHeartbeat();
    this.ws?.close();
    this.ws = null;
    void this.deps.onDisconnected?.();
    void setStatus({ state: "disconnected", connectedAt: undefined });
  }

  /** Force a fresh socket (e.g. after settings change). */
  async reconnect(): Promise<void> {
    this.ws?.close();
    this.ws = null;
    await this.connect();
  }

  private async openSocket(): Promise<void> {
    let config: BridgeClientConfig;
    try {
      config = await this.deps.getConfig();
    } catch (err) {
      await this.fail(`config error: ${err instanceof Error ? err.message : String(err)}`);
      return;
    }
    if (!config.token) {
      await setStatus({
        state: "error",
        lastError: "no token set — open the dashboard and paste the bridge token"
      });
      return;
    }

    await setStatus({ state: "connecting", lastError: undefined });
    let ws: WebSocket;
    try {
      ws = new WebSocket(config.url);
    } catch (err) {
      await this.fail(`bad bridge URL: ${err instanceof Error ? err.message : String(err)}`);
      return;
    }
    this.ws = ws;

    ws.addEventListener("open", () => {
      if (ws !== this.ws) {
        return;
      }
      ws.send(
        encodeFrame(
          helloFrame(config.token, {
            role: "extension",
            client: this.deps.clientName,
            id: config.id,
            label: config.label,
            browser: config.browser
          })
        )
      );
    });
    ws.addEventListener("message", (event) => this.onMessage(ws, String(event.data)));
    ws.addEventListener("close", () => this.onClose(ws));
    ws.addEventListener("error", () => {
      // The close event always follows; let it handle reconnect.
    });
  }

  private async onMessage(ws: WebSocket, raw: string): Promise<void> {
    // A socket replaced during reconnect can still deliver buffered events —
    // ignore anything that isn't from the current active socket.
    if (ws !== this.ws) {
      return;
    }
    const frame = decodeFrame(raw);
    if (!frame) {
      return;
    }
    switch (frame.type) {
      case "ready":
        this.handshakeRejected = false;
        // Link is up — stop the keepalive so the alarm does not wake the worker
        // every 30 s for nothing; a later drop re-arms it.
        this.clearReconnect();
        this.startHeartbeat();
        if (frame.executor && this.deps.onServerPreference) {
          await this.deps.onServerPreference(frame.executor as ExecutorMode);
        }
        await setStatus({
          state: "connected",
          connectedAt: Date.now(),
          lastError: undefined,
          executor: this.deps.executorKind()
        });
        return;
      case "command":
        await this.runCommand(ws, frame);
        return;
      case "rejected": {
        // The broker refused our handshake. Don't hammer — but don't go dormant
        // either: the cause is often host-side (a stale/rotated token, or an old
        // host still holding the port), fixed without touching us. `onClose`
        // (which follows immediately) sees `handshakeRejected` and schedules a
        // slow retry, so we self-heal when a good host returns. The message is
        // neutral — re-pasting may not be the fix.
        this.handshakeRejected = true;
        this.stopHeartbeat();
        const hint =
          frame.reason === "bad_token"
            ? "bridge rejected the token — it may be stale/rotated, or another host is running. Re-paste the current token, or restart the host that owns the bridge port."
            : `bridge rejected the connection (${frame.reason})`;
        await setStatus({ state: "error", lastError: hint, connectedAt: undefined });
        return;
      }
      case "release":
        await this.deps.onRelease?.();
        return;
      case "cancel":
        this.deps.onCancel?.(frame.id);
        return;
      case "ping":
        ws.send(encodeFrame({ v: PROTOCOL_VERSION, type: "pong" }));
        return;
      default:
        return;
    }
  }

  private async runCommand(ws: WebSocket, frame: CommandFrame): Promise<void> {
    try {
      const data = await this.deps.onCommand(frame);
      ws.send(encodeFrame(resultFrame(frame.id, data)));
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      ws.send(encodeFrame(errorFrame(frame.id, message)));
    }
  }

  private onClose(ws: WebSocket): void {
    // Ignore the close of a socket we already replaced (manual reconnect /
    // settings change) — otherwise it would schedule a duplicate keepalive
    // reconnect that races the fresh socket.
    if (ws !== this.ws) {
      return;
    }
    this.stopHeartbeat();
    if (this.stopped) {
      return;
    }
    // Release control either way (detach the debugger) while disconnected.
    void this.deps.onDisconnected?.();
    if (this.handshakeRejected) {
      // Handshake was rejected — keep the actionable error status visible and
      // let the keepalive retry, so we recover automatically when a good host
      // returns, without flooding meanwhile.
      this.scheduleReconnect();
      return;
    }
    // Server went away — a later command re-attaches if it comes back.
    void setStatus({ state: "connecting", connectedAt: undefined });
    this.scheduleReconnect();
  }

  private async fail(message: string): Promise<void> {
    await setStatus({ state: "error", lastError: message });
    if (!this.stopped) {
      this.scheduleReconnect();
    }
  }

  /**
   * Arm the keepalive alarm. A fixed 30 s cadence replaces the old exponential
   * setTimeout backoff: the alarm is the only timer that survives an MV3 worker
   * eviction, and it no longer needs a separate slower cadence for a rejected
   * token, because 30 s already neither floods nor gives up.
   */
  private scheduleReconnect(): void {
    void chrome.alarms.create(KEEPALIVE_ALARM, { periodInMinutes: KEEPALIVE_PERIOD_MIN });
  }

  private clearReconnect(): void {
    void chrome.alarms.clear(KEEPALIVE_ALARM);
  }

  /**
   * Keepalive tick (wired in the background entrypoint). Re-dials only when no
   * socket is live, so an alarm that fires between scheduling and `ready` is
   * harmless.
   */
  async resume(): Promise<void> {
    if (this.stopped) {
      return;
    }
    const state = this.ws?.readyState;
    if (state === WebSocket.OPEN || state === WebSocket.CONNECTING) {
      return;
    }
    await this.openSocket();
  }

  private startHeartbeat(): void {
    this.stopHeartbeat();
    this.heartbeatTimer = setInterval(() => {
      if (this.ws?.readyState === WebSocket.OPEN) {
        this.ws.send(encodeFrame({ v: PROTOCOL_VERSION, type: "ping" }));
      }
    }, HEARTBEAT_MS);
  }

  private stopHeartbeat(): void {
    if (this.heartbeatTimer) {
      clearInterval(this.heartbeatTimer);
      this.heartbeatTimer = null;
    }
  }
}
