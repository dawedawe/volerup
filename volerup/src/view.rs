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

pub(crate) fn view(model: &Model, frame: &mut Frame) {
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
        .constraints([Constraint::Min(70), Constraint::Min(2)].as_ref())
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

    let footer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
        .split(whole_screen_chunks[1]);
    let error_msg_rect = center_horizontal(footer_chunks[0], 40);
    let help_msg_rect = center_horizontal(footer_chunks[1], 113);

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

    let editor_block = {
        let (title, block_style) = if model.focus == Focus::Program {
            (" Program* ", style.add_modifier(Modifier::BOLD))
        } else {
            (" Program ", style)
        };

        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .style(block_style)
    };

    frame.render_widget(editor_block, prog_rect);
    let editor_rect = Layout::default()
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(prog_rect);
    frame.render_widget(&model.program_textarea, editor_rect[0]);

    if let Some(msg) = model.error_msg {
        let style = Style::default().fg(Color::Red);
        let error_msg_paragraph = Paragraph::new(msg).style(style);
        frame.render_widget(error_msg_paragraph, error_msg_rect);
    }

    let msg = vec![
        Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": load program and reset CPU, "),
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
