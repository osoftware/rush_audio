use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{Context, Result, anyhow};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, Sample, SampleFormat, SizedSample, Stream, StreamConfig, SupportedStreamConfig};
use rustysynth::{MidiFile, MidiFileSequencer, SoundFont, Synthesizer, SynthesizerSettings};

pub struct RushSequencer {
    sequencer: Arc<Mutex<MidiFileSequencer>>,
    stream: Option<Stream>,
}

unsafe impl Sync for RushSequencer {}
unsafe impl Send for RushSequencer {}

impl RushSequencer {
    pub fn new(soundfont_path: String) -> Result<Self> {
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

        let sequencer = Arc::new(Mutex::new(MidiFileSequencer::new(synthesizer)));
        let sequencer_for_cb = sequencer.clone();

        let stream = match config.sample_format() {
            SampleFormat::F32 => build_stream::<f32>(&device, &config, sequencer_for_cb),
            SampleFormat::I16 => build_stream::<i16>(&device, &config, sequencer_for_cb),
            SampleFormat::U16 => build_stream::<u16>(&device, &config, sequencer_for_cb),
            _ => Err(anyhow!("Unsupported sample format")),
        }?;

        stream.pause().context("Failed to pause output stream")?;

        Ok(Self {
            sequencer,
            stream: Some(stream),
        })
    }

    pub fn play(&mut self, midi_path: String, play_loop: bool) -> Result<()> {
        let mut midi_file = fs::File::open(midi_path).context("Failed to open soundfont file")?;
        let midi = MidiFile::new(&mut midi_file).context("Failed to parse MIDI file")?;

        let mut guard = self.sequencer.lock().unwrap();
        guard.play(&Arc::new(midi), play_loop);

        match &self.stream {
            Some(stream) => stream.play().context("Failed to start output stream"),
            None => Err(anyhow!("No output stream to start")),
        }
    }

    pub fn stop(&mut self) -> Result<()> {
        let mut guard = self.sequencer.lock().unwrap();
        guard.stop();

        match &self.stream {
            Some(stream) => stream.pause().context("Failed to pause output stream"),
            None => Err(anyhow!("No output stream to pause")),
        }
    }

    pub fn set_speed(&mut self, speed: f64) {
        let mut guard = self.sequencer.lock().unwrap();
        guard.set_speed(speed);
    }

    pub fn end_of_sequence(&self) -> bool {
        let guard = self.sequencer.lock().unwrap();
        guard.end_of_sequence()
    }
}

impl Drop for RushSequencer {
    fn drop(&mut self) {
        if let Some(stream) = self.stream.take() {
            stream.pause().ok();
            std::thread::sleep(Duration::from_millis(200));
            drop(stream);
        }
    }
}

fn build_stream<T: FromSample<f32> + SizedSample>(
    device: &Device,
    config: &SupportedStreamConfig,
    sequencer: Arc<Mutex<MidiFileSequencer>>,
) -> Result<Stream> {
    let stream = device
        .build_output_stream(
            &StreamConfig {
                buffer_size: cpal::BufferSize::Fixed(128),
                ..config.config()
            },
            move |data: &mut [T], _| fill_output_buffer(data, &sequencer),
            move |err| eprintln!("CPAL output error: {}", err),
            None,
        )
        .context("Failed to create output stream")?;
    Ok(stream)
}

fn fill_output_buffer<T: FromSample<f32> + SizedSample>(
    out: &mut [T],
    sequencer: &Arc<Mutex<MidiFileSequencer>>,
) {
    let len = out.len() / 2;
    let mut left = vec![0.0f32; len];
    let mut right = vec![0.0f32; len];

    let mut guard = sequencer.lock().unwrap();
    guard.render(&mut left, &mut right);
    drop(guard);

    for i in 0..len {
        out[2 * i] = left[i].to_sample();
        out[2 * i + 1] = right[i].to_sample();
    }
}
