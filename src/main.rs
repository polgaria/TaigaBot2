extern crate serenity;
extern crate serde;
extern crate serde_json;
extern crate reqwest;

use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

group!({
    name: "taiga",
    options: {},
    commands: [taiga],
});

use serde::Deserialize;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

struct Handler;

impl EventHandler for Handler {}

#[derive(Deserialize)]
struct Config {
    prefix: String,
    token: String
}

fn main() {
    let mut file = File::open("config.json").expect("config");
    let mut config_json = String::new();
    file.read_to_string(&mut config_json);
    let config: Config = serde_json::from_str(&config_json.to_owned()).unwrap();
    let mut client = Client::new(&config.token, Handler)
        .expect("Error creating client");

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix(&config.prefix))
        .group(&TAIGA_GROUP));

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn taiga(ctx: &mut Context, msg: &Message) -> CommandResult {
	let post = reqwest::get("https://reddit.com/r/taiga/random.json")?.text()?;
	println!("{}", post);
	let json: serde_json::Value = serde_json::from_str(&post)?;
	if json[0]["data"]["children"][0]["data"]["is_self"].as_bool().unwrap() {
        // probably doesn't work
        msg.channel_id.say(&ctx.http, format!("https://reddit.com{}", json[0]["data"]["children"][0]["data"]["permalink"].as_str().unwrap()));
    } else {
		msg.channel_id.say(&ctx.http, json[0]["data"]["children"][0]["data"]["url"].as_str().unwrap());
	}

    Ok(())
}
