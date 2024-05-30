fn fact(number: u128) -> u128 {
    (1..=number).product()
}

fn powf(num1: f64, num2: u128) -> f64 {
    num1.powi(num2 as i32)
}

fn lerp(p0: &[f64], p1: &[f64], u: f64) -> Vec<f64> {
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
    let mut interpolated_points = points.to_vec();

    for i in (0..interpolated_points.len() - 1).rev() {
        for j in 0..=i {
            interpolated_points[j] = lerp(&interpolated_points[j], &interpolated_points[j + 1], u);
        }
    }
    interpolated_points[0].clone()
}

pub fn dk_bezier(u: f64, i: i128, n: u128, k: usize) -> f64 {
    if i > n as i128 || i < 0 {
        return 0.0;
    }

    if k == 0 {
        return fact(n) as f64 / (fact(i as u128) as f64 * fact((n as i128 - i) as u128) as f64)
            * powf(u, i as u128)
            * powf(1.0 - u, n - i as u128);
    }

    let first_term = dk_bezier(u, i - 1, n - 1, k - 1);
    let second_term = dk_bezier(u, i, n - 1, k - 1);

    n as f64 * (first_term - second_term)
}

pub fn derivative_bezier(points: &[Vec<f64>], u: f64, k: usize) -> Vec<f64> {
    let n = points.len() - 1;
    let mut p = vec![0.0; points[0].len()];

    for (i, point) in points.iter().enumerate() {
        let dk_b = dk_bezier(u, i as i128, n as u128, k);
        for (j, coord) in point.iter().enumerate() {
            p[j] += coord * dk_b;
        }
    }

    p
}
