mod bspline;
mod bezier;
mod plotter;
mod join;

use ndarray::Array;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut plotter = plotter::Plotter::new();
    let mut plotter2 = plotter::Plotter::new();
    let mut plotter3 = plotter::Plotter::new();

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

    let num_points = 100000;
    let t: Vec<f64> = Array::linspace(0.0, n as f64 - degree as f64 + 2.0 - 0.00000001, num_points).into_raw_vec();
    let mut bspline_curve_points = Vec::new();

    for &u in &t {
        let point = bspline::bspline(&bspline_control_points, u, degree, &knots);
        bspline_curve_points.push(point);
    }

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bspline_curve_points.iter().map(|point| (point[0], point[1])).unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bspline_control_points.iter().map(|point| (point[0], point[1])).unzip();

    plotter.line(x_values.clone(), y_values.clone(), "B-Spline Curve", false);
    plotter2.line(x_values, y_values, "B-Spline Curve", false);
    plotter.line(x_control_points.clone(), y_control_points.clone(), "Control lines", true);
    plotter2.line(x_control_points.clone(), y_control_points.clone(), "Control lines", true);
    plotter.markers(x_control_points.clone(), y_control_points.clone(), "Control Points");
    plotter2.markers(x_control_points, y_control_points, "Control Points");

    plotter.plot("5th degree B Spline Curve", "results/bspline.html", false, true);
    // B-Spline end

    // Bezier start
    let mut plotter = plotter::Plotter::new();

    let bezier_control_points = vec![
        vec![0.0, 0.0, 0.0],
        vec![0.5, 1.5, 0.0],
        vec![1.25, 2.0, 0.0],
        vec![2.5, 1.5, 0.0],
        vec![1.5, 0.5, 0.0],
        vec![4.0, -1.5, 0.0],
    ];

    let num_points = 100000;
    let t: Vec<f64> = Array::linspace(0.0, 1.0 - 0.000000001, num_points).into_raw_vec();
    let mut bezier_curve_points = Vec::new();

    for &u in &t {
        let point = bezier::bezier(&bezier_control_points, u);
        bezier_curve_points.push(point);
    }

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bezier_curve_points.iter().map(|point| (point[0], point[1])).unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bezier_control_points.iter().map(|point| (point[0], point[1])).unzip();

    plotter.line(x_values.clone(), y_values.clone(), "Bezier Curve", false);
    plotter2.line(x_values, y_values, "Bezier Curve", false);
    plotter.line(x_control_points.clone(), y_control_points.clone(), "Control lines", true);
    plotter2.line(x_control_points.clone(), y_control_points.clone(), "Control lines", true);
    plotter.markers(x_control_points.clone(), y_control_points.clone(), "Control Points");
    plotter2.markers(x_control_points, y_control_points, "Control Points");

    plotter.plot("5th degree Bezier Curve", "results/bezier.html", false, true);
    plotter2.plot("Quintic Spline And Quintic Bezier", "results/bezier-and-spline.html", false, true);

    // Bezier end

    // C0 start
    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bspline_curve_points.iter().map(|point| (point[0], point[1])).unzip();
    plotter3.line(x_values, y_values, "B-spline curve", false);

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = join::c0_continuity(bspline_curve_points, bezier_curve_points).iter().map(|point| (point[0], point[1])).unzip();

    plotter3.line(x_values, y_values, "Bezier curve", false);
    plotter3.plot("C0 continuity", "results/c0.html", false, true);

    // C0 end
    Ok(())
}
