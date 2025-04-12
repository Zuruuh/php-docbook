use clap::Parser;
use color_eyre::Result;
use fancy_regex::Captures;
use parser::XmlParser;

mod parser;

#[derive(Default, clap::Subcommand)]
pub enum Subcommand {
    Setup,
    #[default]
    View,
}

#[derive(Parser)]
pub struct CliArguments {
    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let cli_args = CliArguments::parse();

    match cli_args.subcommand.unwrap_or_default() {
        Subcommand::Setup => {
            replace_entities_i_hate_my_life().await?;
        }
        Subcommand::View => {
            let file_content = tokio::fs::read_to_string(
                "./.data/doc-en/reference/array/functions/array-reduce.xml",
            )
            .await?;

            let parser = XmlParser::default();

            let function = parser.parse_function(file_content)?;
            println!("{function}");
        }
    };
    // let terminal = ratatui::init();
    // let result = run(terminal).await;

    // ratatui::restore();
    Ok(())
}

async fn replace_entities_i_hate_my_life() -> std::io::Result<()> {
    let regex = fancy_regex::Regex::new("&(?!(amp|quot|gt|lt)\\b)([a-z]+);").unwrap();
    for file in glob::glob("./.data/**/functions/**/*.xml").unwrap() {
        let file = file.unwrap();
        let file_content = tokio::fs::read_to_string(&file).await?;
        let replaced_content = regex.replace_all(&file_content, |e: &Captures| {
            format!("<constant>{}</constant>", e.get(2).unwrap().as_str())
        });

        tokio::fs::write(file, replaced_content.to_string()).await?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::parser::{self, XmlParser};

    #[test]
    pub fn yeah() {
        let parser = XmlParser::default();
        for file in glob::glob("./.data/**/functions/**/*.xml").unwrap() {
            let file = file.unwrap();

            println!("Parsing file {}", &file.to_string_lossy());
            let function = parser.parse_function(std::fs::read(file).unwrap());
            let function = function.unwrap();
            println!("{function}");
        }
    }
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
