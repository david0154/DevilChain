'use client';
import { useEffect, useState } from 'react';
import { useParams } from 'next/navigation';
import axios from 'axios';

const API = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8545';

function Row({ label, value }: { label: string; value: any }) {
  return (
    <div className="flex justify-between text-sm border-b border-white/5 py-2">
      <span className="text-white/40 w-40 shrink-0">{label}</span>
      <span className="text-white/80 font-mono break-all">{String(value ?? '—')}</span>
    </div>
  );
}

export default function TxPage() {
  const { hash } = useParams();
  const [tx, setTx] = useState<any>(null);
  const [error, setError] = useState('');

  useEffect(() => {
    axios.get(`${API}/api/tx/${hash}`)
      .then(r => { if (r.data.error) setError(r.data.error); else setTx(r.data); })
      .catch(() => setError('Failed to fetch transaction'));
  }, [hash]);

  return (
    <main className="min-h-screen bg-[#0D0D0D] text-white">
      <header className="bg-[#1A1A1A] border-b border-[#CC0000] px-6 py-4">
        <a href="/" className="text-[#CC0000] font-bold text-xl">DevilScan</a>
        <span className="text-white/40 mx-3">/</span>
        <span className="text-white/60 text-sm">Transaction</span>
      </header>
      <div className="max-w-4xl mx-auto px-6 py-8">
        <h1 className="text-2xl font-bold text-white mb-6">Transaction Details</h1>
        {error && (
          <div className="bg-red-900/30 border border-red-800 rounded-xl p-4 text-red-400">{error}</div>
        )}
        {tx && (
          <div className="bg-[#1A1A1A] border border-[#330000] rounded-xl p-6 space-y-1">
            <Row label="TX Hash" value={tx.tx_hash} />
            <Row label="From" value={tx.from} />
            <Row label="To" value={tx.to} />
            <Row label="Amount" value={`${tx.amount} DVC`} />
            <Row label="Gas Fee" value={`${tx.gas_fee} DVC`} />
            <Row label="Timestamp" value={new Date((tx.timestamp ?? 0) * 1000).toLocaleString()} />
            <Row label="Signature" value={tx.signature} />
            <div className="mt-4 pt-2">
              <span className="inline-block bg-green-800/40 border border-green-700 text-green-400 text-xs px-3 py-1 rounded-full">✓ Confirmed</span>
            </div>
          </div>
        )}
      </div>
    </main>
  );
}
