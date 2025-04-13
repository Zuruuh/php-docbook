use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Paragraph, StatefulWidget, Widget},
};

use super::{
    TerminalState,
    event::{EventHandler, EventHandlerResult},
    modal::{Modal, SearchModalType},
};

#[derive(Default, Debug)]
pub struct HomeScreen;

/// Original Art by Donovan Bake
/// https://www.asciiart.eu/books/books
const BOOK_ASCII: &str = indoc::indoc! {r#"
     __...--~~~~~-._   _.-~~~~~--...__
   //    THE PHP    `V'    DOCBOOK    \\
  //                 |                 \\
 //__...--~~~~~~-._  |  _.-~~~~~~--...__\\
//__.....----~~~~._\ | /_.~~~~----.....__\\
\===================\\|//=================/
"#};

impl StatefulWidget for HomeScreen {
    type State = TerminalState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let [ascii_area, page_area] =
            Layout::vertical([Constraint::Length(6), Constraint::Fill(1)]).split(area)[..]
        else {
            unreachable!()
        };
        Paragraph::new(BOOK_ASCII)
            .centered()
            .render(ascii_area, buf);

        Paragraph::new(format!(
            "{} {}/{} definition files",
            if state.parsed_files == state.total_files_to_parse {
                "Parsed"
            } else {
                "Parsing"
            },
            state.parsed_files,
            state.total_files_to_parse
        ))
        .centered()
        .render(page_area, buf);
    }
}

impl EventHandler for HomeScreen {
    async fn on_key_event(
        &mut self,
        key: &KeyEvent,
        state: &mut TerminalState,
    ) -> EventHandlerResult {
        match key.code {
            KeyCode::Char('s') | KeyCode::Char('S') => {
                state.open_modal = Some(Modal::SearchModal {
                    r#type: SearchModalType::Function,
                    query: String::new(),
                });

                EventHandlerResult::Handled
            }
            _ => EventHandlerResult::Pass,
        }
    }
}
