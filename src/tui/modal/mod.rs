use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Margin},
    prelude::{Buffer, Rect},
    widgets::{Block, Clear, StatefulWidget, Widget, block::Position},
};
use tui_input::Input;

use super::{EventHandler, EventHandlerResult, Message, SharedState};

mod search_modal;

#[derive(Debug)]
pub(super) enum Modal {
    SearchModal {
        r#type: SearchModalType,
        query: Input,
    },
}

impl Modal {
    pub fn new_search_modal(r#type: SearchModalType) -> Self {
        Self::SearchModal {
            r#type,
            query: Input::default(),
        }
    }

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
        Clear::default().render(container_area, buf);
        container.render(area, buf);

        match self {
            Modal::SearchModal { r#type, query } => search_modal::render_search_modal(
                r#type,
                query,
                container_area.inner(Margin::new(1, 0)),
                buf,
                state,
            ),
        }
    }
}

#[derive(Debug)]
pub(super) enum SearchModalType {
    Function,
}

impl EventHandler for Modal {
    async fn on_key_event(&mut self, key: &KeyEvent) -> EventHandlerResult {
        match key.code {
            KeyCode::Char('s') | KeyCode::Char('S') => {
                EventHandlerResult::HandledWithMessage(Message::OpenFunctionSearchModal)
            }
            _ => EventHandlerResult::Pass,
        }
    }
}
