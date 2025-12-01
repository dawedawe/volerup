use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

use crate::model::{Focus, Model};

fn render_list(
    values: &[u8],
    title: &str,
    style: Style,
    focused: bool,
    vertical_scroll: usize,
    rect: Rect,
    frame: &mut Frame,
) {
    let items = values
        .iter()
        .enumerate()
        .map(|(idx, reg)| {
            let s = format!("{:2}: 0x{:02X} ", idx, reg);
            Line::from(s).style(style)
        })
        .collect::<Vec<Line>>();

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    let mut scrollbar_state = ScrollbarState::new(items.len()).position(vertical_scroll);

    let block_style = if focused {
        Style::default().add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let paragraph = Paragraph::new(items.clone())
        .scroll((vertical_scroll as u16, 0))
        .block(
            Block::bordered()
                .style(style)
                .title(format!(" {}{} ", title, if focused { "*" } else { "" }))
                .title_style(block_style),
        );

    frame.render_widget(paragraph, rect);
    frame.render_stateful_widget(
        scrollbar,
        rect.inner(Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut scrollbar_state,
    );
}

pub fn view(model: &Model, frame: &mut Frame) {
    let style: Style = Style::default().fg(Color::Green);

    fn center_horizontal(area: Rect, width: u16) -> Rect {
        let [area] = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(area);
        area
    }

    let whole_screen_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(50), Constraint::Min(1)].as_ref())
        .split(frame.area());

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints(
            [
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
            ]
            .as_ref(),
        )
        .split(whole_screen_chunks[0]);

    let help_msg_rect = center_horizontal(whole_screen_chunks[1], 80);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(main_chunks[0]);

    let cpu_state_rect = center_horizontal(left_chunks[0], 30);
    let pc_rect = center_horizontal(left_chunks[1], 30);
    let instr_reg_rect = center_horizontal(left_chunks[2], 30);
    let regs_rect = center_horizontal(main_chunks[1], 20);
    let mem_rect = center_horizontal(main_chunks[2], 20);
    let prog_rect = center_horizontal(main_chunks[3], 20);

    let cpu_state = if model.cpu.halted {
        "HALTED"
    } else {
        "RUNNING"
    };
    let cpu_state_paragraph = Paragraph::new(cpu_state)
        .style(style)
        .block(Block::default().borders(Borders::ALL).title(" CPU State "));
    frame.render_widget(cpu_state_paragraph, cpu_state_rect);

    let pc_paragraph = Paragraph::new(model.cpu.program_counter.to_string())
        .style(style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Program Counter "),
        );
    frame.render_widget(pc_paragraph, pc_rect);

    let opcode = if let Some(opcode) = model.cpu.decode() {
        format!("({})", opcode)
    } else {
        "".to_string()
    };
    let instr = format!("0x{:02X} {}", model.cpu.instruction_register, opcode);
    let instr_reg_paragraph = Paragraph::new(instr).style(style).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Instruction Register "),
    );
    frame.render_widget(instr_reg_paragraph, instr_reg_rect);

    render_list(
        &model.cpu.registers,
        "Registers",
        style,
        model.focus == Focus::Registers,
        model.registers_scroll,
        regs_rect,
        frame,
    );

    render_list(
        &model.cpu.memory,
        "Main Memory",
        style,
        model.focus == Focus::Memory,
        model.memory_scroll,
        mem_rect,
        frame,
    );

    render_list(
        &model.program,
        "Program",
        style,
        model.focus == Focus::Program,
        model.program_scroll,
        prog_rect,
        frame,
    );

    let msg = vec![
        Span::styled("Space", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": exec CPU cycle, "),
        Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": focus next control, "),
        Span::styled("↑/↓", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": scroll up/down, "),
        Span::styled("Esc/q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": exit"),
    ];
    let text = Text::from(Line::from(msg));
    let help_message = Paragraph::new(text);
    frame.render_widget(help_message, help_msg_rect);
}
