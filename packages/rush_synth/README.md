# rush_synth

A high-performance Flutter plugin for real-time MIDI synthesis and playback. Built with Rust for optimal performance across Android, iOS, macOS, Linux, and Windows.

## Features

- 🎹 **Real-time MIDI Synthesis** - Play notes instantly with low latency
- 🎵 **MIDI Sequencer** - Play standard MIDI files with loop support
- 🔊 **SoundFont Support** - Load and use SF2 SoundFont files
- ⚡ **Rust Backend** - High-performance synthesis engine [RustySynth](https://github.com/sinshu/rustysynth)
- 📱 **Cross-Platform** - Support for Android, iOS, macOS, Linux, and Windows with [CPAL](https://crates.io/crates/cpal)

## Requirements

| Platform | Engine | Runtime Requirements | Build Requirements |
|----------|--------|----------------------|-------------------|
| Android | AAudio | • Android 8.0 (API level 26) or higher | • NDK r26 or higher |
| iOS | CoreAudio | • iOS 12.0 or higher | • Xcode 13 or higher |
| macOS | CoreAudio | • macOS 10.13 or higher | • Xcode 13 or higher |
| Linux | ALSA | - |• ALSA development libraries |
| Windows | WASAPI | - | • Visual Studio 2019 or later (C++ build tools) |
| All Platforms | - | - | • Flutter SDK 3.3.0 or higher<br/>• Dart SDK 3.10.0 or higher<br>• Rust 1.82 toolchain |

## Installation

Add `rush_synth` to your `pubspec.yaml`:

```bash
flutter pub add rush_synth
```

### Rust Toolchain Setup

`rush_synth` requires the Rust toolchain to compile the native engine. Follow these steps to set up Rust:

#### Install Rust

If you don't have Rust installed, download and install it using rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

On Windows, download the installer from [rustup.rs](https://rustup.rs/).

After installation, add Rust to your system PATH:

```bash
source $HOME/.cargo/env
```

#### Install Cross-Compilation Targets

Depending on your app's targeted platforms, install the necessary Rust targets:

```bash
# For Android:
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android

# For iOS:
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios
rustup target add aarch64-apple-ios-sim

# For Linux:
rustup target add x86_64-unknown-linux-gnu

# For macOS:
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin

# For Windows:
rustup target add x86_64-pc-windows-msvc
```

#### Install Required Build Tools

**On macOS:**
Install Xcode Command Line Tools (if not already installed):
```bash
xcode-select --install
```

**On Linux:**
Install build essentials:
```bash
sudo apt-get update
sudo apt-get install build-essential
```

**On Windows:**
Ensure Visual Studio Build Tools or Visual Studio Community is installed with C++ support.

### Platform-Specific Setup

#### iOS/macOS
Run the following after adding the dependency:

```bash
cd ios
pod install
cd ..
```

#### Linux
Install ALSA dev libraries:
```bash
sudo apt-get install libasound2-dev
```

## Usage

### Initialize the Library

Before using `rush_synth`, you must initialize it. Usually, `main()` function is a good place to do that:

```dart
import 'package:rush_synth/rush_synth.dart';

Future<void> main() async {
  await RushSynthLib.init();
  runApp(const MyApp());
}
```

### Real-Time MIDI Synthesis

Create a synth from a SoundFont file (asset or file path):

```dart
final synth = await RushSynth.fromAsset('assets/soundfont.sf2');
// or
final synth = await RushSynth.fromFile('/path/to/soundfont.sf2');

// Open audio stream
await synth.start();

// Play a note (channel 0, middle C = key 60, velocity 100)
await synth.noteOn(channel: 0, key: 60, velocity: 100);

// Stop the note
await synth.noteOff(channel: 0, key: 60);

// Stop all notes
await synth.allNotesOff();

// Pause audio stream without releasing resources
await synth.pause();

// Close audio stream and release resources
await synth.stop();

// Resume or recreate audio stream
await synth.start();

// Permanently release all native resources
synth.dispose();
```

### MIDI Sequencer

Load and play MIDI files:

```dart
final sequencer = await RushSequencer.fromAsset('assets/soundfont.sf2');
// or
final sequencer = await RushSequencer.fromFile('/path/to/soundfont.sf2');

// Play a MIDI file with looping
await sequencer.play(
  midiPath: 'path/to/file.mid',
  playLoop: true,
);

// Adjust playback speed (1.0 = normal speed, 0.5 = half speed, 2.0 = double speed)
await sequencer.setSpeed(1.5);

// Check if sequence has ended
final ended = sequencer.endOfSequence;

// Check playback position
final pos = sequencer.position;

// Stop playback
await sequencer.stop();

// Permanently release all native resources
sequencer.dispose();
```

## Asset Setup

To use SoundFont files from your Flutter project assets:

1. Create an `assets` folder in your project root
2. Add your `.sf2` files to this folder
3. Update your `pubspec.yaml`:

```yaml
flutter:
  assets:
    - assets/soundfont.sf2
```

## Troubleshooting

### Audio not playing
- Ensure `await RushSynthLib.init()` is called before creating any synth/sequencer instances
- Check that the SoundFont file exists and is a valid SF2 file
- Verify audio permissions are granted on the target platform

### Platform-specific issues

**Linux:** If you get audio-related errors, install ALSA development libraries:
```bash
sudo apt-get install libasound2-dev
```

**macOS/iOS:** Ensure `pod install` was run in the iOS directory

**Android:** Verify minimum API level is 26 or higher in `android/app/build.gradle`

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.

## Supporting

* [Buy me a coffee](https://buymeacoffee.com/ostudio)

* Donate eCash (XEC): ecash:qrx0l5a2cjg679pghc7kalc8ff7uczf58qu7n49rwa

## License

This plugin is licensed under the MIT License. See LICENSE for details.

## Who's using it

[![Cecile - learn to sing Gregorian chant](https://cecile.orb.net.pl/img/badge-small.png "Cecile - learn to sing Gregorian chant")](https://cecile.orb.net.pl/?utm_source=pub.dev&utm_medium=badge&utm_campaign=dev&utm_id=dev.0)
