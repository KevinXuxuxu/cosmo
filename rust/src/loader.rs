use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use glam::f32::Vec3;

use crate::engine::Triangle;
use crate::engine::Visible;

pub fn parse_file(filename: &str) -> Vec<Box<dyn Visible>> {
    let mut points: HashMap<String, Vec3> = HashMap::new();
    let mut result: Vec<Box<dyn Visible>> = vec![];
    let file = File::open(&filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("fail to read line");
        let parts: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();
        match parts[0].as_str() {
            "P" => {
                points.insert(
                    parts[1].clone(),
                    Vec3::new(
                        parts[2].parse::<f32>().unwrap(),
                        parts[3].parse::<f32>().unwrap(),
                        parts[4].parse::<f32>().unwrap(),
                    ),
                );
            }
            "T" => {
                let t = Triangle::new(
                    points.get(&parts[1]).unwrap().clone(),
                    points.get(&parts[2]).unwrap().clone(),
                    points.get(&parts[3]).unwrap().clone(),
                    parts[4].chars().nth(0).unwrap(),
                );
                result.push(Box::new(t));
            }
            _ => {
                panic!("Unknown line type: {}", line);
            }
        }
    }
    result
}
