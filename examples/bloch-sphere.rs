use num_complex::Complex64;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use std::error::Error;
use std::f64::consts::TAU;
use std::fs;

#[derive(Clone, Copy)]
struct Point3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone, Copy)]
struct Point2Depth {
    x: f64,
    y: f64,
    depth: f64,
}

fn project(p: Point3) -> Point2Depth {
    let azimuth = (-35.0_f64).to_radians();
    let elevation = 20.0_f64.to_radians();

    let x1 = p.x * azimuth.cos() - p.y * azimuth.sin();
    let y1 = p.x * azimuth.sin() + p.y * azimuth.cos();
    let z1 = p.z;

    let y2 = y1 * elevation.cos() - z1 * elevation.sin();
    let z2 = y1 * elevation.sin() + z1 * elevation.cos();

    Point2Depth {
        x: x1,
        y: z2,
        depth: y2,
    }
}

fn circle_xy(radius: f64, z: f64, samples: usize) -> Vec<Point3> {
    (0..=samples)
        .map(|i| {
            let t = i as f64 * TAU / samples as f64;
            Point3 {
                x: radius * t.cos(),
                y: radius * t.sin(),
                z,
            }
        })
        .collect()
}

fn circle_xz(radius: f64, y: f64, samples: usize) -> Vec<Point3> {
    (0..=samples)
        .map(|i| {
            let t = i as f64 * TAU / samples as f64;
            Point3 {
                x: radius * t.cos(),
                y,
                z: radius * t.sin(),
            }
        })
        .collect()
}

fn draw_curve<DB: DrawingBackend>(
    chart: &mut ChartContext<'_, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    curve: &[Point3],
    front: &ShapeStyle,
    back: &ShapeStyle,
) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
    for pair in curve.windows(2) {
        let p1 = project(pair[0]);
        let p2 = project(pair[1]);
        let style = if (p1.depth + p2.depth) * 0.5 >= 0.0 {
            *front
        } else {
            *back
        };

        chart.draw_series(std::iter::once(PathElement::new(
            vec![(p1.x, p1.y), (p2.x, p2.y)],
            style,
        )))?;
    }
    Ok(())
}

fn parse_angles_degrees() -> (f64, f64) {
    let mut args = std::env::args().skip(1);
    let theta_deg = args
        .next()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(60.0);
    let phi_deg = args
        .next()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(45.0);
    (theta_deg, phi_deg)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (theta_deg, phi_deg) = parse_angles_degrees();
    let theta = theta_deg.to_radians();
    let phi = phi_deg.to_radians();

    let alpha = (theta / 2.0).cos();
    let beta = Complex64::from_polar((theta / 2.0).sin(), phi);

    let bloch = Point3 {
        x: theta.sin() * phi.cos(),
        y: theta.sin() * phi.sin(),
        z: theta.cos(),
    };

    fs::create_dir_all("target")?;
    let output_path = "target/bloch-sphere.svg";
    let root = SVGBackend::new(output_path, (900, 700)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Bloch Sphere", ("sans-serif", 42))
        .build_cartesian_2d(-1.45_f64..1.45_f64, -1.25_f64..1.25_f64)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_labels(0)
        .y_labels(0)
        .draw()?;

    let outline: Vec<(f64, f64)> = (0..=360)
        .map(|i| {
            let t = i as f64 * TAU / 360.0;
            (t.cos(), t.sin())
        })
        .collect();
    chart.draw_series(std::iter::once(PathElement::new(
        outline,
        BLACK.mix(0.7).stroke_width(2),
    )))?;

    let front_style = ShapeStyle {
        color: RGBColor(70, 130, 180).to_rgba(),
        filled: false,
        stroke_width: 2,
    };
    let back_style = ShapeStyle {
        color: RGBColor(70, 130, 180).mix(0.25),
        filled: false,
        stroke_width: 1,
    };

    draw_curve(
        &mut chart,
        &circle_xy(1.0, 0.0, 300),
        &front_style,
        &back_style,
    )?;
    draw_curve(
        &mut chart,
        &circle_xz(1.0, 0.0, 300),
        &front_style,
        &back_style,
    )?;

    let axes = [
        (
            Point3 {
                x: -1.1,
                y: 0.0,
                z: 0.0,
            },
            Point3 {
                x: 1.1,
                y: 0.0,
                z: 0.0,
            },
            "x",
        ),
        (
            Point3 {
                x: 0.0,
                y: -1.1,
                z: 0.0,
            },
            Point3 {
                x: 0.0,
                y: 1.1,
                z: 0.0,
            },
            "y",
        ),
        (
            Point3 {
                x: 0.0,
                y: 0.0,
                z: -1.1,
            },
            Point3 {
                x: 0.0,
                y: 0.0,
                z: 1.1,
            },
            "z",
        ),
    ];

    for (from, to, label) in axes {
        let p1 = project(from);
        let p2 = project(to);
        chart.draw_series(std::iter::once(PathElement::new(
            vec![(p1.x, p1.y), (p2.x, p2.y)],
            BLACK.mix(0.55).stroke_width(1),
        )))?;
        chart.draw_series(std::iter::once(Text::new(
            label.to_string(),
            (p2.x, p2.y),
            ("sans-serif", 24).into_font().color(&BLACK),
        )))?;
    }

    let origin = project(Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });
    let tip = project(bloch);
    chart.draw_series(std::iter::once(PathElement::new(
        vec![(origin.x, origin.y), (tip.x, tip.y)],
        RED.stroke_width(4),
    )))?;
    chart.draw_series(std::iter::once(Circle::new(
        (tip.x, tip.y),
        6,
        RED.mix(0.9).filled(),
    )))?;

    let basis_points = [
        (
            Point3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            "|0>",
        ),
        (
            Point3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            "|1>",
        ),
        (
            Point3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            "|+>",
        ),
        (
            Point3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            "|->",
        ),
        (
            Point3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            "|+i>",
        ),
        (
            Point3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            "|-i>",
        ),
    ];

    for (pt, label) in basis_points {
        let p = project(pt);
        chart.draw_series(std::iter::once(Text::new(
            label.to_string(),
            (p.x, p.y),
            ("sans-serif", 19).into_font().color(&BLACK.mix(0.75)),
        )))?;
    }

    chart.draw_series(std::iter::once(Text::new(
        format!(
            "theta = {:.1} deg, phi = {:.1} deg\n|psi> = {:.4}|0> + ({:.4} {:+.4}i)|1>",
            theta_deg, phi_deg, alpha, beta.re, beta.im
        ),
        (-1.40, -1.18),
        ("sans-serif", 20).into_font().color(&BLACK.mix(0.85)),
    )))?;

    root.present()?;

    println!("Wrote {}", output_path);
    println!("Use: cargo run --example bloch-sphere -- <theta_deg> <phi_deg>");
    println!(
        "Current Bloch vector = ({:.4}, {:.4}, {:.4})",
        bloch.x, bloch.y, bloch.z
    );

    Ok(())
}
