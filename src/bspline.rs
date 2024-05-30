pub fn basis_function(u: f64, i: usize, degree: usize, knots: &[f64]) -> f64 {
    if degree == 1 {
        if knots[i] <= u && u < knots[i + 1] {
            return 1.0;
        } else {
            return 0.0;
        }
    }

    let first_term = if knots[i + degree - 1] - knots[i] != 0.0 {
        (u - knots[i]) * basis_function(u, i, degree - 1, knots)
            / (knots[i + degree - 1] - knots[i])
    } else {
        0.0
    };

    let second_term = if knots[i + degree] - knots[i + 1] != 0.0 {
        (knots[i + degree] - u) * basis_function(u, i + 1, degree - 1, knots)
            / (knots[i + degree] - knots[i + 1])
    } else {
        0.0
    };

    first_term + second_term
}

pub fn bspline(control_points: &[Vec<f64>], u: f64, degree: usize, knots: &[f64]) -> Vec<f64> {
    let mut p = vec![0.0; 3];
    let n = control_points.len() - 1;
    for i in 0..=n {
        for j in 0..3 {
            p[j] += control_points[i][j] * basis_function(u, i, degree, knots);
        }
    }
    p
}

pub fn generate_knot_vector(n: usize, degree: usize) -> Vec<f64> {
    let mut t = Vec::with_capacity(n + degree + 1);
    for j in 0..=n + degree {
        if j < degree {
            t.push(0.0);
        } else if degree <= j && j <= n {
            t.push((j - degree + 1) as f64);
        } else {
            t.push((n - degree + 2) as f64);
        }
    }

    t
}

pub fn dk_bspline(u: f64, i: usize, degree: usize, knots: &[f64], k: usize) -> f64 {
    if k == 0 {
        return basis_function(u, i, degree, knots);
    }

    let first_term = if i + degree < knots.len() && knots[i + degree] - knots[i] != 0.0 {
        (degree as f64) / (knots[i + degree] - knots[i])
            * dk_bspline(u, i, degree - 1, knots, k - 1)
    } else {
        0.0
    };

    let second_term = if i + degree + 1 < knots.len() && knots[i + degree + 1] - knots[i + 1] != 0.0
    {
        (degree as f64) / (knots[i + degree + 1] - knots[i + 1])
            * dk_bspline(u, i + 1, degree - 1, knots, k - 1)
    } else {
        0.0
    };

    first_term - second_term
}

pub fn derivative_bspline(
    points: &[Vec<f64>],
    u: f64,
    k: usize,
    degree: usize,
    knots: &[f64],
) -> Vec<f64> {
    let dim = points[0].len();
    let mut p = vec![0.0; dim];

    for (i, point) in points.iter().enumerate() {
        let dk_b = dk_bspline(u, i, degree, knots, k);
        for (j, coord) in point.iter().enumerate() {
            p[j] += coord * dk_b;
        }
    }

    p
}
