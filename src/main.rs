use clap::Parser;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseEventKind,
    },
    execute, terminal,
};
use std::io;

mod tap_tempo;
use tap_tempo::TapTempo;

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
