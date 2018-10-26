#![feature(await_macro, async_await, futures_api)]

#[macro_use]
extern crate tokio;

#[macro_use]
extern crate serde_derive;

pub mod api;

use reqwest::r#async::{Client, Decoder};
use std::io::{self, Cursor};
use std::mem;
use tokio::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Data {
    width: u32,
    height: u32,
    filename: String,
    storename: String,
    size: usize,
    path: String,
    hash: String,
    timestamp: usize,
    url: String,
    delete: String,
}

#[derive(Debug, Deserialize)]
pub struct History {
    code: String,
    data: Vec<Data>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    code: String,
    msg: String,
}

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

pub async fn upload(filename: &str) -> Result<(), ()> {
    unimplemented!()
}

pub struct SmmsClient {
    client: Client,
}

impl SmmsClient {
    pub fn new() -> SmmsClient {
        SmmsClient {
            client: Client::new(),
        }
    }

    pub async fn list(&self) -> Result<(), ()> {
        let mut res = await!(self.client.get(api::HISTORY).send())
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
}
