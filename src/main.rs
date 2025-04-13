use crate::cli::replace_entities_i_hate_my_life;
use clap::Parser;
use color_eyre::Result;
use tui::TerminalState;

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
            let state = TerminalState::default();
            let result = state.run(terminal).await;

            ratatui::restore();

            result
        }
    }
}
