'use client';
import { useEffect, useState } from 'react';
import { useParams } from 'next/navigation';
import axios from 'axios';

const API = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8545';

export default function WalletPage() {
  const { address } = useParams();
  const [wallet, setWallet] = useState<any>(null);
  const [error, setError] = useState('');

  useEffect(() => {
    axios.get(`${API}/api/wallet/${address}`)
      .then(r => { if (r.data.error) setError(r.data.error); else setWallet(r.data); })
      .catch(() => setError('Failed to fetch wallet'));
  }, [address]);

  return (
    <main className="min-h-screen bg-[#0D0D0D] text-white">
      <header className="bg-[#1A1A1A] border-b border-[#CC0000] px-6 py-4">
        <a href="/" className="text-[#CC0000] font-bold text-xl">DevilScan</a>
        <span className="text-white/40 mx-3">/</span>
        <span className="text-white/60 text-sm">Wallet</span>
      </header>
      <div className="max-w-4xl mx-auto px-6 py-8">
        <h1 className="text-2xl font-bold mb-6">Wallet Details</h1>
        {error && <div className="bg-red-900/30 border border-red-800 rounded-xl p-4 text-red-400">{error}</div>}
        {wallet && (
          <>
            <div className="bg-[#1A1A1A] border border-[#330000] rounded-xl p-6 mb-6">
              <p className="text-white/40 text-xs mb-1">Address</p>
              <p className="font-mono text-[#CC0000] break-all text-lg">{wallet.address}</p>
              <div className="mt-4 flex items-end gap-2">
                <span className="text-5xl font-bold text-white">{wallet.balance?.toLocaleString()}</span>
                <span className="text-white/60 text-xl mb-1">DVC</span>
              </div>
              <p className="text-white/30 text-sm mt-1">DevilCoin Balance</p>
            </div>
            {wallet.transactions && wallet.transactions.length > 0 && (
              <div className="bg-[#1A1A1A] border border-[#330000] rounded-xl p-6">
                <h2 className="text-white/70 font-bold mb-4">Transaction History</h2>
                {wallet.transactions.map((tx: any) => (
                  <div key={tx.tx_hash} className="flex justify-between items-center py-2 border-b border-white/5 text-sm">
                    <span className="font-mono text-[#CC0000] text-xs">{tx.tx_hash?.slice(0, 20)}...</span>
                    <span className={tx.to === wallet.address ? 'text-green-400' : 'text-red-400'}>
                      {tx.to === wallet.address ? '+' : '-'}{tx.amount} DVC
                    </span>
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
