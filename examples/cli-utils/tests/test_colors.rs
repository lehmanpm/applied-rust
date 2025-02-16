use cli_utils::colors::{ColorString, Color};

#[test]
fn test_red_coloring() {
    let mut color_string = ColorString {
        color: Color::Red,
        string: "Red".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[31mRed\x1b[0m");
}

#[test]
fn test_green_coloring() {
    let mut color_string = ColorString {
        color: Color::Green,
        string: "Green".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[32mGreen\x1b[0m");
}

#[test]
fn test_blue_coloring() {
    let mut color_string = ColorString {
        color: Color::Blue,
        string: "Blue".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[34mBlue\x1b[0m");
}

#[test]
fn test_bold_coloring() {
    let mut color_string = ColorString {
        color: Color::Bold,
        string: "Yellow".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[1mYellow\x1b[0m");
}