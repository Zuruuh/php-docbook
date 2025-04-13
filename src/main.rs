use std::sync::Arc;

use clap::Parser;
use color_eyre::Result;
use futures_util::lock::Mutex;

use self::{
    cli::replace_entities_i_hate_my_life,
    parser::{XmlError, XmlParser, function::Function},
    tui::TerminalState,
};

mod cli;
mod parser;
mod tui;

#[derive(Default, clap::Subcommand)]
pub enum Subcommand {
    /// [WIP] replace all xml entities in the source
    Setup,
    /// [default] Enter the terminal UI
    #[default]
    Start,
}

pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .header(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .invalid(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))))
        .valid(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}

#[derive(Parser)]
#[command(version, about, styles=get_styles())]
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

            Ok(())
        }
        Subcommand::Start => {
            let terminal = ratatui::init();

            let files = glob::glob("./.data/**/functions/**/*.xml")?
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;

            let mut state = TerminalState::default();
            state.shared_state.total_files_to_parse = files.len();

            let parsed_files = Arc::new(Mutex::new(Vec::<Function>::new()));
            let xml_parser = Arc::new(XmlParser::default());
            let parsed_files_arc_for_tokio_task = Arc::clone(&parsed_files);

            tokio::spawn(async move {
                let futures = files
                    .into_iter()
                    .map(async |filepath| -> Result<(), XmlError> {
                        let file_content = std::fs::read(filepath).map_err(XmlError::IOError)?;
                        let result = Arc::clone(&xml_parser).parse_function(file_content)?;

                        Arc::clone(&parsed_files_arc_for_tokio_task)
                            .lock()
                            .await
                            .push(result);

                        Ok(())
                    })
                    .collect::<Vec<_>>();

                futures_util::future::join_all(futures).await;
            });

            let result = state
                .run(
                    terminal,
                    Box::new(async move |state: &mut TerminalState| {
                        state.shared_state.parsed_files_snapshot = Arc::clone(&parsed_files)
                            .lock()
                            .await
                            .to_vec()
                            .into_iter()
                            .collect();
                    }),
                )
                .await;

            ratatui::restore();

            result
        }
    }
}
