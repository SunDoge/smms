#![feature(await_macro, async_await, futures_api)]

#[macro_use]
extern crate tokio;

#[macro_use]
extern crate clap;

// use tokio::prelude::*;

use smms::history;
use clap::{Arg, App};
use glob::glob;

fn main() {
    let matches = App::new("smms cli")
    .version(crate_version!())
    .author(crate_authors!())
    .about("do something")
    .arg(Arg::with_name("file").long("file").short("f").takes_value(true))
    .arg(Arg::with_name("delete").long("delete").short("d").takes_value(true))
    .arg(Arg::with_name("list").long("list").short("l"))
    .arg(Arg::with_name("history").long("history"))
    .arg(Arg::with_name("clear").long("clear"))
    .arg(Arg::with_name("filename").takes_value(true))
    .get_matches();

    tokio::run_async(async move {
        if matches.is_present("history") {
            await!(history()).unwrap();
        }
    });
}

// async fn run() {
//     await!(history()).unwrap();
// }
