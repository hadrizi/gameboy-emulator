use std::{sync::mpsc, thread, time::{Duration, Instant}, io};
use crossterm::{terminal::{enable_raw_mode, disable_raw_mode}, event::{self, Event as CEvent, KeyCode}};
use tui::{backend::CrosstermBackend, Terminal, layout::{Layout, Direction, Constraint, Alignment}, widgets::{Paragraph, Block, Borders, BorderType, Tabs, ListState}, style::{Style, Color, Modifier}, text::{Spans, Span}};

use crate::cpu::CPU;

mod ui;

pub struct Debugger {
    cpu: CPU
}

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    CPU,
    VRAM,
    Memory
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::CPU => 0,
            MenuItem::VRAM => 1,
            MenuItem::Memory => 2,
        }
    }
}

impl Debugger {
    pub fn new(cpu: CPU) -> Debugger {
        Debugger{
            cpu
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.cpu.check_table();
        enable_raw_mode().expect("can run in raw mode");
        let (tx, rx) = mpsc::channel();
        let tick_rate = Duration::from_millis(200);
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).expect("poll works") {
                    if let CEvent::Key(key) = event::read().expect("can read events") {
                        tx.send(Event::Input(key)).expect("can send events");
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    if let Ok(_) = tx.send(Event::Tick) {
                        last_tick = Instant::now();
                    }
                }
            }
        });

        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        let menu_titles = vec!["CPU", "VRAM", "Memory"];
        let mut active_menu_item = MenuItem::CPU;

        let mut asm_list_state = ListState::default();
        let asm_map = self.cpu.disassemble();

        loop {
            terminal.draw(|rect| {
                let size = rect.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Min(2),
                            Constraint::Length(3),
                        ]
                        .as_ref(),
                    )
                    .split(size);
                
                let controls = ui::build_cpu_controls();

                let menu = menu_titles
                    .iter()
                    .map(|t| {
                        let (first, rest) = t.split_at(1);
                        Spans::from(vec![
                            Span::styled(
                                first,
                                Style::default()
                                    .fg(Color::Yellow)
                                    .add_modifier(Modifier::UNDERLINED),
                            ),
                            Span::styled(rest, Style::default().fg(Color::White)),
                        ])
                    })
                    .collect();

                let tabs = Tabs::new(menu)
                    .select(active_menu_item.into())
                    .block(Block::default().title("Menu").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().fg(Color::Yellow))
                    .divider(Span::raw("|"));
                
                rect.render_widget(tabs, chunks[0]);
                rect.render_widget(controls, chunks[2]);

                match active_menu_item {
                    MenuItem::CPU => {
                        let cpu_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(60), Constraint::Percentage(40)].as_ref()
                        )
                        .split(chunks[1]);
                        let registers_chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints(
                                [Constraint::Percentage(50), Constraint::Percentage(50)].as_ref()
                            )
                            .split(cpu_chunks[1]);
                        
                        let (asm, register, stack) = ui::render_cpu(
                            &asm_map,
                            cpu_chunks[0].height,
                            &self.cpu,
                        );
                        asm_list_state.select(Some(asm.1));
                        rect.render_stateful_widget(asm.0, cpu_chunks[0], &mut asm_list_state);
                        rect.render_widget(register, registers_chunks[0]);
                        rect.render_widget(stack, registers_chunks[1]);
                    },
                    MenuItem::VRAM => { },
                    MenuItem::Memory => { }
                }
            })?;

            match rx.recv()? {
                Event::Input(event) => match event.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        disable_raw_mode()?;
                        terminal.clear()?;
                        terminal.show_cursor()?;
                        break;
                    }
                    KeyCode::Char('c') => active_menu_item = MenuItem::CPU,
                    KeyCode::Char('m') => active_menu_item = MenuItem::Memory,
                    KeyCode::Char('v') => active_menu_item = MenuItem::VRAM,
                    KeyCode::Char(' ') => {
                        self.cpu.clock();
                        while self.cpu.cycles != 0 {
                            self.cpu.clock();
                        }
                    },
                    _ => {}
                },
                Event::Tick => {}
            }
        }

        Ok(())
    }

    // fn get_asm_list() -> Vec<_> {
    // }
}