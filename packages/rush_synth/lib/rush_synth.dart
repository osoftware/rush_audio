library;

import 'src/rust/api/sequencer.dart';
import 'src/rust/api/synth.dart';
import 'util.dart';

export 'src/rust/api/sequencer.dart';
export 'src/rust/api/synth.dart';
export 'src/rust/frb_generated.dart' show RushSynthLib;

class Synth {
  static Future<RushSynth> fromFile(String soundfontPath) =>
      RushSynth.newInstance(soundfontPath: soundfontPath);

  static Future<RushSynth> fromAsset(String asset) async =>
      RushSynth.newInstance(soundfontPath: await loadAsset(asset));
}

class Sequencer {
  static Future<RushSequencer> fromFile(String soundfontPath) =>
      RushSequencer.newInstance(soundfontPath: soundfontPath);

  static Future<RushSequencer> fromAsset(String asset) async =>
      RushSequencer.newInstance(soundfontPath: await loadAsset(asset));
}
