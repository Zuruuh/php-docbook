use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Alignment,
    prelude::{Buffer, Rect},
    widgets::{Block, Clear, Widget, block::Position},
};
use tui_input::Input;

use super::{EventHandler, EventHandlerResult, Message};

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

    fn render_search_modal(
        r#type: &mut SearchModalType,
        input: &mut Input,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let container = Block::bordered()
            .title_position(Position::Top)
            .title_alignment(Alignment::Left)
            .title("Search functions");

        let container_area = container.inner(area);
        Clear::default().render(container_area, buf);
        container.render(area, buf);
    }
}

impl Widget for &mut Modal {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Modal::SearchModal { r#type, query } => {
                Modal::render_search_modal(r#type, query, area, buf)
            }
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
