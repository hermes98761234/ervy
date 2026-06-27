use crate::datum::{Datum, DatumValue};
use crate::options::ScatterOptions;

type GridCell = Option<(char, Option<String>)>;

pub fn scatter(data: &[Datum], opts: &ScatterOptions) -> String {
    if data.is_empty() {
        return String::new();
    }

    let width = opts.width;
    let height = width; // square grid

    // Find bounds
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for d in data {
        if let DatumValue::Point([x, y]) = &d.value {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }
    }

    if max_x == f64::NEG_INFINITY {
        return String::new();
    }

    let x_range = (max_x - min_x).max(1.0);
    let y_range = (max_y - min_y).max(1.0);

    // Create grid (stores Option<(char, style_str)>)
    let mut grid: Vec<Vec<GridCell>> = vec![vec![None; width]; height];

    for d in data {
        if let DatumValue::Point([x, y]) = &d.value {
            let col = ((x - min_x) / x_range * (width - 1) as f64).round() as usize;
            let row = ((max_y - y) / y_range * (height - 1) as f64).round() as usize;
            let col = col.min(width - 1);
            let row = row.min(height - 1);

            let ch = d
                .style
                .as_ref()
                .and_then(|s| s.fg)
                .map(|(_, ch)| ch)
                .unwrap_or('*');

            let style_str = d
                .style
                .as_ref()
                .and_then(|s| s.fg)
                .map(|(color, ch)| format!("\x1b[{}m{}\x1b[0m", color.fg_code(), ch));

            grid[row][col] = Some((ch, style_str));
        }
    }

    // Render
    let mut result = String::new();
    for row in &grid {
        for cell in row {
            match cell {
                Some((_ch, Some(style_str))) => result.push_str(style_str),
                Some((ch, None)) => result.push(*ch),
                None => result.push(' '),
            }
        }
        result.push('\n');
    }

    // Legend
    for d in data {
        if let DatumValue::Point([x, y]) = &d.value {
            let x_str = if x.fract() == 0.0 {
                format!("{:.0}", x)
            } else {
                format!("{}", x)
            };
            let y_str = if y.fract() == 0.0 {
                format!("{:.0}", y)
            } else {
                format!("{}", y)
            };
            result.push_str(&format!("{}: ({}, {})\n", d.key, x_str, y_str));
        }
    }

    result.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{fg, Color};
    use crate::datum::Datum;

    #[test]
    fn scatter_basic() {
        let data = vec![
            Datum {
                key: "A".into(),
                value: DatumValue::Point([3.0, 4.0]),
                style: None,
                sides: None,
            },
            Datum {
                key: "B".into(),
                value: DatumValue::Point([2.0, 6.0]),
                style: None,
                sides: Some([2, 2]),
            },
        ];
        let result = scatter(&data, &ScatterOptions::default());
        assert!(result.contains("A"));
    }

    #[test]
    fn scatter_empty() {
        let data: Vec<Datum> = vec![];
        let result = scatter(&data, &ScatterOptions::default());
        assert!(result.is_empty());
    }

    #[test]
    fn scatter_single_point() {
        let data = vec![Datum::point("P", 5.0, 5.0)];
        let result = scatter(&data, &ScatterOptions::default());
        assert!(result.contains("P: (5, 5)"));
    }

    #[test]
    fn scatter_with_style() {
        let data = vec![Datum {
            key: "X".into(),
            value: DatumValue::Point([1.0, 1.0]),
            style: Some(fg(Color::Cyan, '#')),
            sides: None,
        }];
        let result = scatter(&data, &ScatterOptions::default());
        assert!(result.contains("\x1b[36m#"));
    }

    #[test]
    fn scatter_plots_correctly() {
        let data = vec![
            Datum::point("Origin", 0.0, 0.0),
            Datum::point("Max", 10.0, 10.0),
        ];
        let result = scatter(&data, &ScatterOptions::default());
        assert!(result.contains("Origin: (0, 0)"));
        assert!(result.contains("Max: (10, 10)"));
    }
}
