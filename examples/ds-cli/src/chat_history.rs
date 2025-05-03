use deepseek_api::request::MessageRequest;
use ratatui::prelude::*;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::{Paragraph, Scrollbar, ScrollbarState, Wrap},
};

// 新增 ChatHistory 组件
pub(crate) struct ChatHistory;

struct MessageContent{
    alignment: Alignment,
    content: String,
    reasoning_content: Option<String>,
}

impl ChatHistory {
    pub(crate) fn render(
        f: &mut Frame,
        area: Rect,
        history: &[MessageRequest],
        scroll_offset: &mut usize,
        is_requesting: bool,
    ) {
        let msg_contents = Self::extract_content(history);
        let layout = Layout::horizontal([Constraint::Min(1), Constraint::Length(1)]).split(area);
        let (content_area, scrollbar_area) = (layout[0], layout[1]);

        let content_width = content_area.width.saturating_sub(1) as usize;
        let visible_height = content_area.height as usize;

        let total_lines = Self::calculate_total_lines(&msg_contents, content_width);
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
            Self::get_visible_lines(&msg_contents, *scroll_offset, content_width, visible_height);
        let constraints = vec![Constraint::Length(1); content_area.height as usize];
        let inner_layout = Layout::vertical(constraints).split(content_area);
        for i in 0..content_area.height as usize {
            if let Some((text, alignment)) = visible_lines.get(i) {
                let para = Paragraph::new(text.clone())
                    .alignment(*alignment)
                    .wrap(Wrap { trim: true });
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

    fn extract_content(history: &[MessageRequest]) -> Vec<MessageContent> {
        history
            .iter().enumerate()
            .map(|(idx, msg)| {
                let content = msg.get_content();
                let reasoning_content = match msg {
                    MessageRequest::Assistant(assistant) => {
                        assistant.reasoning_content.as_ref()
                    }
                    _ => None,
                };
                let alignment = if idx % 2 == 0 {
                    Alignment::Right
                } else {
                    Alignment::Left
                };
                MessageContent {
                    alignment,
                    content: content.clone(),
                    reasoning_content: reasoning_content.map(|s| s.clone()),
                }
            })
            .collect()
    }

    fn calculate_total_lines(history: &[MessageContent], width: usize) -> usize {
        history
            .iter()
            .map(|msg| textwrap::wrap(&msg.content, width).len())
            .sum()
    }

    fn get_visible_lines(
        history: &[MessageContent],
        scroll_offset: usize,
        width: usize,
        height: usize,
    ) -> Vec<(String, Alignment)> {
        let mut lines = Vec::with_capacity(height);
        let mut current_line = 0;
        let end_line = scroll_offset + height;

        for msg in history.iter() {
            let msg_lines = textwrap::wrap(&msg.content, width)
                .into_iter()
                .map(|cow| cow.into_owned())
                .collect::<Vec<_>>();

            for line in msg_lines {
                if current_line >= scroll_offset && current_line < end_line {
                    lines.push((line, msg.alignment));
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
