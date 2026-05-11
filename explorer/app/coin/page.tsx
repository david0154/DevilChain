'use client';
import { useEffect, useState } from 'react';
import Link from 'next/link';
import axios from 'axios';

const API = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8545';

const ALLOC = [
  { label: 'Mining Rewards', pct: 35, amount: '350,000,000', color: '#CC0000', wallet: 'db1xmining_pool' },
  { label: 'Ecosystem',      pct: 20, amount: '200,000,000', color: '#f59e0b', wallet: 'db1xecosystem' },
  { label: 'DAO Treasury',   pct: 15, amount: '150,000,000', color: '#60a5fa', wallet: 'db1xdao_treasury' },
  { label: 'Team & Dev',     pct: 10, amount: '100,000,000', color: '#34d399', wallet: 'db1xteam' },
  { label: 'Validators',     pct: 10, amount: '100,000,000', color: '#a78bfa', wallet: 'db1xvalidator_pool' },
  { label: 'Investors',      pct:  5, amount:  '50,000,000', color: '#fb923c', wallet: 'db1xinvestors' },
  { label: 'Community',      pct:  5, amount:  '50,000,000', color: '#f472b6', wallet: 'db1xcommunity' },
];

export default function CoinPage() {
  const [coin, setCoin] = useState<any>(null);

  useEffect(() => {
    axios.get(`${API}/api/coin`).then(r => setCoin(r.data)).catch(() => {});
  }, []);

  return (
    <main className="min-h-screen bg-[#0D0D0D] text-white">
      <header className="bg-[#111] border-b border-[#CC0000]/40 px-6 py-4 flex items-center gap-3">
        <Link href="/" className="text-[#CC0000] font-black text-xl">DevilScan</Link>
        <span className="text-white/20">/</span>
        <span className="text-white/50">DevilCoin (DVC/DVL)</span>
      </header>
      <div className="max-w-4xl mx-auto px-6 py-8">
        <div className="flex items-center gap-4 mb-8">
          <div className="w-16 h-16 rounded-full bg-gradient-to-br from-[#CC0000] to-[#660000] flex items-center justify-center text-3xl font-black">D</div>
          <div>
            <h1 className="text-3xl font-black">DevilCoin <span className="text-[#CC0000]">DVC</span></h1>
            <p className="text-white/40">Native coin of DevilChain Network · Ticker: DVL</p>
          </div>
        </div>

        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-10">
          {[
            { label: 'Total Supply',  value: '1,000,000,000 DVC' },
            { label: 'Decimals',      value: '18' },
            { label: 'Block Reward',  value: '50 DVC' },
            { label: 'Min Gas Fee',   value: '0.01 DVC' },
            { label: 'Min Stake',     value: '100 DVC' },
            { label: 'Consensus',     value: 'DHP (PoS+μPoW+AI)' },
            { label: 'Block Time',    value: '3 seconds' },
            { label: 'Max TPS',       value: '20,000' },
          ].map(s => (
            <div key={s.label} className="bg-[#151515] border border-[#2a0000] rounded-2xl p-4">
              <p className="text-white/30 text-xs mb-1">{s.label}</p>
              <p className="text-[#CC0000] font-bold">{s.value}</p>
            </div>
          ))}
        </div>

        <h2 className="text-white font-bold mb-4">Token Allocation</h2>
        <div className="bg-[#151515] border border-[#2a0000] rounded-2xl overflow-hidden mb-8">
          {ALLOC.map(a => (
            <div key={a.label} className="flex items-center gap-4 px-6 py-4 border-b border-[#1a0000] last:border-0">
              <div className="w-3 h-3 rounded-full flex-shrink-0" style={{ background: a.color }} />
              <div className="flex-1">
                <div className="flex justify-between mb-1">
                  <span className="text-white/80 text-sm">{a.label}</span>
                  <span className="font-bold" style={{ color: a.color }}>{a.pct}%</span>
                </div>
                <div className="h-1.5 bg-[#1a0000] rounded-full">
                  <div className="h-full rounded-full" style={{ width: `${a.pct * 2.86}%`, background: a.color }} />
                </div>
                <div className="flex justify-between mt-1">
                  <Link href={`/wallet/${a.wallet}`} className="text-white/30 text-xs font-mono hover:text-[#CC0000]">{a.wallet}</Link>
                  <span className="text-white/30 text-xs">{a.amount} DVC</span>
                </div>
              </div>
            </div>
          ))}
        </div>

        <h2 className="text-white font-bold mb-4">How to Get DVC</h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          {[
            { title: '⛏️ Mine', desc: 'Run a DevilMine node. Earn 50 DVC per block + AI bonuses.' },
            { title: '🪙 Stake', desc: 'Stake 100+ DVC as a validator. Earn % of block rewards.' },
            { title: '🚰 Faucet', desc: 'Get 1000 DVC free on testnet via /api/faucet.' },
          ].map(c => (
            <div key={c.title} className="bg-[#151515] border border-[#2a0000] rounded-2xl p-5">
              <p className="text-xl mb-2">{c.title}</p>
              <p className="text-white/40 text-sm">{c.desc}</p>
            </div>
          ))}
        </div>
      </div>
    </main>
  );
}
