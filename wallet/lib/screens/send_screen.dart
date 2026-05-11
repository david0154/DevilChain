import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../providers/wallet_provider.dart';

class SendScreen extends StatefulWidget {
  const SendScreen({super.key});
  @override
  State<SendScreen> createState() => _SendScreenState();
}

class _SendScreenState extends State<SendScreen> {
  final _toCtrl = TextEditingController();
  final _amountCtrl = TextEditingController();
  bool _sending = false;

  Future<void> _send() async {
    final wallet = context.read<WalletProvider>();
    setState(() => _sending = true);
    try {
      await wallet.sendDVC(_toCtrl.text.trim(), double.parse(_amountCtrl.text), 0.01);
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('Transaction sent!'), backgroundColor: Color(0xFFCC0000)));
        Navigator.pop(context);
      }
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Error: $e'), backgroundColor: Colors.red));
    }
    setState(() => _sending = false);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0D0D0D),
      appBar: AppBar(
        backgroundColor: const Color(0xFF1A1A1A),
        title: const Text('Send DVC', style: TextStyle(color: Color(0xFFCC0000))),
      ),
      body: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          children: [
            TextField(
              controller: _toCtrl,
              style: const TextStyle(color: Colors.white),
              decoration: const InputDecoration(
                labelText: 'Recipient Address (db1x...)',
                labelStyle: TextStyle(color: Colors.white38),
                enabledBorder: OutlineInputBorder(borderSide: BorderSide(color: Color(0xFFCC0000))),
                focusedBorder: OutlineInputBorder(borderSide: BorderSide(color: Colors.red)),
              ),
            ),
            const SizedBox(height: 16),
            TextField(
              controller: _amountCtrl,
              keyboardType: TextInputType.number,
              style: const TextStyle(color: Colors.white),
              decoration: const InputDecoration(
                labelText: 'Amount (DVC)',
                labelStyle: TextStyle(color: Colors.white38),
                enabledBorder: OutlineInputBorder(borderSide: BorderSide(color: Color(0xFFCC0000))),
                focusedBorder: OutlineInputBorder(borderSide: BorderSide(color: Colors.red)),
              ),
            ),
            const SizedBox(height: 8),
            const Text('Gas Fee: 0.01 DVC', style: TextStyle(color: Colors.white38)),
            const SizedBox(height: 32),
            SizedBox(
              width: double.infinity,
              child: ElevatedButton(
                onPressed: _sending ? null : _send,
                style: ElevatedButton.styleFrom(backgroundColor: const Color(0xFFCC0000), padding: const EdgeInsets.symmetric(vertical: 16)),
                child: _sending ? const CircularProgressIndicator(color: Colors.white) : const Text('Send DVC', style: TextStyle(fontSize: 18, color: Colors.white)),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
