// Makes YOUR own release-signing keypair (Ed25519) for the in-app updater.
//
//   node installer/make-keys.mjs
//
// It writes mimo-sign-key.pem (the PRIVATE half — this goes into the GitHub secret MIMO_SIGN_KEY and
// nowhere else) and prints the public half as the KEY array to paste into src-tauri/src/update.rs.
// After that, every release the workflow builds is signed with your key and only your updates install.
import { generateKeyPairSync, createHash } from 'node:crypto';
import { writeFileSync } from 'node:fs';

const { privateKey, publicKey } = generateKeyPairSync('ed25519');
const pem = privateKey.export({ type: 'pkcs8', format: 'pem' }).toString();
writeFileSync('mimo-sign-key.pem', pem, { mode: 0o600 });

const raw = publicKey.export({ type: 'spki', format: 'der' }).subarray(-32); // last 32 bytes of SPKI = the key
const hex = [...raw].map((b) => b.toString().padStart(3)).join(', ');
const fingerprint = createHash('sha256').update(raw).digest('hex').slice(0, 16);

console.log(`
Private key  -> ${'mimo-sign-key.pem'}  (keep it secret; put it in the GitHub secret MIMO_SIGN_KEY)
Fingerprint  -> ${fingerprint}

Paste this into src-tauri/src/update.rs, replacing the old KEY line:

const KEY: [u8; 32] = [${hex}];
`);
