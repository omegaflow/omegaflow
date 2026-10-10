const ISSUER = "https://token.actions.githubusercontent.com";

export function b64urlDecode(str) {
  const pad = str.length % 4 === 0 ? "" : "=".repeat(4 - (str.length % 4));
  const b64 = (str + pad).replace(/-/g, "+").replace(/_/g, "/");
  const bin = atob(b64);
  const bytes = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i++) bytes[i] = bin.charCodeAt(i);
  return bytes;
}

export function decodeJwt(token) {
  const parts = token.split(".");
  if (parts.length !== 3) throw new Error("token is not a JWS");
  const decoder = new TextDecoder();
  const header = JSON.parse(decoder.decode(b64urlDecode(parts[0])));
  const payload = JSON.parse(decoder.decode(b64urlDecode(parts[1])));
  return {
    header,
    payload,
    signingInput: parts[0] + "." + parts[1],
    signature: b64urlDecode(parts[2]),
  };
}

async function importJwk(jwk) {
  return crypto.subtle.importKey(
    "jwk",
    jwk,
    { name: "RSASSA-PKCS1-v1_5", hash: "SHA-256" },
    false,
    ["verify"],
  );
}

export async function verifyOidc(token, { jwks, audience, repository, now }) {
  const { header, payload, signingInput, signature } = decodeJwt(token);
  if (header.alg !== "RS256") throw new Error("unexpected alg " + header.alg);
  const jwk = jwks.keys.find((k) => k.kid === header.kid);
  if (!jwk) throw new Error("kid not in jwks");
  const key = await importJwk(jwk);
  const ok = await crypto.subtle.verify(
    "RSASSA-PKCS1-v1_5",
    key,
    signature,
    new TextEncoder().encode(signingInput),
  );
  if (!ok) throw new Error("signature does not verify");
  const t = now ?? Math.floor(Date.now() / 1000);
  if (typeof payload.exp !== "number" || payload.exp < t) throw new Error("token expired");
  if (typeof payload.nbf === "number" && payload.nbf > t + 60) {
    throw new Error("token not yet valid");
  }
  if (payload.iss !== ISSUER) throw new Error("issuer mismatch");
  const aud = Array.isArray(payload.aud) ? payload.aud : [payload.aud];
  if (audience && !aud.includes(audience)) throw new Error("audience mismatch");
  if (repository && payload.repository !== repository) throw new Error("repository mismatch");
  return payload;
}
