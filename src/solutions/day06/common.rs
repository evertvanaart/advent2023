pub struct Match {
    pub time: i64,
    pub distance: i64,
}

fn solve_equation(t: f64, d: f64, sign: f64) -> f64 {
    (t + sign * (t.powi(2) - 4.0 * (d + 0.5)).sqrt()) / 2.0
}

pub fn count_winning_values(m: Match) -> i64 {
    let min: f64 = solve_equation(m.time as f64, m.distance as f64, -1.0);
    let max: f64 = solve_equation(m.time as f64, m.distance as f64,  1.0);
    max.ceil() as i64 - min.ceil() as i64
}
