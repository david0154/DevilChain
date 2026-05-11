import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';
import 'package:qr_flutter/qr_flutter.dart';
import '../providers/wallet_provider.dart';

class ReceiveScreen extends StatelessWidget {
  const ReceiveScreen({super.key});
  @override
  Widget build(BuildContext context) {
    final wallet = context.watch<WalletProvider>();
    final address = wallet.address ?? 'No wallet';
    return Scaffold(
      backgroundColor: const Color(0xFF0D0D0D),
      appBar: AppBar(
        backgroundColor: const Color(0xFF1A1A1A),
        title: const Text('Receive DVC', style: TextStyle(color: Color(0xFFCC0000))),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            QrImageView(data: address, size: 220, backgroundColor: Colors.white),
            const SizedBox(height: 24),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 24),
              child: Text(address, style: const TextStyle(color: Colors.white70, fontSize: 13), textAlign: TextAlign.center),
            ),
            const SizedBox(height: 16),
            ElevatedButton.icon(
              icon: const Icon(Icons.copy, color: Colors.white),
              label: const Text('Copy Address', style: TextStyle(color: Colors.white)),
              style: ElevatedButton.styleFrom(backgroundColor: const Color(0xFFCC0000)),
              onPressed: () {
                Clipboard.setData(ClipboardData(text: address));
                ScaffoldMessenger.of(context).showSnackBar(
                    const SnackBar(content: Text('Address copied!'), backgroundColor: Color(0xFFCC0000)));
              },
            ),
          ],
        ),
      ),
    );
  }
}
