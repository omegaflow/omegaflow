import { test } from "node:test";
import assert from "node:assert/strict";

import { verifyOidc } from "../src/verify.js";

const ISSUER = "https://token.actions.githubusercontent.com";
const AUDIENCE = "omegaflow-r2-verifier";
const REPOSITORY = "omegaflow/omegaflow";

function b64url(bytes) {
  let bin = "";
  for (const b of bytes) bin += String.fromCharCode(b);
  return btoa(bin).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}

const enc = (value) => b64url(new TextEncoder().encode(JSON.stringify(value)));

async function fixture(overrides = {}, jwkOverrides = {}) {
  const pair = await crypto.subtle.generateKey(
    {
      name: "RSASSA-PKCS1-v1_5",
      modulusLength: 2048,
      publicExponent: new Uint8Array([1, 0, 1]),
      hash: "SHA-256",
    },
    true,
    ["sign", "verify"],
  );
  const jwk = await crypto.subtle.exportKey("jwk", pair.publicKey);
  Object.assign(jwk, { kid: "test-kid", alg: "RS256", use: "sig" }, jwkOverrides);
  const now = Math.floor(Date.now() / 1000);
  const payload = {
    iss: ISSUER,
    aud: AUDIENCE,
    repository: REPOSITORY,
    exp: now + 600,
    nbf: now - 60,
    ...overrides,
  };
  const header = { alg: "RS256", kid: jwkOverrides.kid ?? "test-kid", typ: "JWT" };
  const signingInput = enc(header) + "." + enc(payload);
  const signature = await crypto.subtle.sign(
    "RSASSA-PKCS1-v1_5",
    pair.privateKey,
    new TextEncoder().encode(signingInput),
  );
  return {
    token: signingInput + "." + b64url(new Uint8Array(signature)),
    jwks: { keys: [jwk] },
    now,
  };
}

test("a well-formed GitHub OIDC token verifies", async () => {
  const { token, jwks, now } = await fixture();
  const payload = await verifyOidc(token, { jwks, audience: AUDIENCE, repository: REPOSITORY, now });
  assert.equal(payload.repository, REPOSITORY);
});

test("a foreign audience is refused", async () => {
  const { token, jwks, now } = await fixture();
  await assert.rejects(
    () => verifyOidc(token, { jwks, audience: "someone-else", repository: REPOSITORY, now }),
    /audience mismatch/,
  );
});

test("a foreign repository is refused", async () => {
  const { token, jwks, now } = await fixture({ repository: "attacker/repo" });
  await assert.rejects(
    () => verifyOidc(token, { jwks, audience: AUDIENCE, repository: REPOSITORY, now }),
    /repository mismatch/,
  );
});

test("an expired token is refused", async () => {
  const { token, jwks, now } = await fixture({ exp: 1000 });
  await assert.rejects(
    () => verifyOidc(token, { jwks, audience: AUDIENCE, repository: REPOSITORY, now }),
    /token expired/,
  );
});

test("a tampered signature is refused", async () => {
  const { token, jwks, now } = await fixture();
  const parts = token.split(".");
  parts[1] = enc({ iss: ISSUER, aud: AUDIENCE, repository: REPOSITORY, exp: now + 600 });
  await assert.rejects(
    () => verifyOidc(parts.join("."), { jwks, audience: AUDIENCE, repository: REPOSITORY, now }),
    /signature does not verify/,
  );
});

test("an unknown kid is refused", async () => {
  const { token, jwks, now } = await fixture();
  jwks.keys[0].kid = "other";
  await assert.rejects(
    () => verifyOidc(token, { jwks, audience: AUDIENCE, repository: REPOSITORY, now }),
    /kid not in jwks/,
  );
});
