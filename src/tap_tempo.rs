use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct TapTempo {
    sample_size: usize,
    timeout: Duration,
    timestamps: VecDeque<Instant>,
}

impl TapTempo {
    pub fn new(sample_size: usize, timeout_secs: usize) -> Self {
        TapTempo {
            sample_size,
            timeout: Duration::from_secs(timeout_secs as u64),
            timestamps: VecDeque::with_capacity(sample_size),
        }
    }

    pub fn tap(&mut self) -> Option<f64> {
        let now = Instant::now();

        // Remove timestamps older than the sample size
        self.timestamps.truncate(self.sample_size);

        // Reset timestamps if the duration since the last timestamp exceeds the timeout
        if let Some(&last_timestamp) = self.timestamps.front() {
            if now.duration_since(last_timestamp) > self.timeout {
                self.timestamps.clear();
            }
        }

        // Store the timestamp
        self.timestamps.push_front(now);

        // Calculate average duration between key presses
        if self.timestamps.len() > 1 {
            let average_duration = self
                .timestamps
                .front()
                .unwrap()
                .duration_since(*self.timestamps.back().unwrap())
                / (self.timestamps.len() - 1) as u32;

            let tempo = 60_000.0 / average_duration.as_millis() as f64; // Convert to BPM

            return Some(tempo);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_tap_tempo_single_tap() {
        let mut tap_tempo = TapTempo::new(5, 5);
        assert_eq!(tap_tempo.tap(), None);
    }

    #[test]
    fn test_tap_tempo_multiple_taps() {
        let mut tap_tempo = TapTempo::new(5, 5);
        tap_tempo.tap();
        sleep(Duration::from_millis(500));
        let bpm = tap_tempo.tap().unwrap();
        assert!(bpm > 100.0 && bpm < 130.0); // Expecting around 120 BPM
    }

    #[test]
    fn test_tap_tempo_average() {
        let mut tap_tempo = TapTempo::new(5, 5);
        tap_tempo.tap();
        tap_tempo.tap();
        sleep(Duration::from_millis(500));
        let bpm = tap_tempo.tap().unwrap();
        assert!(bpm > 239.0 && bpm < 241.0); // Expecting around 240 BPM
    }

    #[test]
    fn test_tap_tempo_timeout() {
        let mut tap_tempo = TapTempo::new(5, 1);
        tap_tempo.tap();
        sleep(Duration::from_secs(2));
        assert_eq!(tap_tempo.tap(), None);
    }

    #[test]
    fn test_tap_tempo_sample_size() {
        let mut tap_tempo = TapTempo::new(3, 5);
        tap_tempo.tap();
        tap_tempo.tap();
        tap_tempo.tap();
        sleep(Duration::from_millis(500));
        tap_tempo.tap();
        sleep(Duration::from_millis(500));
        tap_tempo.tap();
        sleep(Duration::from_millis(500));
        let bpm = tap_tempo.tap().unwrap();
        assert!(bpm > 100.0 && bpm < 130.0); // Expecting around 120 BPM
    }
}
