/**
 * DevilChain JavaScript SDK
 * Connect to DevilChain Network from web applications
 */

const API_BASE = process.env.DEVILCHAIN_API || "http://localhost:8545";

async function fetchAPI(path, options = {}) {
  const res = await fetch(`${API_BASE}${path}`, options);
  if (!res.ok) throw new Error(`DevilChain API error: ${res.status}`);
  return res.json();
}

/** Get latest block */
export async function getLatestBlock() {
  return fetchAPI("/api/block/latest");
}

/** Get block by height */
export async function getBlock(height) {
  return fetchAPI(`/api/block/${height}`);
}

/** Get transaction by hash */
export async function getTransaction(hash) {
  return fetchAPI(`/api/tx/${hash}`);
}

/** Get wallet balance */
export async function getWallet(address) {
  return fetchAPI(`/api/wallet/${address}`);
}

/** Send transaction */
export async function sendTransaction(tx) {
  return fetchAPI("/api/send", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(tx),
  });
}

/** Stake DVC */
export async function stake(address, amount, signature) {
  return fetchAPI("/api/stake", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ address, amount, signature }),
  });
}

/** Get network status */
export async function getStatus() {
  return fetchAPI("/api/status");
}

export default { getLatestBlock, getBlock, getTransaction, getWallet, sendTransaction, stake, getStatus };
