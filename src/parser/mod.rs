use std::io::{prelude::*, BufReader};
use std::{fs::File, io};

use regex::Regex;

enum State {
    Start,
    Info,
    AegisGarbage,
    Styles,
    Events,
}

#[derive(Clone)]
pub struct AssStyle {
    label: String,
}

#[derive(Clone)]
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

#[derive(Clone, Copy)]
pub struct Time {
    h: u8,
    min: u8,
    sec: f32,
}

pub struct SubtitleFile {
    lines: Vec<Line>,
}

pub fn parse_ass(file_name: String) -> io::Result<()> {
    let f = File::open(&file_name)?;

    let mut file_lenght = 0;
    let mut event_starting = 0;

    let event_regex =
        Regex::new(r"(?<Format>[a-z A-Z]*): (?<Layer>[0-9]*?),(?<yStart>[0-9]{1}):(?<minStart>[0-9]{2}):(?<sStart>[0-9]{2}.[0-9]{2}),(?<yEnd>[0-9]{1}):(?<minEnd>[0-9]{2}):(?<sEnd>[0-9]{2}.[0-9]{2}),(?<Style>.*?),(?<Name>.*?),(?<MarginL>[0-9]*?),(?<MarginR>[0-9]*?),(?<MarginV>[0-9]*?),(?<Effect>.*?),(?<Text>.*)").unwrap();

    for m in BufReader::new(f).lines() {
        file_lenght += 1;
        if m.unwrap().as_str() == "[Events]" {
            event_starting = file_lenght + 1
        }
    }

    let f = File::open(file_name)?;

    let reader = BufReader::new(f);
    println!("{}", file_lenght - event_starting);

    let mut lines: Vec<Line> = vec![];

    //let mut lines: Vec<Line> = vec![
    //    Line {
    //        start: Time {
    //            h: 0,
    //            min: 0,
    //            sec: 0.0
    //        },
    //        end: Time {
    //            h: 0,
    //            min: 0,
    //            sec: 0.0
    //        }
    //    };
    //    file_lenght - event_starting
    //];

    let mut state = State::Start;

    for (index, line) in reader.lines().enumerate() {
        match line.as_ref().unwrap().as_str() {
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
            State::Info => {}
            State::AegisGarbage => {}
            State::Styles => {}
            State::Events => {
                let Some(args) = event_regex.captures(line.as_ref().unwrap().as_str()) else {
                    println!("No match!");
                    return Ok(());
                };

                //println!("{}", &args["Text"]);

                let text_format = args["Format"].to_string();
                let layer = args["Layer"].to_string().parse::<u8>().unwrap();
                let start = Time {
                    h: args["yStart"].to_string().parse::<u8>().unwrap(),
                    min: args["minStart"].to_string().parse::<u8>().unwrap(),
                    sec: args["sStart"].to_string().parse::<f32>().unwrap(),
                };
                let end = Time {
                    h: args["yEnd"].to_string().parse::<u8>().unwrap(),
                    min: args["minEnd"].to_string().parse::<u8>().unwrap(),
                    sec: args["sEnd"].to_string().parse::<f32>().unwrap(),
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
                //let times: Vec<Vec<&str>> = args.iter().map(|m| m.split(":").collect()).collect();
                //
                //let start = Time {
                //    h: times[0][0].to_string().parse::<u32>().unwrap(),
                //    min: times[0][1].to_string().parse::<u32>().unwrap(),
                //    sec: times[0][2].to_string().parse::<f32>().unwrap(),
                //};
                //let end = Time {
                //    h: times[1][0].to_string().parse::<u32>().unwrap(),
                //    min: times[1][1].to_string().parse::<u32>().unwrap(),
                //    sec: times[1][2].to_string().parse::<f32>().unwrap(),
                //};
                ////println!("{:?}", index - event_starting);
                ////lines[index - event_starting] = Line { start, end };
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

    //let lineaa = reader
    //    .lines()
    //    .take_while(|m| m.as_ref().unwrap().as_str() != "[Event]");
    //lineaa
    //    .into_iter()
    //    .for_each(move |m| println!("{:?}", m.as_ref()));

    let subtitle_file = SubtitleFile { lines };
    println!("{:?}", subtitle_file.lines.len());

    Ok(())
}
