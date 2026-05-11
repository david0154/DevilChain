import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'screens/home_screen.dart';
import 'screens/send_screen.dart';
import 'screens/stake_screen.dart';
import 'screens/dao_screen.dart';
import 'screens/chat_screen.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  SystemChrome.setSystemUIOverlayStyle(
    const SystemUiOverlayStyle(statusBarColor: Colors.transparent));
  runApp(const DevilXWalletApp());
}

class DevilXWalletApp extends StatelessWidget {
  const DevilXWalletApp({super.key});
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'DevilX Wallet',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        useMaterial3: true,
        colorScheme: const ColorScheme.dark(
          primary: Color(0xFFCC0000),
          secondary: Color(0xFFFF3333),
          surface: Color(0xFF151515),
          background: Color(0xFF0D0D0D),
        ),
        scaffoldBackgroundColor: const Color(0xFF0D0D0D),
        fontFamily: 'Inter',
      ),
      initialRoute: '/',
      routes: {
        '/':      (_) => const HomeScreen(),
        '/send':  (_) => const SendScreen(),
        '/stake': (_) => const StakeScreen(),
        '/dao':   (_) => const DaoScreen(),
        '/chat':  (_) => const ChatScreen(),
      },
    );
  }
}
