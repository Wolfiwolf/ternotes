use crate::{config, notes};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Paragraph},
};
use std::io::Write;

struct YankState<'a> {
    current_line: usize,
    lines: Vec<&'a str>,
}

fn go_to_next_line(lines: &Vec<&str>, start_line: usize, godown: bool) -> usize {
    let mut current_line: i32 = start_line as i32;

    current_line += if godown { 1 } else { -1 };

    loop {
        if godown {
            if current_line >= lines.len() as i32 {
                current_line = start_line as i32;
                break;
            }
        } else {
            if current_line < 0 {
                current_line = start_line as i32;
                break;
            }
        }

        if !lines[current_line as usize].is_empty() {
            break;
        }

        current_line += if godown { 1 } else { -1 };
    }

    current_line as usize
}

fn do_yank(line: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conf = config::get()?;

    match conf.clipboard {
        config::ClipboardManager::WlCopy => {
            std::process::Command::new("wl-copy")
                .arg(line)
                .spawn()?
                .wait()?;
        }
        config::ClipboardManager::XClip => {
            let mut child = std::process::Command::new("xclip")
                .args(["-selection", "clipboard"])
                .stdin(std::process::Stdio::piped())
                .spawn()?;

            child.stdin.as_mut().unwrap().write_all(line.as_bytes())?;

            drop(child.stdin.take());

            child.wait()?;
        }
        config::ClipboardManager::None => {
            return Err(format!(
                "There is no valid clipboard manager specified in the {} file.",
                config::get_config_filepath()
            ))?;
        }
    }

    Ok(())
}

fn handle_key(
    ystate: &mut YankState,
    keycode: KeyCode,
) -> Result<bool, Box<dyn std::error::Error>> {
    match keycode {
        KeyCode::Char('j') => {
            ystate.current_line = go_to_next_line(&ystate.lines, ystate.current_line, true)
        }
        KeyCode::Char('k') => {
            ystate.current_line = go_to_next_line(&ystate.lines, ystate.current_line, false)
        }
        KeyCode::Char('y') => {
            do_yank(ystate.lines[ystate.current_line])?;
            return Ok(true);
        }
        KeyCode::Char('q') => return Ok(true),
        _ => {}
    }

    Ok(false)
}

fn render(frame: &mut Frame, ystate: &YankState) {
    let mut lines: Vec<Line> = Vec::with_capacity(0);

    for (i, line) in ystate.lines.iter().enumerate() {
        let l: String;

        if i == ystate.current_line {
            l = format!("-> {}<-", *line);
        } else {
            l = format!("{}: {}", i, *line);
        }

        let rline: Line = Line::from(l);

        lines.push(rline);
    }

    let paragraph = Paragraph::new(Text::from(lines))
        .block(Block::default())
        .style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, frame.area());
}

pub fn _yank(note_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let fcontent = notes::get_note_content(note_id)?;
    let lines: Vec<&str> = fcontent.lines().collect();

    let mut ystate = YankState {
        current_line: 0,
        lines: lines,
    };

    color_eyre::install()?;
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(|f| render(f, &ystate))?;
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                let res = handle_key(&mut ystate, key_event.code);
                if res.is_err() {
                    ratatui::restore();
                    return Err(res.unwrap_err());
                }

                /* Loop should close */
                if res.unwrap() {
                    break;
                }
            }
            _ => {}
        }
    }
    ratatui::restore();

    return Ok(());
}
