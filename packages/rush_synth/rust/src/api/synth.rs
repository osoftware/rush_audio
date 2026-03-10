use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{Context, Result, anyhow};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{
    Device, FromSample, Sample, SampleFormat, SizedSample, Stream, StreamConfig,
    SupportedStreamConfig,
};
use flutter_rust_bridge::frb;
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};

#[frb(dart_code = "
    import '../../../util.dart';
    /// Create a new RushSynth using a soundfont from the assets bundle.
    static Future<RushSynth> fromAsset(String asset) async =>
      RushSynth.fromFile(await loadAsset(asset));
")]
pub struct RushSynth {
    device: Device,
    config: SupportedStreamConfig,
    synth: Arc<Mutex<Synthesizer>>,
    stream: Option<cpal::Stream>,
}

unsafe impl Sync for RushSynth {}
unsafe impl Send for RushSynth {}

impl RushSynth {
    /// Create a new RushSynth using a soundfont from the file system.
    #[frb(positional)]
    pub fn from_file(soundfont_path: String) -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow!("No output device found"))?;

        let config = device
            .default_output_config()
            .context("Failed to get default output config")?;
        let settings = SynthesizerSettings::new(config.sample_rate() as i32);

        let mut sf2 = fs::File::open(soundfont_path).context("Failed to open soundfont file")?;
        let sound_font = SoundFont::new(&mut sf2).context("Failed to parse soundfont")?;
        let synthesizer = Synthesizer::new(&Arc::new(sound_font), &settings)
            .context("Failed to initialize synthesizer")?;

        let rush_synth = Self {
            device,
            config,
            synth: Arc::new(Mutex::new(synthesizer)),
            stream: None,
        };

        Ok(rush_synth)
    }

    fn build_stream(&mut self) -> Result<&Stream> {
        let synth_for_cb = self.synth.clone();
        let stream = match (&self.config).sample_format() {
            SampleFormat::F32 => build_stream::<f32>(&self.device, &self.config, synth_for_cb),
            SampleFormat::I16 => build_stream::<i16>(&self.device, &self.config, synth_for_cb),
            SampleFormat::U16 => build_stream::<u16>(&self.device, &self.config, synth_for_cb),
            _ => Err(anyhow!("Unsupported sample format")),
        }?;

        self.stream = Some(stream);
        Ok(self.stream.as_ref().unwrap())
    }

    /// Start a new stream or resume a paused stream.
    pub fn start(&mut self) -> Result<()> {
        let stream = match &self.stream {
            Some(stream) => stream,
            None => self.build_stream()?,
        };
        stream.play().context("Failed to start output stream")
    }

    /// Pause the stream without releasing resources.
    pub fn pause(&mut self) -> Result<()> {
        match &self.stream {
            Some(stream) => stream.pause().context("Failed to pause output stream"),
            None => Ok(()),
        }
    }

    /// Turn a MIDI note on.
    /// 
    /// # Arguments
    /// 
    /// * `channel` - The channel of the note.
    /// * `key` - The key of the note.
    /// * `velocity` - The velocity of the note.
    pub fn note_on(&self, channel: i32, key: i32, velocity: i32) {
        let mut s = self.synth.lock().unwrap();
        s.note_on(channel, key, velocity);
    }

    /// Turn a MIDI note off
    /// 
    /// # Arguments
    /// 
    /// * `channel` - The channel of the note.
    /// * `key` - The key of the note.
    pub fn note_off(&self, channel: i32, key: i32) {
        let mut s = self.synth.lock().unwrap();
        s.note_off(channel, key);
    }

    /// Turn off all MIDI notes that are on
    pub fn all_notes_off(&self) {
        let mut s = self.synth.lock().unwrap();
        s.note_off_all(true);
    }

    /// Stop the stream and release resources
    pub fn stop(&mut self) {
        if let Some(stream) = self.stream.take() {
            stream.pause().ok();
            std::thread::sleep(Duration::from_millis(200));
            drop(stream);
        }
    }
}

impl Drop for RushSynth {
    fn drop(&mut self) {
        self.stop();
    }
}

fn build_stream<T: FromSample<f32> + SizedSample>(
    device: &Device,
    config: &SupportedStreamConfig,
    synth_for_cb: Arc<Mutex<Synthesizer>>,
) -> Result<Stream> {
    let stream = device
        .build_output_stream(
            &StreamConfig {
                buffer_size: cpal::BufferSize::Fixed(128),
                ..config.config()
            },
            move |data: &mut [T], _| fill_output_buffer(data, &synth_for_cb),
            move |err| eprintln!("CPAL output error: {}", err),
            None,
        )
        .context("Failed to create output stream")?;
    Ok(stream)
}

fn fill_output_buffer<T: FromSample<f32> + SizedSample>(
    out: &mut [T],
    synth: &Arc<Mutex<Synthesizer>>,
) {
    let len = out.len() / 2;
    let mut left = vec![0.0f32; len];
    let mut right = vec![0.0f32; len];

    let mut guard = synth.lock().unwrap();
    guard.render(&mut left, &mut right);
    drop(guard);

    for i in 0..len {
        out[2 * i] = left[i].to_sample();
        out[2 * i + 1] = right[i].to_sample();
    }
}
