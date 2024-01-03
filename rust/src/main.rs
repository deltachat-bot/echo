//! Echo bot example in Rust.
//!
//! This is a bot example which echoes back the text of any message it receives in direct
//! 1:1 chats.  It contains a number of steps:
//!
//! - Create the directory for deltachat storage.  Deltachat is statefull and all the state
//!   is stored here.
//! - Creates the [`Context`].
//! - On the first run: configures the credentials.
//! - Spawns a Tokio task to receive and handle events from deltachat-core.
//! - Starts the tasks for deltchat-core to send and receive messages.
//! - Waits for the shutdown signal.
//! - Stops the tasks for deltachat-core to send and receive messages.
//!
//! Central to deltachat-core is the [`Context`], it is a struct which contains all the
//! state to run a single DeltaChat account.  Once a [`Context`] is created all operations
//! work on this context.
//!
//! # Running the bot
//!
//! To run the bot, the first time you need to set the credentials in the environment:
//!
//! ```shell
//! addr=$yourEmail mail_pw=$yourPassword cargo run
//! ```
//!
//! This will create a subdirectory called `deltachat-db` in the current working directory,
//! this is where deltachat stores the state.  The credentials will be stored here so
//! further invocations do not need them specified again if the `deltachat-db` directory can
//! be found again.
//!
//! # API Stability
//!
//! It should be noted there is currently no API stability guarantee for the Rust API.  The
//! only API stability provided by the DeltaChat library is on the C API via the
//! `deltachat_ffi` crate.  However when using Rust it is probably better to live with the
//! API instability instead of going via the unsafe `deltachat_ffi` crate.  In practice the
//! API does not change drastically between tagged releases.
//!
//! The main reason for this is that the "`pub`" API of the `deltachat` crate are not always
//! very streamlined or Rust-like, e.g. you may notice missing idioms like RAII in parts.
//! You will also notice is misses a lot of documentation on how things work.  It was
//! originally converted from a C library and still contains oddities based on this.
//!
//! The API surface has also never been checked to see it makes sense, so you may be able to
//! find items that really should be private and internal.  So please use caution when
//! accessing some deep internals, it might be worth speaking to core developers about your
//! usecase.
//!
//! # Crate versions
//!
//! The `deltachat` crate is currently not released on crates.io, this means you need to use
//! it from a git reference.  Releases are made regularly and [tagged in the
//! repository](https://github.com/deltachat/deltachat-core-rust/tags).  The tags for the
//! `deltachat` crate, also known as the "core", are of the `1.XX.Y` format where `Y` is
//! usually `0`.  To use this in `Cargo.toml` use a line like:
//!
//! ```toml
//! [dependencies]
//! deltachat = { git = "https://github.com/deltachat/deltachat-core-rust", tag="1.98.0"}
//! ```

#![forbid(unsafe_code)]
#![warn(
    clippy::correctness,
    missing_debug_implementations,
    missing_docs,
    unused,
    clippy::all,
    clippy::cast_lossless,
    clippy::indexing_slicing,
    clippy::needless_borrow,
    clippy::unused_async,
    clippy::wildcard_imports
)]

use std::env;

use anyhow::{Context as _, Result};
use deltachat::chat::{self, Chat, ChatId};
use deltachat::config::Config;
use deltachat::constants::Chattype;
use deltachat::context::{Context, ContextBuilder};
use deltachat::message::{Message, MsgId, Viewtype};
use deltachat::EventType;
use tokio::signal;

/// Main entry.
///
/// The deltachat-core library uses Tokio extensively internally so must be used from inside
/// a Tokio runtime.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dbdir = env::current_dir()?.join("deltachat-db");
    std::fs::create_dir_all(dbdir.clone()).context("failed to create data folder")?;
    let dbfile = dbdir.join("db.sqlite");
    println!("creating database {:?}", dbfile);
    let mut ctx = ContextBuilder::new(dbfile)
        .open()
        .await
        .context("Failed to create context")?;

    let info = ctx.get_info().await;
    println!("info: {:#?}", info);

    // Create a single event emitter, this should only be done once for a context.  It is
    // immediately passed to a new tokio task which loops handling events from it.  This
    // even handler does the main work of receiving messages and responding.  This task will
    // never terminate by itself: since it owns the Context it keeps it open and the event
    // stream will never terminate.
    let events_emitter = ctx.get_event_emitter();
    let emitter_ctx = ctx.clone();
    tokio::spawn(async move {
        while let Some(event) = events_emitter.recv().await {
            handle_event(&emitter_ctx, event.typ).await;
        }
    });

    // The first time this runs it needs to be configured.  The configuration is stored in
    // the database so if the database was found earlier no further configuration is needed.
    let is_configured = ctx.get_config_bool(Config::Configured).await?;
    if !is_configured {
        println!("configuring");
        configure_from_env(&ctx).await?;
        println!("configuration done");
    } else {
        println!("account is already configured");
    }

    // Starts the deltachat send and receive task, from now on messages will be sent and
    // received.
    println!("------ RUN ------");
    ctx.start_io().await;

    // wait for user interrupt using ctrc+c.
    signal::ctrl_c().await?;

    // Stop the deltachat tasks again.
    ctx.stop_io().await;

    Ok(())
}

/// Configures the context from environment variables.
///
/// Uses the `addr` and `mail_pw` environment variables which should contain an email
/// address and password to use and will try and use autoconfiguration on the detected email
/// domain.
///
/// It is possible to do a more detailed configuration for SMTP & IMAP addresses, usernames
/// and passwords.  For this look at the [`Config::MailServer`], [`Config::MailUser`],
/// [`Config::MailPw`] etc variables to configure IMAP and the [`Config::SendServer`],
/// [`Config::SendUser`], [`Config::SendPw`] etc variables to configure SMTP.
async fn configure_from_env(ctx: &Context) -> Result<()> {
    let addr = env::var("addr")?;
    ctx.set_config(Config::Addr, Some(&addr)).await?;
    let pw = env::var("mail_pw")?;
    ctx.set_config(Config::MailPw, Some(&pw)).await?;
    ctx.set_config(Config::Bot, Some("1")).await?;
    ctx.set_config(Config::E2eeEnabled, Some("1")).await?;

    // Invoke configuration, this will fail if deltachat can not log into the SMTP and IMAP
    // servers using the provided credentials.
    ctx.configure()
        .await
        .context("configure failed, incorrect credentials?")?;

    Ok(())
}

/// Handles events emitted by the deltachat-core [`Context`].
///
/// Events are used for pretty much everything, this function shows handling some of the
/// more important ones:
///
/// - [`Info`], [`Warning`] and [`Error`] are the logging mechanism of deltachat-core, which
///   is always per-context.  Commonly these might be written to a logfile.
///
/// - [`IncomingMsg`] indicates a new message has arrived.
///
/// [`Info`]: EventType::Info
/// [`Warning`]: EventType::Warning
/// [`Error`]: EventType::Error
/// [`IncomingMsg`]: EventType::IncomingMsg
async fn handle_event(ctx: &Context, event: EventType) {
    match event {
        EventType::ConfigureProgress { progress, comment } => {
            println!("  progress: {progress} {comment:?}")
        }
        EventType::Info(msg) => println!(" I: {msg}"),
        EventType::Warning(msg) => println!(" W: {msg}"),
        EventType::Error(msg) => println!(" E: {msg}"),
        EventType::ConnectivityChanged => {
            println!("ConnectivityChanged: {:?}", ctx.get_connectivity().await)
        }
        EventType::IncomingMsg { chat_id, msg_id } => {
            if let Err(err) = handle_message(ctx, chat_id, msg_id).await {
                println!("error handling message: {err}");
            }
        }
        other => {
            println!("[unhandled event] {other:?}");
        }
    }
}

/// Handles a single incoming message.
///
/// Each message belongs to a chat, which is a conversation of messages between multiple
/// participants.
async fn handle_message(ctx: &Context, chat_id: ChatId, msg_id: MsgId) -> Result<()> {
    let chat = Chat::load_from_db(ctx, chat_id).await?;
    let msg = Message::load_from_db(ctx, msg_id).await?;

    println!(
        "received message '{}' in chat with type {:?}",
        msg.get_text(),
        chat.get_type()
    );

    // Only respond to messages from a chat with only a single participant other than
    // ourselves.  This is also known as a "1:1" chat.
    if chat.get_type() == Chattype::Single {
        let mut message = Message::new(Viewtype::Text);
        message.set_text(msg.get_text());
        chat::send_msg(ctx, chat_id, &mut message).await?;
    }

    Ok(())
}
