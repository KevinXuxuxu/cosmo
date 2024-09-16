pub mod camera;
pub mod light;
pub mod player;
pub mod triangle;
pub mod util;

use glam::Vec3;

use crate::camera::Camera;
use crate::light::Light;
use crate::player::Player;
use crate::triangle::Triangle;

const DEGREE_90: f32 = std::f32::consts::PI / 2.;

fn main() {
    let A = Vec3::new(0., 0., 8.660254);
    let B = Vec3::new(0., 0., -8.660254);
    let C = Vec3::new(8.164965, 0., 2.886751);
    let D = Vec3::new(-4.082483, 7.071067, 2.886751);
    let E = Vec3::new(-4.082483, -7.071067, 2.886751);
    let F = Vec3::new(4.082483, 7.071067, -2.886751);
    let G = Vec3::new(-8.164965, 0., -2.886751);
    let H = Vec3::new(4.082483, -7.071067, -2.886751);
    let triangles: Vec<Triangle> = vec![
        Triangle::new(A, C, D, DEGREE_90),
        Triangle::new(C, F, D, DEGREE_90),
        Triangle::new(A, D, E, DEGREE_90),
        Triangle::new(D, G, E, DEGREE_90),
        Triangle::new(A, E, C, DEGREE_90),
        Triangle::new(E, H, C, DEGREE_90),
        Triangle::new(D, F, G, DEGREE_90),
        Triangle::new(F, B, G, DEGREE_90),
        Triangle::new(C, H, F, DEGREE_90),
        Triangle::new(H, B, F, DEGREE_90),
        Triangle::new(E, G, H, DEGREE_90),
        Triangle::new(G, B, H, DEGREE_90),
    ];

    let camera = Camera::new(80, 40, 0.5);
    let light = Light {
        d: Vec3::new(-1., -0.5, -1.).normalize(),
        intensity: 0.8,
    };
    let mut player = Player::new(80, 40, 24, triangles, camera, light);
    player.play(10.);
}
