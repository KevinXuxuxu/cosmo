use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use glam::f32::Vec3;

use crate::engine::Thing;
use crate::engine::Triangle;
use crate::movement::Movement;
use crate::movement::Rotate;
use crate::util::Ray;

fn parse_vec3(parts: &[String]) -> Vec3 {
    Vec3::new(
        parts[0].parse::<f32>().unwrap(),
        parts[1].parse::<f32>().unwrap(),
        parts[2].parse::<f32>().unwrap(),
    )
}

fn parse_movement(parts: &[String]) -> Option<Box<dyn Movement>> {
    match parts[0].as_str() {
        "R" => Some(Box::new(Rotate::new(
            parts[1].parse::<f32>().unwrap(),
            Ray {
                p: parse_vec3(&parts[2..5]),
                d: parse_vec3(&parts[5..8]).normalize(),
            },
        ))),
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

pub fn parse_file(filename: &str) -> Vec<Box<dyn Thing>> {
    let mut points: HashMap<String, Vec3> = HashMap::new();
    let mut result: Vec<Box<dyn Thing>> = vec![];
    let file = File::open(&filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("fail to read line");
        let parts: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();
        match parts[0].as_str() {
            "P" => {
                points.insert(parts[1].clone(), parse_vec3(&parts[2..5]));
            }
            "T" => result.push(parse_triangle(&parts[1..], &points)),
            _ => {
                panic!("Unknown line type: {}", line);
            }
        }
    }
    result
}
