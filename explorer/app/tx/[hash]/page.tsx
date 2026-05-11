'use client';
import { useEffect, useState } from 'react';
import { useParams } from 'next/navigation';
import axios from 'axios';

const API = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8545';

export default function TxPage() {
  const { hash } = useParams();
  const [tx, setTx] = useState<any>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!hash) return;
    axios.get(`${API}/api/tx/${hash}`)
      .then(r => { setTx(r.data); setLoading(false); })
      .catch(() => setLoading(false));
  }, [hash]);

  const fields = [
    { label: 'TX Hash', value: tx?.tx_hash },
    { label: 'From', value: tx?.from },
    { label: 'To', value: tx?.to },
    { label: 'Amount', value: tx?.amount ? `${tx.amount} DVC` : undefined },
    { label: 'Gas Fee', value: tx?.gas_fee ? `${tx.gas_fee} DVC` : undefined },
    { label: 'Timestamp', value: tx?.timestamp ? new Date(tx.timestamp * 1000).toLocaleString() : undefined },
    { label: 'AI Score', value: tx?.ai_score },
    { label: 'Status', value: tx?.status ?? 'Confirmed' },
  ];

  return (
    <main className="min-h-screen bg-[#0D0D0D] text-white">
      <header className="bg-[#1A1A1A] border-b border-[#CC0000] px-6 py-4 flex items-center gap-4">
        <a href="/" className="text-[#CC0000] font-bold text-xl">DevilScan</a>
        <span className="text-white/40">/</span>
        <span className="text-white/70">Transaction</span>
      </header>
      <div className="max-w-3xl mx-auto px-6 py-8">
        <h1 className="text-2xl font-bold text-[#CC0000] mb-6">Transaction Details</h1>
        {loading && <p className="text-white/40">Loading...</p>}
        {!loading && !tx && (
          <div className="bg-[#1A1A1A] border border-[#330000] rounded-xl p-6 text-center">
            <p className="text-red-400">Transaction not found</p>
            <p className="text-white/40 text-sm mt-2 font-mono">{hash}</p>
          </div>
        )}
        {tx && (
          <div className="bg-[#1A1A1A] border border-[#330000] rounded-xl overflow-hidden">
            {fields.map(f => f.value !== undefined && (
              <div key={f.label} className="flex justify-between items-start px-6 py-4 border-b border-[#220000] last:border-0">
                <span className="text-white/40 text-sm w-32 flex-shrink-0">{f.label}</span>
                <span className="text-white/90 font-mono text-sm break-all text-right">{String(f.value)}</span>
              </div>
            ))}
          </div>
        )}
      </div>
    </main>
  );
}
