use serde::{Deserialize, Serialize};
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Article {
    by: String,
    descendants: i32,
    id: i32,
    kids: Option<Vec<i32>>,
    score: i32,
    time: i32,
    title: String,
    r#type: String,
    url: String,
}

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            "!news top" => {
                let mut resp =
                    reqwest::blocking::get("https://hacker-news.firebaseio.com/v0/topstories.json")
                        .unwrap()
                        .json::<Vec<u64>>()
                        .unwrap();
                resp.truncate(10);
                for id in resp.into_iter() {
                    let article = reqwest::blocking::get(
                        format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id).as_str(),
                    )
                    .unwrap()
                    .json::<Article>()
                    .unwrap();
                    if let Err(why) = msg
                        .channel_id
                        .say(&ctx.http, format!("{} {}", article.title, article.url))
                    {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
            "!news best" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, "University!") {
                    println!("Error sending message: {:?}", why);
                }
            }
            _ => {}
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
