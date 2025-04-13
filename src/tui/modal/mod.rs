use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Alignment, Margin},
    prelude::{Buffer, Rect},
    widgets::{Block, Clear, StatefulWidget, Widget, block::Position},
};
use search_modal::SearchModal;

use super::{EventHandler, EventHandlerResult, SharedState};

pub mod search_modal;

#[derive(Debug)]
pub enum Modal {
    SearchModal(SearchModal),
}

impl Modal {
    pub fn title(&self) -> &'static str {
        match self {
            Modal::SearchModal { .. } => "Search functions",
        }
    }
}

impl StatefulWidget for &mut Modal {
    type State = SharedState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut SharedState) {
        let container = Block::bordered()
            .title_position(Position::Top)
            .title_alignment(Alignment::Left)
            .title(self.title());

        let container_area = container.inner(area);
        Clear.render(container_area, buf);
        container.render(area, buf);

        match self {
            Modal::SearchModal(modal) => {
                modal.render(container_area.inner(Margin::new(1, 0)), buf, state)
            }
        }
    }
}

impl EventHandler for Modal {
    async fn on_key_event(&mut self, key: &KeyEvent) -> EventHandlerResult {
        match self {
            Modal::SearchModal(modal) => modal.on_key_event(key),
        }
        .await
    }
}
