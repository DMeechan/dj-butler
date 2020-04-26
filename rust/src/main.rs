extern crate reqwest;
extern crate chrono;
// extern crate clap;

use chrono::{Datelike, Timelike, Utc};
use std::env;
use std::io::Read;
// use clap::{Arg, App, SubCommand};

/*
    Accepts commands:
        - time
        - ip
        - weather (location & weather APIs)
        - stocks
        - set (name)
        -
*/
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 2 {
        // throw error
    }

    let query = &args[1];

    let param: Option<String> = if args.len() > 2 {
        Some(args[2].to_string())
    } else {
        None
    };
    println!("param: {:?}", param);

    // let response: String = match query.as_str() {
    //     "time" => fetch_time(),
    //     "ip" => fetch_ip_address(),
    //     "weather" => fetch_weather(),
    //     "stocks" => fetch_stocks(),
    //     _ => fetch_unknown(),
    // };
    let response = reqwest::get("https://api.ipify.org")
        .await?
        .text()
        .await?;

    println!("=> {}", response);
    Ok(())

    // let helloCommand = SubCommand::with_name("time")
    //     .about("tells you the time");

    // let matches = App::new("dj")
    //     .version("0.1.0")
    //     .author("Daniel Meechan")
    //     .about("Butler CLI written in Rust 🦀")
    //     .subcommand(helloCommand)
    //     .get_matches();
}

fn fetch_time() -> String {
    let time = Utc::now().to_rfc2822();
    format!("The time is {}", time)
}

// Using https://www.ipify.org/
// async fn fetch_ip_address() -> Result<String, Err> {
//     // just returning "blah" is actually a:
//     // &'static str
//     // see moar: https://stackoverflow.com/a/43080280/4752388
//     // // let FETCH_IP_SERVICE_URL = "https://www.ipify.org/";
//     let body = reqwest::get("https://www.ipify.org/")
//     .await?.text().await?;
//     println!("{}", body)
//     String::from("you want your IP")
// }

fn fetch_weather() -> String {
    "you want the weather!".to_string()
}

fn fetch_stocks() -> String {
    "you want the stocks!".to_string()
}

fn fetch_unknown() -> String {
    "I dont even know what u want :(".to_string()
}
