use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::Paragraph,
};
use std::io;

fn draw(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Hello, World!\nPress q or Esc to quit."),
        frame.area(),
    );
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = (|| {
        loop {
            terminal.draw(draw)?;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
                {
                    break;
                }
            }
        }
        Ok(())
    })();
    ratatui::restore();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn renders_greeting_and_exit_hint() {
        let mut terminal = Terminal::new(TestBackend::new(40, 3)).unwrap();
        terminal.draw(draw).unwrap();
        let buffer = terminal.backend().buffer();
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        assert!(text.contains("Hello, World!"));
        assert!(text.contains("Press q or Esc to quit."));
    }
}
