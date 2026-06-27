use crate::color::Style;
use crate::datum::Datum;
use crate::options::BulletOptions;

fn render_bar(ch: char, count: usize, style: &Option<Style>) -> String {
    let s = ch.to_string().repeat(count);
    match style {
        Some(st) => {
            if let Some((color, _)) = st.fg {
                format!("\x1b[{}m{}\x1b[0m", color.fg_code(), s)
            } else if let Some((color, _)) = st.bg {
                format!("\x1b[{}m{}\x1b[0m", color.bg_code(), " ".repeat(count))
            } else {
                s
            }
        }
        None => s,
    }
}

pub fn bullet(data: &[Datum], opts: &BulletOptions) -> String {
    if data.is_empty() {
        return String::new();
    }

    let max_val = data
        .iter()
        .filter_map(|d| d.value_as_scalar())
        .fold(f64::NEG_INFINITY, f64::max);
    if max_val <= 0.0 {
        return String::new();
    }

    let &BulletOptions {
        width,
        bar_width: _,
        style,
        height: _,
    } = opts;

    let key_width = data.iter().map(|d| d.key.len()).max().unwrap_or(0);
    // bar_area = width - key_width - 3 (for " |" + "|" = 3 chars)
    let bar_area = width.saturating_sub(key_width + 3);
    if bar_area == 0 {
        return String::new();
    }

    let mut result = String::new();

    for datum in data {
        let val = datum.value_as_scalar().unwrap_or(0.0);
        let filled = ((val / max_val) * bar_area as f64).round() as usize;
        let empty = bar_area - filled;

        let bar_char = datum
            .style
            .as_ref()
            .and_then(|s| s.fg)
            .map(|(_, ch)| ch)
            .unwrap_or(style);
        let bar = render_bar(bar_char, filled, &datum.style);
        let empty_fill = " ".repeat(empty);

        result.push_str(&format!(
            "{:key_width$} |{}{}|\n",
            datum.key, bar, empty_fill
        ));
    }

    result.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bullet_basic() {
        let data = vec![
            Datum::scalar("Month", 5.0),
            Datum::scalar("Week", 3.0),
            Datum::scalar("Day", 20.0),
            Datum::scalar("Now", 15.0),
        ];
        let result = bullet(&data, &BulletOptions::default());
        assert!(result.contains("Month"));
        assert!(result.contains("Day"));
    }

    #[test]
    fn bullet_empty() {
        let data: Vec<Datum> = vec![];
        let result = bullet(&data, &BulletOptions::default());
        assert!(result.is_empty());
    }

    #[test]
    fn bullet_styled() {
        use crate::color::{fg, Color};
        let data = vec![
            Datum::styled("A", 5.0, fg(Color::Red, '*')),
            Datum::scalar("B", 3.0),
        ];
        let result = bullet(&data, &BulletOptions::default());
        assert!(result.contains("\x1b[31m"));
        assert!(result.contains("A"));
        assert!(result.contains("B"));
    }

    #[test]
    fn bullet_with_zero_max() {
        let data = vec![Datum::scalar("A", 0.0), Datum::scalar("B", 0.0)];
        let result = bullet(&data, &BulletOptions::default());
        assert!(result.is_empty());
    }

    #[test]
    fn bullet_single_datum() {
        let data = vec![Datum::scalar("Only", 10.0)];
        let result = bullet(&data, &BulletOptions::default());
        assert!(result.contains("Only"));
        assert!(result.contains("|"));
    }
}
