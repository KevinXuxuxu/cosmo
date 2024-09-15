pub mod camera;
pub mod player;
pub mod triangle;
pub mod util;

use glam::Vec3;

use crate::camera::Camera;
use crate::player::Player;
use crate::triangle::Triangle;

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
        Triangle::new(A, C, D),
        Triangle::new(C, F, D),
        Triangle::new(A, D, E),
        Triangle::new(D, G, E),
        Triangle::new(A, E, C),
        Triangle::new(E, H, C),
        Triangle::new(D, F, G),
        Triangle::new(F, B, G),
        Triangle::new(C, H, F),
        Triangle::new(H, B, F),
        Triangle::new(E, G, H),
        Triangle::new(G, B, H),
    ];

    let camera = Camera::new(80, 40, 0.5);
    let mut player = Player::new(80, 40, 24, triangles, camera);
    player.play(10.);
}
