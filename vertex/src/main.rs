use std::fs;
use nalgebra::{Matrix4, Point3};

const SCALE: f32 = 100.;
const OFFSET: f32 = 0.;
type Vector3 = Point3<f32>;

fn main() {
    let cornell_box = tobj::load_obj("cube.obj", &tobj::GPU_LOAD_OPTIONS);
    let (models, materials) = cornell_box.expect("Failed to load OBJ file");
    let mesh = &models[0].mesh;
    println!("{:?}", mesh.positions);

    let obj = fs::read("cube.obj").expect("Improper OBJ");
    let chars = obj.iter().map(|x| *x as char).collect::<String>();
    let lines = chars.lines().collect::<Vec<&str>>();

    let mut html = String::new();
    let mut vertices: Vec<Vector3> = Vec::new();
    let mut faces: Vec<Vec<u8>> = Vec::new();

    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() < 1 { continue; }

        if parts[0] == "v" {
            let p = read_point_value(parts);
            html.push_str(&*format_point(p));
            vertices.push(p);
        }

        else if parts[0] == "f" && parts.len() == 5 {
            let f = read_face_value(parts, &vertices);
            html.push_str(&*format_face(f));
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

fn format_face(matrix: Matrix4<f32>) -> String {
    format!("<div class=\"face\" style=\"
    transform: matrix3d({});\"></div>\n",
            format_matrix_for_css(&matrix))
}

fn read_face_value(parts: Vec<&str>, points: &Vec<Vector3>) -> Matrix4<f32> {
    let x = points[get_v_ref(parts[1])];
    let y = points[get_v_ref(parts[2])];
    let z = points[get_v_ref(parts[3])];
    let w = points[get_v_ref(parts[4])];

    create_matrix_from_points(&x, &y, &z, &w)
}

fn get_v_ref(s: &str) -> usize {
    s.parse().expect(&format!("{s} not valid number"))
}

fn read_point_value(parts: Vec<&str>) -> Point3<f32> {
    let x = parts[1].parse::<f32>().unwrap();
    let y = parts[2].parse::<f32>().unwrap();
    let z = parts[3].parse::<f32>().unwrap();

    Point3::new(x, y, z)
}

fn format_point(p: Point3<f32>) -> String {
    format!("<div class=\"point\" \
    style=\"--x: {0}; --y: {1}; --z: {2};\
    transform: \
    translateX(calc({0} * var(--scale) + {3}px)) \
    translateY(calc({1} * var(--scale) + {3}px)) \
    translateZ(calc({2} * var(--scale) + {3}px)) \">\
    </div>\n", p.x, p.y, p.z, OFFSET)
}


fn create_matrix_from_points(p1: &Point3<f32>, p2: &Point3<f32>, p3: &Point3<f32>, p4: &Point3<f32>) -> Matrix4<f32> {
    let matrix = Matrix4::from_fn(|i, j| match (i, j) {
        (0, 0) => p1.x,
        (0, 1) => p1.y,
        (0, 2) => p1.z,
        (0, 3) => 1.0,
        (1, 0) => p2.x,
        (1, 1) => p2.y,
        (1, 2) => p2.z,
        (1, 3) => 1.0,
        (2, 0) => p3.x,
        (2, 1) => p3.y,
        (2, 2) => p3.z,
        (2, 3) => 1.0,
        (3, 0) => p4.x,
        (3, 1) => p4.y,
        (3, 2) => p4.z,
        (3, 3) => 1.0,
        (_, _) => 0.0,
    });

    matrix
}

fn format_matrix_for_css(matrix: &Matrix4<f32>) -> String {
    let mut css_matrix = String::new();
    for i in 0..4 {
        for j in 0..4 {
            let value = matrix[(i, j)];
            css_matrix.push_str(&format!("{:.6},", value)); // Format the value with 6 decimal places
        }
    }
    css_matrix.pop(); // Remove the trailing comma

    css_matrix
}


fn remap(n: f32) -> u8 {
    (((n + 1.) / 2.) * 255.) as u8
}
