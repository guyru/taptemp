use clap::Parser;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseEventKind,
    },
    execute, terminal,
};
use std::collections::VecDeque;
use std::io;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(author, version, about = "Measure your tap tempo")]
struct Cli {
    /// Number of samples to take for tempo calculation
    #[arg(short, long, default_value_t = 5)]
    sample_size: usize,

    /// Set the time in seconds to reset the computation
    #[arg(short, long, default_value_t = 5)]
    timeout: usize,

    /// Precision of the BPM output
    #[arg(short, long, default_value_t = 0)]
    precision: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    if args.sample_size == 0 {
        eprintln!("Error: sample size must be positive.");
        std::process::exit(1);
    }

    setup_terminal(&mut io::stdout());

    println!("Tap any key to measure tempo. Press 'Esc' to exit.\r");

    let mut tap_tempo = TapTempo::new(args.sample_size, args.timeout);

    while block_until_key_press() {
        if let Some(bpm) = tap_tempo.tap() {
            println!(
                "Current tempo: {:.precision$} BPM\r",
                bpm,
                precision = args.precision
            );
        }
    }

    cleanup_terminal(&mut io::stdout());
    Ok(())
}

struct TapTempo {
    sample_size: usize,
    timeout: Duration,
    timestamps: VecDeque<Instant>,
}

impl TapTempo {
    fn new(sample_size: usize, timeout_secs: usize) -> Self {
        TapTempo {
            sample_size,
            timeout: Duration::from_secs(timeout_secs as u64),
            timestamps: VecDeque::with_capacity(sample_size),
        }
    }

    fn tap(&mut self) -> Option<f64> {
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

/// Blocks the execution until a key press or relevant mouse event is detected.
///
/// Returns `false` only if the 'Esc' key or 'Ctrl+C' is pressed; otherwise, returns `true`
/// when a key or relevant mouse event is detected.
fn block_until_key_press() -> bool {
    // we need the loop, so we can ignore unwanted mouse events.
    loop {
        // Block until an event is available
        if let Ok(event) = event::read() {
            match event {
                // Handle key events
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Esc => return false,
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        return false;
                    }
                    _ => return true,
                },
                // Handle mouse events
                Event::Mouse(mouse_event) => match mouse_event.kind {
                    MouseEventKind::Down(_) => return true,
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn setup_terminal(stdout: &mut io::Stdout) {
    terminal::enable_raw_mode().expect("Can't put terminal into raw mode");
    execute!(stdout, EnableMouseCapture).expect("Can't capture mouse");
}

fn cleanup_terminal(stdout: &mut io::Stdout) {
    terminal::disable_raw_mode().expect("Can't disable termnal raw mode");
    execute!(stdout, DisableMouseCapture).expect("Can't disable mouse capture");
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
