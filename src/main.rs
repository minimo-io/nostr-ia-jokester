use dotenv::dotenv;
use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let bot_nsec = std::env::var("BOT_NSEC").expect("> You need an nsec honey.");
    let keys = Keys::parse(bot_nsec).expect("Could not derive keys."); // get keys from secret
    println!("Public key: {}", keys.public_key);

    // Create keys -------------------------------------------------------------
    // let my_keys: Keys = Keys::generate();
    // let hex_pubkey: String = my_keys.public_key().to_hex();
    // let nsec = my_keys.secret_key();
    // match nsec{
    //     Ok(nsec) => println!("Secret key: {:?}", nsec.to_bech32()),
    //     Err(error) => eprintln!("Error: {:?}", error),
    // }
    // println!("Hex Pub: {}" , hex_pubkey);

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
