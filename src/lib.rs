//! Requires the 'framework' feature flag be enabled in your project's
//! `Cargo.toml`.
//!
//! This can be enabled by specifying the feature in the dependency section:
//!
//! ```toml
//! [dependencies.serenity]
//! git = "https://github.com/serenity-rs/serenity.git"
//! features = ["framework", "standard_framework"]
//! ```
use std::{collections::HashMap, sync::Arc};
use serenity::{async_trait, client::{bridge::gateway::ShardManager}, framework::standard::{
        DispatchError,
    }, model::{
        channel::Message,
        gateway::Ready,
    }};

use serenity::prelude::*;
use tokio::sync::Mutex;

pub mod commands;
pub mod checks;
pub mod hooks;
pub mod groups;

pub use crate::commands::*;

// A container type is created for inserting into the Client's `data`, which
// allows for data to be accessible across all events and framework commands, or
// anywhere else that has a copy of the `data` Arc.
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

// You can construct a hook without the use of a macro, too.
// This requires some boilerplate though and the following additional import.
use serenity::{futures::future::BoxFuture, FutureExt};
fn _dispatch_error_no_macro<'fut>(ctx: &'fut mut Context, msg: &'fut Message, error: DispatchError) -> BoxFuture<'fut, ()> {
    async move {
        if let DispatchError::Ratelimited(duration) = error {
            let _ = msg
                .channel_id
                .say(&ctx.http, &format!("Try this again in {} seconds.", duration.as_secs()))
                .await;
        };
    }.boxed()
}
