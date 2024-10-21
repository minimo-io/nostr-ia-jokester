use clap::{Arg, Command};
use dotenvy::dotenv;
use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse arguemnts
    let m = Command::new("Nostr IA Jokester")
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
        .get_matches();

    // Parse arguments
    if m.get_flag("generate-keys") {
        println!("> Generating a new keypair for a new bot.");
        let my_keys: Keys = Keys::generate();
        // let hex_pubkey: String = my_keys.public_key().to_hex();
        let npub = my_keys.public_key().to_bech32().unwrap();
        let nsec = my_keys.secret_key().to_bech32().unwrap();

        println!("npub: {}", npub); // npub
        println!("nsec: {}", nsec); // nsec
    } else if m.get_flag("list-keys") {
        println!("ToDo -- Let's do some listing dudette!");
    } else if m.get_flag("run") {
        println!("> Let's run this thing.");
        // Get bot nsec & public keys from .env
        dotenv().ok();

        let bot_nsec =
            std::env::var("BOT_NSEC").expect("> You need an nsec in your .env honey (BOT_NSEC=).");
        // get public keys from nsec
        let keys = Keys::parse(&bot_nsec).expect("Could not derive keys.");
        let npub = keys.public_key.to_bech32().unwrap();
        println!("hex: {}", keys.public_key);
        println!("npub: {npub}");
    }

    // -------------------------------------------------------------------------

    // let my_keys = match Keys::parse(bot_nsec) {
    //     Ok(keys) => keys,
    //     Err(error) => {
    //         eprintln!("Error parsing keys: {:?}", error);
    //         return Err(error.into());
    //     }
    // };

    // let bech32_pubkey = match my_keys.public_key().to_bech32() {
    //     Ok(pubkey) => pubkey,
    //     Err(error) => {
    //         eprintln!("Error converting to Bech32: {:?}", error);
    //         return Err(error.into());
    //     }
    // };

    // println!("Bech32 PubKey: {}", bech32_pubkey);

    // // Directly create a client without matching against Result
    // let client = Client::new(&my_keys);

    // client.add_relay("wss://relay.damus.io").await?;
    // client.connect().await;

    // // Publish a text note
    // let note_result = client
    //     .publish_text_note("Rustification process continues; first bot -> checked!", [])
    //     .await?;
    // println!("Note ID is: {}", note_result.to_string());

    Ok(())
}
