import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../providers/wallet_provider.dart';
import '../services/devilchain_service.dart';

class StakingScreen extends StatefulWidget {
  const StakingScreen({super.key});
  @override
  State<StakingScreen> createState() => _StakingScreenState();
}

class _StakingScreenState extends State<StakingScreen> {
  final _amountCtrl = TextEditingController();
  bool _staking = false;

  Future<void> _stake() async {
    final wallet = context.read<WalletProvider>();
    if (wallet.address == null) return;
    setState(() => _staking = true);
    try {
      await DevilChainService().stake(wallet.address!, double.parse(_amountCtrl.text));
      if (mounted) ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Staked successfully!'), backgroundColor: Color(0xFFCC0000)));
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Error: $e')));
    }
    setState(() => _staking = false);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0D0D0D),
      appBar: AppBar(backgroundColor: const Color(0xFF1A1A1A), title: const Text('Stake DVC', style: TextStyle(color: Color(0xFFCC0000)))),
      body: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text('Minimum Stake: 100 DVC', style: TextStyle(color: Colors.white60)),
            const SizedBox(height: 16),
            TextField(
              controller: _amountCtrl,
              keyboardType: TextInputType.number,
              style: const TextStyle(color: Colors.white),
              decoration: const InputDecoration(
                labelText: 'Amount to Stake (DVC)',
                labelStyle: TextStyle(color: Colors.white38),
                enabledBorder: OutlineInputBorder(borderSide: BorderSide(color: Color(0xFFCC0000))),
                focusedBorder: OutlineInputBorder(borderSide: BorderSide(color: Colors.red)),
              ),
            ),
            const SizedBox(height: 24),
            SizedBox(
              width: double.infinity,
              child: ElevatedButton(
                onPressed: _staking ? null : _stake,
                style: ElevatedButton.styleFrom(backgroundColor: const Color(0xFFCC0000), padding: const EdgeInsets.symmetric(vertical: 16)),
                child: _staking ? const CircularProgressIndicator(color: Colors.white) : const Text('Stake DVC', style: TextStyle(fontSize: 18, color: Colors.white)),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
