use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::{Block, Clear, Paragraph, Widget, block::Position},
};
use tui_input::Input;

use crate::tui::SharedState;

use super::SearchModalType;

pub(super) fn render_search_modal(
    r#type: &mut SearchModalType,
    input: &mut Input,
    area: Rect,
    buf: &mut Buffer,
    shared_state: &mut SharedState,
) {
    Paragraph::new(
        shared_state
            .parsed_files_snapshot
            .last()
            .map(ToString::to_string)
            .unwrap_or_default(),
    )
    .render(area, buf);
}
