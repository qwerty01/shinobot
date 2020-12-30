use serenity::{client::Context, framework::standard::{CommandResult, macros::command}, model::channel::Message};
use std::fmt::Write;

use crate::CommandCounter;

// Commands can be created via the attribute `#[command]` macro.
#[command]
// Options are passed via subsequent attributes.
// Make this command use the "complicated" bucket.
#[bucket = "complicated"]
pub async fn commands(ctx: &Context, msg: &Message) -> CommandResult {
    let mut contents = "Commands used:\n".to_string();

    let data = ctx.data.read().await;
    let counter = data.get::<CommandCounter>().expect("Expected CommandCounter in TypeMap.");

    for (k, v) in counter {
        writeln!(contents, "- {name}: {amount}", name=k, amount=v)?;
    }

    msg.channel_id.say(&ctx.http, &contents).await?;

    Ok(())
}
