import 'package:flutter/foundation.dart';
import '../services/devilchain_service.dart';

class ChainProvider extends ChangeNotifier {
  Map<String, dynamic>? _status;
  Map<String, dynamic>? _latestBlock;

  Map<String, dynamic>? get status => _status;
  Map<String, dynamic>? get latestBlock => _latestBlock;

  Future<void> refresh() async {
    try {
      _status = await DevilChainService().getStatus();
      _latestBlock = await DevilChainService().getLatestBlock();
      notifyListeners();
    } catch (_) {}
  }
}
