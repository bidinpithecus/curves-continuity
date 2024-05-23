mod bspline;
mod bezier;
mod plotter;
mod join;

use ndarray::Array;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bspline_plotter = plotter::Plotter::new();
    let mut bezier_plotter = plotter::Plotter::new();
    let mut both_plotter = plotter::Plotter::new();
    let mut c0_plotter = plotter::Plotter::new();
    let mut c1_plotter = plotter::Plotter::new();
    let mut c2_plotter = plotter::Plotter::new();
    let num_points = 10000;

    // B-Spline start
    let bspline_control_points = vec![
        vec![0.0, 0.0, 0.0],
        vec![0.5, 1.5, 0.0],
        vec![1.0, -1.5, 0.0],
        vec![1.5, 1.0, 0.0],
        vec![2.0, -1.25, 0.0],
        vec![2.5, 1.0, 0.0],
        vec![3.0, 1.5, 0.0],
        vec![3.5, 1.0, 0.0],
        vec![4.0, 1.0, 0.0],
        vec![4.5, 0.0, 0.0],
    ];

    let n = bspline_control_points.len() - 1;
    let degree = 5;
    let knots = bspline::generate_knot_vector(n, degree);

    let t: Vec<f64> = Array::linspace(0.0, n as f64 - degree as f64 + 2.0 - 0.00000001, num_points).into_raw_vec();
    let mut bspline_curve_points = Vec::new();

    for &u in &t {
        let point = bspline::bspline(&bspline_control_points, u, degree, &knots);
        bspline_curve_points.push(point);
    }

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bspline_curve_points.iter().map(|point| (point[0], point[1])).unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bspline_control_points.iter().map(|point| (point[0], point[1])).unzip();

    bspline_plotter.line(x_values.clone(), y_values.clone(), "B-Spline Curve", false);
    bspline_plotter.line(x_control_points.clone(), y_control_points.clone(), "Control lines", true);
    bspline_plotter.markers(x_control_points.clone(), y_control_points.clone(), "Control Points");

    bspline_plotter.plot("5th degree B Spline Curve", "results/bspline.html", false, true);
    // B-Spline end

    // Bezier start
    let bezier_control_points = vec![
        vec![0.0, 0.0, 0.0],
        vec![0.5, 1.5, 0.0],
        vec![1.25, 2.0, 0.0],
        vec![2.5, 1.5, 0.0],
        vec![1.5, 0.5, 0.0],
        vec![4.0, -1.5, 0.0],
    ];

    let t: Vec<f64> = Array::linspace(0.0, 1.0 - 0.000000001, num_points).into_raw_vec();
    let mut bezier_curve_points = Vec::new();

    for &u in &t {
        let point = bezier::bezier(&bezier_control_points, u);
        bezier_curve_points.push(point);
    }

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bezier_curve_points.iter().map(|point| (point[0], point[1])).unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bezier_control_points.iter().map(|point| (point[0], point[1])).unzip();

    bezier_plotter.line(x_values, y_values, "Bezier Curve", false);
    bezier_plotter.line(x_control_points.clone(), y_control_points.clone(), "Control lines", true);
    bezier_plotter.markers(x_control_points, y_control_points, "Control Points");

    bezier_plotter.plot("5th degree Bezier Curve", "results/bezier.html", false, true);
    // Bezier end

    // Both curves start
    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bspline_curve_points.iter().map(|point| (point[0], point[1])).unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bspline_control_points.iter().map(|point| (point[0], point[1])).unzip();

    both_plotter.line(x_values, y_values, "B-Spline Curve", false);
    both_plotter.line(x_control_points.clone(), y_control_points.clone(), "Control lines", true);
    both_plotter.markers(x_control_points, y_control_points, "Control Points");


    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bezier_curve_points.iter().map(|point| (point[0], point[1])).unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bezier_control_points.iter().map(|point| (point[0], point[1])).unzip();

    both_plotter.line(x_values, y_values, "Bezier Curve", false);
    both_plotter.line(x_control_points.clone(), y_control_points.clone(), "Control lines", true);
    both_plotter.markers(x_control_points, y_control_points, "Control Points");

    both_plotter.plot("Quintic Spline And Quintic Bezier", "results/bezier-and-spline.html", false, true);
    // Both curves end


    // C0 start
    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bspline_curve_points.iter().map(|point| (point[0], point[1])).unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bspline_control_points.iter().map(|point| (point[0], point[1])).unzip();

    c0_plotter.line(x_values, y_values, "B-Spline Curve", false);
    c0_plotter.line(x_control_points.clone(), y_control_points.clone(), "Control lines", true);
    c0_plotter.markers(x_control_points, y_control_points, "Control Points");

    let bezier_control_points = join::c0_continuity(&bspline_control_points, &bezier_control_points);

    let mut bezier_curve_points = Vec::new();

    for &u in &t {
        let point = bezier::bezier(&bezier_control_points, u);
        bezier_curve_points.push(point);
    }

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bezier_curve_points.iter().map(|point| (point[0], point[1])).unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bezier_control_points.iter().map(|point| (point[0], point[1])).unzip();

    c0_plotter.line(x_values, y_values, "Bezier Curve", false);
    c0_plotter.line(x_control_points.clone(), y_control_points.clone(), "Control lines", true);
    c0_plotter.markers(x_control_points, y_control_points, "Control Points");

    c0_plotter.plot("C0 continuity", "results/c0.html", false, true);
    // C0 end

    // C1 start
    let h = 2.2250738585072014e-10;
    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bspline_curve_points.iter().map(|point| (point[0], point[1])).unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bspline_control_points.iter().map(|point| (point[0], point[1])).unzip();

    c1_plotter.line(x_values, y_values, "B-Spline Curve", false);
    c1_plotter.line(x_control_points.clone(), y_control_points.clone(), "Control lines", true);
    c1_plotter.markers(x_control_points, y_control_points, "Control Points");

    let bezier_control_points = join::c1_continuity(&bspline_control_points, &bezier_control_points, degree, &knots, h);

    let mut bezier_curve_points = Vec::new();

    for &u in &t {
        let point = bezier::bezier(&bezier_control_points, u);
        bezier_curve_points.push(point);
    }

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bezier_curve_points.iter().map(|point| (point[0], point[1])).unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bezier_control_points.iter().map(|point| (point[0], point[1])).unzip();

    c1_plotter.line(x_values, y_values, "Bezier Curve", false);
    c1_plotter.line(x_control_points.clone(), y_control_points.clone(), "Control lines", true);
    c1_plotter.markers(x_control_points, y_control_points, "Control Points");

    c1_plotter.plot("C1 continuity", "results/c1.html", false, true);
    // C1 end

    // C2 start
    let h = 2.2250738585072014e-10;
    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bspline_curve_points.iter().map(|point| (point[0], point[1])).unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bspline_control_points.iter().map(|point| (point[0], point[1])).unzip();

    c2_plotter.line(x_values, y_values, "B-Spline Curve", false);
    c2_plotter.line(x_control_points.clone(), y_control_points.clone(), "Control lines", true);
    c2_plotter.markers(x_control_points, y_control_points, "Control Points");

    let bezier_control_points = join::c2_continuity(&bspline_control_points, &bezier_control_points, degree, &knots, h);

    let mut bezier_curve_points = Vec::new();

    for &u in &t {
        let point = bezier::bezier(&bezier_control_points, u);
        bezier_curve_points.push(point);
    }

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bezier_curve_points.iter().map(|point| (point[0], point[1])).unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bezier_control_points.iter().map(|point| (point[0], point[1])).unzip();

    c2_plotter.line(x_values, y_values, "Bezier Curve", false);
    c2_plotter.line(x_control_points.clone(), y_control_points.clone(), "Control lines", true);
    c2_plotter.markers(x_control_points, y_control_points, "Control Points");

    c2_plotter.plot("C2 continuity", "results/c2.html", false, true);
    // C2 end

    Ok(())
}
