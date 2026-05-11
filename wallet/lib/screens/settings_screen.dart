import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../providers/wallet_provider.dart';

class SettingsScreen extends StatelessWidget {
  const SettingsScreen({super.key});
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0D0D0D),
      appBar: AppBar(backgroundColor: const Color(0xFF1A1A1A), title: const Text('Settings', style: TextStyle(color: Color(0xFFCC0000)))),
      body: ListView(
        children: [
          ListTile(
            leading: const Icon(Icons.info_outline, color: Color(0xFFCC0000)),
            title: const Text('DevilX Wallet v1.0.0', style: TextStyle(color: Colors.white)),
            subtitle: const Text('DevilChain Network', style: TextStyle(color: Colors.white38)),
          ),
          const Divider(color: Colors.white12),
          ListTile(
            leading: const Icon(Icons.logout, color: Colors.red),
            title: const Text('Logout / Remove Wallet', style: TextStyle(color: Colors.red)),
            onTap: () async {
              final confirm = await showDialog<bool>(
                context: context,
                builder: (ctx) => AlertDialog(
                  backgroundColor: const Color(0xFF1A1A1A),
                  title: const Text('Remove Wallet?', style: TextStyle(color: Colors.white)),
                  content: const Text('Make sure you have your mnemonic backed up!', style: TextStyle(color: Colors.white60)),
                  actions: [
                    TextButton(onPressed: () => Navigator.pop(ctx, false), child: const Text('Cancel')),
                    TextButton(onPressed: () => Navigator.pop(ctx, true), child: const Text('Remove', style: TextStyle(color: Colors.red))),
                  ],
                ),
              );
              if (confirm == true && context.mounted) {
                await context.read<WalletProvider>().logout();
                Navigator.popUntil(context, (r) => r.isFirst);
              }
            },
          ),
        ],
      ),
    );
  }
}
