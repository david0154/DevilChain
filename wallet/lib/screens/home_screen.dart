import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import '../services/wallet_service.dart';
import '../services/node_service.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});
  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  final WalletService _wallet = WalletService();
  final NodeService   _node   = NodeService();
  double balance  = 0.0;
  double staked   = 0.0;
  bool   loading  = true;
  String address  = '';
  List<Map<String, dynamic>> txHistory = [];

  @override
  void initState() {
    super.initState();
    _loadWallet();
  }

  Future<void> _loadWallet() async {
    final addr = await _wallet.getAddress();
    final info = await _node.getWallet(addr);
    final txs  = await _node.getTransactions(addr);
    setState(() {
      address   = addr;
      balance   = (info['balance'] ?? 0.0).toDouble();
      staked    = (info['staked']  ?? 0.0).toDouble();
      txHistory = List<Map<String, dynamic>>.from(txs);
      loading   = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0D0D0D),
      body: SafeArea(
        child: loading
          ? const Center(child: CircularProgressIndicator(color: Color(0xFFCC0000)))
          : RefreshIndicator(
              color: const Color(0xFFCC0000),
              onRefresh: _loadWallet,
              child: ListView(
                padding: const EdgeInsets.all(20),
                children: [
                  // Header
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      const Text('DevilX Wallet',
                        style: TextStyle(color: Color(0xFFCC0000),
                          fontSize: 22, fontWeight: FontWeight.w900)),
                      IconButton(
                        icon: const Icon(Icons.settings_outlined, color: Colors.white54),
                        onPressed: () {}),
                    ],
                  ),
                  const SizedBox(height: 24),
                  // Balance Card
                  Container(
                    padding: const EdgeInsets.all(24),
                    decoration: BoxDecoration(
                      gradient: const LinearGradient(
                        colors: [Color(0xFF2a0000), Color(0xFF1a0000)],
                        begin: Alignment.topLeft, end: Alignment.bottomRight),
                      borderRadius: BorderRadius.circular(20),
                      border: Border.all(color: const Color(0xFFCC0000).withOpacity(0.3))),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text('Total Balance', style: TextStyle(color: Colors.white38, fontSize: 12)),
                        const SizedBox(height: 8),
                        Text('${balance.toStringAsFixed(4)} DVC',
                          style: const TextStyle(color: Colors.white,
                            fontSize: 32, fontWeight: FontWeight.w900)),
                        const SizedBox(height: 4),
                        Text('Staked: ${staked.toStringAsFixed(2)} DVC',
                          style: const TextStyle(color: Color(0xFFCC0000), fontSize: 13)),
                        const SizedBox(height: 16),
                        GestureDetector(
                          onTap: () {
                            Clipboard.setData(ClipboardData(text: address));
                            ScaffoldMessenger.of(context).showSnackBar(
                              const SnackBar(content: Text('Address copied!')));
                          },
                          child: Row(
                            children: [
                              Text(
                                address.length > 20
                                  ? '${address.substring(0, 10)}...${address.substring(address.length - 10)}'
                                  : address,
                                style: const TextStyle(color: Colors.white54,
                                  fontSize: 12, fontFamily: 'monospace')),
                              const SizedBox(width: 6),
                              const Icon(Icons.copy, color: Colors.white38, size: 14),
                            ],
                          ),
                        ),
                      ],
                    ),
                  ),
                  const SizedBox(height: 20),
                  // Action Buttons
                  Row(
                    children: [
                      _ActionBtn(icon: Icons.send, label: 'Send',
                        onTap: () => Navigator.pushNamed(context, '/send')),
                      const SizedBox(width: 12),
                      _ActionBtn(icon: Icons.account_balance, label: 'Stake',
                        onTap: () => Navigator.pushNamed(context, '/stake')),
                      const SizedBox(width: 12),
                      _ActionBtn(icon: Icons.how_to_vote, label: 'DAO',
                        onTap: () => Navigator.pushNamed(context, '/dao')),
                      const SizedBox(width: 12),
                      _ActionBtn(icon: Icons.chat_bubble_outline, label: 'Chat',
                        onTap: () => Navigator.pushNamed(context, '/chat')),
                    ],
                  ),
                  const SizedBox(height: 24),
                  // TX History
                  const Text('Recent Transactions',
                    style: TextStyle(color: Colors.white, fontSize: 16,
                      fontWeight: FontWeight.bold)),
                  const SizedBox(height: 12),
                  if (txHistory.isEmpty)
                    const Center(child: Text('No transactions yet',
                      style: TextStyle(color: Colors.white38)))
                  else
                    ...txHistory.map((tx) => _TxItem(tx: tx, myAddress: address)),
                ],
              ),
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
    return Expanded(
      child: GestureDetector(
        onTap: onTap,
        child: Container(
          padding: const EdgeInsets.symmetric(vertical: 14),
          decoration: BoxDecoration(
            color: const Color(0xFF1a1a1a),
            borderRadius: BorderRadius.circular(14),
            border: Border.all(color: const Color(0xFF330000))),
          child: Column(
            children: [
              Icon(icon, color: const Color(0xFFCC0000), size: 22),
              const SizedBox(height: 4),
              Text(label, style: const TextStyle(color: Colors.white70, fontSize: 11)),
            ],
          ),
        ),
      ),
    );
  }
}

class _TxItem extends StatelessWidget {
  final Map<String, dynamic> tx;
  final String myAddress;
  const _TxItem({required this.tx, required this.myAddress});
  @override
  Widget build(BuildContext context) {
    final isSent = tx['from'] == myAddress;
    return Container(
      margin: const EdgeInsets.only(bottom: 10),
      padding: const EdgeInsets.all(14),
      decoration: BoxDecoration(
        color: const Color(0xFF151515),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: const Color(0xFF2a0000))),
      child: Row(
        children: [
          Icon(isSent ? Icons.arrow_upward : Icons.arrow_downward,
            color: isSent ? const Color(0xFFCC0000) : Colors.green, size: 18),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(isSent ? 'Sent' : 'Received',
                  style: const TextStyle(color: Colors.white70, fontSize: 13)),
                Text(
                  isSent
                    ? 'To: ${(tx['to'] ?? '').toString().substring(0, 14)}...'
                    : 'From: ${(tx['from'] ?? '').toString().substring(0, 14)}...',
                  style: const TextStyle(color: Colors.white38, fontSize: 11,
                    fontFamily: 'monospace')),
              ],
            ),
          ),
          Text('${isSent ? "-" : "+"}${tx['amount']} DVC',
            style: TextStyle(
              color: isSent ? const Color(0xFFCC0000) : Colors.green,
              fontWeight: FontWeight.bold, fontSize: 14)),
        ],
      ),
    );
  }
}
