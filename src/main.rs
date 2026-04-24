use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::{DefaultTerminal, Frame};

struct App {
    todos: Vec<String>,
    selected: usize,
    //completed: Vec<Todo>,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer_layout[0]);

    let items = vec![ListItem::new("Test1"), ListItem::new("Test2")];

    let list = List::new(items);
    let block = Block::default().title("Mein Block").borders(Borders::ALL);
    let p = Paragraph::new("Hello, World!").block(Block::new().borders(Borders::ALL));

    frame.render_widget(block, outer_layout[0]);
    frame.render_widget(list, inner_layout[0]);
    frame.render_widget(p, outer_layout[1]);
}
