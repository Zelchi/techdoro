use std::{sync::mpsc, time::Duration};

#[derive(Clone, Copy)]
pub enum Sound {
    Click,
    Alarm,
}

enum AudioCommand {
    Play(Sound, u32),
    Volume(u32),
}

pub struct Audio {
    sender: mpsc::SyncSender<AudioCommand>,
}

impl Audio {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::sync_channel::<AudioCommand>(32);
        std::thread::spawn(move || {
            let mut output = None;
            let mut sinks: Vec<rodio::Sink> = Vec::new();
            loop {
                sinks.retain(|sink| !sink.empty());
                if sinks.is_empty() {
                    output = None;
                }
                let message = if output.is_some() {
                    match receiver.recv_timeout(Duration::from_millis(250)) {
                        Ok(message) => message,
                        Err(mpsc::RecvTimeoutError::Timeout) => continue,
                        Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    }
                } else {
                    match receiver.recv() {
                        Ok(message) => message,
                        Err(_) => break,
                    }
                };
                let (sound, volume) = match message {
                    AudioCommand::Play(sound, volume) => (sound, volume),
                    AudioCommand::Volume(volume) => {
                        for sink in &sinks {
                            sink.set_volume(volume.min(100) as f32 / 100.0);
                        }
                        continue;
                    }
                };
                if output.is_none() {
                    match rodio::OutputStream::try_default() {
                        Ok(stream) => output = Some(stream),
                        Err(error) => {
                            eprintln!("Audio unavailable: {error}");
                            continue;
                        }
                    }
                }
                let bytes: &[u8] = match sound {
                    Sound::Click => include_bytes!("../../assets/click.mp3"),
                    Sound::Alarm => include_bytes!("../../assets/alarme.mp3"),
                };
                if let (Ok(source), Ok(sink)) = (
                    rodio::Decoder::new(std::io::Cursor::new(bytes)),
                    rodio::Sink::try_new(&output.as_ref().unwrap().1),
                ) {
                    sink.set_volume(volume.min(100) as f32 / 100.0);
                    sink.append(source);
                    sinks.push(sink);
                }
            }
        });
        Self { sender }
    }

    pub fn play(&self, sound: Sound, volume: u32) {
        if volume > 0 {
            let _ = self.sender.try_send(AudioCommand::Play(sound, volume));
        }
    }

    pub fn set_volume(&self, volume: u32) {
        let _ = self.sender.try_send(AudioCommand::Volume(volume));
    }
}
