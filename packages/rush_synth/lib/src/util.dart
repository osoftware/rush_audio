import 'dart:io';
import 'package:flutter/services.dart';
import 'package:path_provider/path_provider.dart';

Future<String> loadAsset(String assetPath) async {
  final tempDir = await getTemporaryDirectory();
  final tempFile = File('${tempDir.path}/${assetPath.split('/').last}');
  if (!tempFile.existsSync()) {
    final byteData = await rootBundle.load(assetPath);
    final buffer = byteData.buffer;
    await tempFile.writeAsBytes(
      buffer.asUint8List(byteData.offsetInBytes, byteData.lengthInBytes),
    );
  }
  return tempFile.path;
}
