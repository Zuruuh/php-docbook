use color_eyre::Result;
use crossterm::event::{Event, KeyEvent, KeyEventKind};
use futures_util::{FutureExt, StreamExt};

use super::TerminalState;

#[derive(Debug)]
pub(super) enum EventHandlerResult {
    Handled,
    Pass,
}

pub(super) trait EventHandler {
    async fn on_key_event(
        &mut self,
        key: &KeyEvent,
        state: &mut TerminalState,
    ) -> EventHandlerResult;
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
