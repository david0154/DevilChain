import 'dart:convert';
import 'package:http/http.dart' as http;

class DevilChainService {
  static const String baseUrl = 'http://localhost:8545';

  Future<Map<String, dynamic>> getStatus() async {
    final res = await http.get(Uri.parse('$baseUrl/api/status'));
    return jsonDecode(res.body);
  }

  Future<Map<String, dynamic>> getLatestBlock() async {
    final res = await http.get(Uri.parse('$baseUrl/api/block/latest'));
    return jsonDecode(res.body);
  }

  Future<Map<String, dynamic>> getWallet(String address) async {
    final res = await http.get(Uri.parse('$baseUrl/api/wallet/$address'));
    return jsonDecode(res.body);
  }

  Future<Map<String, dynamic>> sendTransaction({
    required String from, required String to,
    required double amount, required double gasFee,
  }) async {
    final res = await http.post(
      Uri.parse('$baseUrl/api/send'),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode({'from': from, 'to': to, 'amount': amount, 'gas_fee': gasFee}),
    );
    return jsonDecode(res.body);
  }

  Future<Map<String, dynamic>> stake(String address, double amount) async {
    final res = await http.post(
      Uri.parse('$baseUrl/api/stake'),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode({'address': address, 'amount': amount}),
    );
    return jsonDecode(res.body);
  }

  Future<List<dynamic>> getDaoProposals() async {
    final res = await http.get(Uri.parse('$baseUrl/api/dao/proposals'));
    return jsonDecode(res.body);
  }
}
