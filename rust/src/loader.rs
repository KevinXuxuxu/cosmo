use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use glam::f32::Vec3;

use crate::camera::Camera;
use crate::camera::OrthoCamera;
use crate::camera::PerspectiveCamera;
use crate::engine::Thing;
use crate::engine::Triangle;
use crate::light::DirectionalLight;
use crate::light::Light;
use crate::light::PointLight;
use crate::movement::Movement;
use crate::movement::Rotate;
use crate::util::to_rad;
use crate::util::Ray;

fn parse_f32(part: &String) -> f32 {
    part.parse::<f32>().unwrap()
}

fn parse_vec3(parts: &[String]) -> Vec3 {
    Vec3::new(
        parse_f32(&parts[0]),
        parse_f32(&parts[1]),
        parse_f32(&parts[2]),
    )
}

fn parse_movement(parts: &[String]) -> Option<Box<dyn Movement>> {
    match parts[0].as_str() {
        "R" => Some(Box::new(Rotate {
            rad: to_rad(parse_f32(&parts[1])),
            axis: Ray {
                p: parse_vec3(&parts[2..5]),
                d: parse_vec3(&parts[5..8]).normalize(),
            },
        })),
        _ => None,
    }
}

fn parse_triangle(parts: &[String], points: &HashMap<String, Vec3>) -> Box<Triangle> {
    Box::new(Triangle::new(
        points.get(&parts[0]).unwrap().clone(),
        points.get(&parts[1]).unwrap().clone(),
        points.get(&parts[2]).unwrap().clone(),
        parts[3].chars().nth(0).unwrap(),
        parse_movement(&parts[4..]),
    ))
}

fn parse_camera(parts: &[String], w: usize, h: usize) -> Option<Box<dyn Camera>> {
    match parts[0].as_str() {
        "O" => {
            let d = parse_vec3(&parts[1..4]).normalize();
            let p = parse_vec3(&parts[4..7]);
            let scale = parse_f32(&parts[7]);
            Some(Box::new(OrthoCamera::new(d, p, scale, w, h)))
        }
        "P" => {
            let d = parse_vec3(&parts[1..4]).normalize();
            let p = parse_vec3(&parts[4..7]);
            let scale = parse_f32(&parts[7]);
            let f = parse_f32(&parts[8]);
            Some(Box::new(PerspectiveCamera::new(d, p, scale, f, w, h)))
        }
        _ => None,
    }
}

fn parse_light(parts: &[String]) -> Box<dyn Light> {
    match parts[0].as_str() {
        "D" => Box::new(DirectionalLight {
            d: parse_vec3(&parts[1..4]).normalize(),
            l: parse_f32(&parts[4]),
        }),
        "P" => Box::new(PointLight {
            p: parse_vec3(&parts[1..4]),
            l: parse_f32(&parts[4]),
        }),
        _ => panic!("Unknown light type: {}", parts[0].as_str()),
    }
}

pub fn parse_file(
    filename: &str,
    w: usize,
    h: usize,
) -> (Vec<Box<dyn Thing>>, Box<dyn Camera>, Vec<Box<dyn Light>>) {
    let mut points: HashMap<String, Vec3> = HashMap::new();
    let mut things: Vec<Box<dyn Thing>> = vec![];
    let file = File::open(&filename).unwrap();
    let reader = BufReader::new(file);
    let mut camera: Option<Box<dyn Camera>> = None;
    let mut lights: Vec<Box<dyn Light>> = vec![];

    for line in reader.lines() {
        let line = line.expect("fail to read line");
        let parts: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();
        match parts[0].as_str() {
            "P" => {
                points.insert(parts[1].clone(), parse_vec3(&parts[2..5]));
            }
            "T" => things.push(parse_triangle(&parts[1..], &points)),
            "C" => match camera {
                None => {
                    camera = parse_camera(&parts[1..], w, h);
                }
                _ => {}
            },
            "L" => lights.push(parse_light(&parts[1..])),
            "//" => { /* ignore comment */ }
            _ => {
                panic!("Unknown line type: {}", line);
            }
        }
    }
    (things, camera.unwrap(), lights)
}
