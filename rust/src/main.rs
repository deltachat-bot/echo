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
use std::sync::Arc;

use anyhow::{Context as _, Result};
use deltachat::chat::{self, Chat, ChatId};
use deltachat::config;
use deltachat::constants::Chattype;
use deltachat::context::Context;
use deltachat::message::{Message, MsgId, Viewtype};
use deltachat::EventType;
use deltachat::Events;
use futures_lite::future::FutureExt;
use tokio::signal;

async fn handle_message(
    ctx: &Context,
    chat_id: ChatId,
    msg_id: MsgId,
) -> Result<(), anyhow::Error> {
    let chat = Chat::load_from_db(ctx, chat_id).await?;
    let msg = Message::load_from_db(ctx, msg_id).await?;

    println!(
        "recieved message '{}' in chat with type {:?}",
        msg.get_text().unwrap_or_else(|| "".to_owned()),
        chat.get_type()
    );

    if chat.get_type() == Chattype::Single {
        let mut message = Message::new(Viewtype::Text);
        message.set_text(msg.get_text());
        chat::send_msg(ctx, chat_id, &mut message).await?;
    }

    Ok(())
}

async fn cb(ctx: &Context, event: EventType) {
    //println!("[{:?}]", event);

    match event {
        EventType::ConfigureProgress { progress, comment } => {
            println!("  progress: {} {:?}", progress, comment);
        }
        EventType::Info(msg) | EventType::Warning(msg) | EventType::Error(msg) => {
            println!(" {}", msg);
        }
        EventType::ConnectivityChanged => {
            println!("ConnectivityChanged: {:?}", ctx.get_connectivity().await);
        }
        EventType::IncomingMsg { chat_id, msg_id } => {
            match handle_message(ctx, chat_id, msg_id).await {
                Err(err) => {
                    print!("{}", err);
                }
                Ok(_val) => {}
            }
        }
        ev => {
            println!("[EV] {:?}", ev);
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dbdir = env::current_dir()
        .context("failed to get current working directory")?
        .join("deltachat-db");
    std::fs::create_dir_all(dbdir.clone()).context("failed to create data folder")?;
    let dbfile = dbdir.join("db.sqlite");
    println!("creating database {:?}", dbfile);
    let ctx = Context::new(dbfile.as_path(), 1, Events::new())
        .await
        .context("Failed to create context")?;

    let info = ctx.get_info().await;
    println!("info: {:#?}", info);
    let ctx = Arc::new(ctx);

    let events = ctx.get_event_emitter();

    let is_configured = ctx.get_config_bool(config::Config::Configured).await?;
    if !is_configured {
        println!("configuring");
        if let Some(addr) = env::vars().find(|key| key.0 == "addr") {
            ctx.set_config(config::Config::Addr, Some(&addr.1))
                .await
                .context("set config failed")?;
        } else {
            panic!("no addr ENV var specified");
        }
        if let Some(pw) = env::vars().find(|key| key.0 == "mail_pw") {
            ctx.set_config(config::Config::MailPw, Some(&pw.1))
                .await
                .context("set config failed")?;
        } else {
            panic!("no mail_pw ENV var specified");
        }
        ctx.set_config(config::Config::Bot, Some("1"))
            .await
            .context("set config failed")?;
        ctx.set_config(config::Config::E2eeEnabled, Some("1"))
            .await
            .context("set config failed")?;

        ctx.configure().await.context("configure failed")?;
        println!("configuration done");
    } else {
        println!("account is already configured");
    }

    println!("------ RUN ------");
    ctx.start_io().await;

    // wait for ctrl+c or event
    while let Some(event) = async {
        signal::ctrl_c().await.expect("failed to listen for event");
        None
    }
    .race(events.recv())
    .await
    {
        cb(&ctx, event.typ).await;
    }

    println!("stopping");
    ctx.stop_io().await;
    println!("closing");
    drop(ctx);

    while let Some(event) = events.recv().await {
        println!("ignoring event {:?}", event);
    }

    Ok(())
}
