#[allow(unused)]
#[allow(unused_imports)]

use std::fs;
use std::path::Path;
use std::env;
use log::Level;
use log::{info, trace, warn};

fn main() {
    println!("start application");
    for arg in env::args() {
        println!("arg is {}", arg);
    }
    console_log::init_with_level(Level::Debug);

    info!("finish application");
    trace!("ffdfdf");
    warn!("fdsfdfdf");
}
