//! Colorized output utilities for the terminal using ANSI escape codes.
//! # Examples:
//! ```
//! use cli_utils::colors::*;
//! println!("{}{}{}", red("Red"), green("Green"), blue("Blue"));
//! ```

/// Returns a string with the ANSI escape code for red.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", red("Red"));
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for green.
/// # Examples:
/// ```
/// use cli_utils::colors::*; 
/// println!("{}", green("Green"));
/// ```
pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for blue.
/// # Examples:
/// ```
/// use cli_utils::colors::*; 
/// println!("{}", blue("Blue"));
/// ```
pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for bold.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", bold("Bold"));
/// ```
pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for reset.
/// # Examples:
/// ```
/// use cli_utils::colors::*; 
/// println!("{}", reset("Reset"));
/// ```
pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}

/// An enum representing the color of the text.
pub enum Color{
    Red,
    Green,
    Blue,
    Bold,
}

/// A struct that contains a string, a color, and a colorized string.
pub struct ColorString {
    pub color: Color,
    pub string: String,
    pub colorized: String
}


impl ColorString {
    /// Create a new ColorString with the given string and color.
    /// # Examples:
    /// ```
    /// use cli_utils::colors::*;
    /// let color_string = ColorString::new(Color::Red, "Red".to_string());
    /// ```
    ///  # Examples:
    /// ```
    /// use cli_utils::colors::*;
    /// let color_str = String::new();
    /// let color_string = ColorString::new(Color::Red, color_str);
    /// assert_eq!(color_string.string, "\x1b[31m\x1b[0m".to_string());
    /// ```
    /// # Panics:
    /// The `new` function will panic if it fails to create a new ColorString.
    pub fn new(color: Color, string: String) -> ColorString {
        let mut color_string = ColorString {
            color,
            string,
            colorized: String::new()
        };
        color_string.paint();
        color_string
    }

    /// Create a new ColorString with the given string and color.\
    /// Example:
    /// ```
    /// use cli_utils::colors::*;
    /// let color_string = ColorString::new(Color::Red, "Red".to_string());
    /// ```
    pub fn paint(&mut self) {
        match self.color {
            Color::Red => self.colorized = red(&self.string),
            Color::Green => self.colorized = green(&self.string),
            Color::Blue => self.colorized = blue(&self.string),
            Color::Bold => self.colorized = bold(&self.string),
        };
    }

    /// Reset the color of the string.
    /// # Examples:
    /// ```
    /// use cli_utils::colors::*;
    /// let mut color_string = ColorString::new(Color::Red, "Red".to_string());
    /// color_string.reset();
    /// ```
    pub fn reset(&mut self) {
        self.colorized = reset(&self.string);
    }

}
