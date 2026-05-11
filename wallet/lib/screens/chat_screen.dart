import 'package:flutter/material.dart';
import '../services/wallet_service.dart';

class ChatScreen extends StatefulWidget {
  const ChatScreen({super.key});
  @override
  State<ChatScreen> createState() => _ChatScreenState();
}

class _ChatScreenState extends State<ChatScreen> {
  final _msgCtrl  = TextEditingController();
  final _toCtrl   = TextEditingController();
  String _myAddr  = '';
  List<Map<String, dynamic>> _messages = [];

  @override
  void initState() {
    super.initState();
    WalletService().getAddress().then((a) => setState(() => _myAddr = a));
  }

  void _sendMsg() {
    final to  = _toCtrl.text.trim();
    final msg = _msgCtrl.text.trim();
    if (to.isEmpty || msg.isEmpty) return;
    setState(() {
      _messages.add({'from': _myAddr, 'to': to, 'text': msg,
        'time': DateTime.now().toIso8601String()});
      _msgCtrl.clear();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0D0D0D),
      appBar: AppBar(
        backgroundColor: const Color(0xFF111111),
        title: const Text('DevilChat', style: TextStyle(color: Color(0xFFCC0000))),
        iconTheme: const IconThemeData(color: Colors.white),
        subtitle: const Text('E2E Encrypted Wallet-to-Wallet',
          style: TextStyle(color: Colors.white38, fontSize: 11))),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(12),
            child: TextField(
              controller: _toCtrl,
              style: const TextStyle(color: Colors.white, fontSize: 12, fontFamily: 'monospace'),
              decoration: InputDecoration(
                hintText: 'Recipient wallet address (db1x...)',
                hintStyle: const TextStyle(color: Colors.white24, fontSize: 12),
                filled: true, fillColor: const Color(0xFF1a1a1a),
                border: OutlineInputBorder(borderRadius: BorderRadius.circular(10),
                  borderSide: const BorderSide(color: Color(0xFF330000))),
                enabledBorder: OutlineInputBorder(borderRadius: BorderRadius.circular(10),
                  borderSide: const BorderSide(color: Color(0xFF330000))),
                focusedBorder: OutlineInputBorder(borderRadius: BorderRadius.circular(10),
                  borderSide: const BorderSide(color: Color(0xFFCC0000)))),
            ),
          ),
          Expanded(
            child: _messages.isEmpty
              ? const Center(child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Icon(Icons.lock, color: Color(0xFFCC0000), size: 40),
                    SizedBox(height: 12),
                    Text('E2E Encrypted', style: TextStyle(color: Colors.white54)),
                    Text('Messages signed with your wallet key',
                      style: TextStyle(color: Colors.white24, fontSize: 11)),
                  ]))
              : ListView.builder(
                  padding: const EdgeInsets.symmetric(horizontal: 12),
                  itemCount: _messages.length,
                  itemBuilder: (_, i) {
                    final m    = _messages[i];
                    final mine = m['from'] == _myAddr;
                    return Align(
                      alignment: mine ? Alignment.centerRight : Alignment.centerLeft,
                      child: Container(
                        margin: const EdgeInsets.only(bottom: 8),
                        padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
                        decoration: BoxDecoration(
                          color: mine ? const Color(0xFF330000) : const Color(0xFF1a1a1a),
                          borderRadius: BorderRadius.circular(16)),
                        child: Text(m['text'],
                          style: const TextStyle(color: Colors.white, fontSize: 14)),
                      ),
                    );
                  }),
          ),
          Padding(
            padding: const EdgeInsets.all(12),
            child: Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: _msgCtrl,
                    style: const TextStyle(color: Colors.white),
                    decoration: InputDecoration(
                      hintText: 'Message (E2E encrypted)',
                      hintStyle: const TextStyle(color: Colors.white24),
                      filled: true, fillColor: const Color(0xFF1a1a1a),
                      border: OutlineInputBorder(borderRadius: BorderRadius.circular(12),
                        borderSide: const BorderSide(color: Color(0xFF330000))),
                      enabledBorder: OutlineInputBorder(borderRadius: BorderRadius.circular(12),
                        borderSide: const BorderSide(color: Color(0xFF330000))),
                      focusedBorder: OutlineInputBorder(borderRadius: BorderRadius.circular(12),
                        borderSide: const BorderSide(color: Color(0xFFCC0000)))),
                  ),
                ),
                const SizedBox(width: 8),
                GestureDetector(
                  onTap: _sendMsg,
                  child: Container(
                    padding: const EdgeInsets.all(14),
                    decoration: BoxDecoration(
                      color: const Color(0xFFCC0000),
                      borderRadius: BorderRadius.circular(12)),
                    child: const Icon(Icons.send, color: Colors.white, size: 20))),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
