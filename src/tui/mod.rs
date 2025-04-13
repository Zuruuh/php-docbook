use color_eyre::Result;
use crossterm::event::{EventStream, KeyCode, KeyEvent, KeyModifiers};
use event::EventHandler;
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
        };

        let modal = match self.open_modal.as_mut() {
            None => return,
            Some(modal) => modal,
        };

        let modal_area = area.inner(Margin::new(3, 0));

        let [_, modal_area, _] = Layout::vertical([
            Constraint::Ratio(1, 5),
            Constraint::Ratio(3, 5),
            Constraint::Ratio(1, 5),
        ])
        .split(modal_area)[..] else {
            unreachable!()
        };

        modal.render(modal_area, buf);
    }

    async fn on_key_event(&mut self, key: KeyEvent) {
        if let Some(modal) = self.open_modal.as_mut() {
            match modal.on_key_event(&key).await {
                EventHandlerResult::Handled => return,
                EventHandlerResult::HandledWithMessage(message) => {
                    message.handle(self).await;

                    return;
                }
                EventHandlerResult::Pass => {}
            };
        }
        if self.open_modal.is_some() {
            match &key.code {
                KeyCode::Esc => {
                    self.open_modal = None;
                    return;
                }
                _ => {}
            }
        }

        let result = match self.screen {
            Screen::Home => HomeScreen.on_key_event(&key).await,
        };
        match result {
            EventHandlerResult::Handled => {
                return;
            }
            EventHandlerResult::HandledWithMessage(message) => {
                message.handle(self).await;
                return;
            }
            EventHandlerResult::Pass => {}
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
