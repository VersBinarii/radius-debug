#[macro_use]
extern crate lazy_static;

use clap::{App, Arg};
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

lazy_static! {
    static ref AUTH_RE: Regex =
        Regex::new(r"^\(([0-9]*)\).*Access-Request.*").unwrap();
    static ref ACC_RE: Regex =
        Regex::new(r"^\(([0-9]*)\).*Accounting-Request.*").unwrap();
    static ref PACKET_NUM: Regex = Regex::new(r"^\(([0-9]*)\).*").unwrap();
}

#[derive(Copy, Clone, Debug)]
enum RadiusPacket {
    Authentication,
    Accounting,
}

fn main() {
    let mut packet_debug: HashMap<u32, Vec<String>> = HashMap::new();
    let mut packet_of_interest = Vec::new();
    let mut packet_with_user_pattern = Vec::new();
    let mut current_packet_number = 0;

    let matches = App::new("FreeRADIUS-debug-helper")
        .version("0.1.0")
        .author("Kris <versbinarii@gmail.com>")
        .about("Filters out FreeRADIUS debug")
        .arg(
            Arg::with_name("packet type")
                .short("t")
                .long("type")
                .takes_value(true)
                .required(true)
                .possible_values(&["auth", "acct"])
                .help("Filter packet types."),
        )
        .arg(
            Arg::with_name("pattern")
                .short("p")
                .long("pattern")
                .takes_value(true)
                .required(false)
                .help("Search for pattern in the packets"),
        )
        .get_matches();

    let packet_type = match matches.value_of("packet type").unwrap() {
        "auth" => RadiusPacket::Authentication,
        "acct" => RadiusPacket::Accounting,
        _ => panic!("Unknown packet type"),
    };

    let user_pattern = matches.value_of("pattern");

    for line in io::stdin().lock().lines().map(|l| l.unwrap()) {
        if let Some(packet_num) = find_packet_number(&line) {
            // Capture all the packets and keep them grouped by packet number
            current_packet_number = packet_num;
        }
        if line.starts_with(&format!("({})", current_packet_number)) {
            packet_debug
                .entry(current_packet_number)
                .and_modify(|v| v.push(line.clone()))
                .or_insert_with(|| {
                    let mut v = Vec::new();
                    v.push(line.clone());
                    v
                });
        }

        if let Some(packet_num) = find_packet_type(&line, packet_type) {
            // Find the actual packet number that matched requirement
            packet_of_interest.push(packet_num);
        }

        if let Some(pattern) = user_pattern {
            if let Some(packet_num) = find_packet_with_pattern(&line, &pattern)
            {
                // Find the actual packet number that matched requirement
                packet_with_user_pattern.push(packet_num);
            }
        }

        let mut ordered: Vec<_> = packet_debug.drain().collect();
        // Having the packets shown in order is nice
        ordered.sort_by_key(|&(k, _)| k);

        let _: Vec<Result<(), _>> = ordered
            .iter()
            .filter(|(k, _)| packet_of_interest.iter().any(|e| e == k))
            .filter(|(k, _)| {
                if packet_with_user_pattern.len() > 0 {
                    packet_with_user_pattern.iter().any(|e| e == k)
                } else {
                    true
                }
            })
            .map(|(_, v)| {
                v.into_iter()
                    .map(|s| writeln!(io::stdout(), "{}", s))
                    .collect()
            })
            .collect();

        if packet_of_interest.len() > 200 {
            packet_of_interest.clear();
        }
    }
}

fn get_first_group(cap: Captures) -> Option<String> {
    cap.get(1).map(|packet_num| packet_num.as_str().to_owned())
}

fn find_packet_type(
    input_line: &str,
    packet_type: RadiusPacket,
) -> Option<u32> {
    let capture = match packet_type {
        RadiusPacket::Accounting => ACC_RE.captures(&input_line),
        RadiusPacket::Authentication => AUTH_RE.captures(&input_line),
    };

    let packet_number = capture.and_then(get_first_group)?;

    Some(packet_number.parse().unwrap())
}

fn find_packet_with_pattern(input_line: &str, pattern: &str) -> Option<u32> {
    if input_line.contains(pattern) {
        //it contains user patter to capture the packet number
        find_packet_number(input_line)
    } else {
        None
    }
}

fn find_packet_number(input_line: &str) -> Option<u32> {
    let packet_number: String =
        PACKET_NUM.captures(&input_line).and_then(get_first_group)?;

    Some(packet_number.parse().unwrap())
}
