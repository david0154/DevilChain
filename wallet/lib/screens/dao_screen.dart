import 'package:flutter/material.dart';
import '../services/devilchain_service.dart';

class DaoScreen extends StatefulWidget {
  const DaoScreen({super.key});
  @override
  State<DaoScreen> createState() => _DaoScreenState();
}

class _DaoScreenState extends State<DaoScreen> {
  List<dynamic> _proposals = [];
  bool _loading = true;

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    try {
      final data = await DevilChainService().getDaoProposals();
      setState(() { _proposals = data; _loading = false; });
    } catch (_) {
      setState(() => _loading = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0D0D0D),
      appBar: AppBar(backgroundColor: const Color(0xFF1A1A1A), title: const Text('DevilChain DAO', style: TextStyle(color: Color(0xFFCC0000)))),
      body: _loading
          ? const Center(child: CircularProgressIndicator(color: Color(0xFFCC0000)))
          : _proposals.isEmpty
              ? const Center(child: Text('No proposals yet', style: TextStyle(color: Colors.white38)))
              : ListView.builder(
                  itemCount: _proposals.length,
                  itemBuilder: (ctx, i) {
                    final p = _proposals[i];
                    return Card(
                      color: const Color(0xFF1A1A1A),
                      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                      child: ListTile(
                        title: Text(p['title'] ?? 'Proposal', style: const TextStyle(color: Colors.white)),
                        subtitle: Text(p['description'] ?? '', style: const TextStyle(color: Colors.white60)),
                        trailing: Text(p['status'] ?? 'Active', style: const TextStyle(color: Color(0xFFCC0000))),
                      ),
                    );
                  },
                ),
    );
  }
}
