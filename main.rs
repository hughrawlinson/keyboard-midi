extern crate midir;

use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

use midir::MidiOutput;

const ZERO_DURATION: Duration = Duration::from_millis(0);

type MusicalDuration = f32;
type MusicalNote = u8;
type MusicalScore<TimeUnit> = Vec<(MusicalNote, TimeUnit)>;

fn main() {
    let score: MusicalScore<MusicalDuration> = vec![
        (54, 0.5),
        (53, 0.5),
        (51, 0.5),
        (49, 0.5),
        (47, 0.5),
        (46, 0.5),
        (44, 0.5),
        (42, 0.5),
        (0, 0.0),
    ];

    let tempo = 200;

    let output_connection_result = get_midi_output();
    if let Err(_) = output_connection_result {
        println!("Error: failed to create a midi output");
    };
    let midi_out = output_connection_result.unwrap();

    let run_result = run2(score, tempo, midi_out);
    if let Err(err) = run_result {
        println!("Error: {}", err);
    }
}

fn get_midi_output() -> Result<usize, Box<dyn Error>> {
    let midi_out = MidiOutput::new("My Test Output")?;

    // Get an output port (read from console if multiple are available)
    let out_port = match midi_out.port_count() {
        0 => return Err("no output port found".into()),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(0).unwrap()
            );
            0
        }
        _ => {
            println!("\nAvailable output ports:");
            for i in 0..midi_out.port_count() {
                println!("{}: {}", i, midi_out.port_name(i).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            input.trim().parse()?
        }
    };

    Ok(out_port)
    // println!("\nOpening connection");
    // let conn_out = midi_out.connect(out_port, "midir-test")?;
    // println!("Connection open. Listen!");
    // return Ok(conn_out);
}

fn total_length_from_score(score: &MusicalScore<(Duration, Duration)>) -> Duration {
    match score.last() {
        Some((_, (_, end_time))) => *end_time,
        _ => ZERO_DURATION,
    }
}

fn run2(
    score: MusicalScore<MusicalDuration>,
    tempo: u8,
    output_port: usize,
) -> Result<(), Box<dyn Error>> {
    let playback_position = Instant::now();

    let quarter_note_duration = Duration::from_secs(60) / tempo.into();

    let mut time_score = duration_score_to_start_time_score(score, quarter_note_duration);
    let playback_length = total_length_from_score(&time_score);

    while playback_position.elapsed() < playback_length {
        let (note, (start_time, end_time)) = time_score[0];

        if playback_position.elapsed() > start_time {
            println!("Playing note: {}", note);
            std::thread::spawn(move || {
                const NOTE_ON_MSG: u8 = 0x90;
                const NOTE_OFF_MSG: u8 = 0x80;
                const VELOCITY: u8 = 0x64;

                let midi_out = match MidiOutput::new("Test Connection") {
                    Ok(output) => output,
                    Err(err) => panic!(err),
                };

                let mut output_connection = match midi_out.connect(output_port, "midir_portname") {
                    Ok(oc) => oc,
                    Err(err) => panic!(err),
                };
                {
                    // We're ignoring errors in here
                    let _ = output_connection.send(&[NOTE_ON_MSG, note, VELOCITY]);
                    sleep(end_time - start_time);
                    let _ = output_connection.send(&[NOTE_OFF_MSG, note, 0]);
                }
            });
            time_score.remove(0);
        }
        sleep(Duration::from_millis(10));
    }
    return Ok(());
}

fn duration_score_to_start_time_score(
    score: MusicalScore<MusicalDuration>,
    quarter_note_duration: Duration,
) -> MusicalScore<(Duration, Duration)> {
    score
        .iter()
        .fold(vec![(0, (ZERO_DURATION, ZERO_DURATION))], |acc, el| {
            let mut v = acc.clone();
            match acc.last() {
                None => v.push((el.0, (ZERO_DURATION, quarter_note_duration.mul_f32(el.1)))),
                Some(w) => v.push((
                    el.0,
                    ((w.1).1, quarter_note_duration.mul_f32(el.1) + (w.1).1),
                )),
            };
            v
        })
}
