import { verifyOidc } from "./verify.js";

const JWKS_URL = "https://token.actions.githubusercontent.com/.well-known/jwks";
const AUDIENCE = "omegaflow-r2-verifier";
const REPOSITORY = "omegaflow/omegaflow";
const JWKS_TTL_MS = 3600_000;

let jwksCache = null;

async function jwks() {
  if (jwksCache && jwksCache.expires > Date.now()) return jwksCache.value;
  const response = await fetch(JWKS_URL);
  if (!response.ok) throw new Error("jwks fetch returned " + response.status);
  const value = await response.json();
  jwksCache = { value, expires: Date.now() + JWKS_TTL_MS };
  return value;
}

function hex(bytes) {
  let out = "";
  for (const b of bytes) out += b.toString(16).padStart(2, "0");
  return out;
}

export default {
  async fetch(request, env) {
    if (request.method !== "PUT") {
      return new Response("method not allowed", { status: 405 });
    }
    const auth = request.headers.get("Authorization") || "";
    const token = auth.startsWith("Bearer ") ? auth.slice(7) : "";
    if (!token) return new Response("no oidc token", { status: 401 });
    try {
      await verifyOidc(token, {
        jwks: await jwks(),
        audience: AUDIENCE,
        repository: REPOSITORY,
      });
    } catch (e) {
      return new Response("oidc refused: " + e.message, { status: 403 });
    }
    const url = new URL(request.url);
    const key = decodeURIComponent(url.pathname.replace(/^\/+/, ""));
    if (!key || key.includes("..") || key.startsWith("/")) {
      return new Response("bad key", { status: 400 });
    }
    const body = await request.arrayBuffer();
    const digest = hex(new Uint8Array(await crypto.subtle.digest("SHA-256", body)));
    const expected = request.headers.get("x-amz-content-sha256");
    if (!expected || expected !== digest) {
      return new Response("content hash mismatch", { status: 400 });
    }
    await env.BUCKET.put(key, body);
    return new Response("ok " + key + " sha256=" + digest, { status: 200 });
  },
};
