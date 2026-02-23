use core::fmt;
use std::collections::{self, HashMap};
use std::error::Error;
use std::io::{prelude::*, BufReader, ErrorKind};
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
    file_name: String,
    script_info: HashMap<String, String>,
    video: String,
    audio: String,
    lines: Vec<Line>,
}

pub struct ParseError {
    reason: String,
}

impl ParseError {
    pub fn get_reason(self) -> String {
        self.reason
    }
}

pub fn parse_ass(file_name: String) -> std::result::Result<SubtitleFile, ParseError> {
    let info_regex = Regex::new(r"(?<field>.*?): (?<content>.*)").unwrap();
    let style_regex = Regex::new(
        r"(?<Format>[a-z A-Z]*): (?<Name>.*?),(?<Fontname>.*?),(?<Fontsize>[0-9]*),(?<PrimaryColour>&[A-Z 0-9]{9}),(?<SecondaryColour>&[A-Z 0-9]{9}),(?<OutlineColour>&[A-Z 0-9]{9}),(?<BackColour>&[A-Z 0-9]{9}),(?<Bold>\-?[0-9]),(?<Italic>\-?[0-9]),(?<Underline>\-?[0-9]),(?<StrikeOut>\-?[0-9]),(?<ScaleX>[0-9]{1,3}),(?<ScaleY>[0-9]{1,3}),(?<Spacing>.*?),(?<Angle>.*?),(?<BorderStyle>.*?),(?<Outline>.*?),(?<Shadow>.*?),(?<Alignment>[0-9]),(?<MarginL>.*?),(?<MarginR>.*?),(?<MarginV>.*?),(?<Encoding>[0-9])",
    )
    .unwrap();
    let event_regex =
        Regex::new(r"(?<Format>[a-z A-Z]*): (?<Layer>[0-9]*?),(?<yStart>[0-9]{1}):(?<minStart>[0-9]{2}):(?<sStart>[0-9]{2}.[0-9]{2}),(?<yEnd>[0-9]{1}):(?<minEnd>[0-9]{2}):(?<sEnd>[0-9]{2}.[0-9]{2}),(?<Style>.*?),(?<Name>.*?),(?<MarginL>[0-9]*?),(?<MarginR>[0-9]*?),(?<MarginV>[0-9]*?),(?<Effect>.*?),(?<Text>.*)").unwrap();

    let f = match File::open(&file_name) {
        Ok(T) => T,
        Err(e) => {
            return Err(ParseError {
                reason: String::from(format!("{e:?}")),
            })
        }
    };

    let reader = BufReader::new(f);

    let mut script_info: HashMap<String, String> = collections::HashMap::new();
    let mut aegis_garbage: HashMap<String, String> = collections::HashMap::new();
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
            "Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding" => {
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
                        return Err(ParseError {
                            reason: String::from("Not an Ass File"),
                        });
                    };

                    //println!("{}: {}", &caps["field"], &caps["content"]);
                    script_info.insert(caps["field"].to_string(), caps["content"].to_string());
                }
            }
            State::AegisGarbage => {
                if line.as_ref().unwrap().as_bytes()[0] != ";".as_bytes()[0] {
                    let Some(caps) = info_regex.captures(line.as_ref().unwrap().as_str()) else {
                        println!("Can't bro");
                        return Err(ParseError {
                            reason: String::from("Not an Ass File"),
                        });
                    };

                    //println!("{}: {}", &caps["field"], &caps["content"]);
                    aegis_garbage.insert(caps["field"].to_string(), caps["content"].to_string());
                }
            }
            State::Styles => {
                let Some(caps) = style_regex.captures(line.as_ref().unwrap().as_str()) else {
                    println!("Can't bro");
                    return Err(ParseError {
                        reason: String::from("Not an Ass File"),
                    });
                };
                println!("{:?}", &caps["Encoding"]);
            }
            State::Events => {
                let Some(args) = event_regex.captures(line.as_ref().unwrap().as_str()) else {
                    println!("No match!");
                    return Err(ParseError {
                        reason: String::from("Not an Ass File"),
                    });
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
    let video = if aegis_garbage.contains_key("Video File") {
        aegis_garbage["Video File"].clone()
    } else {
        println!("can't find video");
        "".to_string()
    };

    let audio = if aegis_garbage.contains_key("Audio File") {
        aegis_garbage["Audio File"].clone()
    } else {
        println!("Can't fine audio");
        "".to_string()
    };

    let subtitle_file = SubtitleFile {
        file_name,
        script_info,
        lines,
        video,
        audio,
    };
    println!("{}\n{}", &subtitle_file.video, &subtitle_file.audio);
    //for (field, content) in subtitle_file.script_info {
    //    println!("{}, {}", field, content);
    //}

    Ok(subtitle_file)
}
