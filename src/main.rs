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
    let matches = App::new("smms cli").version(crate_version!()).author(crate_authors!()).about("do something").arg(Arg::with_name("filename").long("file").short("f")).arg(Arg::with_name("delete").long("delete").short("d")).get_matches();

    tokio::run_async(async {
        await!(history()).unwrap()
    });
}
