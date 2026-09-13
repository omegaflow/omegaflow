export default {
  async email(message, env, ctx) {
    const from = message.from || "";
    const to = Array.isArray(message.to) ? message.to.join(", ") : (message.to || "");
    const subject = (message.headers && message.headers.get("subject")) || "";

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

    const base = (env.WEBHOOK_URL || "").replace(/\/+$/);
    if (!base) return;
    const payload = JSON.stringify({ from, to, subject, text: rawText });
    const headers = { "Content-Type": "application/json" };
    if (env.WEBHOOK_TOKEN) headers["Authorization"] = "Bearer " + env.WEBHOOK_TOKEN;
    try {
      await fetch(base + "/mail", { method: "POST", headers, body: payload });
    } catch (e) {
      console.log("webhook absent: " + String(e));
    }
  },
};
