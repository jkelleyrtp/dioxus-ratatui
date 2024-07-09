use dioxus::{
    dioxus_core,
    prelude::{needs_update_any, Component, ScopeId, VirtualDom},
};
use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, EventStream, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use futures::{future::FutureExt, select, StreamExt};
use ratatui::{prelude::*, widgets::*};

pub fn launch(app: fn() -> dioxus_core::Element) {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let dom = VirtualDom::new(app);
            async_main(dom).await.unwrap();
        });
}

pub async fn async_main(mut dom: VirtualDom) -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut reader = EventStream::new();
    let mut app = TuiState {
        command_list: ListState::default(),
    };

    let mut terminal = Terminal::with_options(
        CrosstermBackend::new(stdout()),
        TerminalOptions {
            viewport: Viewport::Fullscreen,
        },
    )?;

    dom.in_runtime(|| needs_update_any(ScopeId::ROOT));

    loop {
        // Draw and then wait for the next event
        terminal.draw(|frame| {
            app.render(frame);
        })?;

        let mut event = reader.next().fuse();

        tokio::select! {
            // Dom is ready, we can render it
            work = dom.wait_for_work() => {
                // println!("virtualdom created some changed");
                // dom.render_immediate(to);
            }

            // Event is ready, we can handle it
            event = event => {
                if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        break;
                    }
                }

                // println!("{:?}", event)
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

struct TuiState {
    command_list: ListState,
}

impl TuiState {
    fn render(&mut self, frame: &mut Frame) {
        // let width = frame.size().width;

        // frame.render_widget(
        //     Block::default()
        //         .title("Block")
        //         .borders(Borders::ALL)
        //         .border_type(BorderType::Rounded),
        //     frame.size(),
        // );

        // a layout that has a title with stats about the program and then the actual console itself
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            // .margin(1)
            .constraints([Constraint::Length(0), Constraint::Min(0)].as_ref())
            .split(frame.size());

        // Render just a paragraph into the top chunks
        frame.render_widget(
            Paragraph::new("dx run -i | rust 1.70 | stable | dx 0.5.2"),
            chunks[0],
        );

        // Render a two-column layout
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Max(20), Constraint::Min(0)].as_ref())
            .split(chunks[1]);

        // The left column is a list of commands that we can interact with
        let commands = vec![
            "Commands",
            "  Console",
            "  Configure",
            "  Edit",
            "  Add dep",
            "  Simulator",
            "  Bundle",
            "  Deploy",
            "  Lookbook",
            "  HTML to RSX",
            "  Builds",
            "  Debug",
            "  Visualize",
            "  Lint/Check",
            "  Share",
            "  Shortcuts",
            "  Learn",
            "  Help",
        ];

        let commands = commands.iter().map(|c| Span::styled(*c, Style::default()));

        let commands = List::new(commands)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        frame.render_stateful_widget(commands, chunks[0], &mut self.command_list);

        // The right is the output of that command
        let output = vec![
            "Output",
            "  Compiling dioxus v0.1.0 (/Users/kevin/Projects/dioxus)",
            "    Finished dev [unoptimized + debuginfo] target(s) in 0.23s",
            "  Running `target/debug/dioxus`",
            "    dx run -i | rust 1.70 | stable | dx 0.5.2
        ",
        ];

        let output = output.iter().map(|c| Span::styled(*c, Style::default()));

        let output = List::new(output)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        frame.render_widget(output, chunks[1]);
    }
}

struct RealDom {}

mod _imports {
    use ratatui::{
        backend::{Backend, CrosstermBackend},
        layout::{Constraint, Direction, Layout, Rect},
        style::{Color, Modifier, Style},
        terminal::{Frame, Terminal},
        text::{Line, Span},
        widgets::{Bar, BarChart, BarGroup, Block, Paragraph},
    };
    use std::{
        error::Error,
        io,
        time::{Duration, Instant},
    };

    use crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
}
