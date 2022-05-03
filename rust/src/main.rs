use futures_lite::future::FutureExt;
use std::env::{current_dir, vars};
use std::sync::Arc;

use deltachat::chat::*;
use deltachat::config;
use deltachat::constants::{Chattype, Viewtype, DC_CONTACT_ID_SELF};
use deltachat::context::*;
use deltachat::error::Error;
use deltachat::message::*;
use deltachat::EventType;

async fn handle_message(_ctx: &Context, chat_id: ChatId, msg_id: MsgId) -> Result<(), Error> {
    let chat = Chat::load_from_db(_ctx, chat_id).await?;
    let msg = Message::load_from_db(_ctx, msg_id).await?;

    if msg.get_from_id() == DC_CONTACT_ID_SELF {
        // prevent loop (don't react to own messages)
        return Ok(());
    }

    println!(
        "recieved message '{}' in chat with type {:?}",
        msg.get_text().unwrap_or("".to_owned()),
        chat.get_type()
    );

    if chat.get_type() == Chattype::Single {
        let mut message = Message::new(Viewtype::Text);
        message.set_text(msg.get_text());
        send_msg(_ctx, chat_id, &mut message).await?;
    }

    Ok(())
}

async fn cb(_ctx: &Context, event: EventType) {
    //println!("[{:?}]", event);

    match event {
        EventType::ConfigureProgress { progress, comment } => {
            println!("  progress: {} {:?}", progress, comment);
        }
        EventType::Info(msg)
        | EventType::Warning(msg)
        | EventType::Error(msg)
        | EventType::ErrorNetwork(msg) => {
            println!(" {}", msg);
        }
        EventType::MsgsChanged { chat_id, msg_id } => {
            match handle_message(_ctx, chat_id, msg_id).await {
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

#[async_std::main]
async fn main() {
    let dbdir = current_dir().unwrap().join("deltachat-db");
    std::fs::create_dir_all(dbdir.clone()).unwrap();
    let dbfile = dbdir.join("db.sqlite");
    println!("creating database {:?}", dbfile);
    let ctx = Context::new("FakeOs".into(), dbfile.into(), 0)
        .await
        .expect("Failed to create context");

    let info = ctx.get_info().await;
    println!("info: {:#?}", info);
    let ctx = Arc::new(ctx);

    let events = ctx.get_event_emitter();

    let (interrupt_send, interrupt_recv) = async_std::sync::channel(1);
    ctrlc::set_handler(move || async_std::task::block_on(interrupt_send.send(())))
        .expect("Error setting Ctrl-C handler");

    let is_configured = ctx.get_config_bool(config::Config::Configured).await;
    if !is_configured {
        println!("configuring");
        if let Some(addr) = vars().find(|key| key.0 == "addr") {
            ctx.set_config(config::Config::Addr, Some(&addr.1))
                .await
                .unwrap();
        } else {
            panic!("no addr ENV var specified");
        }
        if let Some(pw) = vars().find(|key| key.0 == "mail_pw") {
            ctx.set_config(config::Config::MailPw, Some(&pw.1))
                .await
                .unwrap();
        } else {
            panic!("no mail_pw ENV var specified");
        }
        ctx.set_config(config::Config::Bot, Some("1"))
            .await
            .unwrap();
        ctx.set_config(config::Config::E2eeEnabled, Some("1"))
            .await
            .unwrap();

        ctx.configure().await.unwrap();
        println!("configuration done");
    } else {
        println!("account is already configured");
    }

    println!("------ RUN ------");
    ctx.start_io().await;

    // wait for ctrl+c or event
    while let Some(event) = async {
        interrupt_recv.recv().await.unwrap();
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
}
