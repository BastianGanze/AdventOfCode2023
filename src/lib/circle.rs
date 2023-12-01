#[derive(Debug)]
pub enum CircleIntersection {
    Nope,
    Touching((f64, f64)),
    Intersecting(((f64, f64), (f64, f64))),
}

fn calc_circle_intersection(
    s1_y: f64,
    s1_x: f64,
    s1_r: f64,
    s2_y: f64,
    s2_x: f64,
    s2_r: f64,
) -> CircleIntersection {
    let d = ((s2_x - s1_x).powf(2.0) + (s2_y - s1_y).powf(2.0)).sqrt();
    if d > s1_r + s2_r || d < (s1_r - s2_r).abs() || (d == 0.0 && s1_r == s2_r) {
        return CircleIntersection::Nope;
    }

    let a = ((s1_r * s1_r) - (s2_r * s2_r) + (d * d)) / (2.0 * d);
    let h = ((s1_r * s1_r) - (a * a)).sqrt();
    let p5_y = s1_y + (a / d) * (s2_y - s1_y);
    let p5_x = s1_x + (a / d) * (s2_x - s1_x);

    if d == s1_r + s2_r {
        return CircleIntersection::Touching((p5_y, p5_x));
    }

    let p3 = (
        p5_x - ((h * (s2_y - s1_y)) / (d)),
        p5_y + ((h * (s2_x - s1_x)) / (d)),
    );

    let p4 = (
        p5_x + ((h * (s2_y - s1_y)) / (d)),
        p5_y - ((h * (s2_x - s1_x)) / (d)),
    );

    CircleIntersection::Intersecting((p3, p4))
}
