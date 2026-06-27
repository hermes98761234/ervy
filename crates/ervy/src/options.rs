use crate::color::Style;

#[derive(Clone, Debug)]
pub struct BarOptions {
    pub bar_width: usize,
    pub left: usize,
    pub height: usize,
    pub padding: usize,
    pub style: char,
}

impl Default for BarOptions {
    fn default() -> Self {
        Self {
            bar_width: 3,
            left: 1,
            height: 6,
            padding: 3,
            style: '*',
        }
    }
}

#[derive(Clone, Debug)]
pub struct PieOptions {
    pub left: usize,
}

impl Default for PieOptions {
    fn default() -> Self {
        Self { left: 1 }
    }
}

#[derive(Clone, Debug)]
pub struct BulletOptions {
    pub width: usize,
    pub bar_width: usize,
    pub style: char,
    pub height: usize,
}

impl Default for BulletOptions {
    fn default() -> Self {
        Self {
            width: 30,
            bar_width: 1,
            style: '+',
            height: 6,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DonutOptions {
    pub left: usize,
    pub gap_char: Option<Style>,
}

impl Default for DonutOptions {
    fn default() -> Self {
        Self {
            left: 1,
            gap_char: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GaugeOptions {
    pub radius: usize,
    pub style: Option<Style>,
    pub bg_style: Option<Style>,
    pub show_percentage: bool,
}

impl Default for GaugeOptions {
    fn default() -> Self {
        Self {
            radius: 5,
            style: None,
            bg_style: None,
            show_percentage: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ScatterOptions {
    pub width: usize,
    pub legend_gap: usize,
}

impl Default for ScatterOptions {
    fn default() -> Self {
        Self {
            width: 15,
            legend_gap: 18,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bar_defaults() {
        let opts = BarOptions::default();
        assert_eq!(opts.bar_width, 3);
        assert_eq!(opts.left, 1);
        assert_eq!(opts.height, 6);
        assert_eq!(opts.padding, 3);
        assert_eq!(opts.style, '*');
    }

    #[test]
    fn pie_defaults() {
        let opts = PieOptions::default();
        assert_eq!(opts.left, 1);
    }

    #[test]
    fn bullet_defaults() {
        let opts = BulletOptions::default();
        assert_eq!(opts.width, 30);
        assert_eq!(opts.bar_width, 1);
        assert_eq!(opts.style, '+');
        assert_eq!(opts.height, 6);
    }

    #[test]
    fn donut_defaults() {
        let opts = DonutOptions::default();
        assert_eq!(opts.left, 1);
        assert!(opts.gap_char.is_none());
    }

    #[test]
    fn gauge_defaults() {
        let opts = GaugeOptions::default();
        assert_eq!(opts.radius, 5);
        assert!(opts.style.is_none());
        assert!(opts.bg_style.is_none());
        assert!(opts.show_percentage);
    }

    #[test]
    fn scatter_defaults() {
        let opts = ScatterOptions::default();
        assert_eq!(opts.width, 15);
        assert_eq!(opts.legend_gap, 18);
    }
}
