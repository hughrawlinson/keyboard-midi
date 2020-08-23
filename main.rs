extern crate midir;

use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

use midir::MidiOutput;
use midir::MidiOutputConnection;

type NoteStartTime = u16;
type MusicalDuration = u8;
type MusicalNote = u8;
type MusicalScore<TimeUnit> = Vec<(MusicalNote, TimeUnit)>;

fn main() {
    let score: MusicalScore<MusicalDuration> = vec![
        (66, 4),
        (65, 3),
        (63, 1),
        (61, 6),
        (59, 2),
        (58, 4),
        (56, 4),
        (54, 4),
        (0, 0),
    ];

    let tempo = 150;

    let output_connection_result = get_midi_output_connection();
    if let Err(err) = output_connection_result {
        println!("Error: {}", err.description());
    };
    let midi_out = output_connection_result.unwrap();

    let run_result = run2(score, tempo, midi_out);
    if let Err(err) = run_result {
        println!("Error: {}", err.description());
    }
}

fn get_midi_output_connection() -> Result<MidiOutputConnection, Box<dyn Error>> {
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

    println!("\nOpening connection");
    let conn_out = midi_out.connect(out_port, "midir-test")?;
    println!("Connection open. Listen!");
    return Ok(conn_out);
}

fn run2(
    score: MusicalScore<MusicalDuration>,
    tempo: u8,
    outputConnection: MidiOutputConnection,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn duration_score_to_start_time_score(
    score: MusicalScore<MusicalDuration>,
) -> MusicalScore<NoteStartTime> {
    score.iter().fold(vec![(0, 0)], |acc, el| {
        let mut v = acc.clone();
        match acc.last() {
            None => v.push((el.0 as u8, (el.1) as u16)),
            Some(w) => v.push((el.0 as u8, el.1 as u16 + w.1 as u16)),
        };
        v
    })
}

fn run(score: MusicalScore<MusicalDuration>, speed: u16) -> Result<(), Box<dyn Error>> {
    let score_duration: u16 = speed * score.iter().fold(0, |sum, pair| sum + pair.1) as u16;
    let timescore = duration_score_to_start_time_score(score);
    let abstimescore: Vec<(u8, u16)> = timescore.iter().map(|el| (el.0, el.1 * speed)).collect();
    let mut position = 0;
    let mut noteindex: i16 = -1;
    let mut lastnotetime: i16 = -1;
    println!("{:?}", abstimescore);

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

    println!("\nOpening connection");
    let mut conn_out = midi_out.connect(out_port, "midir-test")?;
    println!("Connection open. Listen!");
    {
        // Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
        let mut play_note = move |note: u8, filter_pitch: f32| {
            const NOTE_ON_MSG: u8 = 0x90;
            const NOTE_OFF_MSG: u8 = 0x80;
            const VELOCITY: u8 = 0x64;
            const CONTROL_CHANGE_MESSAGE: u8 = 0xB0;

            // We're ignoring errors in here
            let _ = conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]);

            let v: u8 = (127.0 * filter_pitch) as u8;
            let _ = conn_out.send(&[CONTROL_CHANGE_MESSAGE, 1, v as u8]);
            sleep(Duration::from_millis(speed as u64));
            let _ = conn_out.send(&[NOTE_OFF_MSG, note, 0]);
        };

        while position < score_duration {
            position = position + 10;
            sleep(Duration::from_millis(10));
            let note = abstimescore[(noteindex + 1) as usize];
            println!("{:?}, lastnotetime: {}", note.1, lastnotetime);
            if note.1 as i16 > lastnotetime {
                println!("Inside Loop");
                lastnotetime = note.1 as i16;
                noteindex = noteindex + 1;
                println!(
                    "note: {}, position: {}",
                    note.0,
                    (position as f32 / score_duration as f32)
                );
                play_note(note.0, position as f32 / score_duration as f32);
                // thread::spawn(move || {
                //     play_note(note.0, (scoreDuration/position) as f32);
                // });
            }
        }
    }

    println!("\nClosing connection");
    println!("Connection closed");
    Ok(())
}
