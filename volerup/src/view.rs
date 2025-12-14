use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

use crate::model::{Focus, Model};

fn default_style() -> Style {
    Style::default().fg(Color::Green)
}

fn render_list(
    values: &[u8],
    title: &str,
    line_to_highlight: Option<usize>,
    focused: bool,
    vertical_scroll: usize,
    rect: Rect,
    frame: &mut Frame,
) {
    let style: Style = default_style();

    let len = values.len();
    let items = values
        .iter()
        .enumerate()
        .map(|(idx, value)| {
            let s = if len < 100 {
                format!("{:2} (0x{:02X}): 0x{:02X} ({:3})", idx, idx, value, value)
            } else {
                format!("{:3} (0x{:02X}): 0x{:02X} ({:3})", idx, idx, value, value)
            };
            let style = if let Some(idx_to_highlight) = line_to_highlight
                && idx_to_highlight == idx
            {
                style
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::UNDERLINED)
            } else {
                style
            };
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

/// Render the TUI from the model
pub(crate) fn view(model: &Model, frame: &mut Frame) {
    let style: Style = default_style();

    fn center_horizontal(area: Rect, width: u16) -> Rect {
        let [area] = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(area);
        area
    }

    let whole_screen_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(70), Constraint::Min(2)].as_ref())
        .split(frame.area());

    let help_screen_chunk = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(1)].as_ref())
        .split(whole_screen_chunks[0])[0];

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(34),
                Constraint::Length(24),
                Constraint::Length(25),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(whole_screen_chunks[0]);

    let footer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
        .split(whole_screen_chunks[1]);
    let help_msg_rect = footer_chunks[1];

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(main_chunks[0]);

    let cpu_state_rect = left_chunks[0];
    let cycle_rect = left_chunks[1];
    let pc_rect = left_chunks[2];
    let instr_reg_rect = left_chunks[3];
    let regs_rect = main_chunks[1];
    let mem_rect = main_chunks[2];
    let prog_rect = main_chunks[3];

    let cpu_state_paragraph = {
        let cpu_state = if model.cpu.halted {
            "HALTED"
        } else {
            "RUNNING"
        };
        Paragraph::new(cpu_state)
            .style(style)
            .block(Block::default().borders(Borders::ALL).title(" CPU State "))
    };
    frame.render_widget(cpu_state_paragraph, cpu_state_rect);

    let cycle_paragraph = Paragraph::new(model.cpu.cycle.to_string())
        .style(style)
        .block(Block::default().borders(Borders::ALL).title(" Cycle "));
    frame.render_widget(cycle_paragraph, cycle_rect);

    let pc_paragraph = Paragraph::new(model.cpu.program_counter.to_string())
        .style(style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Program Counter "),
        );
    frame.render_widget(pc_paragraph, pc_rect);

    let instr_reg_paragraph = {
        let opcode = if let Some(opcode) = model.cpu.decode() {
            format!("({})", opcode)
        } else {
            "".to_string()
        };
        let instr = format!("0x{:02X} {}", model.cpu.instruction_register, opcode);
        Paragraph::new(instr).style(style).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Instruction Register "),
        )
    };
    frame.render_widget(instr_reg_paragraph, instr_reg_rect);

    render_list(
        &model.cpu.registers,
        "Registers",
        model.modified_register,
        model.focus == Focus::Registers,
        model.registers_scroll,
        regs_rect,
        frame,
    );

    render_list(
        &model.cpu.memory,
        "Main Memory",
        model.modified_memory,
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
        let error_msg_rect = center_horizontal(footer_chunks[0], msg.len() as u16);
        frame.render_widget(error_msg_paragraph, error_msg_rect);
    }

    let (help_message, help_msg_rect) = {
        let usage_lines = vec![
            Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": load program and reset CPU, "),
            Span::styled("p", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": exec CPU cycle, "),
            Span::styled("P", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": run program, "),
            Span::styled("?", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": help"),
        ];
        let text = Text::from(Line::from(usage_lines));
        let help_msg_rect = center_horizontal(help_msg_rect, text.width() as u16);
        (Paragraph::new(text), help_msg_rect)
    };
    frame.render_widget(help_message, help_msg_rect);

    if model.show_help {
        let instructions_help = vec![
            Line::from("Vole Instructions:"),
            Line::from("0x1 RXY - LOAD memory cell XY into register R"),
            Line::from("0x2 RXY - LOAD value XY into register R"),
            Line::from("0x3 RXY - STORE value in register R in memory cell XY"),
            Line::from("0x4 0RS - MOVE register R to register S"),
            Line::from(
                "0x5 RST - ADD registers S and T as integers, store the result in register R",
            ),
            Line::from("0x6 RST - ADD registers S and T as floats, store the result in register R"),
            Line::from("0x7 RST - OR registers S and T, store the result in register R"),
            Line::from("0x8 RST - AND registers S and T, store the result in register R"),
            Line::from("0x9 RST - XOR registers S and T, store the result in register R"),
            Line::from("0xA R0X - ROTATE register R X times to the right"),
            Line::from(
                "0xB RXY - JUMP to instruction at memory cell XY if register R equals register 0",
            ),
            Line::from("0xC 000 - HALT the execution"),
        ];
        let help_paragraph = Paragraph::new(instructions_help)
            .style(style)
            .block(Block::default().borders(Borders::ALL).title(" Help "));
        frame.render_widget(Clear, help_screen_chunk);
        frame.render_widget(help_paragraph, help_screen_chunk);
    }
}
