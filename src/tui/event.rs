use color_eyre::Result;
use crossterm::event::{Event, KeyEvent, KeyEventKind};
use futures_util::{FutureExt, StreamExt};

use super::{Modal, SearchModalType, TerminalState};

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
                state.open_modal = Some(Modal::new_search_modal(SearchModalType::Function));
            }
        }
    }
}

pub(super) trait EventHandler {
    async fn on_key_event(&mut self, key: &KeyEvent) -> EventHandlerResult;
}

pub(super) trait CrosstermEventHandler {
    async fn handle_crossterm_events(&mut self) -> Result<()>;
}

impl CrosstermEventHandler for super::TerminalState {
    async fn handle_crossterm_events(&mut self) -> Result<()> {
        tokio::select! {
            event = self.event_stream.next().fuse() => {
                match event {
                    Some(Ok(evt)) => {
                        match evt {
                            Event::Key(key)
                                if key.kind == KeyEventKind::Press
                                    => self.on_key_event(key).await,
                            Event::Mouse(_) => {}
                            Event::Resize(_, _) => {}
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {}
        }
        Ok(())
    }
}
