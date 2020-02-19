use std::env::{current_dir, vars};
use std::sync::{Arc, RwLock, atomic};
use std::thread;

use deltachat::chat::*;
use deltachat::config;
use deltachat::constants::{Chattype, Viewtype, DC_CONTACT_ID_SELF};
use deltachat::context::*;
use deltachat::error::Error;
use deltachat::job::{
    perform_inbox_fetch, perform_inbox_idle, perform_inbox_jobs, perform_smtp_idle,
    perform_smtp_jobs,
};
use deltachat::message::*;
use deltachat::Event;

fn handle_message(_ctx: &Context, chat_id: ChatId, msg_id: MsgId) -> Result<(), Error> {
    let chat = Chat::load_from_db(_ctx, chat_id)?;
    let msg = Message::load_from_db(_ctx, msg_id)?;

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
        send_msg(_ctx, chat_id, &mut message)?;
    }

    Ok(())
}

fn cb(_ctx: &Context, event: Event) {
    print!("[{:?}]", event);

    match event {
        Event::ConfigureProgress(progress) => {
            println!("  progress: {}", progress);
        }
        Event::Info(msg) | Event::Warning(msg) | Event::Error(msg) | Event::ErrorNetwork(msg) => {
            println!("  {}", msg);
        }
        Event::MsgsChanged { chat_id, msg_id } => match handle_message(_ctx, chat_id, msg_id) {
            Err(err) => {
                print!("{}", err);
            }
            Ok(_val) => {}
        },
        ev => {
            println!("[EV] {:?}", ev);
        }
    }
}

fn main() {
    let dbdir = current_dir().unwrap().join("deltachat-db");
    std::fs::create_dir_all(dbdir.clone()).unwrap();
    let dbfile = dbdir.join("db.sqlite");
    println!("creating database {:?}", dbfile);
    let ctx =
        Context::new(Box::new(cb), "FakeOs".into(), dbfile).expect("Failed to create context");
    let running = Arc::new(RwLock::new(true));
    let info = ctx.get_info();
    println!("info: {:#?}", info);

    let ctx = Arc::new(ctx);
    let ctx1 = ctx.clone();
    let r1 = running.clone();
    let _t1 = thread::spawn(move || {
        while *r1.read().unwrap() {
            perform_inbox_jobs(&ctx1);
            if *r1.read().unwrap() {
                perform_inbox_fetch(&ctx1);

                if *r1.read().unwrap() {
                    perform_inbox_idle(&ctx1);
                }
            }
        }
    });

    let ctx1 = ctx.clone();
    let r1 = running.clone();
    let _t2 = thread::spawn(move || {
        while *r1.read().unwrap() {
            perform_smtp_jobs(&ctx1);
            if *r1.read().unwrap() {
                perform_smtp_idle(&ctx1);
            }
        }
    });

    println!("configuring");

    if let Some(addr) = vars().find(|key| key.0 == "addr") {
        ctx.set_config(config::Config::Addr, Some(&addr.1)).unwrap();
    } else {
        panic!("no addr ENV var specified");
    }

    if let Some(pw) = vars().find(|key| key.0 == "mailpw") {
        ctx.set_config(config::Config::MailPw, Some(&pw.1)).unwrap();
    } else {
        panic!("no mailpw ENV var specified");
    }

    let interupted = Arc::new(atomic::AtomicBool::new(false));
    let i = interupted.clone();

    ctrlc::set_handler(move || {
        i.store(true, atomic::Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
    
    ctx.configure();

    // wait for ctrl+c
    while !interupted.load(atomic::Ordering::SeqCst) {
       thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("stopping threads");
    *running.write().unwrap() = false;
    deltachat::job::interrupt_inbox_idle(&ctx);
    deltachat::job::interrupt_smtp_idle(&ctx);

    println!("joining");
    _t1.join().unwrap();
    _t2.join().unwrap();

    println!("closing");
}
