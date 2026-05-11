'use client';
import { useEffect, useState } from 'react';
import axios from 'axios';

const API = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8545';

export default function HomePage() {
  const [status, setStatus] = useState<any>(null);
  const [latestBlock, setLatestBlock] = useState<any>(null);
  const [search, setSearch] = useState('');

  useEffect(() => {
    axios.get(`${API}/api/status`).then(r => setStatus(r.data)).catch(() => {});
    axios.get(`${API}/api/block/latest`).then(r => setLatestBlock(r.data)).catch(() => {});
  }, []);

  return (
    <main className="min-h-screen bg-[#0D0D0D] text-white">
      <header className="bg-[#1A1A1A] border-b border-[#CC0000] px-6 py-4 flex items-center gap-4">
        <img src="/logo.png" alt="DevilChain" className="h-10" />
        <h1 className="text-2xl font-bold text-[#CC0000]">DevilScan</h1>
        <span className="text-white/40 text-sm">DevilChain Explorer</span>
      </header>
      <section className="px-6 py-8 max-w-4xl mx-auto">
        <div className="flex gap-2">
          <input value={search} onChange={e => setSearch(e.target.value)}
            placeholder="Search block height, tx hash, or address (db1x...)"
            className="flex-1 bg-[#1A1A1A] border border-[#CC0000] rounded-lg px-4 py-3 text-white placeholder-white/30 focus:outline-none" />
          <button className="bg-[#CC0000] hover:bg-red-700 px-6 py-3 rounded-lg font-bold">Search</button>
        </div>
      </section>
      <section className="px-6 max-w-4xl mx-auto grid grid-cols-2 md:grid-cols-4 gap-4 mb-8">
        {[
          { label: 'Network', value: status?.network ?? 'DevilChain' },
          { label: 'Latest Block', value: latestBlock?.block_height ?? '...' },
          { label: 'TPS Target', value: '5K-20K' },
          { label: 'Coin', value: 'DVC / DVL' },
        ].map(s => (
          <div key={s.label} className="bg-[#1A1A1A] border border-[#330000] rounded-xl p-4">
            <p className="text-white/40 text-xs mb-1">{s.label}</p>
            <p className="text-[#CC0000] font-bold text-lg">{String(s.value)}</p>
          </div>
        ))}
      </section>
      {latestBlock && (
        <section className="px-6 max-w-4xl mx-auto mb-8">
          <h2 className="text-lg font-bold text-white/70 mb-4">Latest Block</h2>
          <div className="bg-[#1A1A1A] border border-[#330000] rounded-xl p-6 space-y-2">
            {Object.entries(latestBlock).map(([k, v]) => (
              <div key={k} className="flex justify-between text-sm">
                <span className="text-white/40 capitalize">{k.replace(/_/g, ' ')}</span>
                <span className="text-white/80 font-mono truncate max-w-xs">{JSON.stringify(v)}</span>
              </div>
            ))}
          </div>
        </section>
      )}
    </main>
  );
}
