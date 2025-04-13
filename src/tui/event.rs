use color_eyre::Result;
use crossterm::event::{Event, KeyEventKind};
use futures_util::{FutureExt, StreamExt};

use super::{
    TerminalState,
    modal::search_modal::{SearchModal, SearchModalType},
};

#[derive(Debug)]
pub(super) enum EventHandlerResult {
    Handled,
    HandledWithMessage(Message),
    Pass,
}

#[derive(Debug)]
pub enum Message {
    OpenFunctionSearchModal,
}

impl Message {
    pub async fn handle(&self, state: &mut TerminalState) {
        match self {
            Message::OpenFunctionSearchModal => {
                state.open_modal = Some(SearchModal::new(SearchModalType::Function));
            }
        }
    }
}

pub(super) trait EventHandler {
    async fn on_key_event(&mut self, key: &crossterm::event::KeyEvent) -> EventHandlerResult;
}

pub(super) trait CrosstermEventHandler {
    async fn handle_crossterm_events(&mut self) -> Result<()>;
}

impl CrosstermEventHandler for super::TerminalState {
    async fn handle_crossterm_events(&mut self) -> Result<()> {
        tokio::select! {
            event = self.event_stream.next().fuse() => {
                if let Some(Ok(Event::Key(key))) = event {
                    if key.kind == KeyEventKind::Press {
                        self.on_key_event(key).await;
                    }
                }
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {}
        };

        Ok(())
    }
}
