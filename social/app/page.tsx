'use client';
import { useState } from 'react';

interface Post { id: number; author: string; content: string; likes: number; timestamp: string; }

const SAMPLE: Post[] = [
  { id: 1, author: 'db1xdev...', content: 'DevilChain testnet is live! #DevilChain #Web3', likes: 42, timestamp: '2m ago' },
  { id: 2, author: 'db1xbuilder...', content: 'Just deployed my first smart contract on DevilProtocol', likes: 17, timestamp: '15m ago' },
];

export default function DevilSocialPage() {
  const [posts, setPosts] = useState<Post[]>(SAMPLE);
  const [newPost, setNewPost] = useState('');
  const publish = () => {
    if (!newPost.trim()) return;
    setPosts([{ id: Date.now(), author: 'db1xme...', content: newPost, likes: 0, timestamp: 'now' }, ...posts]);
    setNewPost('');
  };
  return (
    <main className="min-h-screen bg-[#0D0D0D] text-white">
      <header className="bg-[#1A1A1A] border-b border-[#CC0000] px-6 py-4">
        <span className="text-2xl font-bold text-[#CC0000]">DevilSocial</span>
        <span className="text-white/40 text-sm ml-3">Web3 Social on DevilChain</span>
      </header>
      <div className="max-w-2xl mx-auto px-4 py-6">
        <div className="bg-[#1A1A1A] border border-[#330000] rounded-xl p-4 mb-6">
          <textarea value={newPost} onChange={e => setNewPost(e.target.value)}
            placeholder="What's happening on DevilChain?"
            className="w-full bg-transparent text-white placeholder-white/30 resize-none focus:outline-none" rows={3} />
          <div className="flex justify-end mt-2">
            <button onClick={publish} className="bg-[#CC0000] px-4 py-2 rounded-lg font-bold text-sm">Post</button>
          </div>
        </div>
        {posts.map(p => (
          <div key={p.id} className="bg-[#1A1A1A] border border-[#220000] rounded-xl p-4 mb-4">
            <div className="flex items-center gap-2 mb-2">
              <div className="w-8 h-8 rounded-full bg-[#CC0000] flex items-center justify-center text-xs font-bold">D</div>
              <span className="text-white/70 text-sm font-mono">{p.author}</span>
              <span className="text-white/30 text-xs ml-auto">{p.timestamp}</span>
            </div>
            <p className="text-white/90 mb-3">{p.content}</p>
            <div className="flex gap-4 text-white/40 text-sm">
              <button className="hover:text-[#CC0000]">❤ {p.likes}</button>
              <button className="hover:text-[#CC0000]">Reply</button>
              <button className="hover:text-[#CC0000]">Repost</button>
            </div>
          </div>
        ))}
      </div>
    </main>
  );
}
