'use client';
import { useState, useEffect } from 'react';
import axios from 'axios';

const API = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8545';

interface Post {
  id: string; author: string; content: string;
  likes: number; timestamp: number; tx_hash?: string;
}

export default function DevilSocialPage() {
  const [posts,   setPosts]   = useState<Post[]>([]);
  const [content, setContent] = useState('');
  const [myAddr,  setMyAddr]  = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const addr = localStorage.getItem('devil_address') ||
      'db1x' + Math.random().toString(16).substring(2, 34).padEnd(32, '0');
    localStorage.setItem('devil_address', addr);
    setMyAddr(addr);
    loadPosts();
  }, []);

  const loadPosts = async () => {
    try {
      const r = await axios.get(`${API}/api/social/posts?limit=20`);
      setPosts(r.data?.posts ?? []);
    } catch {
      // demo posts
      setPosts([
        { id: '1', author: 'db1xgenesis...', content: 'DevilChain testnet is live! 🔥 First block mined. #DevilChain #Web3',
          likes: 42, timestamp: Date.now() - 3600000 },
        { id: '2', author: 'db1xval_001...', content: 'Just staked 500 DVC as a validator. Earning block rewards ⚡ #DVC #DevilChain',
          likes: 18, timestamp: Date.now() - 1800000 },
        { id: '3', author: 'db1xdao_001...', content: 'DAO Proposal #1 is live — vote to increase block reward to 75 DVC 🗳️',
          likes: 31, timestamp: Date.now() - 900000 },
      ]);
    }
  };

  const createPost = async () => {
    if (!content.trim()) return;
    setLoading(true);
    const newPost: Post = {
      id: Date.now().toString(), author: myAddr,
      content, likes: 0, timestamp: Date.now(),
    };
    try {
      await axios.post(`${API}/api/social/post`, { author: myAddr, content });
    } catch {}
    setPosts(prev => [newPost, ...prev]);
    setContent('');
    setLoading(false);
  };

  const likePost = (id: string) => {
    setPosts(prev => prev.map(p => p.id === id ? { ...p, likes: p.likes + 1 } : p));
  };

  return (
    <div className="min-h-screen bg-[#0D0D0D] text-white">
      <header className="bg-[#111] border-b border-[#CC0000]/30 sticky top-0 z-50">
        <div className="max-w-2xl mx-auto px-4 py-4 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-full bg-[#CC0000] flex items-center justify-center font-black text-sm">D</div>
            <div>
              <h1 className="font-black text-[#CC0000]">DevilSocial</h1>
              <p className="text-white/30 text-xs">Decentralized Web3 Social</p>
            </div>
          </div>
          <div className="text-white/30 font-mono text-xs">{myAddr.slice(0, 16)}...</div>
        </div>
      </header>

      <div className="max-w-2xl mx-auto px-4 py-6 space-y-4">
        {/* Compose */}
        <div className="bg-[#151515] border border-[#2a0000] rounded-2xl p-4">
          <textarea
            value={content} onChange={e => setContent(e.target.value)}
            placeholder="What's happening on DevilChain? (Posted on-chain, permanent 🔥)"
            rows={3}
            className="w-full bg-transparent text-white placeholder-white/20 resize-none focus:outline-none text-sm"
          />
          <div className="flex justify-between items-center mt-3">
            <span className="text-white/20 text-xs">🔒 Signed with wallet key · Gas: 0.01 DVC</span>
            <button onClick={createPost} disabled={loading}
              className="bg-[#CC0000] hover:bg-[#990000] text-white px-5 py-2 rounded-xl text-sm font-bold transition-colors disabled:opacity-50">
              {loading ? 'Posting...' : 'Post'}
            </button>
          </div>
        </div>

        {/* Feed */}
        {posts.map(post => (
          <div key={post.id} className="bg-[#151515] border border-[#2a0000] rounded-2xl p-5">
            <div className="flex items-start gap-3">
              <div className="w-9 h-9 rounded-full bg-[#CC0000]/20 border border-[#CC0000]/30 flex items-center justify-center flex-shrink-0">
                <span className="text-[#CC0000] text-xs">👿</span>
              </div>
              <div className="flex-1">
                <div className="flex justify-between items-start">
                  <p className="text-white/60 font-mono text-xs">{post.author.slice(0, 20)}...</p>
                  <p className="text-white/20 text-xs">{new Date(post.timestamp).toLocaleTimeString()}</p>
                </div>
                <p className="text-white mt-2 text-sm leading-relaxed">{post.content}</p>
                <div className="flex items-center gap-6 mt-3">
                  <button onClick={() => likePost(post.id)}
                    className="flex items-center gap-1 text-white/30 hover:text-[#CC0000] transition-colors text-xs">
                    <span>❤️</span> <span>{post.likes}</span>
                  </button>
                  {post.tx_hash && (
                    <span className="text-white/20 text-xs font-mono">
                      TX: {post.tx_hash.slice(0, 16)}...
                    </span>
                  )}
                  <span className="text-green-400/50 text-xs">🔗 On-chain</span>
                </div>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
