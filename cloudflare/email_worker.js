export default {
  async email(message, env, ctx) {
    const from = message.from || "";
    const to = Array.isArray(message.to) ? message.to.join(", ") : (message.to || "");
    const subject = (message.headers && message.headers.get("subject")) || "";
    const messageId = (message.headers && message.headers.get("message-id")) || "";

    const forwardTo = env.FORWARD_TO || "";
    if (forwardTo) {
      await message.forward(forwardTo);
    }

    let rawText = "";
    try {
      if (message.raw && typeof message.raw === "object" && typeof message.raw.getReader === "function") {
        const reader = message.raw.getReader();
        const chunks = [];
        while (true) {
          const { done, value } = await reader.read();
          if (done) break;
          if (value) chunks.push(value);
        }
        const bytes = new Uint8Array(chunks.reduce((n, c) => n + c.length, 0));
        let off = 0;
        for (const c of chunks) { bytes.set(c, off); off += c.length; }
        rawText = new TextDecoder().decode(bytes);
      } else if (typeof message.raw === "string") {
        rawText = message.raw;
      }
    } catch (_) {
      rawText = "";
    }

    const base = (env.WEBHOOK_URL || "").replace(/\/+$/, "");
    if (!base) return;
    const payload = JSON.stringify({ from, to, subject, text: rawText, messageId });
    const key = "pending:" + (messageId || String(Date.now()));
    ctx.waitUntil(deliver(base, env, key, payload));
  },

  // cron every 5 min: the tunnel drop of 2026-09-13 healed within minutes.
  async scheduled(event, env, ctx) {
    ctx.waitUntil(drainPending(env));
  },
};

async function deliver(base, env, key, payload) {
  const headers = { "Content-Type": "application/json" };
  if (env.WEBHOOK_TOKEN) headers["Authorization"] = "Bearer " + env.WEBHOOK_TOKEN;
  for (let attempt = 1; attempt <= 3; attempt++) {
    try {
      const resp = await fetch(base + "/mail", { method: "POST", headers, body: payload });
      if (resp.ok) return;
    } catch (e) {
      console.log("webhook unreachable: " + String(e));
    }
    if (attempt < 3) await new Promise((r) => setTimeout(r, attempt * 1000));
  }
  if (!env.MAIL_QUEUE) {
    console.log("MAIL_QUEUE binding absent; " + key + " not buffered");
    return;
  }
  await env.MAIL_QUEUE.put(key, payload, { expirationTtl: 7 * 86400 });
  console.log("buffered pending mail: " + key);
}

async function drainPending(env) {
  if (!env.MAIL_QUEUE) {
    console.log("MAIL_QUEUE binding absent; pending queue not drained");
    return;
  }
  const base = (env.WEBHOOK_URL || "").replace(/\/+$/, "");
  if (!base) return;
  const headers = { "Content-Type": "application/json" };
  if (env.WEBHOOK_TOKEN) headers["Authorization"] = "Bearer " + env.WEBHOOK_TOKEN;
  const list = await env.MAIL_QUEUE.list({ prefix: "pending:" });
  for (const entry of list.keys) {
    const payload = await env.MAIL_QUEUE.get(entry.name);
    if (!payload) continue;
    try {
      const resp = await fetch(base + "/mail", { method: "POST", headers, body: payload });
      if (resp.ok) await env.MAIL_QUEUE.delete(entry.name);
    } catch (e) {
      console.log("pending mail held: " + entry.name);
    }
  }
}
