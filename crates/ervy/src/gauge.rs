use crate::color::Style;
use crate::datum::{Datum, DatumValue};
use crate::options::GaugeOptions;

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

pub fn gauge(data: &[Datum], opts: &GaugeOptions) -> String {
    if data.is_empty() {
        return String::new();
    }

    let radius = opts.radius as f64;
    let datum = &data[0];
    let val = get_datum_value(datum).clamp(0.0, 1.0);

    let rows = (radius * 2.0 + 1.0) as i32;
    let mut result = String::new();

    for r in 0..=(rows / 2) {
        let y = (rows / 2 - r) as f64;
        let x_max = (radius * radius - y * y).sqrt();

        for c in -(x_max.ceil() as i32)..=(x_max.ceil() as i32) {
            let x = c as f64;
            let angle = y.atan2(-x);
            let normalized = if angle < 0.0 {
                angle + 2.0 * std::f64::consts::PI
            } else {
                angle
            };

            if normalized <= std::f64::consts::PI {
                let filled = normalized / std::f64::consts::PI <= val;
                let ch = if filled {
                    datum
                        .style
                        .as_ref()
                        .and_then(|s| s.fg)
                        .map(|(_, ch)| ch)
                        .unwrap_or('█')
                } else {
                    ' '
                };
                result.push_str(&render_styled(ch, &datum.style));
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }

    // Label
    if opts.show_percentage {
        result.push_str(&format!("{}: {:.0}%\n", datum.key, val * 100.0));
    }

    result.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{fg, Color};
    use crate::datum::Datum;

    #[test]
    fn gauge_basic() {
        let data = vec![Datum::scalar("A", 0.5)];
        let result = gauge(&data, &GaugeOptions::default());
        assert!(result.contains("A"));
    }

    #[test]
    fn gauge_empty() {
        let data: Vec<Datum> = vec![];
        let result = gauge(&data, &GaugeOptions::default());
        assert!(result.is_empty());
    }

    #[test]
    fn gauge_shows_percentage() {
        let data = vec![Datum::scalar("CPU", 0.75)];
        let result = gauge(&data, &GaugeOptions::default());
        assert!(result.contains("CPU: 75%"));
    }

    #[test]
    fn gauge_with_style() {
        let data = vec![Datum::styled("A", 0.5, fg(Color::Red, '*'))];
        let result = gauge(&data, &GaugeOptions::default());
        assert!(result.contains("\x1b[31m"));
    }

    #[test]
    fn gauge_clamps_above_one() {
        let data = vec![Datum::scalar("A", 2.0)];
        let result = gauge(&data, &GaugeOptions::default());
        assert!(result.contains("A: 100%"));
    }

    #[test]
    fn gauge_clamps_below_zero() {
        let data = vec![Datum::scalar("A", -1.0)];
        let result = gauge(&data, &GaugeOptions::default());
        assert!(result.contains("A: 0%"));
    }

    #[test]
    fn gauge_hide_percentage() {
        let data = vec![Datum::scalar("A", 0.5)];
        let opts = GaugeOptions {
            show_percentage: false,
            ..Default::default()
        };
        let result = gauge(&data, &opts);
        assert!(!result.contains("50%"));
        assert!(!result.contains("A:"));
    }
}
