use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use rand::Rng;
use delaunator::{Point, triangulate};

#[wasm_bindgen]
pub fn draw_delaunay_pattern(canvas_id: &str, num_points: usize) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| JsValue::from_str("Canvas element not found"))?
        .dyn_into::<HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    context.clear_rect(0.0, 0.0, width, height);

    let mut rng = rand::thread_rng();

    let mut points = Vec::with_capacity(num_points);
    for _ in 0..num_points {
        points.push(Point {
            x: rng.gen_range(0.0..width),
            y: rng.gen_range(0.0..height),
        });
    }
    points.push(Point { x: 0.0, y: 0.0 });
    points.push(Point { x: width, y: 0.0 });
    points.push(Point { x: 0.0, y: height });
    points.push(Point { x: width, y: height });

    let triangulation = triangulate(&points);

    for i in (0..triangulation.triangles.len()).step_by(3) {
        let p1 = &points[triangulation.triangles[i]];
        let p2 = &points[triangulation.triangles[i + 1]];
        let p3 = &points[triangulation.triangles[i + 2]];

        let r = rng.gen_range(0..=255);
        let g = rng.gen_range(0..=255);
        let b = rng.gen_range(0..=255);
        let fill_style = format!("rgb({}, {}, {})", r, g, b);

        // 修正: メソッド名を set_fill_style_str に変更
        context.set_fill_style_str(&fill_style);
        context.begin_path();
        context.move_to(p1.x, p1.y);
        context.line_to(p2.x, p2.y);
        context.line_to(p3.x, p3.y);
        context.close_path();
        context.fill();
        
        // 修正: メソッド名を set_stroke_style_str に変更
        context.set_stroke_style_str("rgba(255, 255, 255, 0.2)");
        context.stroke();
    }

    Ok(())
}