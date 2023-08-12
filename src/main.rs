use std::{io::{self, Stdout, Error}, time::Duration};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{self, Event, KeyCode, KeyEventKind}};
use ratatui::{Terminal, prelude::{CrosstermBackend, Layout, Direction, Constraint}, widgets::{Paragraph, Borders, Block, BorderType}, style::{self, Color, Style}};
use std::env;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::process::Command;

lazy_static! {

    static ref COMMANDS_MAP: HashMap<&'static str, Command> = {

        let fzf_cd: Command = {
            let mut cmd = Command::new("Powershell");
            cmd.arg("fzf | Split-Path | cd");
            cmd
        };

        let fzf_edit: Command = {
            let mut cmd = Command::new("Powershell");
            cmd.arg("fzf | $ { code $_ }");
            cmd
        };

        let mut m = HashMap::new();
        m.insert("fe", fzf_cd);
        m.insert("ef", fzf_edit);
        return m
    };

}

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

fn handle_command(input: String) -> Result<(), Error>{
    if let Some(command) = COMMANDS_MAP.get_mut(&input.as_str()){
        command.output()?;
    }
    else{
        let mut cmd = Command::new("Powershell");
            cmd.arg(format!("{input}"));
            cmd.output()?;
    } 
    Ok(())
}

fn get_footer<'a>(input: String, dir: String) -> Paragraph<'a> {
    let footer = Paragraph::new(format!(" {input}"))
        .style(style::Style::default().fg(Color::LightCyan))
        .block(
            Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title(format!(" PS {dir} "))
            .border_type(BorderType::Plain)
        );
    return footer;
}

fn get_body<'a>() -> Paragraph<'a> {
    let body = Paragraph::new(" Welcome to the CLI app!")
        .style(style::Style::default().fg(Color::LightCyan))
        .block(
            Block::default()
            .borders(Borders::ALL)
            .title(format!(" body "))
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain)
        );
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
                    KeyCode::Enter => {
                        let res = handle_command(input.clone());
                        match res{
                            Ok(()) => (),
                            Err(x) => println!("{x}")
                        }
                    },
                    _ => {}
                }

            }
        }

    })
}

