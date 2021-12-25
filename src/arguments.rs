extern crate clap;
use crate::types::Arguments;
use crate::Mode;
use clap::{Arg, App, SubCommand, ArgMatches};

pub fn get_arguments(matches: ArgMatches) -> Arguments {
    let mut arguments = Arguments::default();

    if matches.is_present("list") {
        arguments.mode = Mode::ShowLast;
    } else if matches.is_present("search") {
        arguments.mode = Mode::Search;
    } else {
        if let Some(t) = matches.value_of("track") {
            arguments.track = Some(t.to_owned());
        }
        if let Some(m) = matches.value_of("message") {
            arguments.message = Some(m.to_owned());
        }
        if let Some(t) = matches.value_of("time") {
            arguments.time = Some(t.to_owned());
        }
        if arguments.time != None || arguments.track != None || arguments.message != None {
            arguments.mode = Mode::Add;
        }
    }
    arguments
}

pub fn get_clap_app() -> App<'static, 'static> {
    App::new("rtrack")
        .version("0.1")
        .author("Fabian D. <kontakt@fabiandarga.de>")
        .about("For easy time tracking")
        .arg(Arg::with_name("time")
            .help("Sets the time")
            .index(1))
        .arg(Arg::with_name("track")
            .short("t")
            .long("track")
            .takes_value(true)
            .help("Set the track name"))
        .arg(Arg::with_name("message")
            .short("m")
            .long("message")
            .takes_value(true)
            .help("Add message to an entry"))
        .subcommand(SubCommand::with_name("search")
            .about("Search for time entries")
            )
        .subcommand(SubCommand::with_name("list")
            .about("List of last time entries")
            )
        .subcommand(SubCommand::with_name("test")
            .about("controls testing features")
            .version("1.3")
            .author("Someone E. <someone_else@other.com>")
            .arg(Arg::with_name("debug")
                .short("d")
                .help("print debug information verbosely")))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_tracking_arguments() {
        let app_conf = get_clap_app();
        let matches = app_conf.get_matches_from(
            vec!["rtrack", "--track", "test", "--message", "message", "2h30m"]);
        let args = get_arguments(matches);
        assert_eq!(args.track, Some("test".to_owned()));
        assert_eq!(args.message, Some("message".to_owned()));
        assert_eq!(args.time, Some("2h30m".to_owned()));
    }
    #[test]
    fn get_no_arguments() {
        let app_conf = get_clap_app();
        let matches = app_conf.get_matches_from(
            vec!["rtrack"]);
        let args = get_arguments(matches);
        assert_eq!(args.track, None);
        assert_eq!(args.message, None);
        assert_eq!(args.time, None);
    }
}