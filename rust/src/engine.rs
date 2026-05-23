use glam::f32::Vec3;

use crate::aabb::AABB;
use crate::bvh::Bvh;
use crate::movement::{Movement, Rotate};
use crate::util::{Color, Ray, Transform};

const NEWTON_MAX_ITER: usize = 20;

// (a, b, c) vertices, color, face normal — all in object space.
pub type RasterTri = (Vec3, Vec3, Vec3, Color, Vec3);

pub trait Updatable {
    fn update(&mut self, t: f32, dt: f32, m: Option<&Box<dyn Movement>>);
}

pub trait Visible {
    fn intersect(&self, ray: &Ray) -> Option<(Vec3, Vec3, Color)>;
    fn update_aabb(&self, aabb: &mut AABB);
    // For the rasterizer: hand back (a, b, c, color, normal) in object space.
    // Only Triangle returns Some; non-triangle primitives (Sphere, Torus) and
    // composite Objects use the default `None`.
    fn raster_tri(&self) -> Option<RasterTri> {
        None
    }
    // Downcast hook used by the rasterizer to reach Object-specific data
    // (transform, flat triangle list) without adding `Any`.
    fn as_object(&self) -> Option<&Object> {
        None
    }
}

pub trait Thing: Updatable + Visible + Sync {}

#[derive(Default)]
pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    color: Color,
    v0: Vec3,
    v1: Vec3,
    dot00: f32,
    dot01: f32,
    dot11: f32,
    inv_denom: f32,
    n: Vec3,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, color: Color) -> Self {
        let mut t = Triangle {
            a,
            b,
            c,
            color,
            ..Default::default()
        };
        t.process();
        t
    }

    fn process(&mut self) {
        self.v0 = self.c - self.a;
        self.v1 = self.b - self.a;
        self.dot00 = self.v0.dot(self.v0);
        self.dot01 = self.v0.dot(self.v1);
        self.dot11 = self.v1.dot(self.v1);
        self.inv_denom = 1. / (self.dot00 * self.dot11 - self.dot01 * self.dot01);
        self.n = self.v1.cross(self.v0).normalize();
    }

    fn contains_point(&self, p: Vec3) -> bool {
        let v2 = p - self.a;
        let dot02 = self.v0.dot(v2);
        let dot12 = self.v1.dot(v2);
        let u = (self.dot11 * dot02 - self.dot01 * dot12) * self.inv_denom;
        let v = (self.dot00 * dot12 - self.dot01 * dot02) * self.inv_denom;
        u >= 0. && v >= 0. && u + v < 1.
    }
}

impl Visible for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<(Vec3, Vec3, Color)> {
        let denom = self.n.dot(ray.d);
        if denom > -1e-6 {
            return None;
        }
        let t = self.n.dot(self.a - ray.p) / denom;
        if t < 0. {
            return None;
        }
        let p = ray.p + t * ray.d;
        if self.contains_point(p) {
            Some((p, self.n, self.color))
        } else {
            None
        }
    }

    fn update_aabb(&self, aabb: &mut AABB) {
        aabb.update(&self.a);
        aabb.update(&self.b);
        aabb.update(&self.c);
    }

    fn raster_tri(&self) -> Option<(Vec3, Vec3, Vec3, Color, Vec3)> {
        Some((self.a, self.b, self.c, self.color, self.n))
    }
}

impl Updatable for Triangle {
    fn update(&mut self, _t: f32, dt: f32, m: Option<&Box<dyn Movement>>) {
        match m {
            Some(mv) => {
                mv.update_point(dt, &mut self.a);
                mv.update_point(dt, &mut self.b);
                mv.update_point(dt, &mut self.c);
                self.process();
            }
            None => {}
        };
    }
}

unsafe impl Sync for Triangle {}

impl Thing for Triangle {}

pub struct Sphere {
    pub o: Vec3,
    pub r: f32,
    pub color: Color,
}

impl Visible for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<(Vec3, Vec3, Color)> {
        let oc = ray.p - self.o;
        let a = ray.d.dot(ray.d);
        let b = 2.0 * oc.dot(ray.d);
        let c = oc.dot(oc) - self.r * self.r;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            let t = if t1 >= 0.0 { t1 } else { t2 };

            if t >= 0.0 {
                let q = ray.p + t * ray.d;
                Some((q, (q - self.o).normalize(), self.color))
            } else {
                None
            }
        }
    }

    fn update_aabb(&self, _aabb: &mut AABB) {}
}

impl Updatable for Sphere {
    fn update(&mut self, _t: f32, dt: f32, m: Option<&Box<dyn Movement>>) {
        match m {
            Some(mv) => {
                mv.update_point(dt, &mut self.o);
            }
            None => {}
        };
    }
}

unsafe impl Sync for Sphere {}

impl Thing for Sphere {}

#[derive(Default)]
pub struct Torus {
    d: Vec3,
    p: Vec3,
    R: f32,
    r: f32,
    color: Color,
    debug: bool,
    rot: Rotate,
}

impl Torus {
    pub fn new(d: Vec3, p: Vec3, R: f32, r: f32, color: Color, debug: bool) -> Self {
        let mut t = Torus {
            d,
            p,
            R,
            r,
            color,
            debug,
            rot: Rotate {
                rad: 0.,
                axis: Ray {
                    p: Vec3::ZERO,
                    d: Vec3::ZERO,
                },
            },
        };
        t.process();
        t
    }

    fn process(&mut self) {
        self.rot = Rotate::get(self.d, Vec3::new(0., 0., 1.), Vec3::ZERO);
    }

    fn dt(&self, ray: &Ray, t: f32) -> (f32, f32) {
        let pt = ray.p + t * ray.d;
        let u = (pt.x.powi(2) + pt.y.powi(2)).sqrt();
        let f = (u - self.R).powi(2) + pt.z.powi(2) - self.r.powi(2);
        let du_dt = pt.x * ray.d.x / u + pt.y * ray.d.y / u;
        let df_dt = 2. * (u - self.R) * du_dt + 2. * pt.z * ray.d.z;
        (f, df_dt)
    }
}

impl Visible for Torus {
    fn intersect(&self, ray: &Ray) -> Option<(Vec3, Vec3, Color)> {
        // Transpose ray as if the torus is at standard position.
        let mut r_p = ray.p - self.p;
        let mut r_d = ray.d;
        self.rot.update_point(1., &mut r_p);
        self.rot.update_point(1., &mut r_d);
        let new_ray = Ray { p: r_p, d: r_d };
        // Use Newton's method to numerically solve intersection.
        let mut t: f32 = 0.01;
        let mut n: usize = 0;
        loop {
            let (f, df_dt) = self.dt(&new_ray, t);
            if df_dt.abs() <= f32::EPSILON {
                break;
            }
            let dt = f / df_dt;
            if dt.abs() < 1e-4 {
                break;
            }
            if n >= NEWTON_MAX_ITER {
                return None;
            }
            t -= dt;
            n += 1;
        }
        if t < 0. {
            return None;
        }
        let mut p = new_ray.p + t * new_ray.d;
        let mut o = self.R * p.with_z(0.).normalize();
        // Transpose intersection and normal vec back to correct position.
        self.rot.update_point(-1., &mut p);
        self.rot.update_point(-1., &mut o);
        p += self.p;
        o += self.p;
        Some((p, (p - o).normalize(), self.color))
    }

    fn update_aabb(&self, _aabb: &mut AABB) {}
}

impl Updatable for Torus {
    fn update(&mut self, _t: f32, dt: f32, m: Option<&Box<dyn Movement>>) {
        match m {
            Some(mv) => {
                mv.update_direction(dt, &mut self.d);
                mv.update_point(dt, &mut self.p);
                self.process();
            }
            None => {}
        };
    }
}

unsafe impl Sync for Torus {}

impl Thing for Torus {}

pub struct Object {
    children: Vec<Box<dyn Thing>>,
    m: Option<Box<dyn Movement>>,
    bvh: Option<Bvh>,
    transform: Transform,
    // Flat triangle list in object space for the rasterizer. Non-triangle
    // children contribute nothing. Built once at construction.
    raster_tris: Vec<(Vec3, Vec3, Vec3, Color, Vec3)>,
}

impl Object {
    pub fn new(
        children: Vec<Box<dyn Thing>>,
        m: Option<Box<dyn Movement>>,
        enable_aabb: bool,
        _debug: bool,
    ) -> Self {
        let bvh = if enable_aabb {
            Some(Bvh::build(&children))
        } else {
            None
        };
        let raster_tris: Vec<_> = children.iter().filter_map(|c| c.raster_tri()).collect();
        Object {
            children,
            m,
            bvh,
            transform: Transform::identity(),
            raster_tris,
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn raster_tris(&self) -> &[(Vec3, Vec3, Vec3, Color, Vec3)] {
        &self.raster_tris
    }
}

impl Visible for Object {
    fn intersect(&self, ray: &Ray) -> Option<(Vec3, Vec3, Color)> {
        // Transform the world-space ray into the body's object space. Both the
        // BVH and the linear-scan fallback intersect against geometry stored in
        // object space, so the input must be converted regardless of which path
        // runs. Quaternion rotation preserves direction length, so primitives
        // that assume a normalized direction (Triangle, Sphere) keep working.
        let local_ray = Ray {
            p: self.transform.world_to_object_point(ray.p),
            d: self.transform.world_to_object_dir(ray.d),
        };
        let local_hit = if let Some(bvh) = &self.bvh {
            bvh.intersect(&local_ray, &self.children)
        } else {
            // --aabb off: first-hit linear scan, pre-existing semantics.
            let mut hit = None;
            for child in &self.children {
                if let Some(rtn) = child.intersect(&local_ray) {
                    hit = Some(rtn);
                    break;
                }
            }
            hit
        };
        local_hit.map(|(p, n, c)| {
            (
                self.transform.object_to_world_point(p),
                self.transform.object_to_world_dir(n),
                c,
            )
        })
    }

    fn update_aabb(&self, _aabb: &mut AABB) {}

    fn as_object(&self) -> Option<&Object> {
        Some(self)
    }
}

impl Updatable for Object {
    fn update(&mut self, _t: f32, dt: f32, _m: Option<&Box<dyn Movement>>) {
        if let Some(mv) = &self.m {
            mv.update_transform(dt, &mut self.transform);
        }
    }
}

unsafe impl Sync for Object {}

impl Thing for Object {}
