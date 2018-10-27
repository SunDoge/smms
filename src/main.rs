#![feature(await_macro, async_await, futures_api)]

#[macro_use]
extern crate tokio;

#[macro_use]
extern crate clap;

// use tokio::prelude::*;

use clap::{App, Arg};
use glob::glob;
use smms::Client;

fn main() {
    let matches = App::new("smms cli")
        .version(crate_version!())
        .author(crate_authors!())
        .about("do something")
        .arg(
            Arg::with_name("upload")
                .long("upload")
                .short("u")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("delete")
                .long("delete")
                .short("d")
                .takes_value(true),
        )
        .arg(Arg::with_name("list").long("list").short("l"))
        .arg(Arg::with_name("history").long("history"))
        .arg(Arg::with_name("clear").long("clear"))
        .get_matches();
    let client = Client::new();

    if matches.is_present("list") {
        let res = client.list().unwrap();
        println!("{:#?}", res);
    } else if let Some(pattern) = matches.value_of("upload") {
        let res = client.upload(pattern).unwrap();
        println!("{:#?}", res);
    } else if let Some(hash) = matches.value_of("delete") {
        let _res = client.delete(hash).expect("delete");
    }

    // tokio::run_async(async move {
    //     if matches.is_present("list") {
    //         await!(client.list()).unwrap();
    //     } else if let Some(pattern) = matches.value_of("upload") {
    //         await!(client.upload(pattern)).unwrap();
    //     }
    // });
}

// async fn run() {
//     await!(history()).unwrap();
// }
