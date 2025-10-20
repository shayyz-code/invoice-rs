use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};
use std::time::Instant;

pub struct SplashScreen {
    start: Instant,
    done: bool,
}

impl SplashScreen {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            done: false,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let elapsed = self.start.elapsed().as_secs_f32();

        // Animate intensity with sine wave
        let intensity = ((elapsed * 2.0).sin() + 1.0) / 2.0;
        let purple = Color::Rgb(
            (120.0 * intensity + 80.0) as u8,
            (40.0 * intensity + 0.0) as u8,
            (180.0 * intensity + 100.0) as u8,
        );

        let ascii_art = r"
            _____ _   ___      ______ _____ _____ ______      _____   _____
           |_   _| \ | \ \    / / __ \_   _/ ____|  ____|    |  __ \ / ____|
             | | |  \| |\ \  / / |  | || || |    | |__ ______| |__) | (___
             | | | . ` | \ \/ /| |  | || || |    |  __|______|  _  / \___ \
            _| |_| |\  |  \  / | |__| || || |____| |____     | | \ \ ____) |
           |_____|_| \_|   \/   \____/_____\_____|______|    |_|  \_\_____/
        ";

        // Find minimum indent to remove
        let min_indent = ascii_art
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
            .min()
            .unwrap_or(0);

        // Trim left spaces and collect lines
        let raw_lines: Vec<String> = ascii_art
            .lines()
            .map(|line| {
                if line.len() > min_indent {
                    line[min_indent..].to_string()
                } else {
                    line.to_string()
                }
            })
            .collect();

        // Compute max width
        let max_width = raw_lines.iter().map(|line| line.len()).max().unwrap_or(0);

        // Pad each line to max width
        let lines: Vec<Line> = raw_lines
            .into_iter()
            .map(|line| {
                let padded = format!("{:<width$}", line, width = max_width);
                Line::from(Span::styled(padded, Style::default().fg(Color::Cyan)))
            })
            .collect();

        // Add RustRaccoon
        let mut final_lines = lines;
        final_lines.push(Line::from(vec![
            Span::styled(
                "shayyzcode",
                Style::default().fg(purple).add_modifier(Modifier::BOLD),
            ),
            Span::styled("\'s ", Style::default().fg(Color::Rgb(180, 140, 255))),
            Span::styled("invoice-rs", Style::default().fg(Color::Rgb(205, 89, 9))),
        ]));

        let paragraph = Paragraph::new(Text::from(final_lines))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::NONE));

        frame.render_widget(paragraph, frame.area());

        // Show splash for 3 seconds
        if elapsed > 3.0 {
            self.done = true;
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}
