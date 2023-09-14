extern crate clap;
use crate::types::Arguments;
use crate::types::Mode;
use clap::{Arg, App, SubCommand, ArgMatches};

pub fn get_arguments(matches: ArgMatches) -> Arguments {
    let mut arguments = Arguments::default();

    if matches.is_present("list") {
        arguments.mode = Mode::ShowLast;
    } else if matches.is_present("search") {
        arguments.mode = Mode::Search;
    } else if matches.is_present("add") {
        arguments.mode = Mode::Add;
        let matches  = matches.subcommand_matches("add").unwrap();
        if let Some(t) = matches.value_of("time") {
            arguments.time = Some(t.to_owned());
        }
        if let Some(t) = matches.value_of("track") {
            arguments.track = Some(t.to_owned());
        }
    
        if let Some(m) = matches.value_of("message") {
            arguments.message = Some(m.to_owned());
        }
    } else if matches.is_present("stop") { 
        arguments.mode = Mode::Stop;
        let matches = matches.subcommand_matches("stop").unwrap();
        if let Some(i) = matches.value_of("index") {
            arguments.index = Some(i.parse::<usize>().unwrap());
        };
    } else {
        if let Some(t) = matches.value_of("track") {
            arguments.track = Some(t.to_owned());
        }
    
        if let Some(m) = matches.value_of("message") {
            arguments.message = Some(m.to_owned());
        }

        if matches.is_present("display") {
            arguments.display = true;
        }

        if arguments.track != None || arguments.message != None {
            arguments.mode = Mode::Track;
        }
    }

    arguments
}

pub fn get_clap_app() -> App<'static, 'static> {
    App::new("rtrack")
        .version("0.1")
        .author("Fabian D. <kontakt@fabiandarga.de>")
        .about("For easy time tracking")
        .arg(Arg::with_name("track")
            .short("t")
            .long("track")
            .takes_value(true)
            .help("Set the track name")
        )
        .arg(Arg::with_name("message")
            .short("m")
            .long("message")
            .takes_value(true)
            .help("Add message to an entry")
        )
        .arg(Arg::with_name("display")
            .short("d")
            .long("display")
            .help("Display the running timers")
        )
        .subcommand(SubCommand::with_name("stop")
            .about("Stop a running timer")
            .arg(Arg::with_name("index")
                .help("The index to be deleted")
                .takes_value(true)
                .short("i")
                .long("index")
            )
        )
        .subcommand(SubCommand::with_name("add")
            .about("Add time entry")
            .arg(Arg::with_name("time")
                .help("Sets the time")
                .index(1)
            )
            .arg(Arg::with_name("track")
                .short("t")
                .long("track")
                .takes_value(true)
                .help("Set the track name")
            )
            .arg(Arg::with_name("message")
                .short("m")
                .long("message")
                .takes_value(true)
                .help("Add message to an entry")
            )
        )
        .subcommand(SubCommand::with_name("search")
            .about("Search for time entries")
        )
        .subcommand(SubCommand::with_name("list")
            .about("List of last time entries")
        )
        .subcommand(SubCommand::with_name("test")
            .about("Controls testing features")
            .version("1.3")
            .author("Someone E. <someone_else@other.com>")
            .arg(Arg::with_name("debug")
                .short("d")
                .help("print debug information verbosely")
            )
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_arguments_for_add_enty() {
        let app_conf = get_clap_app();
        let matches = app_conf.get_matches_from(
            vec!["rtrack", "add", "--track", "test", "--message", "message", "2h30m"]);
        let args = get_arguments(matches);
        assert_eq!(args.mode, Mode::Add);
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
        assert_eq!(args.mode, Mode::None);
        assert_eq!(args.track, None);
        assert_eq!(args.message, None);
        assert_eq!(args.time, None);
    }
    #[test]
    pub fn get_arguments_for_tracking() {
        let app_conf = get_clap_app();
        let matches = app_conf.get_matches_from(vec!["rtrack", "-t", "trackName", "-m", "message"]);
        let args = get_arguments(matches);
        assert_eq!(args.mode, Mode::Track);
        assert_eq!(args.track, Some("trackName".to_owned()));
        assert_eq!(args.message, Some("message".to_owned()));
        assert_eq!(args.time, None);
    }
}
