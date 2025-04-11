#![allow(unused_imports, dead_code, unused_variables)]
use std::collections::BTreeSet;

use ansi_to_tui::IntoText;
use bat::PrettyPrinter;
use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    // let result = run(terminal).await;

    ratatui::restore();

    let file_content =
        tokio::fs::read("./php-docs-en/reference/array/functions/array-map.xml").await?;
    let parser = libxml::parser::Parser::default();
    let doc = parser.parse_string(file_content).unwrap();

    let root = doc.get_root_element().unwrap();
    println!("Root: {}", root.get_name());

    Ok(())
}

struct PhpFunction {
     return_type: PhpType
}

enum PhpType {
    built_in:q
}

// async fn run(mut terminal: DefaultTerminal) -> Result<()> {
// Ok(())

// let mut pretty_xml = String::new();
// PrettyPrinter::new()
//     .input_from_bytes(&file_content[..])
//     .language("xml")
//     .line_numbers(false)
//     .print_with_writer(Some(&mut pretty_xml));
// let pretty_xml = pretty_xml.to_text().unwrap();
//
// loop {
//     terminal.draw(|frame: &mut Frame| {
//         // pretty_xml.print();
//         frame.render_widget(&pretty_xml, frame.area());
//     })?;
//
//     if matches!(event::read()?, Event::Key(_)) {
//         break Ok(());
//     }
// }
// }
