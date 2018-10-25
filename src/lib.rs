#![feature(await_macro, async_await, futures_api)]

#[macro_use]
extern crate tokio;

pub mod api;

use reqwest::r#async::{Client, Decoder};
use std::io::{self, Cursor};
use std::mem;
use tokio::prelude::*;

// pub fn upload(filename: &str) {} -> impl Future<Item=(), Error=()> {
//     Client::new().post("")
// }

pub async fn history() -> Result<(), ()> {
    // await!(Client::new()
    //     .get(api::HISTORY)
    //     .send()
    //     .and_then(|mut res| {
    //         println!("{}", res.status());

    //         let body = mem::replace(res.body_mut(), Decoder::empty());
    //         body.concat2()
    //     })
    //     .map_err(|err| println!("request error: {}", err))
    //     .map(|body| {
    //         let mut body = Cursor::new(body);
    //         let _ = io::copy(&mut body, &mut io::stdout()).map_err(|err| {
    //             println!("stdout error: {}", err);
    //         });
    //     }))

    let mut res = await!(Client::new().get(api::HISTORY).send())
        .map_err(|err| println!("request error: {}", err))?;
    println!("{}", res.status());
    let body = mem::replace(res.body_mut(), Decoder::empty());
    let body = await!(body.concat2()).map_err(|err| println!("concat2 error: {}", err))?;
    let mut body = Cursor::new(body);
    let _ = io::copy(&mut body, &mut io::stdout()).map_err(|err| {
        println!("stdout error: {}", err);
    });

    Ok(())
}
