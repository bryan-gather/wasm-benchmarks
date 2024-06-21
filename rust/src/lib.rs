use std::f64;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn simplify_rdp(points: &[f64], epsilon: f64) -> Vec<f64> {
    fn get_perpendicular_distance(
        point: (f64, f64),
        line_start: (f64, f64),
        line_end: (f64, f64),
    ) -> f64 {
        let (x, y) = point;
        let (x1, y1) = line_start;
        let (x2, y2) = line_end;

        let area = ((x2 - x1) * (y1 - y) - (x1 - x) * (y2 - y1)).abs();
        let base = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
        area / base
    }

    fn rdp(points: &[(f64, f64)], epsilon: f64) -> Vec<(f64, f64)> {
        if points.len() < 3 {
            return points.to_vec();
        }

        let mut dmax = 0.0;
        let mut index = 0;
        let end = points.len() - 1;

        for i in 1..end {
            let d = get_perpendicular_distance(points[i], points[0], points[end]);
            if d > dmax {
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

    console_error_panic_hook::set_once();
    log(&format!("Points: {:?}", points));
    let points: Vec<(f64, f64)> = points.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
    log("Mapping to points");
    let simplified_points = rdp(&points, epsilon);
    log("Simplified points collected");
    simplified_points
        .into_iter()
        .flat_map(|(x, y)| vec![x, y])
        .collect()
}
