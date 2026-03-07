import 'package:integration_test/integration_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:rush_synth/rush_synth.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async => await RustLib.init());
  test('Can call rust function', () async {
    final synth = RushSynth.newInstance(soundfontPath: 'soundfont.sf2');
    expect(synth, isNotNull);
  });
}
