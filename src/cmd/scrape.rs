extern crate reqwest;
extern crate colored;

use colored::*;

use std::{
    fmt,
    error::Error,
    io::{self, BufRead}
};

use serde_json::Value;
use serde::Deserialize;
use reqwest::{Client, header};

use crate::configs::{
    env::Env,
    global::Global
};

use crate::utils::misc::Misc;
use crate::cmd::download::Download;

pub struct Scrape;

impl Scrape {

    // Coming soon...

}
