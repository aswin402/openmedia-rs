use serde::{Deserialize, Serialize};
use openmedia_core::{Result, OpenMediaError};
use crate::SvgBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartPoint {
    pub label: String,
    pub value: f64,
}

pub fn create_chart(
    chart_type: &str,
    title: Option<&str>,
    data: &[ChartPoint],
    width: u32,
    height: u32,
) -> Result<String> {
    if data.is_empty() {
        return Err(OpenMediaError::InvalidParameter {
            param: "data".to_string(),
            reason: "Chart data cannot be empty".to_string(),
        });
    }

    let mut builder = SvgBuilder::new(width, height);

    // Dynamic background
    builder.rect(0.0, 0.0, width as f64, height as f64)
        .fill("#1a1a2e")
        .finish();

    // Standard title
    if let Some(t) = title {
        builder.text(width as f64 / 2.0 - (t.len() as f64 * 4.0), 35.0, t)
            .fill("#ffffff")
            .font_size(18.0)
            .font_family("sans-serif")
            .finish();
    }

    let palette = vec![
        "#2563eb", "#dc2626", "#16a34a", "#9333ea", "#ea580c", "#0891b2", "#4f46e5"
    ];

    match chart_type.to_lowercase().as_str() {
        "bar" => {
            let margin_left = 60.0;
            let margin_right = 40.0;
            let margin_top = 70.0;
            let margin_bottom = 60.0;

            let plot_width = width as f64 - margin_left - margin_right;
            let plot_height = height as f64 - margin_top - margin_bottom;

            let max_val = data.iter().map(|p| p.value).fold(0.0f64, f64::max);
            let max_val = if max_val <= 0.0 { 1.0 } else { max_val };

            // Draw Y-axis grid lines and labels
            let grid_count = 5;
            for i in 0..=grid_count {
                let ratio = i as f64 / grid_count as f64;
                let y_val = margin_top + plot_height * (1.0 - ratio);
                let label_val = ratio * max_val;

                builder.path(&format!("M {} {} L {} {}", margin_left, y_val, width as f64 - margin_right, y_val))
                    .stroke("#333355")
                    .stroke_width(1.0)
                    .fill("none")
                    .finish();

                builder.text(10.0, y_val + 4.0, &format!("{:.1}", label_val))
                    .fill("#94a3b8")
                    .font_size(10.0)
                    .font_family("sans-serif")
                    .finish();
            }

            // Draw bars
            let n = data.len();
            let spacing = 0.25;
            let bar_width = plot_width / n as f64;
            let inner_bar_width = bar_width * (1.0 - spacing);

            for i in 0..n {
                let p = &data[i];
                let bar_height = (p.value / max_val) * plot_height;
                let x = margin_left + i as f64 * bar_width + (bar_width * spacing / 2.0);
                let y = margin_top + plot_height - bar_height;
                let color = palette[i % palette.len()];

                builder.rect(x, y, inner_bar_width, bar_height)
                    .fill(color)
                    .finish();

                // Draw X-axis label
                let label_x = x + inner_bar_width / 2.0 - (p.label.len() as f64 * 3.0);
                builder.text(label_x, margin_top + plot_height + 20.0, &p.label)
                    .fill("#94a3b8")
                    .font_size(10.0)
                    .font_family("sans-serif")
                    .finish();
            }
        }
        "line" => {
            let margin_left = 60.0;
            let margin_right = 40.0;
            let margin_top = 70.0;
            let margin_bottom = 60.0;

            let plot_width = width as f64 - margin_left - margin_right;
            let plot_height = height as f64 - margin_top - margin_bottom;

            let max_val = data.iter().map(|p| p.value).fold(0.0f64, f64::max);
            let max_val = if max_val <= 0.0 { 1.0 } else { max_val };

            // Draw Y-axis grid lines and labels
            let grid_count = 5;
            for i in 0..=grid_count {
                let ratio = i as f64 / grid_count as f64;
                let y_val = margin_top + plot_height * (1.0 - ratio);
                let label_val = ratio * max_val;

                builder.path(&format!("M {} {} L {} {}", margin_left, y_val, width as f64 - margin_right, y_val))
                    .stroke("#333355")
                    .stroke_width(1.0)
                    .fill("none")
                    .finish();

                builder.text(10.0, y_val + 4.0, &format!("{:.1}", label_val))
                    .fill("#94a3b8")
                    .font_size(10.0)
                    .font_family("sans-serif")
                    .finish();
            }

            let n = data.len();
            let mut path_d = String::new();

            // First pass: generate the path line
            for i in 0..n {
                let p = &data[i];
                let x = if n > 1 {
                    margin_left + (i as f64 / (n - 1) as f64) * plot_width
                } else {
                    margin_left + plot_width / 2.0
                };
                let y = margin_top + plot_height - (p.value / max_val) * plot_height;

                if i == 0 {
                    path_d.push_str(&format!("M {} {}", x, y));
                } else {
                    path_d.push_str(&format!(" L {} {}", x, y));
                }
            }

            builder.path(&path_d)
                .stroke("#3b82f6")
                .stroke_width(3.0)
                .fill("none")
                .finish();

            // Second pass: draw marker circles and labels
            for i in 0..n {
                let p = &data[i];
                let x = if n > 1 {
                    margin_left + (i as f64 / (n - 1) as f64) * plot_width
                } else {
                    margin_left + plot_width / 2.0
                };
                let y = margin_top + plot_height - (p.value / max_val) * plot_height;

                builder.circle(x, y, 5.0)
                    .fill("#ffffff")
                    .stroke("#3b82f6")
                    .stroke_width(2.0)
                    .finish();

                // Draw X-axis label
                let label_x = x - (p.label.len() as f64 * 3.0);
                builder.text(label_x, margin_top + plot_height + 20.0, &p.label)
                    .fill("#94a3b8")
                    .font_size(10.0)
                    .font_family("sans-serif")
                    .finish();
            }
        }
        "pie" => {
            let cx = width as f64 * 0.4;
            let cy = height as f64 / 2.0 + 10.0;
            let r = (width.min(height) as f64 * 0.6) / 2.0;

            let total: f64 = data.iter().map(|p| p.value).sum();

            if total <= 0.0 {
                // If total is 0 or negative, render a fallback circle
                builder.circle(cx, cy, r)
                    .fill("#475569")
                    .finish();
            } else {
                let mut current_angle = -std::f64::consts::FRAC_PI_2; // start at 12 o'clock
                let n = data.len();

                for i in 0..n {
                    let p = &data[i];
                    let slice_angle = (p.value / total) * 2.0 * std::f64::consts::PI;
                    let next_angle = current_angle + slice_angle;

                    let x1 = cx + r * current_angle.cos();
                    let y1 = cy + r * current_angle.sin();
                    let x2 = cx + r * next_angle.cos();
                    let y2 = cy + r * next_angle.sin();

                    let large_arc_flag = if slice_angle > std::f64::consts::PI { 1 } else { 0 };

                    let d = format!(
                        "M {} {} L {} {} A {} {} 0 {} 1 {} {} Z",
                        cx, cy, x1, y1, r, r, large_arc_flag, x2, y2
                    );

                    let color = palette[i % palette.len()];

                    builder.path(&d)
                        .fill(color)
                        .stroke("#1a1a2e")
                        .stroke_width(1.5)
                        .finish();

                    current_angle = next_angle;
                }
            }

            // Draw Legend
            let legend_x = width as f64 * 0.7;
            let n = data.len();
            let mut legend_y = cy - (n as f64 * 25.0 / 2.0);

            for i in 0..n {
                let p = &data[i];
                let color = palette[i % palette.len()];

                builder.rect(legend_x, legend_y, 15.0, 15.0)
                    .fill(color)
                    .finish();

                let percentage = if total > 0.0 { (p.value / total) * 100.0 } else { 0.0 };
                builder.text(legend_x + 25.0, legend_y + 12.0, &format!("{}: {} ({:.1}%)", p.label, p.value, percentage))
                    .fill("#ffffff")
                    .font_size(11.0)
                    .font_family("sans-serif")
                    .finish();

                legend_y += 25.0;
            }
        }
        other => {
            return Err(OpenMediaError::InvalidParameter {
                param: "chart_type".to_string(),
                reason: format!("Unsupported chart type: {}", other),
            });
        }
    }

    Ok(builder.build())
}
