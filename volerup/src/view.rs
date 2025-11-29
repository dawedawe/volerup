use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListDirection, ListItem, Paragraph},
};

use crate::model::Model;

fn create_list_widget<'a>(values: &[u8], title: &str, style: Style) -> List<'a> {
    let items = values.iter().map(|reg| -> ListItem<'_> {
        let s = format!(" 0x{:X} ", reg);
        ListItem::new(s).style(style)
    });

    List::new(items)
        .block(Block::bordered().title(format!(" {title} ")))
        .style(style)
        .direction(ListDirection::TopToBottom)
}

pub fn view(model: &Model, frame: &mut Frame) {
    let style: Style = Style::default().fg(Color::Yellow);

    fn center_horizontal(area: Rect, width: u16) -> Rect {
        let [area] = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(area);
        area
    }

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
        .split(frame.area());

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

    let instr = format!("0x{:X}", model.cpu.instruction_register);
    let instr_reg_paragraph = Paragraph::new(instr).style(style).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Instruction Register "),
    );
    frame.render_widget(instr_reg_paragraph, instr_reg_rect);

    let regs_widget = create_list_widget(&model.cpu.registers, "Registers", style);
    frame.render_widget(regs_widget, regs_rect);

    let mem_widget = create_list_widget(&model.cpu.memory, "Main Memory", style);
    frame.render_widget(mem_widget, mem_rect);

    let prog_widget = create_list_widget(&model.program, "Program", style);
    frame.render_widget(prog_widget, prog_rect);
}
