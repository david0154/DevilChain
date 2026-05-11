import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'screens/home_screen.dart';
import 'screens/send_screen.dart';
import 'screens/receive_screen.dart';
import 'screens/staking_screen.dart';
import 'screens/dao_screen.dart';
import 'screens/settings_screen.dart';
import 'providers/wallet_provider.dart';
import 'providers/chain_provider.dart';

void main() {
  runApp(const DevilXApp());
}

class DevilXApp extends StatelessWidget {
  const DevilXApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MultiProvider(
      providers: [
        ChangeNotifierProvider(create: (_) => WalletProvider()),
        ChangeNotifierProvider(create: (_) => ChainProvider()),
      ],
      child: MaterialApp(
        title: 'DevilX Wallet',
        debugShowCheckedModeBanner: false,
        theme: ThemeData(
          colorScheme: ColorScheme.dark(
            primary: const Color(0xFFCC0000),
            secondary: const Color(0xFFFF3333),
            surface: const Color(0xFF1A1A1A),
            background: const Color(0xFF0D0D0D),
          ),
          scaffoldBackgroundColor: const Color(0xFF0D0D0D),
          fontFamily: 'DevilFont',
          useMaterial3: true,
        ),
        initialRoute: '/',
        routes: {
          '/': (ctx) => const HomeScreen(),
          '/send': (ctx) => const SendScreen(),
          '/receive': (ctx) => const ReceiveScreen(),
          '/staking': (ctx) => const StakingScreen(),
          '/dao': (ctx) => const DaoScreen(),
          '/settings': (ctx) => const SettingsScreen(),
        },
      ),
    );
  }
}
