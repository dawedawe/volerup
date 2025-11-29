use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::model::Model;

pub fn view(model: &Model, frame: &mut Frame) {
    fn center_horizontal(area: Rect, width: u16) -> Rect {
        let [area] = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(area);
        area
    }

    let style = Style::default().fg(Color::Yellow);

    let chunks = Layout::default()
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
        .split(frame.area());

    let cpu_state_rect = center_horizontal(chunks[0], 100);
    let pc_rect = center_horizontal(chunks[1], 100);
    let instr_reg_rect = center_horizontal(chunks[2], 100);

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

    let instr = format!("0x{:X}", model.cpu.instruction_register);
    let instr_reg_paragraph = Paragraph::new(instr).style(style).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Instruction Register "),
    );
    frame.render_widget(instr_reg_paragraph, instr_reg_rect);
}
