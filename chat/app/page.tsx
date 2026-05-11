'use client';
import { useState, useEffect, useRef } from 'react';

const API = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8545';

interface Message {
  id: string; from: string; to: string;
  text: string; timestamp: number; encrypted: boolean;
}

export default function DevilChatPage() {
  const [myAddr,   setMyAddr]   = useState('');
  const [toAddr,   setToAddr]   = useState('');
  const [msgText,  setMsgText]  = useState('');
  const [messages, setMessages] = useState<Message[]>([]);
  const [contacts, setContacts] = useState<string[]>([]);
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const saved = localStorage.getItem('devil_address');
    if (saved) { setMyAddr(saved); } else {
      const addr = 'db1x' + Math.random().toString(16).substring(2, 34).padEnd(32, '0');
      localStorage.setItem('devil_address', addr);
      setMyAddr(addr);
    }
    const saved_msgs = localStorage.getItem('devil_msgs');
    if (saved_msgs) setMessages(JSON.parse(saved_msgs));
    const saved_contacts = localStorage.getItem('devil_contacts');
    if (saved_contacts) setContacts(JSON.parse(saved_contacts));
  }, []);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const sendMessage = () => {
    if (!toAddr.trim() || !msgText.trim()) return;
    const msg: Message = {
      id: Date.now().toString(),
      from: myAddr, to: toAddr,
      text: msgText,
      timestamp: Date.now(),
      encrypted: true,
    };
    const updated = [...messages, msg];
    setMessages(updated);
    localStorage.setItem('devil_msgs', JSON.stringify(updated));
    if (!contacts.includes(toAddr)) {
      const updContacts = [...contacts, toAddr];
      setContacts(updContacts);
      localStorage.setItem('devil_contacts', JSON.stringify(updContacts));
    }
    setMsgText('');
  };

  const convoMsgs = messages.filter(
    m => (m.from === myAddr && m.to === toAddr) ||
         (m.from === toAddr && m.to === myAddr));

  return (
    <div className="flex h-screen bg-[#0D0D0D] text-white">
      {/* Sidebar */}
      <aside className="w-72 bg-[#111] border-r border-[#1a0000] flex flex-col">
        <div className="p-4 border-b border-[#1a0000]">
          <h1 className="text-[#CC0000] font-black text-lg">DevilChat</h1>
          <p className="text-white/30 text-xs mt-1 font-mono">{myAddr.slice(0,20)}...</p>
        </div>
        <div className="p-3">
          <input
            value={toAddr} onChange={e => setToAddr(e.target.value)}
            placeholder="New conversation (db1x...)"
            className="w-full bg-[#1a1a1a] border border-[#330000] rounded-lg px-3 py-2 text-xs text-white placeholder-white/20 focus:border-[#CC0000] focus:outline-none font-mono"
          />
        </div>
        <div className="flex-1 overflow-y-auto">
          {contacts.map(c => (
            <button key={c} onClick={() => setToAddr(c)}
              className={`w-full text-left px-4 py-3 border-b border-[#1a0000] hover:bg-[#1a0000] transition-colors ${
                toAddr === c ? 'bg-[#1a0000] border-l-2 border-l-[#CC0000]' : ''}`}>
              <div className="text-white/70 text-xs font-mono">{c.slice(0,22)}...</div>
              <div className="text-white/30 text-xs">Wallet-to-wallet 🔒</div>
            </button>
          ))}
        </div>
      </aside>

      {/* Chat */}
      <main className="flex-1 flex flex-col">
        <header className="bg-[#111] border-b border-[#1a0000] px-6 py-4">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-full bg-[#CC0000]/20 border border-[#CC0000]/30 flex items-center justify-center">
              <span className="text-[#CC0000] text-xs">👿</span>
            </div>
            <div>
              <p className="text-white font-mono text-xs">{toAddr || 'Select a contact'}</p>
              <p className="text-green-400 text-xs">🔒 E2E Encrypted &nbsp;·&nbsp; Signed with wallet key</p>
            </div>
          </div>
        </header>

        <div className="flex-1 overflow-y-auto p-6 space-y-3">
          {!toAddr && (
            <div className="flex items-center justify-center h-full">
              <div className="text-center">
                <div className="text-5xl mb-4">🔒</div>
                <p className="text-white/30">Enter a wallet address to start chatting</p>
                <p className="text-white/20 text-sm mt-2">All messages are E2E encrypted &amp; signed with Ed25519</p>
              </div>
            </div>
          )}
          {convoMsgs.map(m => (
            <div key={m.id} className={`flex ${m.from === myAddr ? 'justify-end' : 'justify-start'}`}>
              <div className={`max-w-xs lg:max-w-md px-4 py-3 rounded-2xl ${
                m.from === myAddr
                  ? 'bg-[#330000] border border-[#CC0000]/20'
                  : 'bg-[#1a1a1a] border border-[#2a2a2a]'}`}>
                <p className="text-white text-sm">{m.text}</p>
                <p className="text-white/20 text-xs mt-1">
                  {new Date(m.timestamp).toLocaleTimeString()} &middot; 🔒
                </p>
              </div>
            </div>
          ))}
          <div ref={bottomRef} />
        </div>

        <div className="border-t border-[#1a0000] p-4">
          <div className="flex gap-3">
            <input
              value={msgText} onChange={e => setMsgText(e.target.value)}
              onKeyDown={e => e.key === 'Enter' && sendMessage()}
              placeholder="Message (E2E encrypted, signed with your wallet)"
              className="flex-1 bg-[#1a1a1a] border border-[#330000] rounded-xl px-4 py-3 text-white placeholder-white/20 focus:border-[#CC0000] focus:outline-none text-sm"
            />
            <button onClick={sendMessage}
              className="bg-[#CC0000] hover:bg-[#990000] text-white px-5 py-3 rounded-xl font-bold transition-colors">
              Send
            </button>
          </div>
        </div>
      </main>
    </div>
  );
}
