use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap};
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
    let mut list_state = ListState::default().with_selected(Some(0));

    loop {
        terminal.draw(|frame| render(frame, &mut list_state))?;
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Char('k') | KeyCode::Up => list_state.select_previous(),
                KeyCode::Char('j') | KeyCode::Down => list_state.select_next(),
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame, list_state: &mut ListState) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer_layout[0]);

    let text: &str = "Die Welt, in der wir leben, ist eine kapitalistische Welt. Kapitalismus, das bedeutet, dass die gesamte wirtschaftliche Tätigkeit, dass jeder Bereich des gesellschaftlichen Lebens darauf ausgerichtet ist, dass das Kapital möglichst hohe Profite erzielen kann; das bedeutet, dass die Produktionsmittel in den Händen von Wenigen sind und nicht von der Gesellschaft für die Gesellschaft genutzt werden; das bedeutet, dass die Bedürfnisse der Menschen nicht Ziel und Zweck der Produktion sind, sondern regelmäßig auf der Strecke bleiben; das bedeutet unermesslichen Reichtum auf der Seite der Wenigen und Armut, Not, Mangel auf der Seite der Vielen. Kapitalismus bedeutet schließlich auch, dass die politische Macht, allem Gerede von der „Demokratie“ zum Trotz, letztlich in den Händen einer kleinen Minderheit von Kapitalisten, also in den Händen derjenigen Klasse liegt, der auch die Unternehmen gehören. Unter dem Zwang, ständig für die Erhaltung und Erhöhung ihrer Profite zu kämpfen, ist es im Interesse der Kapitalisten, den Lebensstandard der arbeitenden Massen nach unten zu drücken und die Welt mit Kriegen um Ressourcen, Absatzmärkte und Investitionsmöglichkeiten zu überziehen. \n 
        Das kapitalistische System gerät tendenziell in immer schwerere Krisen, die Massen von Arbeitern ins Elend stürzen und auf eindrückliche Weise immer wieder beweisen: In der heutigen Epoche des Imperialismus ist der Kapitalismus historisch überholt. Der Kapitalismus lässt sich nicht den menschlichen Bedürfnissen entsprechend gestalten, das Privateigentum an Produktionsmitteln steht tendenziell im immer schärferen Widerspruch zum gesellschaftlichen Charakter der Produktion. Sozialismus ist die nächste Gesellschaftsform, die der Entwicklung der Produktivkräfte durch gesellschaftliche Planung gerecht wird. Ein besseres Leben gibt es nur in einer anderen Gesellschaft.";

    let p = Paragraph::new(text)
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .title("Grundlagenschulung")
                .borders(Borders::all())
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true })
        .scroll((1, 0));

    render_list(frame, inner_layout[1], list_state);
    frame.render_widget(p, inner_layout[0]);
}

pub fn render_list(frame: &mut Frame, area: Rect, list_state: &mut ListState) {
    let items = vec![
        ListItem::new("Test1"),
        ListItem::new("Test2"),
        ListItem::new("Test3"),
    ];

    //let list = List::new(items).block(Block::default().borders(Borders::all()));

    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, area, list_state);
}
