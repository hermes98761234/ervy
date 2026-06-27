use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    pub fn fg_code(&self) -> u8 {
        match self {
            Color::Black => 30,
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Blue => 34,
            Color::Magenta => 35,
            Color::Cyan => 36,
            Color::White => 37,
        }
    }

    pub fn bg_code(&self) -> u8 {
        match self {
            Color::Black => 40,
            Color::Red => 41,
            Color::Green => 42,
            Color::Yellow => 43,
            Color::Blue => 44,
            Color::Magenta => 45,
            Color::Cyan => 46,
            Color::White => 47,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Color::Black => "black",
            Color::Red => "red",
            Color::Green => "green",
            Color::Yellow => "yellow",
            Color::Blue => "blue",
            Color::Magenta => "magenta",
            Color::Cyan => "cyan",
            Color::White => "white",
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct Style {
    pub fg: Option<(Color, char)>,
    pub bg: Option<(Color, usize)>,
}

impl Style {
    pub fn render_fg(&self) -> String {
        match self.fg {
            Some((color, ch)) => format!("\x1b[{}m{}\x1b[0m", color.fg_code(), ch),
            None => String::new(),
        }
    }

    pub fn render_bg(&self, width: usize) -> String {
        match self.bg {
            Some((color, _)) => format!("\x1b[{}m{}\x1b[0m", color.bg_code(), " ".repeat(width)),
            None => " ".repeat(width),
        }
    }

    pub fn render_bg_exact(&self) -> String {
        match self.bg {
            Some((color, len)) => format!("\x1b[{}m{}\x1b[0m", color.bg_code(), " ".repeat(len)),
            None => String::new(),
        }
    }
}

pub fn fg(color: Color, ch: char) -> Style {
    Style {
        fg: Some((color, ch)),
        bg: None,
    }
}

pub fn bg(color: Color, len: usize) -> Style {
    Style {
        fg: None,
        bg: Some((color, len)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fg_creates_style_with_fg() {
        let s = fg(Color::Red, '*');
        assert_eq!(s.fg, Some((Color::Red, '*')));
        assert!(s.bg.is_none());
    }

    #[test]
    fn bg_creates_style_with_bg() {
        let s = bg(Color::Green, 4);
        assert_eq!(s.bg, Some((Color::Green, 4)));
        assert!(s.fg.is_none());
    }

    #[test]
    fn color_display_fg() {
        let s = fg(Color::Red, '*');
        let rendered = s.render_fg();
        assert!(rendered.contains("\x1b[31m"));
        assert!(rendered.contains('*'));
    }

    #[test]
    fn color_display_bg() {
        let s = bg(Color::Blue, 3);
        let rendered = s.render_bg(3);
        assert!(rendered.contains("\x1b[44m"));
    }
}
