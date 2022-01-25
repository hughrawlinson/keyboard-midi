extern crate midir;
extern crate ux;

use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::sync::mpsc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use ux::u7;

use midir::{MidiOutput, MidiOutputPort};

mod musical_score;
use musical_score::{MusicalScore, MusicalScoreNote, MusicalScoreReader};

fn main() {
  let tempo = 200;
  let quarter_note_duration = Duration::from_secs(60) / tempo;

  let score: MusicalScore = MusicalScore {
    score: vec![
      MusicalScoreNote {
        midi_note: u7::new(54),
        duration: quarter_note_duration.mul_f32(0.5),
        start_time: quarter_note_duration.mul_f32(1.0 * 0.5),
        velocity: u7::new(127),
      },
      MusicalScoreNote {
        midi_note: u7::new(53),
        duration: quarter_note_duration.mul_f32(0.5),
        start_time: quarter_note_duration.mul_f32(2.0 * 0.5),
        velocity: u7::new(127),
      },
      MusicalScoreNote {
        midi_note: u7::new(51),
        duration: quarter_note_duration.mul_f32(0.5),
        start_time: quarter_note_duration.mul_f32(3.0 * 0.5),
        velocity: u7::new(127),
      },
      MusicalScoreNote {
        midi_note: u7::new(49),
        duration: quarter_note_duration.mul_f32(0.5),
        start_time: quarter_note_duration.mul_f32(4.0 * 0.5),
        velocity: u7::new(127),
      },
      MusicalScoreNote {
        midi_note: u7::new(47),
        duration: quarter_note_duration.mul_f32(0.5),
        start_time: quarter_note_duration.mul_f32(5.0 * 0.5),
        velocity: u7::new(127),
      },
      MusicalScoreNote {
        midi_note: u7::new(46),
        duration: quarter_note_duration.mul_f32(0.5),
        start_time: quarter_note_duration.mul_f32(6.0 * 0.5),
        velocity: u7::new(127),
      },
      MusicalScoreNote {
        midi_note: u7::new(44),
        duration: quarter_note_duration.mul_f32(0.5),
        start_time: quarter_note_duration.mul_f32(7.0 * 0.5),
        velocity: u7::new(127),
      },
      MusicalScoreNote {
        midi_note: u7::new(42),
        duration: quarter_note_duration.mul_f32(0.5),
        start_time: quarter_note_duration.mul_f32(8.0 * 0.5),
        velocity: u7::new(127),
      },
      MusicalScoreNote {
        midi_note: u7::new(47),
        duration: quarter_note_duration.mul_f32(0.5),
        start_time: quarter_note_duration.mul_f32(9.0 * 0.5),
        velocity: u7::new(127),
      },
      MusicalScoreNote {
        midi_note: u7::new(0),
        duration: quarter_note_duration.mul_f32(0.5),
        start_time: quarter_note_duration.mul_f32(10.0 * 0.5),
        velocity: u7::new(127),
      },
    ],
  };

  let output_connection_result = get_midi_output();
  if let Err(_) = output_connection_result {
    println!("Error: failed to create a midi output");
  };
  let midi_out = output_connection_result.unwrap();

  let run_result = run2(score, midi_out);
  if let Err(err) = run_result {
    println!("Error: {}", err);
  }
}

fn get_midi_output() -> Result<MidiOutputPort, Box<dyn Error>> {
  let midi_out = MidiOutput::new("My Test Output")?;
  let mut ports = midi_out.ports();

  // Get an output port (read from console if multiple are available)
  let out_port = match ports.len() {
    0 => return Err("no output port found".into()),
    1 => ports.remove(0),
    _ => {
      println!("\nAvailable output ports:");
      for i in 0..ports.len() {
        println!("{}: {}", i, midi_out.port_name(&ports[i]).unwrap());
      }
      print!("Please select output port: ");
      stdout().flush()?;
      let mut input = String::new();
      stdin().read_line(&mut input)?;
      let parsed: usize = input.trim().parse().unwrap();
      ports.remove(parsed)
    }
  };

  Ok(out_port)
}

fn run2(score: MusicalScore, output_port: MidiOutputPort) -> Result<(), Box<dyn Error>> {
  let playback_position = Instant::now();

  let mut score_reader = MusicalScoreReader::new(score);
  let playback_length = score_reader.length();

  let (tx, rx) = mpsc::channel();

  let midi_out = MidiOutput::new("Test Connection").unwrap();

  let mut output_connection = midi_out.connect(&output_port, "midir_portname").unwrap();

  while playback_position.elapsed() < playback_length {
    let current_note = match score_reader.peek() {
      Some(note) => note,
      None => break,
    };
    let cloned_tx = tx.clone();
    if playback_position.elapsed() > current_note.start_time {
      std::thread::spawn(move || {
        println!("Playing note: {}", current_note.midi_note);
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;

        cloned_tx
          .send([
            NOTE_ON_MSG,
            u8::from(current_note.midi_note),
            u8::from(current_note.velocity),
          ])
          .unwrap();
        sleep(current_note.duration);
        cloned_tx
          .send([
            NOTE_OFF_MSG,
            u8::from(current_note.midi_note),
            u8::from(current_note.velocity),
          ])
          .unwrap();
      });
      score_reader.shift();
    }
    sleep(Duration::from_millis(1));
  }

  for received in rx {
    output_connection.send(&received).unwrap();
  }

  return Ok(());
}