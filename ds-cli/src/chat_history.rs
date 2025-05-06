use std::borrow::Cow;

use ratatui::prelude::*;

use ratatui::widgets::{Block, Borders};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::{Paragraph, Scrollbar, ScrollbarState, Wrap},
};

// 新增 ChatHistory 组件
pub(crate) struct ChatHistory;

#[derive(Debug, Clone)]
pub(crate) struct DisplayContent {
    pub(crate) is_user: bool,
    pub(crate) content: Option<String>,
    pub(crate) reasoning_content: Option<String>,
}

impl ChatHistory {
    pub(crate) fn render(
        f: &mut Frame,
        area: Rect,
        is_cursor: bool,
        msg_contents: &[DisplayContent],
        scroll_offset: &mut usize,
        is_requesting: bool,
    ) {
        //render border
        let mut outer_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Chat ");
        if is_cursor {
            outer_block = outer_block.border_style(Style::default().fg(Color::LightBlue));
        }

        let inner_area = outer_block.inner(area);
        f.render_widget(outer_block, area);

        //render content
        let layout =
            Layout::horizontal([Constraint::Min(1), Constraint::Length(1)]).split(inner_area);
        let (content_area, scrollbar_area) = (layout[0], layout[1]);

        let content_width = content_area.width.saturating_sub(1) as usize;
        let visible_height = content_area.height as usize;

        let total_lines = Self::calculate_total_lines(msg_contents, content_width);
        let scrollable_height = total_lines.saturating_sub(visible_height);
        if *scroll_offset > scrollable_height {
            //limit max scroll height
            *scroll_offset = scrollable_height;
        }

        if is_requesting {
            //if requesting, scroll to the bottom
            *scroll_offset = scrollable_height;
        }

        // render lines
        let visible_lines =
            Self::get_visible_lines(msg_contents, *scroll_offset, content_width, visible_height);
        let constraints = vec![Constraint::Length(1); content_area.height as usize];
        let inner_layout = Layout::vertical(constraints).split(content_area);
        for i in 0..content_area.height as usize {
            if let Some((text, is_content, alignment, is_user)) = visible_lines.get(i) {
                let para = if *is_content {
                    let color = if *is_user {
                        Color::LightBlue
                    } else {
                        Color::White
                    };

                    Paragraph::new(text.clone())
                        .alignment(*alignment)
                        .style(Style::default().fg(color))
                        .wrap(Wrap { trim: true })
                } else {
                    Paragraph::new(text.clone())
                        .style(Style::default().bg(Color::Gray).fg(Color::DarkGray))
                        .alignment(*alignment)
                        .wrap(Wrap { trim: true })
                };

                f.render_widget(para, inner_layout[i]);
            }
        }

        let scrollbar = Scrollbar::default()
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut state = ScrollbarState::new(scrollable_height)
            .position(*scroll_offset)
            .viewport_content_length(visible_height.min(total_lines));

        f.render_stateful_widget(scrollbar, scrollbar_area, &mut state);
    }

    fn calculate_total_lines(history: &[DisplayContent], width: usize) -> usize {
        history
            .iter()
            .map(|msg| {
                msg.content
                    .as_ref()
                    .map_or(0, |s| textwrap::wrap(s, width).len())
                    + msg
                        .reasoning_content
                        .as_ref()
                        .map_or(0, |s| textwrap::wrap(s, width).len())
            })
            .sum()
    }

    fn get_visible_lines(
        history: &[DisplayContent],
        scroll_offset: usize,
        width: usize,
        height: usize,
    ) -> Vec<(Cow<'_, str>, bool, Alignment, bool)> {
        let mut lines = Vec::with_capacity(height);
        let mut current_line = 0;
        let end_line = scroll_offset + height;

        for msg in history.iter() {
            let content_iter = msg
                .content
                .as_ref()
                .map_or(vec![], |s| textwrap::wrap(s, width))
                .into_iter()
                .map(|v| (v, true));
            let reason_iter = msg
                .reasoning_content
                .as_ref()
                .map_or(vec![], |s| textwrap::wrap(s, width))
                .into_iter()
                .map(|v| (v, false));
            let msg_lines = reason_iter.chain(content_iter).collect::<Vec<_>>();

            let alignment = if msg.is_user {
                Alignment::Right
            } else {
                Alignment::Left
            };
            for (line, is_content) in msg_lines {
                if current_line >= scroll_offset && current_line < end_line {
                    lines.push((line, is_content, alignment, msg.is_user));
                }
                current_line += 1;
                if current_line >= end_line {
                    break;
                }
            }
        }
        lines
    }
}
