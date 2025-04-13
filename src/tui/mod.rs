use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures_util::{FutureExt, StreamExt};
use ratatui::{DefaultTerminal, prelude::*, widgets::Block};

mod event;
mod modal;
mod screen;

pub(self) use event::*;
pub(self) use modal::*;
pub(self) use screen::*;

#[derive(Default, Debug)]
pub struct TerminalState {
    event_stream: EventStream,
    pub parsed_files: usize,
    pub total_files_to_parse: usize,
    running: bool,
    screen: Screen,
    open_modal: Option<modal::Modal>,
}

#[derive(Default, Debug)]
enum Screen {
    #[default]
    Home,
}

impl TerminalState {
    pub async fn run<Callback>(
        mut self,
        mut terminal: DefaultTerminal,
        mut update_callback: Callback,
    ) -> Result<()>
    where
        Callback: AsyncFnMut(&mut TerminalState),
    {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events().await?;
            update_callback(&mut self).await;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let block = Block::bordered().title(
            Line::from(format!("PHP DocBook {}", env!("CARGO_PKG_VERSION")))
                .bold()
                .blue()
                .centered(),
        );
        let container = block.inner(area);
        let buf = frame.buffer_mut();
        block.render(area, buf);

        match self.screen {
            Screen::Home => {
                screen::HomeScreen.render(container, buf, self);
            }
        }
    }

    async fn on_key_event(&mut self, key: KeyEvent) {
        let result = match self.screen {
            Screen::Home => screen::HomeScreen.on_key_event(&key, self).await,
        };
        if matches!(result, event::EventHandlerResult::Handled) {
            return;
        }

        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                self.running = false
            }
            _ => {}
        }
    }
}
