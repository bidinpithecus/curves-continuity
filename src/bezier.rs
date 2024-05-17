pub fn lerp(p0: &Vec<f64>, p1: &Vec<f64>, u: f64) -> Vec<f64> {
    assert_eq!(p0.len(), p1.len());
    let dimension = p0.len();

    assert!(dimension > 0);

    let mut interpolated_point = Vec::with_capacity(dimension);
    for i in 0..dimension {
        interpolated_point.push((1.0 - u) * p0[i] + u * p1[i]);
    }

    interpolated_point
}

pub fn bezier(points: &[Vec<f64>], u: f64) -> Vec<f64> {
    let mut interpolated_points = points.iter().cloned().map(|v| v.into_iter().collect::<Vec<f64>>()).collect::<Vec<_>>();

    let n = interpolated_points.len() - 1;
    for i in (0..n).rev() {
        for j in 0..=i {
            let point = lerp(&interpolated_points[j], &interpolated_points[j + 1], u);
            interpolated_points[j] = point.into_iter().collect::<Vec<f64>>();
        }
    }
    interpolated_points[0].clone()
}
