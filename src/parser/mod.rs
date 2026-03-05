use std::collections::{self, HashMap};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

use iced::widget::{row, table, text, Row, Text};
use iced::Element;
use regex::Regex;

use crate::ui::Message;

#[derive(Debug)]
enum State {
    Start,
    Info,
    AegisGarbage,
    Styles,
    Events,
}

#[derive(Clone)]
pub struct Line {
    pub format: String,
    pub layer: u8,
    pub start: Time,
    pub end: Time,
    pub style: String,
    pub name: String,
    pub margin: Vec<(f32, f32, f32)>,
    pub effect: String,
    pub text: String,
}

impl Line {
    pub fn view<'a>(&self) -> Row<'a, Message> {
        row![text(self.format.clone()), text(self.text.clone())]
    }
}

#[derive(Clone)]
pub struct Time {
    hours: u8,
    minutes: u8,
    seconds: f32,
}

#[derive(Default)]
pub struct SubtitleFile {
    file_name: String,
    script_info: HashMap<String, String>,
    styles: HashMap<String, AssStyle>,
    video: String,
    audio: String,
    lines: Vec<Line>,
}

impl SubtitleFile {
    pub fn get_name(&self) -> String {
        self.file_name.clone()
    }
    pub fn get_line(&self, l: usize) -> Line {
        let ass_line = self.lines[l].clone();
        Line {
            format: ass_line.format,
            layer: ass_line.layer,
            start: ass_line.start,
            end: ass_line.end,
            style: ass_line.style,
            name: ass_line.name,
            margin: ass_line.margin,
            effect: ass_line.effect,
            text: ass_line.text,
        }
    }
    pub fn get_lines(&self) -> Vec<Line> {
        self.lines.clone()
    }
    pub fn get_number_of_line(&self) -> usize {
        self.lines.len()
    }
}

#[derive(Debug)]
pub struct ParseError {
    reason: String,
}

impl ParseError {
    pub fn get_reason(self) -> String {
        self.reason
    }
}

pub struct AssColour {
    colour: (u8, u8, u8),
    alpha: u8,
}

impl AssColour {
    pub fn from_ass_colour(colour: String) -> AssColour {
        let blue = u8::from_str_radix(&colour[4..6], 16).unwrap();
        let green = u8::from_str_radix(&colour[6..8], 16).unwrap();
        let red = u8::from_str_radix(&colour[8..10], 16).unwrap();

        let alpha = u8::from_str_radix(&colour[2..4], 16).unwrap();

        AssColour {
            colour: (red, green, blue),
            alpha,
        }
    }
}

pub struct AssStyle {
    name: String,
    font_name: String,
    font_size: f32,
    primary_colour: AssColour,
    secondary_colour: AssColour,
    outline_colour: AssColour,
    back_colour: AssColour,
    bold: bool,
    italic: bool,
    underline: bool,
    strike_out: bool,
    scale: (f32, f32),
    spacing: f32,
    angle: f32,
    border_style: u8,
    outline: f32,
    shadow: f32,
    alignment: u8,
    margin: (f32, f32, f32),
    encoding: u8,
}

pub fn parse_ass(path: PathBuf) -> std::result::Result<SubtitleFile, ParseError> {
    let info_regex = Regex::new(r"(?<field>.*?): (?<content>.*)").unwrap();
    let style_regex = Regex::new(
        r"(?<Format>[a-z A-Z]*): (?<Name>.*?),(?<Fontname>.*?),(?<Fontsize>[0-9]*),(?<PrimaryColour>&[A-Z 0-9]{9}),(?<SecondaryColour>&[A-Z 0-9]{9}),(?<OutlineColour>&[A-Z 0-9]{9}),(?<BackColour>&[A-Z 0-9]{9}),(?<Bold>\-?[0-9]),(?<Italic>\-?[0-9]),(?<Underline>\-?[0-9]),(?<StrikeOut>\-?[0-9]),(?<ScaleX>[0-9]{1,3}),(?<ScaleY>[0-9]{1,3}),(?<Spacing>.*?),(?<Angle>.*?),(?<BorderStyle>.*?),(?<Outline>.*?),(?<Shadow>.*?),(?<Alignment>[0-9]),(?<MarginL>.*?),(?<MarginR>.*?),(?<MarginV>.*?),(?<Encoding>[0-9])",
    )
    .unwrap();
    let event_regex =
        Regex::new(r"(?<Format>[a-z A-Z]*): (?<Layer>[0-9]*?),(?<yStart>[0-9]{1}):(?<minStart>[0-9]{2}):(?<sStart>[0-9]{2}.[0-9]{2}),(?<yEnd>[0-9]{1}):(?<minEnd>[0-9]{2}):(?<sEnd>[0-9]{2}.[0-9]{2}),(?<Style>.*?),(?<Name>.*?),(?<MarginL>[0-9]*?),(?<MarginR>[0-9]*?),(?<MarginV>[0-9]*?),(?<Effect>.*?),(?<Text>.*)").unwrap();

    let f = match File::open(&path) {
        Ok(t) => t,
        Err(e) => {
            return Err(ParseError {
                reason: format!("{e:?}"),
            })
        }
    };

    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

    let reader = BufReader::new(f);

    let mut script_info: HashMap<String, String> = collections::HashMap::new();
    let mut aegis_garbage: HashMap<String, String> = collections::HashMap::new();
    let mut styles: HashMap<String, AssStyle> = collections::HashMap::new();

    let mut lines: Vec<Line> = vec![];

    let mut state = State::Start;

    for line in reader.lines() {
        if line.as_ref().unwrap().is_empty() {
            continue;
        }
        match line.as_ref().unwrap().as_str() {
            "﻿[Script Info]" | "[Script Info]" => {
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
            "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text" | "Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding" => {
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

                let name = caps["Name"].to_string();
                let font_name = caps["Fontname"].to_string();
                let font_size = match caps["Fontsize"].to_string().parse::<f32>() {
                    Ok(t) => t,
                    Err(e) => {
                        return Err(ParseError {
                            reason: format!("{e:?}"),
                        })
                    }
                };

                let primary_colour = AssColour::from_ass_colour(caps["PrimaryColour"].to_string());
                let secondary_colour =
                    AssColour::from_ass_colour(caps["SecondaryColour"].to_string());
                let outline_colour = AssColour::from_ass_colour(caps["OutlineColour"].to_string());
                let back_colour = AssColour::from_ass_colour(caps["BackColour"].to_string());

                let bold = &caps["Bold"] == "-1";
                let italic = &caps["Italic"] == "-1";
                let underline = &caps["Underline"] == "-1";
                let strike_out = &caps["StrikeOut"] == "-1";

                let scale = (
                    caps["ScaleX"].to_string().parse::<f32>().unwrap(),
                    caps["ScaleY"].to_string().parse::<f32>().unwrap(),
                );
                let spacing = caps["Spacing"].to_string().parse::<f32>().unwrap();
                let angle = caps["Angle"].to_string().parse::<f32>().unwrap();

                let border_style = caps["BorderStyle"].to_string().parse::<u8>().unwrap();
                let outline = caps["Outline"].to_string().parse::<f32>().unwrap();
                let shadow = caps["Shadow"].to_string().parse::<f32>().unwrap();
                let alignment = caps["Alignment"].to_string().parse::<u8>().unwrap();
                let margin = (
                    caps["MarginL"].to_string().parse::<f32>().unwrap(),
                    caps["MarginR"].to_string().parse::<f32>().unwrap(),
                    caps["MarginV"].to_string().parse::<f32>().unwrap(),
                );
                let encoding = if &caps["Encoding"] == "-1" {
                    println!("Warning: Auto-detect base direction not implemented yet");
                    1
                } else {
                    caps["Encoding"].to_string().parse::<u8>().unwrap()
                };
                styles.insert(
                    String::from(&name),
                    AssStyle {
                        name,
                        font_name,
                        font_size,
                        primary_colour,
                        secondary_colour,
                        outline_colour,
                        back_colour,
                        bold,
                        italic,
                        underline,
                        strike_out,
                        scale,
                        spacing,
                        angle,
                        border_style,
                        outline,
                        shadow,
                        alignment,
                        margin,
                        encoding,
                    },
                );
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
                let text_style = args["Style"].to_string();
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
        String::new()
    };

    let audio = if aegis_garbage.contains_key("Audio File") {
        aegis_garbage["Audio File"].clone()
    } else {
        println!("Can't fine audio");
        String::new()
    };

    let subtitle_file = SubtitleFile {
        file_name,
        script_info,
        styles,
        video,
        audio,
        lines,
    };
    println!("{}\n{}", &subtitle_file.video, &subtitle_file.audio);

    Ok(subtitle_file)
}
