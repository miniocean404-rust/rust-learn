#[derive(Debug)]
enum Mode2 {
    V1,
}

enum Mode {
    V1,
    V2,
    Match(Mode2),
}

fn show_match(mode: Mode) -> u8 {
    match mode {
        Mode::V1 => 1,
        Mode::V2 => 2,
        Mode::Match(state) => {
            println!("{:#?}", state);
            25
        }
    }
}

fn use_show_match() {
    let enum_value = Mode::Match(Mode2::V1);
    let value: u8 = show_match(enum_value);
}
