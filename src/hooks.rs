use serenity::{client::Context, framework::standard::{CommandResult, DispatchError, macros::hook}, model::channel::Message};

use crate::CommandCounter;

#[hook]
pub async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    println!("Got command '{}' by user '{}' ({})", command_name, msg.author.name, msg.author.id);

    // Increment the number of times this command has been run once. If
    // the command's name does not exist in the counter, add a default
    // value of 0.
    let mut data = ctx.data.write().await;
    let counter = data.get_mut::<CommandCounter>().expect("Expected CommandCounter in TypeMap.");
    let entry = counter.entry(command_name.to_string()).or_insert(0);
    *entry += 1;

    true // if `before` returns false, command processing doesn't happen.
}

#[hook]
pub async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => println!("Processed command '{}'", command_name),
        Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
    }
}

#[hook]
pub async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}

#[hook]
pub async fn normal_message(_ctx: &Context, msg: &Message) {
    println!("Message is not a command '{}'", msg.content);
}


#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    if let DispatchError::Ratelimited(duration) = error {
        let _ = msg
            .channel_id
            .say(&ctx.http, &format!("Try this again in {} seconds.", duration.as_secs()))
            .await;
    }
}