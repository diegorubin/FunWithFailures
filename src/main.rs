extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
extern crate reqwest;

use std::env;

use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;

fn status() {
    let uri: &str = &*env::var("ENDPOINT").unwrap();
    let resp: String = reqwest::get(uri).unwrap()
        .text().unwrap();
    println!("{:#?}", resp);
}


fn main() {
    status();
    let mut core = Core::new().unwrap();

    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let api = Api::configure(token).build(core.handle()).unwrap();

    let future = api.stream().for_each(|update| {

        if let UpdateKind::Message(message) = update.kind {

            if let MessageKind::Text {ref data, ..} = message.kind {
                api.spawn(message.text_reply(
                    format!("Hi, {}! You just wrote '{}'", &message.from.first_name, data)
                ));
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}
