use indexmap::IndexMap;
use tui::{widgets::{Paragraph, Block, Borders, BorderType, List, ListItem}, text::{Spans, Span}, style::{Style, Color, Modifier}, layout::Alignment};

use crate::cpu::{CPU, Flag};

#[allow(overflowing_literals)]
pub fn render_cpu<'a>(
    disas: &'a IndexMap<u16, String>, asm_height: u16, cpu: &CPU, 
) -> ((List<'a>, usize), Paragraph<'a>, Paragraph<'a>) {
    let current: usize = if asm_height % 2 == 1 {
        asm_height as usize / 2 - 1
    } else {
        asm_height as usize / 2
    };
    let asm = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Assembly")
        .border_type(BorderType::Plain);
    let registers = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Registers")
        .border_type(BorderType::Plain);
    let stack = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Stack")
        .border_type(BorderType::Plain);

    let pc_index = disas.get_index_of(&cpu.pc.value).unwrap();
    let range = (pc_index - (asm_height as usize / 2))..(pc_index + (asm_height as usize / 2));
    
    let disas_iter = disas
        .iter()
        .enumerate()
        .skip_while(|i| !range.contains(&i.0))
        .take(asm_height as usize);

    let disas_spans: Vec<ListItem> = disas_iter
        .map(|t| {
            let (addr, inst) = t.1.1.split_once(' ').unwrap();
            ListItem::new(Spans::from(vec![
                Span::styled(addr, Style::default().fg(Color::DarkGray)),
                Span::raw("       "),
                Span::styled(inst, Style::default()),
            ]))
        })
        .collect();

    let list = List::new(disas_spans).block(asm).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD)      
    );

    let p1 = Paragraph::new(vec![
        Spans::from(vec![
            Span::raw(format!("PC ${:04X}", cpu.pc.value)),
        ]),
        Spans::from(vec![
            Span::styled("A", Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" ${reg:02X} [{reg:03}]", reg=cpu.reg_af.hi()), Style::default().fg(Color::LightYellow)),
            Span::raw("     "),
            Span::styled("F", Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" ${reg:02X} [{reg:03}]", reg=cpu.reg_af.lo()), Style::default().fg(Color::LightYellow)),
            Span::raw("     "),
            Span::styled("AF", Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" ${reg:04X} [{reg:05}]", reg=cpu.reg_af.value), Style::default().fg(Color::LightYellow)),
        ]),
        Spans::from(vec![
            Span::styled("B", Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" ${reg:02X} [{reg:03}]", reg=cpu.reg_bc.hi()), Style::default().fg(Color::LightYellow)),
            Span::raw("     "),
            Span::styled("C", Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" ${reg:02X} [{reg:03}]", reg=cpu.reg_bc.lo()), Style::default().fg(Color::LightYellow)),
            Span::raw("     "),
            Span::styled("BC", Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" ${reg:04X} [{reg:05}]", reg=cpu.reg_bc.value), Style::default().fg(Color::LightYellow)),
        ]),
        Spans::from(vec![
            Span::styled("D", Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" ${reg:02X} [{reg:03}]", reg=cpu.reg_de.hi()), Style::default().fg(Color::LightYellow)),
            Span::raw("     "),
            Span::styled("E", Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" ${reg:02X} [{reg:03}]", reg=cpu.reg_de.lo()), Style::default().fg(Color::LightYellow)),
            Span::raw("     "),
            Span::styled("DE", Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" ${reg:04X} [{reg:05}]", reg=cpu.reg_de.value), Style::default().fg(Color::LightYellow)),
        ]),
        Spans::from(vec![
            Span::styled("H", Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" ${reg:02X} [{reg:03}]", reg=cpu.reg_hl.hi()), Style::default().fg(Color::LightYellow)),
            Span::raw("     "),
            Span::styled("L", Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" ${reg:02X} [{reg:03}]", reg=cpu.reg_hl.lo()), Style::default().fg(Color::LightYellow)),
            Span::raw("     "),
            Span::styled("HL", Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" ${reg:04X} [{reg:05}]", reg=cpu.reg_hl.value), Style::default().fg(Color::LightYellow)),
        ]),
        Spans::from(vec![
            Span::styled("Z", Style::default().fg(if cpu.get_flag(Flag::Z) { Color::Green } else { Color::LightRed })),
            Span::raw(" "),
            Span::styled("N", Style::default().fg(if cpu.get_flag(Flag::N) { Color::Green } else { Color::LightRed })),
            Span::raw(" "),
            Span::styled("C", Style::default().fg(if cpu.get_flag(Flag::C) { Color::Green } else { Color::LightRed })),
            Span::raw(" "),
            Span::styled("H", Style::default().fg(if cpu.get_flag(Flag::H) { Color::Green } else { Color::LightRed })),
        ]),
    ])
        .alignment(Alignment::Left)
        .block(registers);
    let p2 = Paragraph::new(Spans::from(vec![
        Span::raw(format!("SP ${:04X}", cpu.stack_pointer.value)),
    ]))
        .alignment(Alignment::Left)
        .block(stack);
    


    // let home = Paragraph::new(vec![
    //     Spans::from(vec![Span::raw("")]),
    //     Spans::from(vec![Span::raw("Welcome")]),
    //     Spans::from(vec![Span::raw("")]),
    //     Spans::from(vec![Span::raw("to")]),
    //     Spans::from(vec![Span::raw("")]),
    //     Spans::from(vec![Span::styled(
    //         "pet-CLI",
    //         Style::default().fg(Color::LightBlue),
    //     )]),
    //     Spans::from(vec![Span::raw("")]),
    //     Spans::from(vec![Span::raw("Press 'p' to access pets, 'a' to add random new pets and 'd' to delete the currently selected pet.")]),
    // ])
    // .alignment(Alignment::Center)
    // .block(
    //     Block::default()
    //         .borders(Borders::ALL)
    //         .style(Style::default().fg(Color::White))
    //         .title("Home")
    //         .border_type(BorderType::Plain),
    // );
    // home
    
    ((list, current), p1, p2)
}

pub fn build_cpu_controls<'a>() -> Paragraph<'a> {
    let btn_style = Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD);
    let p = Paragraph::new(vec![
        Spans::from(vec![
            Span::styled("[Space]", btn_style),
            Span::raw(" - next instruction"),
            Span::raw("   "),
            Span::styled("[R]", btn_style),
            Span::raw(" - reset"),
            Span::raw("   "),
        ])
    ])
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Controls")
                .border_type(BorderType::Plain),
        );
    p
}