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
    midi_note: &u7,
    duration: &Duration,
    start_time: &Duration,
    velocity: &u7,
  ) -> MusicalScoreNote {
    MusicalScoreNote {
      midi_note: midi_note.clone(),
      duration: duration.clone(),
      start_time: start_time.clone(),
      velocity: velocity.clone(),
    }
  }
}

#[derive(Copy, Clone)]
pub struct TimingSystem {
  pub thirtysecond: Duration,
  pub sixteenth: Duration,
  pub eigth: Duration,
  pub quarter: Duration,
  pub half: Duration,
  pub whole: Duration,
}

impl TimingSystem {
  pub fn new(bpm: u32) -> TimingSystem {
    let quarter = Duration::from_secs(60) / bpm;
    let eigth = quarter / 2;
    let sixteenth = eigth / 2;
    let thirtysecond = sixteenth / 2;
    let half = quarter * 2;
    let whole = half * 2;

    TimingSystem {
      thirtysecond,
      sixteenth,
      eigth,
      quarter,
      half,
      whole,
    }
  }
}

#[derive(Clone)]
pub struct MusicalScore {
  pub score: Vec<MusicalScoreNote>,
  pub timingsystem: TimingSystem,
}

impl MusicalScore {
  pub fn new(bpm: u32) -> MusicalScore {
    MusicalScore {
      timingsystem: TimingSystem::new(bpm),
      score: vec![],
    }
  }

  pub fn add_note(
    &mut self,
    midi_note: &u7,
    duration: &Duration,
    start_time: &Duration,
    velocity: &u7,
  ) {
    self.score.push(MusicalScoreNote::new(
      midi_note, duration, start_time, velocity,
    ));
  }

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
