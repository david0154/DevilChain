import 'package:flutter/material.dart';
import '../services/wallet_service.dart';
import '../services/node_service.dart';

class SendScreen extends StatefulWidget {
  const SendScreen({super.key});
  @override
  State<SendScreen> createState() => _SendScreenState();
}

class _SendScreenState extends State<SendScreen> {
  final _toCtrl     = TextEditingController();
  final _amountCtrl = TextEditingController();
  bool _sending = false;
  String _status = '';

  Future<void> _send() async {
    final to     = _toCtrl.text.trim();
    final amount = double.tryParse(_amountCtrl.text.trim()) ?? 0;
    if (to.isEmpty || amount <= 0) return;
    setState(() { _sending = true; _status = 'Signing transaction...'; });

    final wallet = WalletService();
    final node   = NodeService();
    final from   = await wallet.getAddress();
    final sig    = await wallet.sign('{"from":"$from","to":"$to","amount":$amount,"gas_fee":0.01}');
    final result = await node.sendTransaction(
      from: from, to: to, amount: amount, gasFee: 0.01, signature: sig);

    setState(() {
      _sending = false;
      _status  = result.containsKey('tx_hash')
        ? '✅ TX sent: ${result["tx_hash"]}'
        : '❌ Error: ${result["error"] ?? "unknown"}';
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0D0D0D),
      appBar: AppBar(
        backgroundColor: const Color(0xFF111111),
        title: const Text('Send DVC', style: TextStyle(color: Color(0xFFCC0000))),
        iconTheme: const IconThemeData(color: Colors.white)),
      body: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          children: [
            _Field(controller: _toCtrl,     label: 'Recipient address (db1x...)', hint: 'db1x...'),
            const SizedBox(height: 16),
            _Field(controller: _amountCtrl, label: 'Amount (DVC)', hint: '0.00',
              keyboardType: TextInputType.number),
            const SizedBox(height: 8),
            const Text('Gas fee: 0.01 DVC (fixed)',
              style: TextStyle(color: Colors.white38, fontSize: 12)),
            const SizedBox(height: 24),
            SizedBox(
              width: double.infinity,
              child: ElevatedButton(
                style: ElevatedButton.styleFrom(
                  backgroundColor: const Color(0xFFCC0000),
                  padding: const EdgeInsets.symmetric(vertical: 16),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(14))),
                onPressed: _sending ? null : _send,
                child: _sending
                  ? const CircularProgressIndicator(color: Colors.white)
                  : const Text('Send DVC', style: TextStyle(color: Colors.white,
                      fontSize: 16, fontWeight: FontWeight.bold)),
              ),
            ),
            if (_status.isNotEmpty) ...[
              const SizedBox(height: 16),
              Text(_status, style: const TextStyle(color: Colors.white70, fontSize: 13)),
            ],
          ],
        ),
      ),
    );
  }
}

class _Field extends StatelessWidget {
  final TextEditingController controller;
  final String label, hint;
  final TextInputType? keyboardType;
  const _Field({required this.controller, required this.label,
    required this.hint, this.keyboardType});
  @override
  Widget build(BuildContext context) {
    return TextField(
      controller: controller,
      keyboardType: keyboardType,
      style: const TextStyle(color: Colors.white),
      decoration: InputDecoration(
        labelText: label,
        hintText: hint,
        labelStyle: const TextStyle(color: Colors.white54),
        hintStyle: const TextStyle(color: Colors.white24),
        filled: true, fillColor: const Color(0xFF1a1a1a),
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: const BorderSide(color: Color(0xFF330000))),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: const BorderSide(color: Color(0xFF330000))),
        focusedBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: const BorderSide(color: Color(0xFFCC0000)))),
    );
  }
}
