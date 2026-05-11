import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../providers/wallet_provider.dart';
import '../providers/chain_provider.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});
  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  @override
  void initState() {
    super.initState();
    Future.microtask(() {
      context.read<WalletProvider>().loadWallet();
      context.read<ChainProvider>().refresh();
    });
  }

  @override
  Widget build(BuildContext context) {
    final wallet = context.watch<WalletProvider>();
    final chain = context.watch<ChainProvider>();
    return Scaffold(
      appBar: AppBar(
        backgroundColor: const Color(0xFF1A1A1A),
        title: Row(
          children: [
            Image.asset('assets/logo.png', height: 32),
            const SizedBox(width: 8),
            const Text('DevilX Wallet', style: TextStyle(color: Color(0xFFCC0000), fontWeight: FontWeight.bold)),
          ],
        ),
        actions: [
          IconButton(icon: const Icon(Icons.settings, color: Colors.white), onPressed: () => Navigator.pushNamed(context, '/settings')),
        ],
      ),
      backgroundColor: const Color(0xFF0D0D0D),
      body: RefreshIndicator(
        onRefresh: () async {
          await wallet.refreshBalance();
          await chain.refresh();
        },
        child: ListView(
          padding: const EdgeInsets.all(16),
          children: [
            // Balance Card
            Container(
              padding: const EdgeInsets.all(24),
              decoration: BoxDecoration(
                gradient: const LinearGradient(colors: [Color(0xFF1A0000), Color(0xFF330000)]),
                borderRadius: BorderRadius.circular(16),
                border: Border.all(color: const Color(0xFFCC0000), width: 1),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('Total Balance', style: TextStyle(color: Colors.white60, fontSize: 14)),
                  const SizedBox(height: 8),
                  Text('${wallet.balance.toStringAsFixed(4)} DVC',
                      style: const TextStyle(color: Colors.white, fontSize: 32, fontWeight: FontWeight.bold)),
                  const SizedBox(height: 4),
                  Text(wallet.address ?? 'No wallet', style: const TextStyle(color: Colors.white38, fontSize: 11)),
                ],
              ),
            ),
            const SizedBox(height: 16),
            // Action Buttons
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceEvenly,
              children: [
                _ActionBtn(icon: Icons.send, label: 'Send', onTap: () => Navigator.pushNamed(context, '/send')),
                _ActionBtn(icon: Icons.qr_code, label: 'Receive', onTap: () => Navigator.pushNamed(context, '/receive')),
                _ActionBtn(icon: Icons.savings, label: 'Stake', onTap: () => Navigator.pushNamed(context, '/staking')),
                _ActionBtn(icon: Icons.how_to_vote, label: 'DAO', onTap: () => Navigator.pushNamed(context, '/dao')),
              ],
            ),
            const SizedBox(height: 24),
            // Network Status
            if (chain.status != null) ...
              [
                const Text('Network Status', style: TextStyle(color: Colors.white70, fontSize: 16, fontWeight: FontWeight.bold)),
                const SizedBox(height: 8),
                _StatusRow('Chain', chain.status!['network'] ?? 'DevilChain'),
                _StatusRow('Latest Block', '${chain.latestBlock?['block_height'] ?? '-'}'),
                _StatusRow('TPS Target', '5,000–20,000'),
              ],
            const SizedBox(height: 24),
            // Recent transactions
            const Text('Recent Transactions', style: TextStyle(color: Colors.white70, fontSize: 16, fontWeight: FontWeight.bold)),
            const SizedBox(height: 8),
            if (wallet.transactions.isEmpty)
              const Center(child: Text('No transactions yet', style: TextStyle(color: Colors.white38)))
            else
              ...wallet.transactions.map((tx) => _TxTile(tx: tx)),
          ],
        ),
      ),
    );
  }
}

class _ActionBtn extends StatelessWidget {
  final IconData icon;
  final String label;
  final VoidCallback onTap;
  const _ActionBtn({required this.icon, required this.label, required this.onTap});
  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap,
      child: Column(
        children: [
          Container(
            padding: const EdgeInsets.all(14),
            decoration: BoxDecoration(
              color: const Color(0xFF1A0000),
              borderRadius: BorderRadius.circular(12),
              border: Border.all(color: const Color(0xFFCC0000)),
            ),
            child: Icon(icon, color: const Color(0xFFCC0000), size: 24),
          ),
          const SizedBox(height: 6),
          Text(label, style: const TextStyle(color: Colors.white60, fontSize: 12)),
        ],
      ),
    );
  }
}

Widget _StatusRow(String label, String value) {
  return Padding(
    padding: const EdgeInsets.symmetric(vertical: 4),
    child: Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        Text(label, style: const TextStyle(color: Colors.white38)),
        Text(value, style: const TextStyle(color: Colors.white70)),
      ],
    ),
  );
}

class _TxTile extends StatelessWidget {
  final Map<String, dynamic> tx;
  const _TxTile({required this.tx});
  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: const Icon(Icons.swap_horiz, color: Color(0xFFCC0000)),
      title: Text('${tx['amount']} DVC', style: const TextStyle(color: Colors.white)),
      subtitle: Text(tx['tx_hash'] ?? '', style: const TextStyle(color: Colors.white38, fontSize: 11)),
      trailing: Text('${tx['gas_fee']} DVC', style: const TextStyle(color: Colors.white60, fontSize: 11)),
    );
  }
}
