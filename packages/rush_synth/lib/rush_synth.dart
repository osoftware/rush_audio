library;

import 'package:rush_synth/soundfont.dart';

import 'src/rust/api/synth.dart';
export 'src/rust/api/synth.dart';
export 'src/rust/frb_generated.dart' show RustLib;

class Synth {
  static Future<RushSynth> fromFile(String path) =>
      RushSynth.newInstance(soundfontPath: path);

  static Future<RushSynth> fromAsset(String asset) async =>
      RushSynth.newInstance(soundfontPath: await loadAsset(assetPath: asset));
}
