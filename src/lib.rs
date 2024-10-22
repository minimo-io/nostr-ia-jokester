use clap::{Arg, Command};

/// Arguments parsing for the bot
pub fn cli() -> Command {
    Command::new("Nostr IA Jokester")
        .about("A (not so) funny AI joker Bot for Nostr written in Rust, using open-source LLMs.")
        // .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("generate-keys")
                .long("generate-keys")
                .short('G')
                .help("Generates new keys")
                .action(clap::ArgAction::SetTrue), // This will set the value to true if the argument is present
        )
        .arg(
            Arg::new("list-keys")
                .long("list-keys")
                .short('L')
                .help("List all bot keys already generateed")
                .action(clap::ArgAction::SetTrue), // This will set the value to true if the argument is present
        )
        .arg(
            Arg::new("run")
                .long("run")
                .short('R')
                .help("Run the jokester!")
                .action(clap::ArgAction::SetTrue), // This will set the value to true if the argument is present
        )
}
