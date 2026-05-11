import 'package:flutter/material.dart';
import '../services/wallet_service.dart';
import '../services/node_service.dart';

class DaoScreen extends StatefulWidget {
  const DaoScreen({super.key});
  @override
  State<DaoScreen> createState() => _DaoScreenState();
}

class _DaoScreenState extends State<DaoScreen> {
  List<dynamic> proposals = [];
  bool loading = true;

  @override
  void initState() { super.initState(); _loadProposals(); }

  Future<void> _loadProposals() async {
    final node = NodeService();
    final p    = await node.getProposals();
    setState(() { proposals = p; loading = false; });
  }

  Future<void> _vote(int proposalId, bool vote) async {
    final wallet = WalletService();
    final node   = NodeService();
    final addr   = await wallet.getAddress();
    final sig    = await wallet.sign('{"proposal_id":$proposalId,"vote":$vote}');
    await node.vote(addr, proposalId, vote, sig);
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text(vote ? '✅ Voted YES on #$proposalId' : '❌ Voted NO on #$proposalId')));
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0D0D0D),
      appBar: AppBar(
        backgroundColor: const Color(0xFF111111),
        title: const Text('DAO Governance', style: TextStyle(color: Color(0xFFCC0000))),
        iconTheme: const IconThemeData(color: Colors.white)),
      body: loading
        ? const Center(child: CircularProgressIndicator(color: Color(0xFFCC0000)))
        : proposals.isEmpty
          ? const Center(child: Text('No proposals yet',
              style: TextStyle(color: Colors.white38)))
          : ListView.builder(
              padding: const EdgeInsets.all(20),
              itemCount: proposals.length,
              itemBuilder: (_, i) {
                final p = proposals[i];
                return Container(
                  margin: const EdgeInsets.only(bottom: 14),
                  padding: const EdgeInsets.all(16),
                  decoration: BoxDecoration(
                    color: const Color(0xFF151515),
                    borderRadius: BorderRadius.circular(14),
                    border: Border.all(color: const Color(0xFF2a0000))),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text('#${p["id"]} — ${p["title"]}',
                        style: const TextStyle(color: Colors.white,
                          fontSize: 14, fontWeight: FontWeight.bold)),
                      const SizedBox(height: 6),
                      Text(p['description'] ?? '',
                        style: const TextStyle(color: Colors.white54, fontSize: 12)),
                      const SizedBox(height: 12),
                      Row(
                        children: [
                          Expanded(
                            child: ElevatedButton(
                              style: ElevatedButton.styleFrom(
                                backgroundColor: Colors.green.shade900),
                              onPressed: () => _vote(p['id'], true),
                              child: const Text('✅ YES',
                                style: TextStyle(color: Colors.green)))),
                          const SizedBox(width: 10),
                          Expanded(
                            child: ElevatedButton(
                              style: ElevatedButton.styleFrom(
                                backgroundColor: const Color(0xFF330000)),
                              onPressed: () => _vote(p['id'], false),
                              child: const Text('❌ NO',
                                style: TextStyle(color: Color(0xFFCC0000))))),
                        ],
                      ),
                    ],
                  ),
                );
              },
            ),
    );
  }
}
