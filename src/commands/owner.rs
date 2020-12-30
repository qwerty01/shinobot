use serenity::{client::Context, framework::standard::{CommandResult, macros::command}, model::channel::Message};
use crate::checks::*;

#[command]
// Limit command usage to guilds.
#[only_in(guilds)]
#[checks(Owner)]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong! : )").await?;

    Ok(())
}
