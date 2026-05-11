'use client';
import { useEffect, useState } from 'react';
import { useParams } from 'next/navigation';
import axios from 'axios';

const API = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8545';

export default function WalletPage() {
  const { address } = useParams();
  const [wallet, setWallet] = useState<any>(null);
  const [txs, setTxs] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!address) return;
    Promise.all([
      axios.get(`${API}/api/wallet/${address}`),
      axios.get(`${API}/api/wallet/${address}/txs`).catch(() => ({ data: { transactions: [] } }))
    ]).then(([wRes, tRes]) => {
      setWallet(wRes.data);
      setTxs(tRes.data?.transactions ?? []);
      setLoading(false);
    }).catch(() => setLoading(false));
  }, [address]);

  return (
    <main className="min-h-screen bg-[#0D0D0D] text-white">
      <header className="bg-[#1A1A1A] border-b border-[#CC0000] px-6 py-4 flex items-center gap-4">
        <a href="/" className="text-[#CC0000] font-bold text-xl">DevilScan</a>
        <span className="text-white/40">/</span>
        <span className="text-white/70">Wallet</span>
      </header>
      <div className="max-w-4xl mx-auto px-6 py-8">
        <h1 className="text-2xl font-bold text-[#CC0000] mb-2">Wallet Details</h1>
        <p className="text-white/40 font-mono text-sm mb-6 break-all">{address}</p>
        {loading && <p className="text-white/40">Loading...</p>}
        {!loading && !wallet && (
          <div className="bg-[#1A1A1A] border border-[#330000] rounded-xl p-6 text-center">
            <p className="text-red-400">Wallet not found or no transactions yet.</p>
          </div>
        )}
        {wallet && (
          <>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-8">
              {[
                { label: 'Balance', value: `${wallet.balance ?? 0} DVC` },
                { label: 'Staked', value: `${wallet.staked ?? 0} DVC` },
                { label: 'Transactions', value: wallet.tx_count ?? 0 },
                { label: 'AI Risk', value: wallet.ai_risk_score ?? 'Low' },
              ].map(s => (
                <div key={s.label} className="bg-[#1A1A1A] border border-[#330000] rounded-xl p-4">
                  <p className="text-white/40 text-xs mb-1">{s.label}</p>
                  <p className="text-[#CC0000] font-bold text-lg">{String(s.value)}</p>
                </div>
              ))}
            </div>
            {txs.length > 0 && (
              <div className="bg-[#1A1A1A] border border-[#330000] rounded-xl overflow-hidden">
                <div className="px-6 py-4 border-b border-[#330000]">
                  <h2 className="font-bold text-white/70">Recent Transactions</h2>
                </div>
                {txs.map((tx: any) => (
                  <div key={tx.tx_hash} className="px-6 py-3 border-b border-[#1A0000] flex justify-between items-center hover:bg-[#220000]">
                    <div>
                      <a href={`/tx/${tx.tx_hash}`} className="text-[#CC0000] font-mono text-xs hover:underline">
                        {tx.tx_hash?.slice(0, 20)}...
                      </a>
                      <p className="text-white/40 text-xs">{tx.from === address ? '→ Sent' : '← Received'}</p>
                    </div>
                    <span className="text-white/80 text-sm font-bold">{tx.amount} DVC</span>
                  </div>
                ))}
              </div>
            )}
          </>
        )}
      </div>
    </main>
  );
}
