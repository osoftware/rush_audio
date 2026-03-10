import 'package:flutter/material.dart';
import 'package:flutter_virtual_piano/flutter_virtual_piano.dart';
import 'package:rush_synth/rush_synth.dart';
import 'package:rush_synth/util.dart';

Future<void> main() async {
  await RushSynthLib.init();
  runApp(const RushPiano());
}

class RushPiano extends StatefulWidget {
  const RushPiano({super.key});

  @override
  State<RushPiano> createState() => _RushPianoState();
}

class _RushPianoState extends State<RushPiano> {
  RushSynth? _synth;
  RushSequencer? _sequencer;
  bool _playing = false;
  double _speed = 1.0;
  final _pressed = <int>{};

  @override
  void initState() {
    super.initState();
    initStateAsync();
  }

  Future<void> initStateAsync() async {
    _synth = await RushSynth.fromAsset('assets/Barharp.sf2');
    _synth!.start();

    _sequencer = await RushSequencer.fromAsset('assets/Barharp.sf2');
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
        body: Column(
          spacing: 20.0,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                ElevatedButton.icon(
                  onPressed: () async {
                    if (_playing) {
                      await _sequencer?.stop();
                    } else {
                      await _sequencer?.play(
                        midiPath: await loadAsset(
                          'assets/toccata_and_fugue_in_d_minor.mid',
                        ),
                        playLoop: true,
                      );
                    }
                    setState(() {
                      _playing = !_playing;
                    });
                  },
                  icon: Icon(_playing ? Icons.stop : Icons.play_arrow),
                  label: Text(_playing ? 'Stop' : 'Play'),
                ),
                SizedBox(width: 20.0),
                Text("Speed ${_speed.toStringAsPrecision(2)}"),
                Slider.adaptive(
                  value: _speed,
                  min: 0.5,
                  max: 2.0,
                  label: 'Speed',
                  onChanged: (s) {
                    _sequencer?.setSpeed(s);
                    setState(() {
                      _speed = s;
                    });
                  },
                ),
              ],
            ),
            AspectRatio(
              aspectRatio: 16 / 3,
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
          ],
        ),
      ),
    );
  }

  @override
  void dispose() {
    _synth?.allNotesOff();
    _synth?.dispose();
    _sequencer?.dispose();
    super.dispose();
  }
}
