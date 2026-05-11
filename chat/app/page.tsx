'use client';
import { useState, useRef, useEffect } from 'react';

interface Message { id: number; from: string; text: string; time: string; }

export default function DevilChatPage() {
  const [messages, setMessages] = useState<Message[]>([
    { id: 1, from: 'db1xalice...', text: 'Hello from DevilChain!', time: '12:00' },
  ]);
  const [input, setInput] = useState('');
  const endRef = useRef<HTMLDivElement>(null);
  useEffect(() => { endRef.current?.scrollIntoView({ behavior: 'smooth' }); }, [messages]);
  const send = () => {
    if (!input.trim()) return;
    setMessages(p => [...p, { id: Date.now(), from: 'db1xme...', text: input, time: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }) }]);
    setInput('');
  };
  return (
    <main className="min-h-screen bg-[#0D0D0D] text-white flex flex-col">
      <header className="bg-[#1A1A1A] border-b border-[#CC0000] px-6 py-4">
        <span className="text-xl font-bold text-[#CC0000]">DevilChat</span>
        <span className="text-white/40 text-sm ml-3">E2E Encrypted · Wallet-to-Wallet</span>
      </header>
      <div className="flex-1 overflow-y-auto p-4 space-y-3">
        {messages.map(m => (
          <div key={m.id} className={`flex ${m.from === 'db1xme...' ? 'justify-end' : 'justify-start'}`}>
            <div className={`max-w-xs rounded-xl px-4 py-2 ${m.from === 'db1xme...' ? 'bg-[#CC0000]' : 'bg-[#1A1A1A]'}`}>
              <p className="text-xs text-white/50 mb-1 font-mono">{m.from}</p>
              <p>{m.text}</p>
              <p className="text-xs text-white/40 mt-1">{m.time} 🔒</p>
            </div>
          </div>
        ))}
        <div ref={endRef} />
      </div>
      <div className="border-t border-[#220000] p-4 flex gap-2">
        <input value={input} onChange={e => setInput(e.target.value)} onKeyDown={e => e.key === 'Enter' && send()}
          placeholder="Type an encrypted message..."
          className="flex-1 bg-[#1A1A1A] border border-[#330000] rounded-lg px-4 py-2 text-white focus:outline-none" />
        <button onClick={send} className="bg-[#CC0000] px-4 py-2 rounded-lg font-bold">Send</button>
      </div>
    </main>
  );
}
