import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import './globals.css';

const inter = Inter({ subsets: ['latin'] });

export const metadata: Metadata = {
  title: 'DevilScan — DevilChain Explorer',
  description: 'Blockchain explorer for DevilChain Network. Search blocks, transactions, wallets, validators, DVC coin.',
  keywords: 'DevilChain, DVC, DVL, DevilCoin, blockchain explorer, cryptocurrency',
  openGraph: {
    title: 'DevilScan — DevilChain Explorer',
    description: 'Explore the DevilChain blockchain network',
    type: 'website',
  },
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body className={inter.className}>{children}</body>
    </html>
  );
}
