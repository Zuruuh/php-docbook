use crossterm::event::{KeyCode, KeyEvent};
use fuzzy_matcher::FuzzyMatcher;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        Block, List, ListItem, ListState, Padding, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState, StatefulWidget, Widget, Wrap,
    },
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
    list_state: ListState,
    vertical_scroll_state: ScrollbarState,
}

impl SearchModal {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(r#type: SearchModalType) -> Modal {
        Modal::SearchModal(Self {
            r#type,
            query: Input::default(),
            list_state: ListState::default(),
            vertical_scroll_state: ScrollbarState::default(),
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
                self.vertical_scroll_state.next();
                self.list_state.select_next();

                return EventHandlerResult::Handled;
            }
            KeyCode::Up => {
                self.vertical_scroll_state.prev();
                self.list_state.select_previous();

                return EventHandlerResult::Handled;
            }
            KeyCode::Home => {
                self.vertical_scroll_state.first();
                self.list_state.select_first();

                return EventHandlerResult::Handled;
            }
            KeyCode::End => {
                self.vertical_scroll_state.last();
                self.list_state.select_last();
            }
            _ => {}
        };

        use tui_input::backend::crossterm::EventHandler as _;
        match self.query.handle_event(&crossterm::event::Event::Key(*key)) {
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
            .filter_map(|function| {
                let (_, indices) = state
                    .fuzzy_matcher
                    .fuzzy_indices(function.name.as_str(), self.query.value())?;

                Some((
                    function,
                    function
                        .name
                        .chars()
                        .enumerate()
                        .map(|(i, char)| (char, indices.contains(&i)))
                        .collect::<Vec<_>>(),
                ))
            })
            .collect::<Vec<_>>();

        let selected_function = items
            .iter()
            .enumerate()
            .find(|(i, _)| Some(*i) == self.list_state.selected())
            .map(|(_, function)| function.0)
            .cloned();

        let items_count = items.len();

        let items = items
            .into_iter()
            .enumerate()
            .map(|(i, (_, function_name))| {
                let mut chars = Vec::<Span>::new();
                // .find(|(i, _)| Some(*i) == self.list_state.selected())
                let is_selected = self
                    .list_state
                    .selected()
                    .map(|selected| selected == i)
                    .unwrap_or_default();
                if is_selected {
                    chars.push(Span::styled("> ", Style::default().fg(Color::LightRed)));
                }

                for (char, matching) in function_name {
                    chars.push(Span::styled(
                        char.to_string(),
                        matching
                            .then_some(Style::default().fg(Color::LightRed))
                            .unwrap_or_default(),
                    ));
                }

                if is_selected {
                    ListItem::new(Line::from(chars))
                        .style(Style::new().italic().bg(Color::DarkGray))
                } else {
                    ListItem::new(Line::from(chars))
                }
            });

        let list = List::new(items).scroll_padding(2);
        StatefulWidget::render(list, list_area, buf, &mut self.list_state);

        self.vertical_scroll_state = self.vertical_scroll_state.content_length(items_count);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        scrollbar.render(list_area, buf, &mut self.vertical_scroll_state);

        let preview = Paragraph::new(
            selected_function
                .clone()
                .map(|function| {
                    let description = function
                        .description
                        .iter()
                        .map(|desc_node| desc_node.to_string())
                        .map(|str| str.trim().to_string())
                        .collect::<Vec<_>>()
                        .join(" ");

                    format!("{function}\n\n{description}")
                })
                // .map(|code| {
                //     use ansi_to_tui::IntoText;
                //
                //     let code = format!("<?php\n{code}");
                //     let mut buffer = String::new();
                //
                //     let _ = PrettyPrinter::new()
                //         .language("php")
                //         .input_from_bytes(code.as_bytes())
                //         .print_with_writer(Some(&mut buffer));
                //
                //     let pretty_code = buffer.lines().skip(1).collect::<String>();
                //
                //     IntoText::into_text(&pretty_code).map(|text| text.clone())
                // })
                // .transpose()
                // .ok()
                // .flatten()
                // .unwrap_or(Text::from("No preview available")),
                .unwrap_or("No preview available".to_string()),
        )
        .wrap(Wrap::default())
        .block(Block::bordered().padding(Padding::horizontal(1)));

        preview.render(preview_area.inner(Margin::new(1, 0)), buf);
    }
}
