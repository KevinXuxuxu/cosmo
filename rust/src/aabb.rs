use glam::{Vec2, Vec3};

use crate::movement::Movement;
use crate::util::{get_sphere_cord, vec2_cmp, Ray};

/// Function to find the orientation of the triplet (p, q, r).
/// Returns:
/// - 0 if p, q and r are collinear
/// - 1 if Clockwise
/// - 2 if Counterclockwise
fn orientation(p: &Vec2, q: &Vec2, r: &Vec2) -> i32 {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
    if val == 0.0 {
        0
    } else if val > 0.0 {
        1
    } else {
        2
    }
}

/// Function to compute the convex hull of a set of 2D points.
fn convex_hull(mut points: Vec<Vec2>) -> Vec<Vec2> {
    // Sort the points lexicographically (by x, then by y)
    points.sort_by(vec2_cmp);

    // Build the lower hull
    let mut lower = Vec::new();
    for p in &points {
        while lower.len() >= 2
            && orientation(&lower[lower.len() - 2], &lower[lower.len() - 1], &p) != 2
        {
            lower.pop();
        }
        lower.push(*p);
    }

    // Build the upper hull
    let mut upper = Vec::new();
    for p in points.iter().rev() {
        while upper.len() >= 2
            && orientation(&upper[upper.len() - 2], &upper[upper.len() - 1], &p) != 2
        {
            upper.pop();
        }
        upper.push(*p);
    }

    // Remove the last point of each half because it is repeated at the beginning of the other half
    lower.pop();
    upper.pop();

    // Concatenate lower and upper hull to get the full hull
    lower.extend(upper);
    lower
}

fn in_convex_hull(points: &Vec<Vec2>, sd: &Vec2) -> bool {
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        if orientation(&points[i], sd, &points[j]) == 2 {
            return false;
        }
    }
    true
}

// Axis-aligned Bounding Box
#[derive(Default)]
pub struct AABB {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
}

// TODO: find better impl for bounding box, now too slow
impl AABB {
    pub fn new() -> Self {
        AABB {
            x: vec![f32::MAX, f32::MIN],
            y: vec![f32::MAX, f32::MIN],
            z: vec![f32::MAX, f32::MIN],
        }
    }

    pub fn clear(&mut self) {
        self.x = vec![f32::MAX, f32::MIN];
        self.y = vec![f32::MAX, f32::MIN];
        self.z = vec![f32::MAX, f32::MIN];
    }

    pub fn update(&mut self, p: &Vec3) {
        self.x[0] = f32::min(p.x, self.x[0]);
        self.x[1] = f32::max(p.x, self.x[1]);
        self.y[0] = f32::min(p.y, self.y[0]);
        self.y[1] = f32::max(p.y, self.y[1]);
        self.z[0] = f32::min(p.z, self.z[0]);
        self.z[1] = f32::max(p.z, self.z[1]);
    }

    pub fn intersect(&self, ray: &Ray) -> bool {
        let mut sps: Vec<Vec2> = vec![];
        for x in &self.x {
            for y in &self.y {
                for z in &self.z {
                    let cor = Vec3::new(*x, *y, *z);
                    let d = (cor - ray.p).normalize();
                    let sd = get_sphere_cord(&d);
                    sps.push(sd);
                }
            }
        }
        let ch = convex_hull(sps);
        let sd = get_sphere_cord(&ray.d);
        in_convex_hull(&ch, &sd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_orientation() {
        let a = Vec2::new(0., 0.);
        let b = Vec2::new(1., 0.);
        let c = Vec2::new(2., 0.);
        let d = Vec2::new(1., 1.);
        assert_eq!(orientation(&a, &b, &c), 0);
        assert_eq!(orientation(&a, &b, &d), 2);
        assert_eq!(orientation(&d, &b, &a), 1);
    }

    #[test]
    fn test_fn_convex_hull() {
        let mut ps_1: Vec<Vec2> = vec![
            Vec2::new(1., 1.),
            Vec2::new(5., 1.),
            Vec2::new(2., 2.),
            Vec2::new(3., 2.),
            Vec2::new(4., 3.),
            Vec2::new(3., 5.),
        ];
        let res_1 = convex_hull(ps_1);
        assert_eq!(res_1.len(), 3);
        assert_eq!(
            res_1,
            vec![Vec2::new(1., 1.), Vec2::new(5., 1.), Vec2::new(3., 5.)]
        );

        let mut ps_2: Vec<Vec2> = vec![
            Vec2::new(3., 1.),
            Vec2::new(6., 1.),
            Vec2::new(1., 2.),
            Vec2::new(4., 2.),
            Vec2::new(4., 5.),
            Vec2::new(7., 5.),
            Vec2::new(2., 6.),
            Vec2::new(5., 6.),
        ];
        let res_2 = convex_hull(ps_2);
        assert_eq!(res_2.len(), 6);
        assert_eq!(
            res_2,
            vec![
                Vec2::new(1.0, 2.0),
                Vec2::new(3.0, 1.0),
                Vec2::new(6.0, 1.0),
                Vec2::new(7.0, 5.0),
                Vec2::new(5.0, 6.0),
                Vec2::new(2.0, 6.0)
            ]
        );
    }

    #[test]
    fn test_fn_in_convex_hull() {
        let ch = vec![
            Vec2::new(1.0, 2.0),
            Vec2::new(3.0, 1.0),
            Vec2::new(6.0, 1.0),
            Vec2::new(7.0, 5.0),
            Vec2::new(5.0, 6.0),
            Vec2::new(2.0, 6.0),
        ];
        assert!(!in_convex_hull(&ch, &Vec2::new(1., 1.)));
        assert!(!in_convex_hull(&ch, &Vec2::new(1., 4.)));
        assert!(in_convex_hull(&ch, &Vec2::new(2., 2.)));
        assert!(in_convex_hull(&ch, &Vec2::new(4., 4.)));
        assert!(in_convex_hull(&ch, &Vec2::new(5., 1.)));
        assert!(!in_convex_hull(&ch, &Vec2::new(6., 6.)));
        assert!(in_convex_hull(&ch, &Vec2::new(7., 5.)));
    }
}
