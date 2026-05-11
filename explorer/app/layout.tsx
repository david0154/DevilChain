import type { Metadata } from 'next';
import './globals.css';
export const metadata: Metadata = { title: 'DevilScan - DevilChain Explorer', description: 'Official blockchain explorer for DevilChain Network' };
export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (<html lang="en"><body>{children}</body></html>);
}
