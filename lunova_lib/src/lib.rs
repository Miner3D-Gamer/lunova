//! An underlying lib both the client and server can use to keep in sync
// use mirl::prelude::*;

/// Stuff that has to do with the unique identifiers of a user
pub mod users;

/// Anything to do with chatting 'n stuff from dms to guilds
pub mod communication;

/// Client related stuff
pub mod client;
/// Server related stuff
pub mod server;

/// The "api", the central communication between clients and a server
pub mod throughput;

/// Saving/Loading client and server stuff
pub mod fs;
