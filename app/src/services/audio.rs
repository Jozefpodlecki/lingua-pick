use std::{path::PathBuf, sync::{Arc, Mutex}, time::{Duration, Instant}};

use anyhow::{anyhow, bail, Result};
use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, SampleFormat, Stream};


pub struct AudioManager(Arc<Mutex<AudioManagerInner>>, Stream);

struct AudioManagerInner {
    samples: Vec<f32>,
    recording: bool,
    start_time: Option<Instant>,
}

impl AudioManager {
     pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow!("No input device available"))?;

        let supported_config = device.default_input_config()?;

        if supported_config.sample_format() != SampleFormat::F32 {
            bail!("Only F32 input devices are supported");
        }

        let config = supported_config.config();

        let inner = Arc::new(Mutex::new(AudioManagerInner {
            samples: Vec::new(),
            recording: false,
            start_time: None,
        }));

        let inner_clone = inner.clone();
        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _| Self::data_callback(&inner_clone, data),
            |err| eprintln!("input stream error: {err}"),
            None,
        )?;

        Ok(Self(inner, stream))
    }

    fn data_callback(inner_clone: &Arc<Mutex<AudioManagerInner>>, data: &[f32]) {
        let mut inner = inner_clone.lock().unwrap();

        if !inner.recording {
            return;
        }

        if let Some(start) = inner.start_time {
            if start.elapsed() > Duration::from_secs(3) {
                return;
            }
        }

        inner.samples.extend(data);
    }

    pub fn start(&self) -> Result<()> {
        {
            let mut inner = self.0.lock().unwrap();
            inner.samples.clear();
            inner.recording = true;
            inner.start_time = Some(Instant::now());
        }

        self.1.play()?;

        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        {
            let mut inner = self.0.lock().unwrap();
            inner.recording = false;
        }

        self.1.pause()?;

        Ok(())
    }

    pub fn take_samples(&self) -> Vec<f32> {
        let mut inner = self.0.lock().unwrap();
        std::mem::take(&mut inner.samples)
    }
}