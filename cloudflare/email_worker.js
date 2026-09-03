export default {
  async email(message, env, ctx) {
    const from = message.from || "";
    const to = Array.isArray(message.to) ? message.to.join(", ") : (message.to || "");
    const subject = (message.headers && message.headers.get("subject")) || "";
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
    const payload = JSON.stringify({ from, to, subject, text: rawText });
    const base = (env.WEBHOOK_URL || "").replace(/\/+$/, "");
    const token = env.WEBHOOK_TOKEN || "";
    if (!base) return;
    const url = base + "/mail";
    const headers = { "Content-Type": "application/json" };
    if (token) headers["Authorization"] = "Bearer " + token;
    await fetch(url, { method: "POST", headers, body: payload });
  },
};
