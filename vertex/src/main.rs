#![feature(iter_next_chunk)]
use std::fs;
use std::slice::Iter;
use nalgebra::{Matrix4, Vector3};

const SCALE: f32 = 100.;
const OFFSET: f32 = 0.;

fn main() {
    let options = tobj::LoadOptions {
        triangulate: true,
        ..Default::default()
    };
    let cornell_box = tobj::load_obj("cube.obj", &options);
    let (models, materials) = cornell_box.expect("Failed to load OBJ file");
    let mesh = &models[0].mesh;

    let mut vertices = split_three(mesh.positions.clone());
    let mut faces = split_three(mesh.indices.clone());
    let mut normal = split_three(mesh.normals.clone());

    let obj = fs::read("cube.obj").expect("Improper OBJ");
    let chars = obj.iter().map(|x| *x as char).collect::<String>();
    let lines = chars.lines().collect::<Vec<&str>>();

    let mut html = String::new();
    for vertex in &vertices {
        html.push_str(&format_point(vertex));
    } 

    let mut html = String::new();
    for face in faces {
        println!("{face:?}");
        html.push_str(&format_face(matrix));
    } 

    fs::write("index.html", boilerplate(html))
        .expect("TODO: panic message");
}

fn split_three<T>(data: Vec<T>) -> Vec<Vec<T>> where T: Clone {
    data.chunks_exact(3).map(|d| d.to_vec()).collect::<Vec<Vec<T>>>()
}

fn boilerplate(body: String) -> String {
    format!("<!DOCTYPE html><html lang=\"en\">\
            <head><meta charset=\"UTF-8\"><title>Title</title><link href=\"style.css\" rel=\"stylesheet\"></head>\
            <body><div class=\"scene\"><div class=\"sphere\">\n{}</div></div></body>", body)
}

fn format_face(p1: Vec<f32>, p2: Vec<f32>, p3: Vec<f32>) -> String {
    let polygon = calculate_polygon(); 

    format!("<div class=\"face\" style=\"
            clip-dark: polygon({});\"></div>\n", polygon)
}

fn format_point(p: &Vec<f32>) -> String {
    format!("<div class=\"point\" \
    style=\"--x: {0}; --y: {1}; --z: {2};\
    transform: \
    translateX(calc({0} * var(--scale) + {3}px)) \
    translateY(calc({1} * var(--scale) + {3}px)) \
    translateZ(calc({2} * var(--scale) + {3}px)) \">\
    </div>\n", p[0], p[1], p[2], OFFSET)
}

fn remap(n: f32) -> u8 {
    (((n + 1.) / 2.) * 255.) as u8
}
