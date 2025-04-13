use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, Padding, Paragraph, StatefulWidget, Widget, Wrap},
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
    selection_offset: usize,
}

impl SearchModal {
    pub fn new(r#type: SearchModalType) -> Modal {
        Modal::SearchModal(Self {
            r#type,
            query: Input::default(),
            selection_offset: 0,
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
        match key.code {
            KeyCode::Down => {
                self.selection_offset += 1;

                return EventHandlerResult::Handled;
            }
            KeyCode::Up => {
                if self.selection_offset > 0 {
                    self.selection_offset -= 1;
                }
                return EventHandlerResult::Handled;
            }
            _ => {}
        };

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
            Layout::horizontal([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)])
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
            .filter(|function| {
                self.query
                    .value()
                    .split(' ')
                    .all(|word| function.name.contains(word))
            })
            .collect::<Vec<_>>();

        if self.selection_offset >= items.len() {
            self.selection_offset = items.len().checked_sub(1).unwrap_or_default()
        }

        let selected_function = items
            .iter()
            .enumerate()
            .find(|(i, _)| *i == self.selection_offset)
            .map(|(_, function)| *function)
            .cloned();

        let items = items.into_iter().enumerate().map(|(i, function)| {
            if i == self.selection_offset {
                ListItem::new(Line::from(vec![
                    Span::styled("> ", Style::default().fg(Color::LightRed)),
                    Span::styled(format!("{}(...)", function.name), Style::default().italic()),
                ]))
                .style(Style::default().bg(Color::DarkGray))
            } else {
                ListItem::new(format!("  {}(...)", function.name))
            }
        });

        let list = List::new(items);
        StatefulWidget::render(list, list_area, buf, &mut ListState::default());

        let preview = Paragraph::new(
            selected_function
                .map(|function| function.to_string())
                .unwrap_or("No preview available".to_string()),
        )
        .wrap(Wrap::default())
        .block(Block::bordered().padding(Padding::horizontal(1)));

        preview.render(preview_area.inner(Margin::new(1, 0)), buf);
    }
}
