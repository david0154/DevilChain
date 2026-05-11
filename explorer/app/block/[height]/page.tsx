'use client';
import { useEffect, useState } from 'react';
import { useParams } from 'next/navigation';
import Link from 'next/link';
import axios from 'axios';

const API = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8545';

export default function BlockPage() {
  const { height } = useParams();
  const [block, setBlock] = useState<any>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!height) return;
    axios.get(`${API}/api/block/${height}`)
      .then(r => { setBlock(r.data); setLoading(false); })
      .catch(() => setLoading(false));
  }, [height]);

  const fields = [
    { label: 'Height',       value: block?.block_height ?? block?.height },
    { label: 'Hash',         value: block?.block_hash },
    { label: 'Previous',     value: block?.previous_hash },
    { label: 'Validator',    value: block?.validator },
    { label: 'TX Count',     value: block?.tx_count },
    { label: 'Nonce',        value: block?.nonce },
    { label: 'AI Score',     value: block?.ai_score },
    { label: 'Timestamp',    value: block?.timestamp ? new Date(block.timestamp * 1000).toLocaleString() : undefined },
  ];

  return (
    <main className="min-h-screen bg-[#0D0D0D] text-white">
      <header className="bg-[#111] border-b border-[#CC0000]/40 px-6 py-4 flex items-center gap-3">
        <Link href="/" className="text-[#CC0000] font-black text-xl">DevilScan</Link>
        <span className="text-white/20">/</span>
        <span className="text-white/50">Block #{height}</span>
      </header>
      <div className="max-w-3xl mx-auto px-6 py-8">
        <h1 className="text-2xl font-bold text-[#CC0000] mb-6">Block Details</h1>
        {loading && <p className="text-white/40">Loading...</p>}
        {!loading && !block && (
          <div className="bg-[#151515] border border-[#330000] rounded-2xl p-8 text-center">
            <p className="text-red-400">Block #{height} not found</p>
          </div>
        )}
        {block && (
          <div className="bg-[#151515] border border-[#2a0000] rounded-2xl overflow-hidden">
            {fields.map(f => f.value !== undefined && (
              <div key={f.label} className="flex justify-between items-start px-6 py-4 border-b border-[#1a0000] last:border-0">
                <span className="text-white/40 text-sm w-32 flex-shrink-0">{f.label}</span>
                <span className="text-white/90 font-mono text-sm break-all text-right">{String(f.value)}</span>
              </div>
            ))}
            {block.transactions?.length > 0 && (
              <div className="px-6 py-4">
                <p className="text-white/40 text-sm mb-3">Transactions ({block.transactions.length})</p>
                {block.transactions.map((tx: any) => (
                  <Link key={tx} href={`/tx/${tx}`} className="block text-[#CC0000] font-mono text-xs hover:underline mb-1">
                    {tx}
                  </Link>
                ))}
              </div>
            )}
          </div>
        )}
      </div>
    </main>
  );
}
