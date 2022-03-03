mod command;
mod discord_init;
use serenity::{async_trait, model::{channel::Message, gateway::Ready}, prelude::*};

//Commands
const HELLO_COMMAND: &str = "/hello";
const HELLO_MESSAGE: &str = "Hello!";

const LOOP_COMMAND: &str = "/loop";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELLO_COMMAND {
            if let Err(why) = msg.reply(&ctx.http, HELLO_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content == LOOP_COMMAND {
            let ret_msg = command::loop_test("");
            if let Err(why) = msg.channel_id.say(&ctx.http, &ret_msg).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let mut client = Client::builder(discord_init::token_discord("")).event_handler(Handler).await.expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
    println!("Hello, world!");
}