use std::fs;

const SCALE: f32 = 100.;
const OFFSET: f32 = 0.;
struct Vector3 (f32, f32, f32);

fn main() {
    let obj = fs::read("fish.obj").expect("Improper OBJ");
    let chars = obj.iter().map(|x| *x as char).collect::<String>();
    let lines = chars.lines().collect::<Vec<&str>>();

    let mut html = String::new();
    let mut vertices: Vec<Vector3> = Vec::new();
    let mut faces: Vec<Vec<u8>> = Vec::new();

    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() < 1 { continue; }

        match parts[0] {
            "v" => html.push_str(&point(parts)),
            "f" => html.push_str(&face(parts)),
            &_ => {}
        }
    }

    let data_input = "<style style=\"display: block;whitespace: pre;\" contenteditable=\"true\"></style>";

    fs::write("index.html", boilerplate(html))
        .expect("TODO: panic message");
}

fn boilerplate(body: String) -> String {
    format!("<!DOCTYPE html><html lang=\"en\">\
            <head><meta charset=\"UTF-8\"><title>Title</title><link href=\"style.css\" rel=\"stylesheet\"></head>\
            <body><div class=\"scene\"><div class=\"sphere\">\n{}</div></div></body>", body)
}

fn vertex_from_str(s: Vec<&str>) -> Vector3 {
    Vector3(0., 4., 5.)
}

fn face(parts: Vec<&str>) -> String {
    let x = parts[1].chars().nth(0).unwrap() as u8;
    let y = parts[2].chars().nth(0).unwrap() as u8;
    let z = parts[3].chars().nth(0).unwrap() as u8;
    let w = parts[4].chars().nth(0).unwrap() as u8;

    format!("<div class=\"face\" style=\"
    transform: matrix3d(100, 0, 0, 0,
                        0, 100, 0, 0,
                        0, 0, 100, 0,
                        0, 0, 0, 100);
    \"></div>\n")
}

fn point(parts: Vec<&str>) -> String {

    let x = parts[1].parse::<f32>().unwrap();
    let y = parts[2].parse::<f32>().unwrap();
    let z = parts[3].parse::<f32>().unwrap();

    format!("<div class=\"point\" style=\"transform: \
    rotateY(90deg)
    translateX(calc({0} * var(--scale) + {3}px)) \
    translateY(calc({1} * var(--scale) + {3}px))\
    translateZ(calc({2} * var(--scale) + {3}px))\">\
    🐟</div>\n",
            x, y, z, OFFSET)
}

fn remap(n: f32) -> u8 {
    (((n + 1.) / 2.) * 255.) as u8
}