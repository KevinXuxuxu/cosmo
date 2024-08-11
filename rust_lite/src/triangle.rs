use glam::Vec3;

struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    n: Vec3,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        let n = (b - a).cross(c - a).normalize();
        Triangle { a, b, c, n }
    }

    pub fn intersect(&self, p: Vec3, d: Vec3) -> Option<Vec3> {
        let n_dot_d = self.n.dot(d);
        if n_dot_d >= 0. {
            // Check for positive side
            return None;
        }
        // Solve for Q
        let t = self.n.dot(self.a - p) / n_dot_d;
        let q = p + t * d;
        // Check if Q is in triangle
        if (self.b - self.a).cross(q - self.a).dot(self.n) < 0.
            || (self.c - self.b).cross(q - self.b).dot(self.n) < 0.
            || (self.a - self.c).cross(q - self.c).dot(self.n) < 0.
        {
            return None;
        }
        Some(q)
    }
}
