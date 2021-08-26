use chrono::prelude::{DateTime, Local, Utc};
use chrono::TimeZone;
use clap::{crate_authors, crate_version, App, Arg, ArgMatches};
use std::fmt::Display;

pub trait NowGenerator {
    fn generate(&self) -> DateTime<TimeZone>;
}

struct LocalTimeGenerator;
struct UtcTimeGenerator;

impl NowGenerator for LocalTimeGenerator {
    fn generate(&self) -> DateTime<Local> {
        return Local::now();
    }
}

impl NowGenerator for UtcTimeGenerator {
    fn generate(&self) -> DateTime<Utc> {
        return Utc::now();
    }
}

pub trait Formatter<T> {
    fn convert(&self, now: DateTime<T>) -> String;
}
struct DefaultFormatter;
struct Rfc2822Formatter;

impl<T: TimeZone + Display> Formatter<T> for DefaultFormatter {
    fn convert(&self, now: DateTime<T>) -> String {
        return now.to_rfc3339();
    }
}

impl<T: TimeZone + Display> Formatter<T> for Rfc2822Formatter {
    fn convert(&self, now: DateTime<T>) -> String {
        return now.to_rfc2822();
    }
}

fn formatter<Tz: TimeZone>(matches: &ArgMatches) -> Box<dyn Formatter<Tz>> {
    match matches.value_of("format") {
        Some("rfc2822") => return Box::new(Rfc2822Formatter {}),
        _ => return Box::new(DefaultFormatter {}),
    }
}

fn generator(matches: &ArgMatches) -> Box<dyn NowGenerator> {
    if matches.is_present("utc") {
        return Box::new(UtcTimeGenerator {});
    }
    return Box::new(LocalTimeGenerator {});
}

fn perform(matches: ArgMatches) {
    let now = Local::now();
    let formatter = formatter(&matches);
    println!("{}", formatter.convert(now));
}

fn main() {
    let matches =  App::new("now")
        .version(crate_version!())
        .author(crate_authors!())
        .about("display the date and time.")
        .arg(
            Arg::with_name("utc")
                .short("u")
                .long("utc")
                .help("Display the date and time in UTC (coordinate universal) time.")
        )
        .arg(
            Arg::with_name("relative")
                .short("R")
                .long("relative")
                .takes_value(true)
                .value_name("TIME")
                .help("Display the date and time in the relative time of the local time, such as -9:00 (shows UTC if the local time is in Asia/Tokyo)")
        )
        .arg(
            Arg::with_name("relative-utc")
                .short("r")
                .long("relative-utc")
                .takes_value(true)
                .value_name("TIME")
                .help("Display the date and time in the relative time of UTC, such as +9:00 (means Asia/Tokyo)")
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .takes_value(true)
                .value_name("FORMAT")
                .help("Display the date and time in specified format. Available values are: ISO8601, rfc3339, rfc2822")
        )
        .arg(
            Arg::with_name("timezone")
                .short("t")
                .long("timezone")
                .takes_value(true)
                .value_name("TZ")
                .help("Display the date and time in the specified timezone, such as Asia/Tokyo.")
        )
        .get_matches();

    perform(matches)
}
