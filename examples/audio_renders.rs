use kantera::{
    audio_buffer::AudioBuffer,
    audio_render::{AudioRender, render_to_buffer},
    audio_renders::{
        sequencer::Sequencer,
        note::Note,
        audio_buffer::AudioBufferRender,
        audio_clip::AudioClip
    },
    interpolation::NearestNeighbor,
    ffmpeg::{import_audio, export_audio}
};

fn note(dur: f64, nn: i32, vel: f64, pan: f64) -> Box<dyn AudioRender> {
    Box::new(Note {
        frequency: 440.0 * 2.0f64.powf((nn - 69) as f64 / 12.0),
        duration: dur,
        gain: vel,
        pan: pan
    })
}

fn main() {
    let audio_buffer = import_audio("./scc.mp3");
    let audio_buffer_render = AudioBufferRender {
        audio_buffer: audio_buffer,
        interpolation: NearestNeighbor
    };
    let audio_clip = AudioClip {
        audio_render: audio_buffer_render,
        gain: 0.9,
        pan: 0.0,
        start: 26.0,
        duration: 10.0,
        pitch: 0.5,
        fadein: 3.0,
        fadeout: 3.0
    };
    let render = Sequencer::new()
        .append(0.00, note(0.25, 60, 0.01, 0.0))
        .append(0.25, note(0.25, 60, 0.02, 0.0))
        .append(0.50, note(0.25, 60, 0.04, 0.0))
        .append(0.75, note(0.25, 60, 0.08, 0.0))
        .append(1.00, note(0.25, 60, 0.08, -1.0))
        .append(1.25, note(0.25, 60, 0.08, 1.0))
        .append(1.50, note(0.25, 62, 0.08, 0.0))
        .append(1.75, note(0.25, 64, 0.08, 0.0))
        .append(2.00, note(1.00, 60, 0.04, 0.0))
        .append(2.00, note(1.00, 64, 0.04, -0.5))
        .append(2.00, note(1.00, 69, 0.04, 0.5))
        .append(3.00, Box::new(audio_clip));
    let audio_buffer = render_to_buffer(&render, 44100);
    let audio_buffer = AudioBuffer::<u16>::from(&audio_buffer);
    export_audio(&audio_buffer, "audio_renders.mp3", true);
}
