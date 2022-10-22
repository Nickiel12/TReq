use std::process::Command;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, ListState, Paragraph, Tabs},
    Frame, Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // std::process::Command::new("/usr/bin/sh")
    //     .arg("-c")
    //     .arg("vim")
    //     .arg("file")
    //     .spawn()
    //     .expect("Error: Failed to run editor")
    //     .wait()
    //     .expect("Error: Editor returned a non-zero status");

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size = f.size();

    // // Surrounding block
    // let block = Block::default()
    //     .borders(Borders::ALL)
    //     .title("Main block with round corners")
    //     .title_alignment(Alignment::Center)
    //     .border_type(BorderType::Rounded);
    // f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                // Request List Tab
                Constraint::Percentage(10),
                Constraint::Percentage(85),
                Constraint::Percentage(5),
            ]
            .as_ref(),
        )
        .split(f.size());

    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let request_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(content_layout[0]);

    let header_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(request_layout[0]);

    let method = Paragraph::new("GET")
        .style(Style::default().bg(Color::Blue).fg(Color::Black))
        .alignment(Alignment::Center);
    f.render_widget(method, header_layout[0]);

    let url = Block::default()
        .borders(Borders::ALL)
        .title("URL")
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Rounded);
    f.render_widget(url, header_layout[1]);

    let body = Block::default()
        .borders(Borders::ALL)
        .title("BODY / Headers / Options")
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Rounded);
    f.render_widget(body, request_layout[1]);

    // Block ALL Request
    let request_block = Block::default()
        .borders(Borders::ALL)
        .title("Request")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(request_block, content_layout[0]);

    let response_block = Block::default()
        .borders(Borders::ALL)
        .title("Response")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(response_block, content_layout[1]);

    // let request_tab_list = Block::default()
    //     .title("Request History")
    //     .title_alignment(Alignment::Left)
    //     .border_type(BorderType::Rounded)
    //     .borders(Borders::ALL);
    // f.render_widget(request_tab_list, chunks[0]);

    let tabs_spans = vec![
        Spans::from(vec![Span::from("1 Tab Meu bom")]),
        Spans::from(vec![Span::from("2 Tabs")]),
        Spans::from(vec![Span::from("3 Tab")]),
        Spans::from(vec![Span::from("4 Tab")]),
    ];
    let tabs = Tabs::new(tabs_spans)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Tabs"),
        )
        .select(0)
        // .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, chunks[0]);

    let log_block = Block::default().borders(Borders::TOP).title("Logs");
    f.render_widget(log_block, chunks[2]);

    // // Top two inner blocks
    // let top_chunks = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
    //     .split(chunks[0]);

    // // Top left inner block with green background
    // let block = Block::default()
    //     .title(vec![
    //         Span::styled("With", Style::default().fg(Color::Yellow)),
    //         Span::from(" background"),
    //     ])
    //     .style(Style::default().bg(Color::Green));
    // f.render_widget(block, top_chunks[0]);

    // // Top right inner block with styled title aligned to the right
    // let block = Block::default()
    //     .title(Span::styled(
    //         "Styled title",
    //         Style::default()
    //             .fg(Color::White)
    //             .bg(Color::Red)
    //             .add_modifier(Modifier::BOLD),
    //     ))
    //     .title_alignment(Alignment::Right);
    // f.render_widget(block, top_chunks[1]);

    // // Bottom two inner blocks
    // let bottom_chunks = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
    //     .split(chunks[1]);

    // // Bottom left block with all default borders
    // let block = Block::default().title("With borders").borders(Borders::ALL);
    // f.render_widget(block, bottom_chunks[0]);

    // Bottom right block with styled left and right border
    // let block = Block::default()
    //     .title("With styled borders and doubled borders")
    //     .border_style(Style::default().fg(Color::Cyan))
    //     .borders(Borders::LEFT | Borders::RIGHT)
    //     .border_type(BorderType::Double);
    // f.render_widget(block, bottom_chunks[1]);
}
