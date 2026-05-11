'use client';
import { useEffect, useState } from 'react';
import axios from 'axios';

const API = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8545';

export default function ValidatorsPage() {
  const [validators, setValidators] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    axios.get(`${API}/api/validators`)
      .then(r => { setValidators(r.data?.validators ?? []); setLoading(false); })
      .catch(() => setLoading(false));
  }, []);

  return (
    <main className="min-h-screen bg-[#0D0D0D] text-white">
      <header className="bg-[#1A1A1A] border-b border-[#CC0000] px-6 py-4 flex items-center gap-4">
        <a href="/" className="text-[#CC0000] font-bold text-xl">DevilScan</a>
        <span className="text-white/40">/</span>
        <span className="text-white/70">Validators</span>
      </header>
      <div className="max-w-5xl mx-auto px-6 py-8">
        <div className="flex justify-between items-center mb-6">
          <h1 className="text-2xl font-bold text-[#CC0000]">Active Validators</h1>
          <span className="bg-[#1A1A1A] border border-[#330000] px-3 py-1 rounded-full text-sm text-white/60">
            {validators.length} validators
          </span>
        </div>
        {loading && <p className="text-white/40">Loading validators...</p>}
        {!loading && validators.length === 0 && (
          <div className="bg-[#1A1A1A] border border-[#330000] rounded-xl p-8 text-center">
            <p className="text-white/40">No validators found. Testnet starting soon.</p>
          </div>
        )}
        {validators.length > 0 && (
          <div className="bg-[#1A1A1A] border border-[#330000] rounded-xl overflow-hidden">
            <table className="w-full">
              <thead>
                <tr className="border-b border-[#330000]">
                  {['#', 'Address', 'Staked DVC', 'Reputation', 'Blocks', 'Voting Power', 'Status'].map(h => (
                    <th key={h} className="text-left px-4 py-3 text-white/40 text-xs font-medium">{h}</th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {validators.map((v: any, i: number) => (
                  <tr key={v.address} className="border-b border-[#1A1A1A] hover:bg-[#220000] transition-colors">
                    <td className="px-4 py-3 text-white/40 text-sm">{i + 1}</td>
                    <td className="px-4 py-3 font-mono text-sm text-[#CC0000]">{v.address}</td>
                    <td className="px-4 py-3 text-sm">{v.staked_dvc} DVC</td>
                    <td className="px-4 py-3 text-sm">{v.reputation_score?.toFixed(2)}</td>
                    <td className="px-4 py-3 text-sm">{v.blocks_validated}</td>
                    <td className="px-4 py-3 text-sm">{(v.staked_dvc + v.reputation_score)?.toFixed(2)}</td>
                    <td className="px-4 py-3">
                      <span className={`px-2 py-0.5 rounded-full text-xs ${v.active ? 'bg-green-900 text-green-400' : 'bg-red-900 text-red-400'}`}>
                        {v.active ? 'Active' : 'Inactive'}
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </main>
  );
}
