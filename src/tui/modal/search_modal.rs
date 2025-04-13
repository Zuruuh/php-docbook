use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};
use tui_input::Input;

use crate::{
    parser::function::Function,
    tui::{EventHandler, EventHandlerResult, SharedState},
};

use super::Modal;

#[derive(Debug)]
pub struct SearchModal {
    r#type: SearchModalType,
    query: Input,
}

impl SearchModal {
    pub fn new(r#type: SearchModalType) -> Modal {
        Modal::SearchModal(Self {
            r#type,
            query: Input::default(),
        })
    }

    pub fn r#type(&self) -> &SearchModalType {
        &self.r#type
    }
}

#[derive(Debug)]
pub enum SearchModalType {
    Function,
}

impl EventHandler for SearchModal {
    async fn on_key_event(&mut self, key: &KeyEvent) -> EventHandlerResult {
        use tui_input::backend::crossterm::EventHandler as _;
        match self
            .query
            .handle_event(&crossterm::event::Event::Key(key.clone()))
        {
            Some(_) => EventHandlerResult::Handled,
            None => EventHandlerResult::Pass,
        }
    }
}

impl StatefulWidget for &mut SearchModal {
    type State = SharedState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [input_area, preview_and_list_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).split(area)[..]
        else {
            unreachable!()
        };

        let [list_area, preview_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                .split(preview_and_list_area)[..]
        else {
            unreachable!()
        };

        let input_widget = Paragraph::new(self.query.value()).block(Block::bordered());
        input_widget.render(input_area, buf);

        let items = state
            .parsed_files_snapshot
            .iter()
            .filter_map(|function| match function {
                Function::Definition(function_definition) => Some(function_definition),
                Function::Alias(_) => None,
            })
            .map(|function| format!("{}(...)", function.name))
            .filter(|name| {
                self.query
                    .value()
                    .split(' ')
                    .all(|word| name.contains(word))
            })
            .map(ListItem::new);

        let list = List::new(items);
        StatefulWidget::render(list, list_area, buf, &mut ListState::default());

        // Paragraph::new(
        //     shared_state
        //         .parsed_files_snapshot
        //         .iter()
        //         .filter_map(|function| match function {
        //             Function::Definition(def) => Some(def),
        //             _ => None,
        //         })
        //         .last()
        //         .map(ToString::to_string)
        //         .unwrap_or_default(),
        // )
        // .render(area, buf);
    }
}
