#![allow(unused_imports, dead_code, unused_variables)]
use std::collections::BTreeSet;

use ansi_to_tui::IntoText;
use bat::PrettyPrinter;
use color_eyre::Result;
use crossterm::event::{self, Event};
use derive_builder::Builder;
use libxml::tree::Node;
use parser::XmlParser;
use ratatui::{DefaultTerminal, Frame, symbols::line::DOUBLE_HORIZONTAL_DOWN};

mod parser;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    // let result = run(terminal).await;

    ratatui::restore();

    let file_content =
        tokio::fs::read_to_string("./php-docs-en/reference/array/functions/array-reduce.xml")
            .await?;

    let parser = XmlParser::default();

    let function = parser.parse_function(file_content)?;
    let mut buf = format!("{}(", function.name);
    for (i, arg) in function.arguments.into_iter().enumerate() {
        buf.push_str(
            format!(
                "{}{} {}${}{}",
                (i > 0).then(|| ", ").unwrap_or_default(),
                arg.r#type,
                arg.repeat.then(|| "...").unwrap_or_default(),
                arg.name,
                arg.default_value
                    .map(|value| format!(" = {value}"))
                    .unwrap_or_default(),
            )
            .as_str(),
        );
    }
    buf.push_str(format!("): {};", function.return_type).as_str());
    println!("{buf}");

    Ok(())
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
