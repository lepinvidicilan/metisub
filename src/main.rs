mod parser;
mod ui;

fn main() -> iced::Result {
    iced::run(ui::App::update, ui::App::view)
}
