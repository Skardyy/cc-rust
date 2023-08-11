use std::{io::{self, Stdout, Error}, time::Duration};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{self, Event, KeyCode, KeyEventKind}, cursor::EnableBlinking, queue, style::SetBackgroundColor};
use ratatui::{Terminal, prelude::{CrosstermBackend, Layout, Direction, Constraint}, widgets::{Paragraph, Borders, Block}, style::{self, Color}};
use std::env;


fn main() -> Result<(), io::Error> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Error> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    let enable_blinking = EnableBlinking;
    queue!(stdout, enable_blinking)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>,) -> Result<(), Error> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}   

fn get_cwd() -> String{
    let current_dir = env::current_dir().unwrap();
    match current_dir.to_str(){
        Some(x) => x.to_string(),
        _ => String::new()
    }
}

fn handle_command(input: String){

}

fn get_footer<'a>(input: String, dir: String) -> Paragraph<'a> {
    let footer = Paragraph::new(format!(" PS {dir} {input}"))
        .style(style::Style::default().fg(Color::LightCyan))
        .block(Block::default().borders(Borders::ALL));
    return footer;
}

fn get_body<'a>() -> Paragraph<'a> {
    let body = Paragraph::new(" Welcome to the CLI app!")
        .style(style::Style::default().fg(Color::LightCyan))
        .block(Block::default().borders(Borders::ALL));
    return body;
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Error> {

    let mut dir = get_cwd();
    let mut input = String::new();
    
    Ok(loop {

        terminal.draw(|frame| {

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(frame.size());

            let body = get_body();
            let footer = get_footer(input.clone(), dir.clone());

            frame.render_widget(body, chunks[0]);
            frame.render_widget(footer, chunks[1]);

        })?;
        
        if event::poll(Duration::from_secs(0))? {

            if let Event::Key(key) = event::read()? {

                match key.code {
                    KeyCode::Char(c) if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        if c == 'c' {
                            break;
                        }
                    },
                    KeyCode::Char(c) if key.kind == KeyEventKind::Release => input.push(c),
                    KeyCode::Backspace if key.kind == KeyEventKind::Release => {
                        input.pop();
                    },
                    KeyCode::Enter => handle_command(input.clone()),
                    _ => {}
                }

            }
        }

    })
}

