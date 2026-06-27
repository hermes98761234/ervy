use clap::{Parser, Subcommand};
use ervy::Datum;

// ── Data loading ─────────────────────────────────────────────────────────

fn load_data(path: Option<&str>) -> Vec<Datum> {
    let json = match path {
        Some(p) => std::fs::read_to_string(p).unwrap_or_else(|e| {
            eprintln!("Error reading file '{}': {}", p, e);
            std::process::exit(1);
        }),
        None => {
            use std::io::Read;
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .unwrap_or_else(|e| {
                    eprintln!("Error reading stdin: {}", e);
                    std::process::exit(1);
                });
            buf
        }
    };

    let data: Vec<Datum> = serde_json::from_str(&json).unwrap_or_else(|e| {
        eprintln!("Error parsing JSON: {}", e);
        std::process::exit(1);
    });
    data
}

// ── Color helpers ─────────────────────────────────────────────────────────

fn parse_color(s: &str) -> Option<ervy::Color> {
    match s.to_lowercase().as_str() {
        "black" => Some(ervy::Color::Black),
        "red" => Some(ervy::Color::Red),
        "green" => Some(ervy::Color::Green),
        "yellow" => Some(ervy::Color::Yellow),
        "blue" => Some(ervy::Color::Blue),
        "magenta" => Some(ervy::Color::Magenta),
        "cyan" => Some(ervy::Color::Cyan),
        "white" => Some(ervy::Color::White),
        _ => None,
    }
}

// ── CLI definition ───────────────────────────────────────────────────────

#[derive(Parser)]
#[command(name = "ervy")]
#[command(about = "Terminal charting CLI — renders charts from JSON data")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Horizontal bar chart
    Bar {
        /// JSON file (default: stdin)
        file: Option<String>,
        /// Bar width
        #[arg(long, default_value = "3")]
        bar_width: usize,
        /// Left margin
        #[arg(long, default_value = "1")]
        left: usize,
        /// Chart height
        #[arg(long, default_value = "6")]
        height: usize,
        /// Label padding
        #[arg(long, default_value = "3")]
        padding: usize,
        /// Bar character
        #[arg(long, default_value = "*")]
        style: char,
        /// Foreground color (name: red, green, blue, ...)
        #[arg(long)]
        fg: Option<String>,
        /// Foreground character (default uses style char)
        #[arg(long)]
        fg_char: Option<char>,
        /// Background color (name: red, green, blue, ...)
        #[arg(long)]
        bg: Option<String>,
    },
    /// Pie chart
    Pie {
        /// JSON file (default: stdin)
        file: Option<String>,
        /// Left margin
        #[arg(long, default_value = "1")]
        left: usize,
        /// Foreground color
        #[arg(long)]
        fg: Option<String>,
        /// Foreground character
        #[arg(long)]
        fg_char: Option<char>,
        /// Background color
        #[arg(long)]
        bg: Option<String>,
    },
    /// Bullet chart
    Bullet {
        /// JSON file (default: stdin)
        file: Option<String>,
        /// Total width
        #[arg(long, default_value = "30")]
        width: usize,
        /// Bar width
        #[arg(long, default_value = "1")]
        bar_width: usize,
        /// Bar character
        #[arg(long, default_value = "+")]
        style: char,
        /// Chart height
        #[arg(long, default_value = "6")]
        height: usize,
        /// Foreground color
        #[arg(long)]
        fg: Option<String>,
        /// Foreground character
        #[arg(long)]
        fg_char: Option<char>,
        /// Background color
        #[arg(long)]
        bg: Option<String>,
    },
    /// Donut chart
    Donut {
        /// JSON file (default: stdin)
        file: Option<String>,
        /// Left margin
        #[arg(long, default_value = "1")]
        left: usize,
        /// Background gap color (name: red, green, ...)
        #[arg(long)]
        gap_color: Option<String>,
        /// Foreground color
        #[arg(long)]
        fg: Option<String>,
        /// Foreground character
        #[arg(long)]
        fg_char: Option<char>,
        /// Background color
        #[arg(long)]
        bg: Option<String>,
    },
    /// Gauge chart
    Gauge {
        /// JSON file (default: stdin)
        file: Option<String>,
        /// Radius (half-width in characters)
        #[arg(long, default_value = "5")]
        radius: usize,
        /// Show percentage label
        #[arg(long, default_value_t = true)]
        show_percentage: bool,
        /// Foreground color
        #[arg(long)]
        fg: Option<String>,
        /// Foreground character
        #[arg(long)]
        fg_char: Option<char>,
        /// Background color
        #[arg(long)]
        bg: Option<String>,
    },
    /// Scatter chart
    Scatter {
        /// JSON file (default: stdin)
        file: Option<String>,
        /// Chart width (columns)
        #[arg(long, default_value = "15")]
        width: usize,
        /// Legend gap (columns between chart and legend)
        #[arg(long, default_value = "18")]
        legend_gap: usize,
        /// Foreground color
        #[arg(long)]
        fg: Option<String>,
        /// Foreground character
        #[arg(long)]
        fg_char: Option<char>,
        /// Background color
        #[arg(long)]
        bg: Option<String>,
    },
}

// ── Main ──────────────────────────────────────────────────────────────────

fn main() {
    let cli = Cli::parse();

    let output = match cli.command {
        Commands::Bar {
            file,
            bar_width,
            left,
            height,
            padding,
            style,
            fg,
            fg_char,
            bg,
        } => {
            let data = load_data(file.as_deref());
            let opts = ervy::BarOptions {
                bar_width,
                left,
                height,
                padding,
                style,
            };

            if let Some(fg_name) = fg {
                let color = parse_color(&fg_name).unwrap_or_else(|| {
                    eprintln!("Unknown color: {}", fg_name);
                    std::process::exit(1);
                });
                let ch = fg_char.unwrap_or(style);
                apply_fg(&data, color, ch, &opts)
            } else if let Some(bg_name) = bg {
                let color = parse_color(&bg_name).unwrap_or_else(|| {
                    eprintln!("Unknown color: {}", bg_name);
                    std::process::exit(1);
                });
                apply_bg(&data, color, &opts)
            } else {
                ervy::bar(&data, &opts)
            }
        }

        Commands::Pie {
            file,
            left,
            fg,
            fg_char,
            bg,
        } => {
            let data = load_data(file.as_deref());
            let opts = ervy::PieOptions { left };

            if let Some(fg_name) = fg {
                let color = parse_color(&fg_name).unwrap_or_else(|| {
                    eprintln!("Unknown color: {}", fg_name);
                    std::process::exit(1);
                });
                let ch = fg_char.unwrap_or('*');
                ervy::pie(&apply_fg_static(&data, color, ch), &opts)
            } else if let Some(bg_name) = bg {
                let color = parse_color(&bg_name).unwrap_or_else(|| {
                    eprintln!("Unknown color: {}", bg_name);
                    std::process::exit(1);
                });
                ervy::pie(&apply_bg_static(&data, color), &opts)
            } else {
                ervy::pie(&data, &opts)
            }
        }

        Commands::Bullet {
            file,
            width,
            bar_width,
            style,
            height,
            fg,
            fg_char,
            bg,
        } => {
            let data = load_data(file.as_deref());
            let opts = ervy::BulletOptions {
                width,
                bar_width,
                style,
                height,
            };

            if let Some(fg_name) = fg {
                let color = parse_color(&fg_name).unwrap_or_else(|| {
                    eprintln!("Unknown color: {}", fg_name);
                    std::process::exit(1);
                });
                let ch = fg_char.unwrap_or(style);
                let styled = apply_fg_static(&data, color, ch);
                ervy::bullet(&styled, &opts)
            } else if let Some(bg_name) = bg {
                let color = parse_color(&bg_name).unwrap_or_else(|| {
                    eprintln!("Unknown color: {}", bg_name);
                    std::process::exit(1);
                });
                let styled = apply_bg_static(&data, color);
                ervy::bullet(&styled, &opts)
            } else {
                ervy::bullet(&data, &opts)
            }
        }

        Commands::Donut {
            file,
            left,
            gap_color,
            fg,
            fg_char,
            bg,
        } => {
            let data = load_data(file.as_deref());
            let gap_char_style = gap_color
                .as_ref()
                .and_then(|c| parse_color(c).map(|color| ervy::bg(color, 0)));
            let opts = ervy::DonutOptions {
                left,
                gap_char: gap_char_style,
            };

            if let Some(fg_name) = fg {
                let color = parse_color(&fg_name).unwrap_or_else(|| {
                    eprintln!("Unknown color: {}", fg_name);
                    std::process::exit(1);
                });
                let ch = fg_char.unwrap_or('*');
                ervy::donut(&apply_fg_static(&data, color, ch), &opts)
            } else if let Some(bg_name) = bg {
                let color = parse_color(&bg_name).unwrap_or_else(|| {
                    eprintln!("Unknown color: {}", bg_name);
                    std::process::exit(1);
                });
                ervy::donut(&apply_bg_static(&data, color), &opts)
            } else {
                ervy::donut(&data, &opts)
            }
        }

        Commands::Gauge {
            file,
            radius,
            show_percentage,
            fg,
            fg_char,
            bg,
        } => {
            let data = load_data(file.as_deref());

            let fg_style = match (fg.as_ref(), fg_char) {
                (Some(fg_name), ch) => {
                    let color = parse_color(fg_name).unwrap_or_else(|| {
                        eprintln!("Unknown color: {}", fg_name);
                        std::process::exit(1);
                    });
                    let c = ch.unwrap_or('●');
                    Some(ervy::fg(color, c))
                }
                _ => None,
            };

            let bg_style = match bg.as_ref() {
                Some(bg_name) => {
                    let color = parse_color(bg_name).unwrap_or_else(|| {
                        eprintln!("Unknown color: {}", bg_name);
                        std::process::exit(1);
                    });
                    Some(ervy::bg(color, 0))
                }
                _ => None,
            };

            let opts = ervy::GaugeOptions {
                radius,
                style: fg_style,
                bg_style,
                show_percentage,
            };
            ervy::gauge(&data, &opts)
        }

        Commands::Scatter {
            file,
            width,
            legend_gap,
            fg,
            fg_char,
            bg,
        } => {
            let data = load_data(file.as_deref());
            let opts = ervy::ScatterOptions { width, legend_gap };

            if let Some(fg_name) = fg {
                let color = parse_color(&fg_name).unwrap_or_else(|| {
                    eprintln!("Unknown color: {}", fg_name);
                    std::process::exit(1);
                });
                let ch = fg_char.unwrap_or('●');
                ervy::scatter(&apply_fg_static(&data, color, ch), &opts)
            } else if let Some(bg_name) = bg {
                let color = parse_color(&bg_name).unwrap_or_else(|| {
                    eprintln!("Unknown color: {}", bg_name);
                    std::process::exit(1);
                });
                ervy::scatter(&apply_bg_static(&data, color), &opts)
            } else {
                ervy::scatter(&data, &opts)
            }
        }
    };

    print!("{}", output);
}

// ── Color application helpers ────────────────────────────────────────────

fn apply_fg(data: &[Datum], color: ervy::Color, ch: char, opts: &ervy::BarOptions) -> String {
    let styled: Vec<Datum> = data
        .iter()
        .map(|d| {
            let mut d2 = d.clone();
            let new_style = ervy::fg(color, ch);
            d2.style = match &d.style {
                Some(existing) => Some(ervy::Style {
                    fg: new_style.fg,
                    bg: existing.bg,
                }),
                None => Some(new_style),
            };
            d2
        })
        .collect();
    ervy::bar(&styled, opts)
}

fn apply_fg_static(data: &[Datum], color: ervy::Color, ch: char) -> Vec<Datum> {
    data.iter()
        .map(|d| {
            let mut d2 = d.clone();
            let new_style = ervy::fg(color, ch);
            d2.style = match &d.style {
                Some(existing) => Some(ervy::Style {
                    fg: new_style.fg,
                    bg: existing.bg,
                }),
                None => Some(new_style),
            };
            d2
        })
        .collect()
}

fn apply_bg(data: &[Datum], color: ervy::Color, opts: &ervy::BarOptions) -> String {
    let styled: Vec<Datum> = data
        .iter()
        .map(|d| {
            let mut d2 = d.clone();
            let new_style = ervy::bg(color, 0);
            d2.style = match &d.style {
                Some(existing) => Some(ervy::Style {
                    fg: existing.fg,
                    bg: new_style.bg,
                }),
                None => Some(new_style),
            };
            d2
        })
        .collect();
    ervy::bar(&styled, opts)
}

fn apply_bg_static(data: &[Datum], color: ervy::Color) -> Vec<Datum> {
    data.iter()
        .map(|d| {
            let mut d2 = d.clone();
            let new_style = ervy::bg(color, 0);
            d2.style = match &d.style {
                Some(existing) => Some(ervy::Style {
                    fg: existing.fg,
                    bg: new_style.bg,
                }),
                None => Some(new_style),
            };
            d2
        })
        .collect()
}
