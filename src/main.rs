use std::{collections::HashSet, env};

//use serde::Deserialize;
use serenity::{
    async_trait,
    client::{
        //bridge::gateway::{ShardId, ShardManager},
        Client,
        Context,
        EventHandler,
    },
    framework::standard::{
        //help_commands,
        macros::{check, command, group, help, hook},
        Args,
        //CommandGroup,
        CommandResult,
        //HelpOptions,
        StandardFramework,
    },
    http::Http,
    model::{channel::Message, gateway::Ready, id::UserId, permissions::Permissions},
    prelude::*,
};

#[group]
#[commands(ping, about)]
struct General;

//--------------------------main section start------------
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

// Main section
#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("please provide a discord token");

    // Fetch owners
    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Define the framework
    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .on_mention(Some(bot_id))
                .prefix("!")
                .delimiters(vec![", ", ","])
                .owners(owners)
        })
        .group(&GENERAL_GROUP);

    // Define the client
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

// Commands
#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, format!("Pong!")).await?;

    Ok(())
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "This is a small test-bot! : )")
        .await?;

    Ok(())
}
