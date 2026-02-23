mod parser;

fn main() {
    let _ = match parser::parse_ass(String::from("Ave_mujica_vostfr_ep02_QCHECKDONE.ass")) {
        Ok(t) => t,
        Err(e) => panic!("{}", e.get_reason()),
    };
    let _ = match parser::parse_ass(String::from(
        "[Team Arcedo] BanG Dream! Ave Mujica - 10 VOSTFR.ass",
    )) {
        Ok(t) => t,
        Err(e) => panic!("{}", e.get_reason()),
    };
    let _ = match parser::parse_ass(String::from(
        "[Team Arcedo] BanG Dream! Ave Mujica - 11 VOSTFR.ass",
    )) {
        Ok(t) => t,
        Err(e) => panic!("{}", e.get_reason()),
    };
}
