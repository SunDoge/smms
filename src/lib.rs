#![feature(await_macro, async_await, futures_api)]

#[macro_use]
extern crate tokio;

#[macro_use]
extern crate serde_derive;


#[macro_use]
extern crate serde_json;

pub mod api;

use reqwest::r#async::{Client as AsyncReqwestClient, Decoder};
use reqwest::{Client as ReqwestClient, multipart::Form};
use reqwest::StatusCode;
use std::io::{self, Cursor};
use std::mem;
use tokio::prelude::*;
use std::error;
use std::fmt;
use serde_json::Value;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Msg(String),
    Pattern(glob::PatternError),
    Glob(glob::GlobError),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Reqwest(ref e) =>  e.description(),
            Error::Msg(ref s) => s,
            Error::Pattern(ref e) => e.description(),
            Error::Glob(ref e) => e.description(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Reqwest(ref e) => e.fmt(f),
            Error::Msg(ref s) => f.write_str(s),
            Error::Glob(ref e) => e.fmt(f),
            Error::Pattern(ref e) => e.fmt(f),
        } 
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Reqwest(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Msg(err)
    }
}

impl From<glob::PatternError> for Error {
    fn from(err: glob::PatternError) -> Error {
        Error::Pattern(err)
    }
}

impl From<glob::GlobError> for Error {
    fn from(err: glob::GlobError) -> Error {
        Error::Glob(err)
    }
}


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

pub struct AsyncClient {
    client: AsyncReqwestClient,
}

impl AsyncClient {
    pub fn new() -> AsyncClient {
        AsyncClient {
            client: AsyncReqwestClient::new(),
        }
    }

    pub async fn list(&self) -> Result<Vec<Data>, Error> {
        let mut res = await!(self.client.get(api::LIST).send())?;
        // println!("{}", res.status());
        // let body = mem::replace(res.body_mut(), Decoder::empty());
        // let body = await!(body.concat2()).map_err(|err| println!("concat2 error: {}", err))?;
        // let mut body = Cursor::new(body);
        // let _ = io::copy(&mut body, &mut io::stdout()).map_err(|err| {
        //     println!("stdout error: {}", err);
        // });
        if res.status() == StatusCode::OK {
            let data = await!(res.json::<Vec<Data>>())?;
            Ok(data)
        } else {
            Err(Error::Msg(format!("{}", res.status())))
        }
    }

    pub async fn upload<'a>(&'a self, pattern: &'a str) -> Result<Vec<Data>, Error> {
        for entry in glob::glob(pattern)? {
            match entry {
                Ok(path) => println!("{:?}", path.display()),
                Err(e) => println!("{:?}", e),
            }
        }  

        Ok(Vec::new())
    }
}


pub struct Client {
    client: ReqwestClient,
}

impl Client {
    pub fn new() -> Client {
        Client {
            client: ReqwestClient::new(),
        }
    }

    pub fn list(&self) -> Result<Vec<Data>, Error> {
        let mut res = self.client.get(api::LIST).send()?;

        if res.status() ==  StatusCode::OK {
            let mut json_res: Value = res.json()?;
            if json_res["code"] == json!("success") {
                let data: Vec<Data> = serde_json::from_value(json_res["data"].take()).unwrap();
                Ok(data)
            } else {
                let msg = serde_json::from_value(json_res["msg"].take()).unwrap();
                Err(Error::Msg(msg))
            }
        } else {
            Err(Error::Msg(format!("{}", res.status())))
        }
    }

    pub fn upload(&self, pattern: &str) -> Result<Vec<Data>, Error> {
        let mut result = Vec::new();

        for entry in glob::glob(pattern)? {
            match entry {
                Ok(path) => {
                    println!("{}", path.display());
                    let form = Form::new().file("smfile", path).unwrap();
                    let mut res = self.client.post(api::UPLOAD).multipart(form).send()?;
                    if res.status().is_success() {
                        let mut json_res: Value = res.json()?;
                        if json_res["code"] == json!("success") {
                            let data: Data = serde_json::from_value(json_res["data"].take()).unwrap();
                            result.push(data);
                        } else {
                            println!("{}", json_res);
                        }
                    }
                }
                Err(e) => println!("{:?}", e),
            }
        }  

       

        Ok(result)
    }

    pub fn delete(&self, hash: &str) -> Result<(), Error> {
        println!("hash {}", hash);
        let mut res = self.client.get(&format!("{}/{}", api::DELETE, hash)).query(&[("format", "json")]).send()?;

        if res.status().is_success() {
            let json_res: Value = res.json()?;

            if json_res["code"] == json!("success") {
                
            } else {

            }

            println!("{}", json_res);
        }

        Ok(())
    }

    pub fn clear(&self) -> Result<(), Error> {
        let mut res = self.client.get(api::CLEAR).query(&[("format", "json")]).send()?;

        if res.status().is_success() {
            let json_res: Value = res.json()?;

            if json_res["code"] == json!("success") {
                
            } else {

            }

            println!("{}", json_res);
        }

        Ok(())
    }
}