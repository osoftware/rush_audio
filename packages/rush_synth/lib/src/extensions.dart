import 'package:rush_synth/rush_synth.dart';
import 'package:rush_synth/src/util.dart';

extension AssetLoader on RushSequencer {
  Future<void> playAsset({
    required String midiAsset,
    bool playLoop = false,
  }) async =>
      await play(midiPath: await loadAsset(midiAsset), playLoop: playLoop);
}
