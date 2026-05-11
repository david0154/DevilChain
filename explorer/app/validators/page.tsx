'use client';
import { useEffect, useState } from 'react';
import axios from 'axios';

const API = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8545';

interface Validator {
  address: string;
  staked: number;
  reputation: number;
  voting_power: number;
  blocks_validated: number;
  blocks_missed: number;
  is_active: boolean;
}

export default function ValidatorsPage() {
  const [validators, setValidators] = useState<Validator[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    axios.get(`${API}/api/validators`)
      .then(r => setValidators(r.data.validators ?? []))
      .catch(() => {})
      .finally(() => setLoading(false));
  }, []);

  return (
    <main className="min-h-screen bg-[#0D0D0D] text-white">
      <header className="bg-[#1A1A1A] border-b border-[#CC0000] px-6 py-4">
        <a href="/" className="text-[#CC0000] font-bold text-xl">DevilScan</a>
        <span className="text-white/40 mx-3">/</span>
        <span className="text-white/60 text-sm">Validators</span>
      </header>
      <div className="max-w-5xl mx-auto px-6 py-8">
        <h1 className="text-2xl font-bold mb-6">Active Validators</h1>
        {loading && <p className="text-white/40">Loading validators...</p>}
        {!loading && validators.length === 0 && (
          <div className="bg-[#1A1A1A] border border-[#330000] rounded-xl p-8 text-center text-white/40">
            No validators found. Be the first to stake DVC and validate DevilChain!
          </div>
        )}
        {validators.length > 0 && (
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="text-white/40 border-b border-white/10 text-left">
                  <th className="py-3 pr-4">Address</th>
                  <th className="py-3 pr-4">Staked (DVC)</th>
                  <th className="py-3 pr-4">Reputation</th>
                  <th className="py-3 pr-4">Voting Power</th>
                  <th className="py-3 pr-4">Validated</th>
                  <th className="py-3 pr-4">Missed</th>
                  <th className="py-3">Status</th>
                </tr>
              </thead>
              <tbody>
                {validators.map((v, i) => (
                  <tr key={i} className="border-b border-white/5 hover:bg-white/5">
                    <td className="py-3 pr-4 font-mono text-xs text-[#CC0000]">{v.address}</td>
                    <td className="py-3 pr-4">{v.staked?.toLocaleString()}</td>
                    <td className="py-3 pr-4">{v.reputation?.toFixed(1)}</td>
                    <td className="py-3 pr-4">{v.voting_power?.toFixed(2)}</td>
                    <td className="py-3 pr-4 text-green-400">{v.blocks_validated}</td>
                    <td className="py-3 pr-4 text-red-400">{v.blocks_missed}</td>
                    <td className="py-3">
                      <span className={`text-xs px-2 py-1 rounded-full ${
                        v.is_active ? 'bg-green-900/40 text-green-400 border border-green-800' : 'bg-red-900/40 text-red-400 border border-red-800'
                      }`}>{v.is_active ? 'Active' : 'Inactive'}</span>
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
