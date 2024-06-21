use std::f64;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

#[wasm_bindgen]
pub fn simplify_rdp(points: Vec<Point>, epsilon: f64) -> Vec<Point> {
    fn get_perpendicular_distance(point: Point, line_start: Point, line_end: Point) -> f64 {
        let Point { x, y } = point;
        let Point { x: x1, y: y1 } = line_start;
        let Point { x: x2, y: y2 } = line_end;

        let area = ((x2 - x1) * (y1 - y) - (x1 - x) * (y2 - y1)).abs();
        let base = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
        area / base
    }

    fn rdp(points: &[Point], epsilon: f64) -> Vec<Point> {
        if points.len() < 3 {
            return points.to_vec();
        }

        let mut dmax = 0.0;
        let mut index = 0;
        let end = points.len() - 1;

        for i in 1..end {
            let d = get_perpendicular_distance(points[i], points[0], points[end]);
            if dmax < d {
                index = i;
                dmax = d;
            }
        }

        if dmax > epsilon {
            let mut rec_results1 = rdp(&points[..=index], epsilon);
            let rec_results2 = rdp(&points[index..], epsilon);

            rec_results1.pop(); // Remove the last point to avoid duplication
            rec_results1.extend(rec_results2);
            rec_results1
        } else {
            vec![points[0], points[end]]
        }
    }

    rdp(&points, epsilon)
}
