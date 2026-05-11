import 'dart:convert';
import 'package:http/http.dart' as http;

class NodeService {
  static const String _base = 'http://localhost:8545';

  Future<Map<String, dynamic>> getWallet(String address) async {
    try {
      final r = await http.get(Uri.parse('$_base/api/wallet/$address'))
          .timeout(const Duration(seconds: 8));
      if (r.statusCode == 200) return jsonDecode(r.body);
    } catch (_) {}
    return {'balance': 0.0, 'staked': 0.0, 'tx_count': 0};
  }

  Future<List<dynamic>> getTransactions(String address) async {
    try {
      final r = await http.get(Uri.parse('$_base/api/wallet/$address/txs'))
          .timeout(const Duration(seconds: 8));
      if (r.statusCode == 200) {
        final d = jsonDecode(r.body);
        return d['transactions'] ?? [];
      }
    } catch (_) {}
    return [];
  }

  Future<Map<String, dynamic>> sendTransaction({
    required String from, required String to,
    required double amount, required double gasFee,
    required String signature,
  }) async {
    try {
      final r = await http.post(
        Uri.parse('$_base/api/send'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({
          'from': from, 'to': to,
          'amount': amount, 'gas_fee': gasFee,
          'signature': signature,
        }),
      ).timeout(const Duration(seconds: 10));
      return jsonDecode(r.body);
    } catch (e) {
      return {'error': e.toString()};
    }
  }

  Future<Map<String, dynamic>> stake(
      String address, double amount, String sig) async {
    try {
      final r = await http.post(
        Uri.parse('$_base/api/stake'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({'address': address, 'amount': amount, 'signature': sig}),
      ).timeout(const Duration(seconds: 10));
      return jsonDecode(r.body);
    } catch (e) {
      return {'error': e.toString()};
    }
  }

  Future<Map<String, dynamic>> vote(
      String address, int proposalId, bool vote, String sig) async {
    try {
      final r = await http.post(
        Uri.parse('$_base/api/vote'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({
          'address': address, 'proposal_id': proposalId,
          'vote': vote, 'signature': sig,
        }),
      ).timeout(const Duration(seconds: 10));
      return jsonDecode(r.body);
    } catch (e) {
      return {'error': e.toString()};
    }
  }

  Future<List<dynamic>> getProposals() async {
    try {
      final r = await http.get(Uri.parse('$_base/api/dao/proposals'))
          .timeout(const Duration(seconds: 8));
      if (r.statusCode == 200) {
        final d = jsonDecode(r.body);
        return d['proposals'] ?? [];
      }
    } catch (_) {}
    return [];
  }
}
