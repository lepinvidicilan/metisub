use std::{future::Future, io, iter, path::PathBuf, process::id, sync::Arc};

use iced::{
    widget::{button, column, container, row, text, text_editor, tooltip, Column, Row},
    window, Element,
    Length::Fill,
    Renderer, Task, Theme, Window,
};

use crate::parser;

const EDITOR: &str = "editor";

#[derive(Default)]
pub struct App {
    subtitle_file: parser::SubtitleFile,
    subtitle_file_path: Option<PathBuf>,
    selected_line: text_editor::Content,
    is_loading: bool,
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

#[derive(Clone)]
pub enum Message {
    OpenFile,
    FileOpened(Result<PathBuf, Error>),
}

fn open_file(window: &dyn Window) -> impl Future<Output = Result<PathBuf, Error>> + use<> {
    let dialog = rfd::AsyncFileDialog::new()
        .set_title("Open a text file...")
        .set_parent(&window);

    async move {
        let picked_file = dialog.pick_file().await.ok_or(Error::DialogClosed)?;

        load_file(picked_file).await
    }
}

async fn load_file(path: impl Into<PathBuf>) -> Result<PathBuf, Error> {
    let path = path.into();

    Ok(path)
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenFile => {
                if self.is_loading {
                    Task::none()
                } else {
                    self.is_loading = true;
                    window::oldest()
                        .and_then(|id| window::run(id, open_file))
                        .then(Task::future)
                        .map(Message::FileOpened)
                }
            }

            Message::FileOpened(result) => {
                self.is_loading = false;

                if let Ok(path) = result {
                    self.subtitle_file_path = Some(path.to_owned());
                    self.subtitle_file = parser::parse_ass(path).unwrap()
                }

                self.selected_line =
                    text_editor::Content::with_text(&self.subtitle_file.get_line(0).text);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Column<'_, Message> {
        let action: iced::widget::Button<'_, _, Theme, Renderer> = button(text('\u{0f115}'));
        println!("{}", self.subtitle_file.get_number_of_line());
        println!("{}", self.subtitle_file.get_number_of_line() > 0);

        let mut ass_line: Column<'_, Message> = if self.subtitle_file.get_number_of_line() > 0 {
            column![self.subtitle_file.get_line(0).view()]
        } else {
            column![text("Load File")]
        };

        if self.subtitle_file.get_number_of_line() > 0 {
            if self.subtitle_file.get_number_of_line() < 20 {
                for i in 0..self.subtitle_file.get_number_of_line() {
                    ass_line = ass_line.push(self.subtitle_file.get_line(i).view());
                }
            } else {
                for i in 0..20 {
                    ass_line = ass_line.push(self.subtitle_file.get_line(i).view());
                }
            }
        }

        column![
            row![
                text(self.subtitle_file.get_name()),
                if let Some(on_press) = Some(Message::OpenFile) {
                    tooltip(
                        action.on_press(on_press),
                        "Open File",
                        tooltip::Position::FollowCursor,
                    )
                    .style(container::rounded_box)
                } else {
                    panic!("a");
                },
            ],
            text_editor(&self.selected_line).id(EDITOR),
            ass_line
        ]
    }
}
