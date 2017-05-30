use itertools::Itertools;

enum Gradient {
    Finite(f32),
    Infinity,
    MinusInfinity
}

pub fn triangulate_lines(points: &[(f32, f32)], width: f32) -> (Vec<[f32; 2]>, Vec<u16>) {
    // This isn't a tight bound, but it's only off by a constant
    let vertices = Vec::with_capacity(points.len() * 2);
    let indices = Vec::with_capacity(points.len() * 2 * 3);
    let len = points.len();

    // special cases (if 0 or 1 do nothing)
    if len == 2 {

    } else if len > 2 {
        let mut points_iter = points.iter();
        let first_point = points_iter.next();
    }
    (vertices, indices)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangulate_lines() {

        let input = [(0.0, 0.0), (0.1, 0.3), (1.0, 0.2)];

    }
}
