mod parser;

fn main() {
    let _ = match parser::parse_ass(String::from(
        "[Team Arcedo] BanG Dream! Ave Mujica - 11 VOSTFR.ass",
    )) {
        Ok(T) => T,
        Err(e) => panic!("{}", e.get_reason()),
    };
}
