import 'package:flutter/material.dart';
import 'package:flutter_virtual_piano/flutter_virtual_piano.dart';
import 'package:rush_synth/rush_synth.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const RushPiano());
}

class RushPiano extends StatefulWidget {
  const RushPiano({super.key});

  @override
  State<RushPiano> createState() => _RushPianoState();
}

class _RushPianoState extends State<RushPiano> {
  RushSynth? _synth;
  final _pressed = <int>{};

  @override
  void initState() {
    super.initState();
    initStateAsync();
  }

  Future<void> initStateAsync() async {
    _synth = await Synth.fromAsset('assets/Barharp.sf2');
    _synth!.start();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData.from(
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.deepPurple,
          brightness: Brightness.dark,
        ),
      ),
      home: Scaffold(
        appBar: AppBar(title: const Text('Rush Harpsichord')),
        body: Center(
          child: AspectRatio(
            aspectRatio: 16 / 4,
            child: VirtualPiano(
              noteRange: RangeValues(41, 88),
              highlightedNoteSets: [
                HighlightedNoteSet(_pressed, Colors.purple),
              ],
              onNotePressed: (key, _) async {
                await _synth?.noteOn(channel: 0, key: key, velocity: 127);
                setState(() {
                  _pressed.add(key);
                });
              },
              onNoteReleased: (key) async {
                await _synth?.noteOff(channel: 0, key: key);
                setState(() {
                  _pressed.remove(key);
                });
              },
            ),
          ),
        ),
      ),
    );
  }

  @override
  void dispose() {
    _synth?.allNotesOff();
    _synth?.dispose();
    super.dispose();
  }
}
