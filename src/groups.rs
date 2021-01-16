use serenity::framework::standard::macros::group;
use crate::commands::*;

#[group]
#[commands(about, am_i_admin, say, commands, ping, latency, some_long_command, upper_command, rc)]
pub struct General;

#[group]
// Sets multiple prefixes for a group.
// This requires us to call commands in this group
// via `~emoji` (or `~em`) instead of just `~`.
#[prefixes("emoji", "em")]
// Set a description to appear if a user wants to display a single group
// e.g. via help using the group-name or one of its prefixes.
#[description = "A group with commands providing an emoji as response."]
// Sets a command that will be executed if only a group-prefix was passed.
#[default_command(bird)]
#[commands(cat, dog)]
pub struct Emoji;

#[group]
// Sets a single prefix for this group.
// So one has to call commands in this group
// via `~math` instead of just `~`.
#[prefix = "math"]
#[commands(multiply)]
pub struct Math;

#[group]
#[owners_only]
// Limit all commands to be guild-restricted.
#[only_in(guilds)]
#[commands(slow_mode)]
pub struct Owner;

