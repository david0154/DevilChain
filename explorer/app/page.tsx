'use client';
import { useEffect, useState, useCallback } from 'react';
import axios from 'axios';
import Link from 'next/link';

const API = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8545';

function StatCard({ label, value, sub, color = '#CC0000' }: any) {
  return (
    <div className="bg-[#151515] border border-[#2a0000] rounded-2xl p-5 flex flex-col gap-1 hover:border-[#CC0000] transition-all">
      <span className="text-white/40 text-xs uppercase tracking-widest">{label}</span>
      <span className="text-2xl font-bold" style={{ color }}>{value ?? '—'}</span>
      {sub && <span className="text-white/30 text-xs">{sub}</span>}
    </div>
  );
}

function TxRow({ tx }: any) {
  return (
    <tr className="border-b border-[#1a0000] hover:bg-[#1e0000] transition-colors">
      <td className="px-4 py-3">
        <Link href={`/tx/${tx.tx_hash}`} className="text-[#CC0000] font-mono text-xs hover:underline">
          {tx.tx_hash?.slice(0, 18)}...
        </Link>
      </td>
      <td className="px-4 py-3 font-mono text-xs text-white/50">{tx.from?.slice(0, 14)}...</td>
      <td className="px-4 py-3 font-mono text-xs text-white/50">{tx.to?.slice(0, 14)}...</td>
      <td className="px-4 py-3 text-sm font-bold text-green-400">{tx.amount} DVC</td>
      <td className="px-4 py-3 text-xs text-white/40">{tx.gas_fee ?? '0.01'} DVC</td>
      <td className="px-4 py-3">
        <span className="bg-green-900/50 text-green-400 px-2 py-0.5 rounded-full text-xs">Confirmed</span>
      </td>
    </tr>
  );
}

function BlockRow({ block }: any) {
  return (
    <tr className="border-b border-[#1a0000] hover:bg-[#1e0000] transition-colors">
      <td className="px-4 py-3">
        <Link href={`/block/${block.block_height ?? block.height}`} className="text-[#CC0000] font-mono text-sm font-bold hover:underline">
          #{block.block_height ?? block.height}
        </Link>
      </td>
      <td className="px-4 py-3 font-mono text-xs text-white/50">{block.block_hash?.slice(0, 18) ?? '—'}...</td>
      <td className="px-4 py-3 text-xs text-white/50">{block.validator?.slice(0, 14) ?? 'genesis'}...</td>
      <td className="px-4 py-3 text-sm text-white/70">{block.tx_count ?? 0} txs</td>
      <td className="px-4 py-3 text-xs text-white/40">
        {block.timestamp ? new Date(block.timestamp * 1000).toLocaleTimeString() : 'just now'}
      </td>
      <td className="px-4 py-3">
        <span className="text-yellow-400 text-xs">{block.ai_score?.toFixed ? (block.ai_score * 100).toFixed(0) + '%' : '—'}</span>
      </td>
    </tr>
  );
}

export default function HomePage() {
  const [status, setStatus]     = useState<any>(null);
  const [blocks, setBlocks]     = useState<any[]>([]);
  const [txs, setTxs]           = useState<any[]>([]);
  const [validators, setVals]   = useState<any[]>([]);
  const [coin, setCoin]         = useState<any>(null);
  const [search, setSearch]     = useState('');
  const [loading, setLoading]   = useState(true);
  const [tick, setTick]         = useState(0);

  const fetchAll = useCallback(async () => {
    try {
      const [sRes, bRes, tRes, vRes, cRes] = await Promise.allSettled([
        axios.get(`${API}/api/status`),
        axios.get(`${API}/api/blocks?limit=10`),
        axios.get(`${API}/api/transactions?limit=10`),
        axios.get(`${API}/api/validators`),
        axios.get(`${API}/api/coin`),
      ]);
      if (sRes.status === 'fulfilled') setStatus(sRes.value.data);
      if (bRes.status === 'fulfilled') setBlocks(bRes.value.data?.blocks ?? []);
      if (tRes.status === 'fulfilled') setTxs(tRes.value.data?.transactions ?? []);
      if (vRes.status === 'fulfilled') setVals(vRes.value.data?.validators ?? []);
      if (cRes.status === 'fulfilled') setCoin(cRes.value.data);
    } catch (_) {}
    setLoading(false);
  }, []);

  useEffect(() => { fetchAll(); }, [fetchAll]);
  useEffect(() => {
    const t = setInterval(() => { setTick(n => n + 1); fetchAll(); }, 5000);
    return () => clearInterval(t);
  }, [fetchAll]);

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    const q = search.trim();
    if (!q) return;
    if (q.startsWith('db1x')) window.location.href = `/wallet/${q}`;
    else if (q.startsWith('dvl_') || q.length === 64) window.location.href = `/tx/${q}`;
    else if (!isNaN(Number(q))) window.location.href = `/block/${q}`;
    else window.location.href = `/tx/${q}`;
  };

  return (
    <main className="min-h-screen bg-[#0D0D0D] text-white font-sans">
      {/* HEADER */}
      <header className="bg-[#111] border-b border-[#CC0000]/40 sticky top-0 z-50">
        <div className="max-w-7xl mx-auto px-6 py-3 flex items-center justify-between gap-4">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-full bg-[#CC0000] flex items-center justify-center text-white font-black text-sm">D</div>
            <span className="text-[#CC0000] font-black text-xl tracking-tight">DevilScan</span>
            <span className="text-white/20 text-xs">Explorer</span>
          </div>
          <nav className="hidden md:flex items-center gap-6 text-sm">
            {[['/', 'Home'], ['/validators', 'Validators'], ['/wallet', 'Wallet']].map(([href, label]) => (
              <Link key={href} href={href} className="text-white/50 hover:text-[#CC0000] transition-colors">{label}</Link>
            ))}
          </nav>
          <div className="flex items-center gap-2">
            <div className="w-2 h-2 rounded-full bg-green-400 animate-pulse"/>
            <span className="text-green-400 text-xs">Testnet Live</span>
          </div>
        </div>
      </header>

      {/* HERO SEARCH */}
      <section className="bg-gradient-to-b from-[#1a0000] to-[#0D0D0D] py-12 px-6">
        <div className="max-w-3xl mx-auto text-center">
          <h1 className="text-4xl font-black text-white mb-2">
            Devil<span className="text-[#CC0000]">Chain</span> Explorer
          </h1>
          <p className="text-white/40 mb-8">Search blocks, transactions, wallets on DevilChain testnet</p>
          <form onSubmit={handleSearch} className="flex gap-2">
            <input
              type="text"
              value={search}
              onChange={e => setSearch(e.target.value)}
              placeholder="Search by TX hash / wallet address (db1x...) / block height"
              className="flex-1 bg-[#1a1a1a] border border-[#330000] rounded-xl px-4 py-3 text-white placeholder-white/20 focus:border-[#CC0000] focus:outline-none text-sm"
            />
            <button type="submit"
              className="bg-[#CC0000] hover:bg-[#990000] text-white px-6 py-3 rounded-xl font-bold text-sm transition-colors">
              Search
            </button>
          </form>
        </div>
      </section>

      <div className="max-w-7xl mx-auto px-6 py-8 space-y-10">
        {/* COIN STATS */}
        <section>
          <h2 className="text-white/40 text-xs uppercase tracking-widest mb-4">DevilCoin (DVC/DVL)</h2>
          <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-7 gap-3">
            <StatCard label="Total Supply" value="1B DVC" sub="Fixed" />
            <StatCard label="Symbol"       value="DVC/DVL" sub="Native coin" color="#f59e0b" />
            <StatCard label="Decimals"     value="18" sub="Wei-style" color="#60a5fa" />
            <StatCard label="Block Reward" value="50 DVC" sub="Per block" color="#34d399" />
            <StatCard label="Gas Fee"      value="~0.01 DVC" sub="Per TX" color="#a78bfa" />
            <StatCard label="Min Stake"    value="100 DVC" sub="Validator" color="#fb923c" />
            <StatCard label="Consensus"    value="DHP" sub="PoS+μPoW+AI" color="#f472b6" />
          </div>
        </section>

        {/* NETWORK STATS */}
        <section>
          <h2 className="text-white/40 text-xs uppercase tracking-widest mb-4">Network Status</h2>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
            <StatCard label="Latest Block"  value={status?.chain_length ?? status?.latest_block ?? '—'} sub="Block height" />
            <StatCard label="Network"       value={status?.network ?? 'DevilChain Testnet'} sub={status?.chain_id ?? 'devl-testnet-1'} color="#60a5fa" />
            <StatCard label="Validators"    value={validators.length || status?.validator_count || '—'} sub="Active" color="#34d399" />
            <StatCard label="Total TXs"     value={status?.total_txs ?? txs.length} sub="All time" color="#f59e0b" />
          </div>
        </section>

        {/* BLOCKS + TXS */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* LATEST BLOCKS */}
          <section>
            <div className="flex justify-between items-center mb-4">
              <h2 className="text-white font-bold">Latest Blocks</h2>
              <span className="text-white/30 text-xs">auto-refresh 5s</span>
            </div>
            <div className="bg-[#151515] border border-[#2a0000] rounded-2xl overflow-hidden">
              {loading ? (
                <div className="p-8 text-center text-white/30">Loading blocks...</div>
              ) : blocks.length === 0 ? (
                <div className="p-8 text-center">
                  <p className="text-white/30 mb-2">No blocks indexed yet</p>
                  <p className="text-white/20 text-xs">Mining in progress... blocks will appear here</p>
                </div>
              ) : (
                <table className="w-full">
                  <thead><tr className="border-b border-[#2a0000]">
                    {['Height','Hash','Validator','TXs','Time','AI'].map(h => (
                      <th key={h} className="px-4 py-3 text-left text-white/30 text-xs">{h}</th>
                    ))}
                  </tr></thead>
                  <tbody>{blocks.map((b: any, i: number) => <BlockRow key={i} block={b} />)}</tbody>
                </table>
              )}
            </div>
          </section>

          {/* LATEST TXS */}
          <section>
            <div className="flex justify-between items-center mb-4">
              <h2 className="text-white font-bold">Latest Transactions</h2>
              <span className="text-white/30 text-xs">with gas fees</span>
            </div>
            <div className="bg-[#151515] border border-[#2a0000] rounded-2xl overflow-hidden">
              {loading ? (
                <div className="p-8 text-center text-white/30">Loading transactions...</div>
              ) : txs.length === 0 ? (
                <div className="p-8 text-center">
                  <p className="text-white/30 mb-2">No transactions yet</p>
                  <p className="text-white/20 text-xs">Send DVC to see transactions here</p>
                </div>
              ) : (
                <table className="w-full">
                  <thead><tr className="border-b border-[#2a0000]">
                    {['TX Hash','From','To','Amount','Fee','Status'].map(h => (
                      <th key={h} className="px-4 py-3 text-left text-white/30 text-xs">{h}</th>
                    ))}
                  </tr></thead>
                  <tbody>{txs.map((tx: any, i: number) => <TxRow key={i} tx={tx} />)}</tbody>
                </table>
              )}
            </div>
          </section>
        </div>

        {/* VALIDATORS */}
        <section>
          <h2 className="text-white font-bold mb-4">Active Validators</h2>
          <div className="bg-[#151515] border border-[#2a0000] rounded-2xl overflow-hidden">
            {validators.length === 0 ? (
              <div className="p-6 text-center text-white/30">No validators yet — stake 100 DVC to register</div>
            ) : (
              <table className="w-full">
                <thead><tr className="border-b border-[#2a0000]">
                  {['#','Address','Staked','Reputation','Blocks','Status'].map(h => (
                    <th key={h} className="px-4 py-3 text-left text-white/30 text-xs">{h}</th>
                  ))}
                </tr></thead>
                <tbody>
                  {validators.map((v: any, i: number) => (
                    <tr key={i} className="border-b border-[#1a0000] hover:bg-[#1e0000]">
                      <td className="px-4 py-3 text-white/30 text-sm">{i+1}</td>
                      <td className="px-4 py-3">
                        <Link href={`/wallet/${v.address}`} className="text-[#CC0000] font-mono text-xs hover:underline">
                          {v.address?.slice(0, 20)}...
                        </Link>
                      </td>
                      <td className="px-4 py-3 text-sm text-yellow-400">{v.staked_dvc} DVC</td>
                      <td className="px-4 py-3 text-sm">{v.reputation_score?.toFixed(2)}</td>
                      <td className="px-4 py-3 text-sm">{v.blocks_validated}</td>
                      <td className="px-4 py-3">
                        <span className={`px-2 py-0.5 rounded-full text-xs ${v.active ? 'bg-green-900/50 text-green-400' : 'bg-red-900/50 text-red-400'}`}>
                          {v.active ? 'Active' : 'Inactive'}
                        </span>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </div>
        </section>

        {/* TOKENOMICS PIE */}
        <section>
          <h2 className="text-white font-bold mb-4">DVC Tokenomics Allocation</h2>
          <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-7 gap-3">
            {[
              {label:'Mining', pct:35, color:'#CC0000'},
              {label:'Ecosystem', pct:20, color:'#f59e0b'},
              {label:'DAO Treasury', pct:15, color:'#60a5fa'},
              {label:'Team', pct:10, color:'#34d399'},
              {label:'Validators', pct:10, color:'#a78bfa'},
              {label:'Investors', pct:5, color:'#fb923c'},
              {label:'Community', pct:5, color:'#f472b6'},
            ].map(s => (
              <div key={s.label} className="bg-[#151515] border border-[#2a0000] rounded-2xl p-4 text-center">
                <div className="text-2xl font-black" style={{color: s.color}}>{s.pct}%</div>
                <div className="text-white/50 text-xs mt-1">{s.label}</div>
                <div className="mt-2 h-1 rounded-full" style={{background: s.color, width: `${s.pct * 2.8}%`, minWidth:'20%'}} />
              </div>
            ))}
          </div>
        </section>

        {/* FOOTER */}
        <footer className="border-t border-[#1a0000] pt-6 text-center text-white/20 text-xs">
          DevilChain Network Explorer — DVC/DVL — Testnet 2026 —&nbsp;
          <a href="https://github.com/david0154/DevilChain" className="text-[#CC0000] hover:underline" target="_blank">GitHub</a>
        </footer>
      </div>
    </main>
  );
}
