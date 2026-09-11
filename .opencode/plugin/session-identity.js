export const SessionIdentity = async ({ client }) => {
  const titles = new Map();
  const injected = new Set();

  async function titleOf(sessionID) {
    if (!sessionID) return "";
    if (titles.has(sessionID)) return titles.get(sessionID);
    let title = "";
    try {
      const res = await client.session.get({ path: { id: sessionID } });
      title = res?.data?.title ?? "";
    } catch {}
    if (title) titles.set(sessionID, title);
    return title;
  }

  return {
    "shell.env": async (input, output) => {
      const id = input.sessionID;
      if (!id) return;
      output.env.OPENCODE_SESSION_ID = id;
      output.env.OPENCODE_SESSION_TITLE = await titleOf(id);
    },
    "chat.message": async (input, output) => {
      const id = input.sessionID;
      if (!id || injected.has(id)) return;
      injected.add(id);
      const title = await titleOf(id);
      output.parts.push({
        type: "text",
        text: `Session: ${id}${title ? ` — Titel: ${title}` : ""}`,
        synthetic: true,
      });
    },
  };
};
