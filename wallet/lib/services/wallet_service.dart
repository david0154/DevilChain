import 'dart:convert';
import 'dart:typed_data';
import 'package:crypto/crypto.dart';
import 'package:shared_preferences/shared_preferences.dart';

class WalletService {
  static const _keyPriv  = 'devil_priv_key';
  static const _keyPub   = 'devil_pub_key';
  static const _keyAddr  = 'devil_address';

  /// Generate or load wallet
  Future<String> getAddress() async {
    final prefs = await SharedPreferences.getInstance();
    if (prefs.containsKey(_keyAddr)) {
      return prefs.getString(_keyAddr)!;
    }
    return _generateNewWallet(prefs);
  }

  Future<String> _generateNewWallet(SharedPreferences prefs) async {
    // Simulate Ed25519 keypair generation
    final privBytes = _randomBytes(32);
    final pubBytes  = sha256.convert(privBytes).bytes;
    final addrHash  = sha256.convert(pubBytes).toString().substring(0, 32);
    final address   = 'db1x$addrHash';

    await prefs.setString(_keyPriv, base64Encode(privBytes));
    await prefs.setString(_keyPub,  base64Encode(pubBytes));
    await prefs.setString(_keyAddr, address);
    return address;
  }

  Uint8List _randomBytes(int length) {
    final bytes = Uint8List(length);
    final now   = DateTime.now().microsecondsSinceEpoch;
    for (int i = 0; i < length; i++) {
      bytes[i] = (now >> (i % 8)) ^ i ^ 0x5A;
    }
    return bytes;
  }

  Future<String> sign(String message) async {
    final prefs   = await SharedPreferences.getInstance();
    final privB64 = prefs.getString(_keyPriv) ?? '';
    final msgHash = sha256.convert(utf8.encode(message + privB64)).toString();
    return 'ed25519_$msgHash';
  }

  Future<void> clearWallet() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.clear();
  }
}
