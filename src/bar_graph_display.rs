use ratatui::{
    widgets::{BarChart, Block, Borders},
    DefaultTerminal,
};
use std::cmp::max;

pub struct BarGraphDisplay {
    terminal: DefaultTerminal,
    bpm_history: Vec<f64>,
}

impl BarGraphDisplay {
    pub fn new() -> Self {
        let mut terminal = ratatui::init();
        terminal.clear().expect("Failed to clear terminal"); // Clear the terminal before drawing
        let mut s = Self {
            terminal,
            bpm_history: Vec::new(),
        };
        s.display().expect("Failed to display bar graph");
        s
    }

    pub fn add_bpm(&mut self, bpm: f64) {
        self.bpm_history.push(bpm);
    }

    pub fn display(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.draw(|f| {
            let size = f.area();
            /*
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);
            */

            let bar_width = 3;
            let bar_gap = 1;
            let max_bars = (size.width as usize + bar_gap) / (bar_width + bar_gap);

            let start = if self.bpm_history.len() > max_bars {
                self.bpm_history.len() - max_bars
            } else {
                0
            };

            let max_height = max(
                120,
                self.bpm_history[start..]
                    .iter()
                    .map(|&b| b as u64)
                    .max()
                    .unwrap_or(0),
            );

            let bars: Vec<(&str, u64)> = self.bpm_history[start..]
                .iter()
                .map(|&bpm| ("BPM", bpm as u64))
                .collect();

            let title = "Tap any key to measure tempo. Press 'Esc' to exit.";
            let bar_chart = BarChart::default()
                .block(Block::default().title(title).borders(Borders::ALL))
                .data(&bars)
                .bar_width(bar_width as u16)
                .bar_gap(bar_gap as u16)
                .max(max_height)
                .value_style(ratatui::style::Style::default().fg(ratatui::style::Color::Green))
                .label_style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
                .bar_style(ratatui::style::Style::default().fg(ratatui::style::Color::Blue));

            f.render_widget(bar_chart, f.area());
        })?;
        Ok(())
    }
}

impl Drop for BarGraphDisplay {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
