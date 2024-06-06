use crate::{bezier, bspline};

pub fn c0_continuity(
    first_control_points: &[Vec<f64>],
    second_control_points: &[Vec<f64>],
) -> Vec<Vec<f64>> {
    let first_curve_last_point = &first_control_points[first_control_points.len() - 1];
    let second_curve_first_point = &second_control_points[0];

    let delta = vec![
        first_curve_last_point[0] - second_curve_first_point[0],
        first_curve_last_point[1] - second_curve_first_point[1],
        first_curve_last_point[2] - second_curve_first_point[2],
    ];

    let mut new_control_points = Vec::with_capacity(second_control_points.len() - 1);

    for point in second_control_points {
        new_control_points.push(vec![
            point[0] + delta[0],
            point[1] + delta[1],
            point[2] + delta[2],
        ]);
    }

    new_control_points
}

pub fn c1_continuity(
    first_control_points: &[Vec<f64>],
    second_control_points: &[Vec<f64>],
    first_curve_degree: usize,
    first_curve_knots: &[f64],
    h: f64,
) -> ((Vec<f64>, Vec<f64>), Vec<Vec<f64>>) {
    let mut second_control_points = c0_continuity(first_control_points, second_control_points);

    let n = first_control_points.len() - 1;
    let d_s = bspline::derivative_bspline(
        first_control_points,
        first_curve_knots[n + 1] - h,
        1,
        first_curve_degree,
        first_curve_knots,
    );
    let b_0 = &second_control_points[0];
    let m = second_control_points.len() as f64 - 1.0;

    let b_1 = vec![
        d_s[0] / m + b_0[0],
        d_s[1] / m + b_0[1],
        d_s[2] / m + b_0[2],
    ];

    second_control_points[1] = b_1;

    let d_b = bezier::derivative_bezier(&second_control_points, 0.0, 1);
    ((d_s, d_b), (second_control_points))
}

pub fn c2_continuity(
    first_control_points: &[Vec<f64>],
    second_control_points: &[Vec<f64>],
    first_curve_degree: usize,
    first_curve_knots: &[f64],
    h: f64,
) -> ((Vec<f64>, Vec<f64>), Vec<Vec<f64>>) {
    let mut second_control_points = c1_continuity(
        first_control_points,
        second_control_points,
        first_curve_degree,
        first_curve_knots,
        h,
    )
    .1;

    let n = first_control_points.len() - 1;
    let m = second_control_points.len() as f64 - 1.0;
    let d_s = bspline::derivative_bspline(
        first_control_points,
        first_curve_knots[n + 1] - h,
        2,
        first_curve_degree,
        first_curve_knots,
    );
    let b_0 = &second_control_points[0];
    let b_1 = &second_control_points[1];
    let b_2 = vec![
        d_s[0] / (m * (m - 1.0)) + 2.0 * b_1[0] - b_0[0],
        d_s[1] / (m * (m - 1.0)) + 2.0 * b_1[1] - b_0[1],
        d_s[2] / (m * (m - 1.0)) + 2.0 * b_1[2] - b_0[2],
    ];

    second_control_points[2] = b_2;

    let d_b = bezier::derivative_bezier(&second_control_points, 0.0, 2);
    ((d_s, d_b), (second_control_points))
}
