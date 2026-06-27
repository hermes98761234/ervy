use crate::datum::{Datum, DatumValue};
use crate::options::BarOptions;

fn pad_mid(s: &str, width: usize) -> String {
    let len = s.chars().count();
    if len >= width {
        s.chars().take(width).collect()
    } else {
        let mid = (width - len) / 2;
        let left = " ".repeat(mid);
        let right_total = width - len - mid;
        let right = " ".repeat(right_total);
        format!("{}{}{}", left, s, right)
    }
}

pub fn bar(data: &[Datum], opts: &BarOptions) -> String {
    if data.is_empty() {
        return String::new();
    }

    let bar_width = opts.bar_width;
    let left = opts.left;
    let height = opts.height;
    let padding = opts.padding;
    let style = opts.style;

    let mut result = " ".repeat(left);

    let values: Vec<f64> = data
        .iter()
        .map(|d| match d.value {
            DatumValue::Scalar(v) => v,
            DatumValue::Point(_) => 0.0,
        })
        .collect();

    let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    if max_val <= 0.0 {
        return String::new();
    }

    let length = data.len();

    for i in 0..(height + 2) {
        for j in 0..length {
            let tmp = &data[j];
            let val = values[j];
            let val_str = format!("{:.0}", val);
            let ratio = height as f64 - (height as f64 * val / max_val);
            let ratio_round = ratio.round() as isize;

            let pad_char: String = if ratio > (i as f64 + 2.0) {
                " ".to_string()
            } else if ratio_round == i as isize {
                val_str.clone()
            } else if ratio_round < i as isize {
                // Below bar top — use datum style or default style
                match &tmp.style {
                    Some(s) => match s.fg {
                        Some((_, ch)) => ch.to_string(),
                        None => style.to_string(),
                    },
                    None => style.to_string(),
                }
            } else {
                " ".to_string()
            };

            if pad_char == val_str {
                result.push_str(&pad_mid(&val_str, bar_width));
                result.push_str(&" ".repeat(padding));
            } else if i != height + 1 {
                // Repeat the character bar_width times
                result.push_str(&pad_char.repeat(bar_width));
                result.push_str(&" ".repeat(padding));
            } else {
                // Last row — render key
                let key = &tmp.key;
                if key.chars().count() > bar_width {
                    result.push_str(
                        &key.chars()
                            .take(bar_width)
                            .chain(std::iter::repeat(' '))
                            .take(bar_width + padding)
                            .collect::<String>(),
                    );
                } else {
                    result.push_str(&pad_mid(key, bar_width));
                    result.push_str(&" ".repeat(padding));
                }
            }
        }
        if i != height + 1 {
            result.push('\n');
            result.push_str(&" ".repeat(left));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{fg, Color};
    use crate::datum::Datum;

    #[test]
    fn bar_basic() {
        let data = vec![
            Datum::scalar("A", 5.0),
            Datum::scalar("B", 10.0),
            Datum::scalar("C", 7.0),
        ];
        let opts = BarOptions::default();
        let output = bar(&data, &opts);
        assert!(!output.is_empty());
        // Should contain keys at the bottom
        for line in output.lines() {
            let _ = line;
        }
        assert!(output.contains("A"));
        assert!(output.contains("B"));
        assert!(output.contains("C"));
        // Should contain value labels
        assert!(output.contains("5"));
        assert!(output.contains("10"));
        assert!(output.contains("7"));
    }

    #[test]
    fn bar_empty() {
        let data: Vec<Datum> = vec![];
        let opts = BarOptions::default();
        let output = bar(&data, &opts);
        assert_eq!(output, "");
    }

    #[test]
    fn bar_styled() {
        let data = vec![
            Datum::styled("X", 8.0, fg(Color::Red, '#')),
            Datum::scalar("Y", 4.0),
        ];
        let opts = BarOptions::default();
        let output = bar(&data, &opts);
        assert!(!output.is_empty());
        // The styled bar should use '#' instead of default '*'
        assert!(output.contains('#'));
        assert!(output.contains("X"));
        assert!(output.contains("Y"));
    }
}
