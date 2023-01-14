/*
   Appellation: interface <cli>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use clap::{arg, command, value_parser, ArgAction, ArgMatches, Command};
use std::path::PathBuf;


pub fn base(sc: Command) -> ArgMatches {
    command!()
        .subcommand(sc)
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf))
            .default_value("/config/Conduit.toml"),
        )
        .arg(
            arg!(debug: -d --debug)
                .action(ArgAction::SetTrue)
                .help("Optionally startup the debugger"),
        )
        .arg(
            arg!(release: -r --release)
                .action(ArgAction::SetTrue)
                .help("Optionally startup application in release mode"),
        )
        .arg(
            arg!(up: -u --up)
                .action(ArgAction::SetTrue)
                .help("Signals for a system to turn on"),
        )
        .arg(arg!(verbose: -v --verbose).action(ArgAction::SetTrue))
        .propagate_version(true)
        .subcommand_required(false)
        .arg_required_else_help(true)
        .get_matches()
}

