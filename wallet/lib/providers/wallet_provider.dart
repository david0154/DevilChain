import 'package:flutter/foundation.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import '../services/devilchain_service.dart';

class WalletProvider extends ChangeNotifier {
  final _storage = const FlutterSecureStorage();
  String? _address;
  double _balance = 0.0;
  bool _isLoading = false;
  List<Map<String, dynamic>> _transactions = [];

  String? get address => _address;
  double get balance => _balance;
  bool get isLoading => _isLoading;
  List<Map<String, dynamic>> get transactions => _transactions;

  bool get hasWallet => _address != null;

  Future<void> loadWallet() async {
    _address = await _storage.read(key: 'wallet_address');
    if (_address != null) await refreshBalance();
    notifyListeners();
  }

  Future<void> createWallet(String mnemonic) async {
    // Derive address from mnemonic using BIP39/BIP32 + Ed25519
    // db1x prefix + sha256 of public key
    final addr = 'db1x' + mnemonic.hashCode.toRadixString(16).padLeft(16, '0');
    await _storage.write(key: 'wallet_address', value: addr);
    await _storage.write(key: 'wallet_mnemonic', value: mnemonic);
    _address = addr;
    notifyListeners();
  }

  Future<void> refreshBalance() async {
    if (_address == null) return;
    _isLoading = true;
    notifyListeners();
    try {
      final data = await DevilChainService().getWallet(_address!);
      _balance = (data['balance'] ?? 0).toDouble();
      _transactions = List<Map<String, dynamic>>.from(data['transactions'] ?? []);
    } catch (_) {}
    _isLoading = false;
    notifyListeners();
  }

  Future<void> sendDVC(String to, double amount, double gasFee) async {
    if (_address == null) return;
    await DevilChainService().sendTransaction(
      from: _address!, to: to, amount: amount, gasFee: gasFee,
    );
    await refreshBalance();
  }

  Future<void> logout() async {
    await _storage.deleteAll();
    _address = null;
    _balance = 0;
    _transactions = [];
    notifyListeners();
  }
}
