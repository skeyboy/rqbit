// ignore: unused_import
import 'package:flutter/foundation.dart';
import 'package:get_it/get_it.dart';
import 'package:injectable/injectable.dart';
import 'config.config.dart';

@Singleton()
class Config {
  static int get port => PORT;
  static bool get isDebug => kDebugMode;
  // ignore: non_constant_identifier_names
  static String APIHOST = "http://localhost:$port/";
  // static String get APIHOST => "http://192.168.0.181:8081/";

  // isDebug ? DEBUG_APIHOST : 'https://translate.pnuts.ai';
  // ignore: non_constant_identifier_names
  String get CLIENT_API => isDebug
      ? 'https://api-dev.pnuts.ai/api/v1'
      : 'https://api.pnuts.ai/api/v1';
}

// ignore: constant_identifier_names
const PORT = 8080;

final getIt = GetIt.instance;

@InjectableInit(
  initializerName: 'init', // default
  preferRelativeImports: true, // default
  asExtension: true, // default
)
void configureDependencies() => getIt.init();
