mod bezier;
mod bspline;
mod join;
mod plotter;

use ndarray::Array;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bspline_plotter = plotter::Plotter::new();
    let mut bezier_plotter = plotter::Plotter::new();
    let mut both_plotter = plotter::Plotter::new();
    let mut c0_plotter = plotter::Plotter::new();
    let mut c1_plotter = plotter::Plotter::new();
    let mut c1_derivative_plotter = plotter::Plotter::new();
    let mut c1_complete_plotter = plotter::Plotter::new();
    let mut c2_plotter = plotter::Plotter::new();
    let mut c2_first_derivative_plotter = plotter::Plotter::new();
    let mut c2_second_derivative_plotter = plotter::Plotter::new();
    let mut c2_complete_plotter = plotter::Plotter::new();
    let num_points = 100000;

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

    let bspline_lin_space: Vec<f64> =
        Array::linspace(knots[degree - 1], knots[n + 1] - 1e-10, num_points).into_raw_vec();
    let mut bspline_curve_points = Vec::new();

    for &u in &bspline_lin_space {
        let point = bspline::bspline(&bspline_control_points, u, degree, &knots);
        bspline_curve_points.push(point);
    }

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bspline_curve_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bspline_control_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();

    bspline_plotter.line(x_values.clone(), y_values.clone(), "B-Spline Curve", false);
    bspline_plotter.line(
        x_control_points.clone(),
        y_control_points.clone(),
        "Control Polygon",
        true,
    );
    bspline_plotter.markers(
        x_control_points.clone(),
        y_control_points.clone(),
        "Control Points",
    );

    bspline_plotter.plot(
        "5th degree B Spline Curve",
        "results/bspline/bspline",
        false,
        true,
    );
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

    let bezier_lin_space: Vec<f64> =
        Array::linspace(0.0, 1.0 - 0.000000001, num_points).into_raw_vec();
    let mut bezier_curve_points = Vec::new();

    for &u in &bezier_lin_space {
        let point = bezier::bezier(&bezier_control_points, u);
        bezier_curve_points.push(point);
    }

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bezier_curve_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bezier_control_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();

    bezier_plotter.line(x_values, y_values, "Bezier Curve", false);
    bezier_plotter.line(
        x_control_points.clone(),
        y_control_points.clone(),
        "Control Polygon",
        true,
    );
    bezier_plotter.markers(x_control_points, y_control_points, "Control Points");

    bezier_plotter.plot(
        "5th degree Bezier Curve",
        "results/bezier/bezier",
        false,
        true,
    );
    // Bezier end

    // Both curves start
    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bspline_curve_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bspline_control_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();

    both_plotter.line(x_values, y_values, "B-Spline Curve", false);
    both_plotter.line(
        x_control_points.clone(),
        y_control_points.clone(),
        "Control Polygon",
        true,
    );
    both_plotter.markers(x_control_points, y_control_points, "Control Points");

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bezier_curve_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bezier_control_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();

    both_plotter.line(x_values, y_values, "Bezier Curve", false);
    both_plotter.line(
        x_control_points.clone(),
        y_control_points.clone(),
        "Control Polygon",
        true,
    );
    both_plotter.markers(x_control_points, y_control_points, "Control Points");

    both_plotter.plot(
        "Quintic Spline And Quintic Bezier",
        "results/both-curves/bezier-and-spline",
        false,
        true,
    );
    // Both curves end

    // C0 start
    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bspline_curve_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bspline_control_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();

    c0_plotter.line(x_values, y_values, "B-Spline Curve", false);
    c0_plotter.line(
        x_control_points.clone(),
        y_control_points.clone(),
        "Control Polygon",
        true,
    );
    c0_plotter.markers(x_control_points, y_control_points, "Control Points");

    let bezier_control_points =
        join::c0_continuity(&bspline_control_points, &bezier_control_points);

    let mut bezier_curve_points = Vec::new();

    for &u in &bezier_lin_space {
        let point = bezier::bezier(&bezier_control_points, u);
        bezier_curve_points.push(point);
    }

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bezier_curve_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bezier_control_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();

    c0_plotter.line(x_values, y_values, "Bezier Curve", false);
    c0_plotter.line(
        x_control_points.clone(),
        y_control_points.clone(),
        "Control Polygon",
        true,
    );
    c0_plotter.markers(x_control_points, y_control_points, "Control Points");

    c0_plotter.plot("C0 continuity", "results/c0/c0", false, true);
    // C0 end

    // C1 start
    let h = 2.2250738585072014e-10;
    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bspline_curve_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bspline_control_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();

    c1_plotter.line(x_values.clone(), y_values.clone(), "B-Spline Curve", false);
    c1_complete_plotter.line(x_values, y_values, "B-Spline Curve", false);
    
    c1_plotter.line(
        x_control_points.clone(),
        y_control_points.clone(),
        "Control Polygon",
        true,
    );
    c1_complete_plotter.line(
        x_control_points.clone(),
        y_control_points.clone(),
        "Control Polygon",
        true,
    );

    c1_plotter.markers(x_control_points.clone(), y_control_points.clone(), "Control Points");
    c1_complete_plotter.markers(x_control_points, y_control_points, "Control Points");

    let mut bspline_derivative_curve_points = Vec::new();

    for &u in &bspline_lin_space {
        let point = bspline::derivative_bspline(&bspline_control_points, u, 1, degree, &knots);
        bspline_derivative_curve_points.push(point);
    }

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bspline_derivative_curve_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();
    c1_complete_plotter.line(x_values.clone(), y_values.clone(), "B-Spline First Derivative", false);
    c1_derivative_plotter.line(x_values, y_values, "B-Spline First Derivative", false);

    let bezier_control_points = join::c1_continuity(
        &bspline_control_points,
        &bezier_control_points,
        degree,
        &knots,
        h,
    );

    let mut bezier_curve_points = Vec::new();

    for &u in &bezier_lin_space {
        let point = bezier::bezier(&bezier_control_points, u);
        bezier_curve_points.push(point);
    }

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bezier_curve_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bezier_control_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();

    c1_plotter.line(x_values.clone(), y_values.clone(), "Bezier Curve", false);
    c1_complete_plotter.line(x_values, y_values, "Bezier Curve", false);

    c1_plotter.line(
        x_control_points.clone(),
        y_control_points.clone(),
        "Control Polygon",
        true,
    );
    c1_complete_plotter.line(
        x_control_points.clone(),
        y_control_points.clone(),
        "Control Polygon",
        true,
    );

    c1_plotter.markers(x_control_points.clone(), y_control_points.clone(), "Control Points");
    c1_complete_plotter.markers(x_control_points, y_control_points, "Control Points");

    let mut bezier_derivative_curve_points = Vec::new();

    for &u in &bezier_lin_space {
        let point = bezier::derivative_bezier(&bezier_control_points, u, 1);
        bezier_derivative_curve_points.push(point);
    }

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bezier_derivative_curve_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();
    c1_complete_plotter.line(x_values.clone(), y_values.clone(), "Bezier First Derivative", false);
    c1_derivative_plotter.line(x_values, y_values, "Bezier First Derivative", false);

    c1_plotter.plot("C1 continuity", "results/c1/c1", false, true);
    c1_derivative_plotter.plot("C1 continuity", "results/c1/c1-derivative", false, true);
    c1_complete_plotter.plot("C1 continuity", "results/c1/c1-complete", false, true);
    // C1 end

    // C2 start
    let h = 2.2250738585072014e-10;
    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bspline_curve_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bspline_control_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();

    c2_plotter.line(x_values.clone(), y_values.clone(), "B-Spline Curve", false);
    c2_complete_plotter.line(x_values, y_values, "B-Spline Curve", false);

    c2_plotter.line(
        x_control_points.clone(),
        y_control_points.clone(),
        "Control Polygon",
        true,
    );
    c2_complete_plotter.line(
        x_control_points.clone(),
        y_control_points.clone(),
        "Control Polygon",
        true,
    );

    c2_plotter.markers(x_control_points.clone(), y_control_points.clone(), "Control Points");
    c2_complete_plotter.markers(x_control_points, y_control_points, "Control Points");

    let mut bspline_derivative_curve_points = Vec::new();

    for &u in &bspline_lin_space {
        let point = bspline::derivative_bspline(&bspline_control_points, u, 1, degree, &knots);
        bspline_derivative_curve_points.push(point);
    }

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bspline_derivative_curve_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();
    c2_complete_plotter.line(x_values.clone(), y_values.clone(), "B-Spline First Derivative", false);
    c2_first_derivative_plotter.line(x_values, y_values, "B-Spline First Derivative", false);

    let mut bspline_derivative_curve_points = Vec::new();

    for &u in &bspline_lin_space {
        let point = bspline::derivative_bspline(&bspline_control_points, u, 2, degree, &knots);
        bspline_derivative_curve_points.push(point);
    }

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bspline_derivative_curve_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();
    c2_complete_plotter.line(x_values.clone(), y_values.clone(), "B-Spline Second Derivative", false);
    c2_second_derivative_plotter.line(x_values, y_values, "B-Spline Second Derivative", false);

    let bezier_control_points = join::c2_continuity(
        &bspline_control_points,
        &bezier_control_points,
        degree,
        &knots,
        h,
    );

    let mut bezier_curve_points = Vec::new();

    for &u in &bezier_lin_space {
        let point = bezier::bezier(&bezier_control_points, u);
        bezier_curve_points.push(point);
    }

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bezier_curve_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();
    let (x_control_points, y_control_points): (Vec<f64>, Vec<f64>) = bezier_control_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();

    c2_plotter.line(x_values.clone(), y_values.clone(), "Bezier Curve", false);
    c2_complete_plotter.line(x_values, y_values, "Bezier Curve", false);

    c2_plotter.line(
        x_control_points.clone(),
        y_control_points.clone(),
        "Control Polygon",
        true,
    );
    c2_complete_plotter.line(
        x_control_points.clone(),
        y_control_points.clone(),
        "Control Polygon",
        true,
    );
    
    c2_plotter.markers(x_control_points.clone(), y_control_points.clone(), "Control Points");
    c2_complete_plotter.markers(x_control_points, y_control_points, "Control Points");

    let mut bezier_derivative_curve_points = Vec::new();

    for &u in &bezier_lin_space {
        let point = bezier::derivative_bezier(&bezier_control_points, u, 1);
        bezier_derivative_curve_points.push(point);
    }

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bezier_derivative_curve_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();
    c2_complete_plotter.line(x_values.clone(), y_values.clone(), "Bezier First Derivative", false);
    c2_first_derivative_plotter.line(x_values, y_values, "Bezier First Derivative", false);

    let mut bezier_derivative_curve_points = Vec::new();

    for &u in &bezier_lin_space {
        let point = bezier::derivative_bezier(&bezier_control_points, u, 2);
        bezier_derivative_curve_points.push(point);
    }

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = bezier_derivative_curve_points
        .iter()
        .map(|point| (point[0], point[1]))
        .unzip();
    c2_complete_plotter.line(x_values.clone(), y_values.clone(), "Bezier Second Derivative", false);
    c2_second_derivative_plotter.line(x_values, y_values, "Bezier Second Derivative", false);

    c2_plotter.plot("C2 continuity", "results/c2/c2", false, true);
    c2_first_derivative_plotter.plot("C2 continuity", "results/c2/c2-first-derivative", false, true);
    c2_second_derivative_plotter.plot("C2 continuity", "results/c2/c2-second-derivative", false, true);
    c2_complete_plotter.plot("C2 continuity", "results/c2/c2-complete", false, true);
    // C2 end

    Ok(())
}
