#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};
use std::process::exit;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").help("Key").required(true))
                .arg(Arg::with_name("VALUE").help("value").required(true)),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").help("key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").help("key").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(args)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("get", Some(args)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(args)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => {
            eprintln!("Unknown command");
            exit(1);
        }
    }
}
