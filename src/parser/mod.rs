use std::collections::{self, HashMap};
use std::io::{prelude::*, BufReader};
use std::{fs::File, io};

use regex::Regex;

#[derive(Debug)]
enum State {
    Start,
    Info,
    AegisGarbage,
    Styles,
    Events,
}

pub struct AssStyle {
    label: String,
}

pub struct Line {
    format: String,
    layer: u8,
    start: Time,
    end: Time,
    style: AssStyle,
    name: String,
    margin: Vec<(f32, f32, f32)>,
    effect: String,
    text: String,
}

pub struct Time {
    hours: u8,
    minutes: u8,
    seconds: f32,
}

pub struct SubtitleFile {
    script_info: HashMap<String, String>,
    lines: Vec<Line>,
}

pub fn parse_ass(file_name: String) -> io::Result<()> {
    let info_regex = Regex::new(r"(?<field>.*?): (?<content>.*)").unwrap();
    let event_regex =
        Regex::new(r"(?<Format>[a-z A-Z]*): (?<Layer>[0-9]*?),(?<yStart>[0-9]{1}):(?<minStart>[0-9]{2}):(?<sStart>[0-9]{2}.[0-9]{2}),(?<yEnd>[0-9]{1}):(?<minEnd>[0-9]{2}):(?<sEnd>[0-9]{2}.[0-9]{2}),(?<Style>.*?),(?<Name>.*?),(?<MarginL>[0-9]*?),(?<MarginR>[0-9]*?),(?<MarginV>[0-9]*?),(?<Effect>.*?),(?<Text>.*)").unwrap();

    let f = File::open(file_name)?;

    let reader = BufReader::new(f);

    let mut script_info: HashMap<String, String> = collections::HashMap::new();
    let mut lines: Vec<Line> = vec![];

    let mut state = State::Start;

    for (index, line) in reader.lines().enumerate() {
        if line.as_ref().unwrap().is_empty() {
            continue;
        }
        match line.as_ref().unwrap().as_str() {
            "﻿[Script Info]" => {
                state = State::Info;
                continue;
            }
            "[Script Info]" => {
                state = State::Info;
                continue;
            }
            "[Aegisub Project Garbage]" => {
                state = State::AegisGarbage;
                continue;
            }
            "[V4+ Styles]" => {
                state = State::Styles;
                continue;
            }
            "[Events]" => {
                state = State::Events;
                continue;
            }
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text" => {
                continue;
            }
            &_ => {}
        }

        match state {
            State::Start => {}
            State::Info => {
                if line.as_ref().unwrap().as_bytes()[0] != ";".as_bytes()[0] {
                    let Some(caps) = info_regex.captures(line.as_ref().unwrap().as_str()) else {
                        println!("Can't bro");
                        return Ok(());
                    };

                    //println!("{}: {}", &caps["field"], &caps["content"]);
                    script_info.insert(caps["field"].to_string(), caps["content"].to_string());
                }
            }
            State::AegisGarbage => {}
            State::Styles => {}
            State::Events => {
                let Some(args) = event_regex.captures(line.as_ref().unwrap().as_str()) else {
                    println!("No match!");
                    return Ok(());
                };

                let text_format = args["Format"].to_string();
                let layer = args["Layer"].to_string().parse::<u8>().unwrap();
                let start = Time {
                    hours: args["yStart"].to_string().parse::<u8>().unwrap(),
                    minutes: args["minStart"].to_string().parse::<u8>().unwrap(),
                    seconds: args["sStart"].to_string().parse::<f32>().unwrap(),
                };
                let end = Time {
                    hours: args["yEnd"].to_string().parse::<u8>().unwrap(),
                    minutes: args["minEnd"].to_string().parse::<u8>().unwrap(),
                    seconds: args["sEnd"].to_string().parse::<f32>().unwrap(),
                };
                let text_style = AssStyle {
                    label: args["Style"].to_string(),
                };
                let name = args["Name"].to_string();
                let margin = vec![(
                    args["MarginL"].to_string().parse::<f32>().unwrap(),
                    args["MarginR"].to_string().parse::<f32>().unwrap(),
                    args["MarginV"].to_string().parse::<f32>().unwrap(),
                )];
                let effect = args["Effect"].to_string();
                let ass_text = args["Text"].to_string();
                lines.push(Line {
                    format: text_format,
                    layer,
                    start,
                    end,
                    style: text_style,
                    name,
                    margin,
                    effect,
                    text: ass_text,
                });
            }
        }
    }

    let subtitle_file = SubtitleFile { script_info, lines };
    println!("{:?}", &subtitle_file.lines.len());
    for (field, content) in subtitle_file.script_info {
        println!("{}, {}", field, content);
    }

    Ok(())
}
