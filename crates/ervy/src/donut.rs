use crate::color::Style;
use crate::datum::{Datum, DatumValue};
use crate::options::DonutOptions;

fn get_datum_value(d: &Datum) -> f64 {
    match d.value {
        DatumValue::Scalar(v) => v,
        DatumValue::Point(_) => 0.0,
    }
}

fn render_styled(ch: char, style: &Option<Style>) -> String {
    match style {
        Some(s) => match s.fg {
            Some((color, _)) => format!("\x1b[{}m{}\x1b[0m", color.fg_code(), ch),
            None => ch.to_string(),
        },
        None => ch.to_string(),
    }
}

pub fn donut(data: &[Datum], opts: &DonutOptions) -> String {
    if data.is_empty() {
        return String::new();
    }

    let total: f64 = data.iter().map(|d| get_datum_value(d)).sum();
    if total <= 0.0 {
        return String::new();
    }

    let outer_radius = 5.0;
    let inner_radius = 2.0;
    let left = opts.left;

    // Compute start/end angles for each slice
    let mut angles: Vec<(f64, f64, &Datum)> = Vec::new();
    let mut current = 0.0_f64;
    for d in data {
        let val = get_datum_value(d);
        let sweep = (val / total) * 2.0 * std::f64::consts::PI;
        angles.push((current, current + sweep, d));
        current += sweep;
    }

    let mut result = String::new();
    let row_count = (outer_radius * 2.0 + 1.0) as i32;

    for r in -row_count / 2..=row_count / 2 {
        result.push_str(&" ".repeat(left));
        let y = r as f64;
        let x_max = (outer_radius * outer_radius - y * y).sqrt();

        for c in (-x_max.ceil() as i32)..=(x_max.ceil() as i32) {
            let x = c as f64;
            let dist_sq = x * x + y * y;

            if dist_sq <= outer_radius * outer_radius && dist_sq >= inner_radius * inner_radius {
                let angle = y.atan2(x);
                let normalized = if angle < 0.0 {
                    angle + 2.0 * std::f64::consts::PI
                } else {
                    angle
                };
                let slice = angles
                    .iter()
                    .find(|(start, end, _)| normalized >= *start && normalized < *end);
                match slice {
                    Some((_, _, datum)) => {
                        let ch = datum
                            .style
                            .as_ref()
                            .and_then(|s| s.fg)
                            .map(|(_, ch)| ch)
                            .unwrap_or(' ');
                        let rendered = render_styled(ch, &datum.style);
                        result.push_str(&rendered);
                    }
                    None => result.push(' '),
                }
            } else if dist_sq < inner_radius * inner_radius {
                // Inside the hole — render gap_char or space
                let rendered = match &opts.gap_char {
                    Some(style) => match style.fg {
                        Some((color, ch)) => format!("\x1b[{}m{}\x1b[0m", color.fg_code(), ch),
                        None => ' '.to_string(),
                    },
                    None => ' '.to_string(),
                };
                result.push_str(&rendered);
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }

    // Legend
    for (_, _, datum) in &angles {
        let ch = datum
            .style
            .as_ref()
            .and_then(|s| s.fg)
            .map(|(_, ch)| ch)
            .unwrap_or(' ');
        let val = get_datum_value(datum);
        let val_str = if val == val.floor() {
            format!("{:.0}", val)
        } else {
            format!("{}", val)
        };
        result.push_str(&format!("{}: {} ({})\n", ch, datum.key, val_str));
    }

    result.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{fg, Color};
    use crate::datum::Datum;

    #[test]
    fn donut_basic() {
        let data = vec![
            Datum::scalar("A", 5.0),
            Datum::scalar("B", 10.0),
            Datum::scalar("C", 10.0),
            Datum::scalar("D", 10.0),
        ];
        let result = donut(&data, &DonutOptions::default());
        assert!(result.contains("A"));
        assert!(result.contains("B"));
    }

    #[test]
    fn donut_empty() {
        let data: Vec<Datum> = vec![];
        let result = donut(&data, &DonutOptions::default());
        assert!(result.is_empty());
    }

    #[test]
    fn donut_with_styles() {
        let data = vec![
            Datum::styled("A", 5.0, fg(Color::Red, '*')),
            Datum::scalar("B", 10.0),
        ];
        let result = donut(&data, &DonutOptions::default());
        assert!(result.contains("\x1b[31m"));
    }

    #[test]
    fn donut_single_slice() {
        let data = vec![Datum::scalar("X", 100.0)];
        let result = donut(&data, &DonutOptions::default());
        assert!(result.contains("X"));
    }

    #[test]
    fn donut_zero_total() {
        let data = vec![Datum::scalar("A", 0.0), Datum::scalar("B", 0.0)];
        let result = donut(&data, &DonutOptions::default());
        assert!(result.is_empty());
    }

    #[test]
    fn donut_has_hole() {
        let data = vec![Datum::scalar("A", 10.0), Datum::scalar("B", 10.0)];
        let result = donut(&data, &DonutOptions::default());
        // The center row should have spaces (hole) in the middle
        let lines: Vec<&str> = result.lines().collect();
        let mid_line = lines[lines.len() / 2];
        // There should be at least 2 consecutive spaces in the middle (the hole)
        assert!(mid_line.contains("  "));
    }
}
