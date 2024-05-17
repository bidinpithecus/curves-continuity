pub fn c0_continuity(first_curve_points: Vec<Vec<f64>>, second_curve_points: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let first_curve_last_point = &first_curve_points[first_curve_points.len() - 1];
    let second_curve_first_point = &second_curve_points[0];

    let delta = vec![
        first_curve_last_point[0] - second_curve_first_point[0],
        first_curve_last_point[1] - second_curve_first_point[1],
        first_curve_last_point[2] - second_curve_first_point[2]
    ];

    let mut joined_points = Vec::with_capacity(second_curve_points.len() - 1);

    for point in second_curve_points {
        joined_points.push(vec![
            point[0] + delta[0],
            point[1] + delta[1],
            point[2] + delta[2]
        ]);
    }

    joined_points
}
