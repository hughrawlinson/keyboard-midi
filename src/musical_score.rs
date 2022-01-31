extern crate ux;
use std::time::Duration;
use ux::u7;

#[derive(Copy, Clone)]
pub struct MusicalScoreNote {
  pub midi_note: u7,
  pub duration: Duration,
  pub start_time: Duration,
  pub velocity: u7,
}

impl MusicalScoreNote {
  pub fn new(
    midi_note: u7,
    duration: f32,
    start_time: f32,
    velocity: u7,
    quarter_note_duration: Duration,
  ) -> MusicalScoreNote {
    MusicalScoreNote {
      midi_note,
      duration: quarter_note_duration.mul_f32(duration),
      start_time: quarter_note_duration.mul_f32(start_time),
      velocity,
    }
  }
}

#[derive(Clone)]
pub struct MusicalScore {
  pub score: Vec<MusicalScoreNote>,
}

impl MusicalScore {
  pub fn length(&self) -> Duration {
    match self.score.last() {
      Some(note) => (*note).start_time + (*note).duration,
      None => Duration::from_secs(0),
    }
  }
}

pub struct MusicalScoreReader {
  score: MusicalScore,
  position: usize,
}

impl MusicalScoreReader {
  pub fn new(score: MusicalScore) -> MusicalScoreReader {
    MusicalScoreReader {
      score: score,
      position: 0,
    }
  }

  pub fn peek(&self) -> Option<MusicalScoreNote> {
    self.score.score.get(self.position).cloned()
  }

  pub fn shift(&mut self) -> MusicalScoreNote {
    self.position += 1;
    self.score.score[self.position - 1]
  }

  pub fn length(&self) -> Duration {
    self.score.length()
  }
}
