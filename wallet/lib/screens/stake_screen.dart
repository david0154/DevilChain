import 'package:flutter/material.dart';
import '../services/wallet_service.dart';
import '../services/node_service.dart';

class StakeScreen extends StatefulWidget {
  const StakeScreen({super.key});
  @override
  State<StakeScreen> createState() => _StakeScreenState();
}

class _StakeScreenState extends State<StakeScreen> with SingleTickerProviderStateMixin {
  late TabController _tabs;
  final _amtCtrl = TextEditingController();
  bool _loading = false;
  String _status = '';

  @override
  void initState() { super.initState(); _tabs = TabController(length: 2, vsync: this); }

  Future<void> _stake(bool isUnstake) async {
    final amount = double.tryParse(_amtCtrl.text.trim()) ?? 0;
    if (amount <= 0) return;
    setState(() { _loading = true; _status = ''; });
    final wallet  = WalletService();
    final node    = NodeService();
    final address = await wallet.getAddress();
    final sig     = await wallet.sign('{"address":"$address","amount":$amount}');
    final result  = isUnstake
      ? await node.stake(address, amount, sig)
      : await node.stake(address, amount, sig);
    setState(() {
      _loading = false;
      _status  = result.containsKey('error') ? '❌ ${result["error"]}' : '✅ Done!';
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0D0D0D),
      appBar: AppBar(
        backgroundColor: const Color(0xFF111111),
        title: const Text('Staking', style: TextStyle(color: Color(0xFFCC0000))),
        iconTheme: const IconThemeData(color: Colors.white),
        bottom: TabBar(
          controller: _tabs,
          indicatorColor: const Color(0xFFCC0000),
          labelColor: const Color(0xFFCC0000),
          unselectedLabelColor: Colors.white38,
          tabs: const [Tab(text: 'Stake'), Tab(text: 'Unstake')]),
      ),
      body: TabBarView(
        controller: _tabs,
        children: [_StakeForm(ctrl: _amtCtrl, loading: _loading,
            status: _status, onAction: () => _stake(false), label: 'Stake DVC'),
          _StakeForm(ctrl: _amtCtrl, loading: _loading,
            status: _status, onAction: () => _stake(true), label: 'Unstake DVC'),
        ],
      ),
    );
  }
}

class _StakeForm extends StatelessWidget {
  final TextEditingController ctrl;
  final bool loading;
  final String status;
  final VoidCallback onAction;
  final String label;
  const _StakeForm({required this.ctrl, required this.loading,
    required this.status, required this.onAction, required this.label});
  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(20),
      child: Column(
        children: [
          const SizedBox(height: 12),
          const Text('Minimum stake: 100 DVC',
            style: TextStyle(color: Colors.white38, fontSize: 12)),
          const SizedBox(height: 16),
          TextField(
            controller: ctrl,
            keyboardType: TextInputType.number,
            style: const TextStyle(color: Colors.white),
            decoration: InputDecoration(
              labelText: 'Amount (DVC)', hintText: '100.00',
              labelStyle: const TextStyle(color: Colors.white54),
              hintStyle: const TextStyle(color: Colors.white24),
              filled: true, fillColor: const Color(0xFF1a1a1a),
              border: OutlineInputBorder(borderRadius: BorderRadius.circular(12),
                borderSide: const BorderSide(color: Color(0xFF330000))),
              enabledBorder: OutlineInputBorder(borderRadius: BorderRadius.circular(12),
                borderSide: const BorderSide(color: Color(0xFF330000))),
              focusedBorder: OutlineInputBorder(borderRadius: BorderRadius.circular(12),
                borderSide: const BorderSide(color: Color(0xFFCC0000))))),
          const SizedBox(height: 24),
          SizedBox(
            width: double.infinity,
            child: ElevatedButton(
              style: ElevatedButton.styleFrom(
                backgroundColor: const Color(0xFFCC0000),
                padding: const EdgeInsets.symmetric(vertical: 16),
                shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(14))),
              onPressed: loading ? null : onAction,
              child: loading
                ? const CircularProgressIndicator(color: Colors.white)
                : Text(label, style: const TextStyle(color: Colors.white,
                    fontSize: 16, fontWeight: FontWeight.bold)))),
          if (status.isNotEmpty) ...[
            const SizedBox(height: 16),
            Text(status, style: const TextStyle(color: Colors.white70))],
        ],
      ),
    );
  }
}
