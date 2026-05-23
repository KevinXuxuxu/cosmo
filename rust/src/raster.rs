use glam::{Vec2, Vec3};

use crate::camera::Camera;
use crate::engine::Thing;
use crate::light::{get_color, Light};
use crate::util::Color;

// Signed area of triangle (a, b, c) * 2. Used both as the denominator for
// barycentric weights and (one over) the cell edge function.
fn edge_fn(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

pub fn raster_frame(
    objects: &Vec<Box<dyn Thing>>,
    lights: &Vec<Box<dyn Light>>,
    camera: &dyn Camera,
    framebuffer: &mut Vec<Vec<Color>>,
    w: usize,
    h: usize,
) {
    // Clear framebuffer and depth buffer. Depth uses +infinity initially and
    // the test is "smaller z wins" (z is forward distance from the eye).
    let mut depth: Vec<Vec<f32>> = vec![vec![f32::INFINITY; w]; h];
    for row in framebuffer.iter_mut() {
        for c in row.iter_mut() {
            *c = ' ';
        }
    }

    let eye = camera.eye();
    let has_lights = !lights.is_empty();

    for obj in objects {
        let o = match obj.as_object() {
            Some(o) => o,
            None => continue,
        };
        let t = o.transform();
        for tri in o.raster_tris() {
            let (a_o, b_o, c_o, color, n_o) = *tri;

            // Object space -> world space for vertices and the face normal.
            let a_w = t.object_to_world_point(a_o);
            let b_w = t.object_to_world_point(b_o);
            let c_w = t.object_to_world_point(c_o);
            let n_w = t.object_to_world_dir(n_o);

            // Backface cull in world space: skip if the face normal does not
            // point toward the eye. For ortho this still works because `eye`
            // is in the image plane and any in-front centroid has
            // (eye - centroid) anti-parallel to forward.
            let centroid_w = (a_w + b_w + c_w) / 3.0;
            if n_w.dot(eye - centroid_w) <= 0.0 {
                continue;
            }

            // Project all three vertices. If any is at/behind the near plane,
            // skip the triangle. No clipping in v1.
            let pa = match camera.project(a_w) {
                Some(p) => p,
                None => continue,
            };
            let pb = match camera.project(b_w) {
                Some(p) => p,
                None => continue,
            };
            let pc = match camera.project(c_w) {
                Some(p) => p,
                None => continue,
            };

            let v0 = Vec2::new(pa.0, pa.1);
            let v1 = Vec2::new(pb.0, pb.1);
            let v2 = Vec2::new(pc.0, pc.1);
            let z0 = pa.2;
            let z1 = pb.2;
            let z2 = pc.2;

            let area = edge_fn(v0, v1, v2);
            if area.abs() < 1e-6 {
                continue;
            }
            let inv_area = 1.0 / area;

            // Screen-space bounding box, clamped to the framebuffer.
            let min_x = v0.x.min(v1.x).min(v2.x).floor().max(0.0) as i32;
            let max_x = (v0.x.max(v1.x).max(v2.x).ceil() as i32).min(w as i32);
            let min_y = v0.y.min(v1.y).min(v2.y).floor().max(0.0) as i32;
            let max_y = (v0.y.max(v1.y).max(v2.y).ceil() as i32).min(h as i32);

            for i in min_y..max_y {
                for j in min_x..max_x {
                    let p = Vec2::new(j as f32 + 0.5, i as f32 + 0.5);
                    // Barycentric weights. Signs of the sub-areas match the
                    // sign of `area` iff p is inside the triangle, so the
                    // divided weights all sit in [0, 1].
                    let w0 = edge_fn(v1, v2, p) * inv_area;
                    let w1 = edge_fn(v2, v0, p) * inv_area;
                    let w2 = edge_fn(v0, v1, p) * inv_area;
                    if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                        continue;
                    }

                    let z = w0 * z0 + w1 * z1 + w2 * z2;
                    let iu = i as usize;
                    let ju = j as usize;
                    if z >= depth[iu][ju] {
                        continue;
                    }
                    depth[iu][ju] = z;

                    framebuffer[iu][ju] = if has_lights {
                        let p_world = w0 * a_w + w1 * b_w + w2 * c_w;
                        // disable_shade=true skips the shadow-ray loop.
                        get_color(lights, objects, p_world, n_w, Vec3::ZERO, true)
                    } else {
                        color
                    };
                }
            }
        }
    }
}
