use leptos::prelude::*;
use wasm_bindgen::JsCast;

pub fn draw_cnv(lines_vert: Vec<i32>, lines_hori: Vec<i32>, paint_w_s: String) -> f64 {

    let canvas_ele = document().get_element_by_id("canvas").unwrap();
    let canvas = canvas_ele.dyn_into::<web_sys::HtmlCanvasElement>().ok().unwrap();
    let context = canvas.get_context("2d").unwrap().unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

    let img_ele = document().get_element_by_id("img").unwrap();
    let img_html = img_ele.dyn_into::<web_sys::HtmlImageElement>().ok().unwrap();
    
    let img_w = img_html.natural_width();
    let img_h = img_html.natural_height();

    if img_w > 0 {
            
        let ratio = img_w as f64 / img_h as f64;

        let mut paint_w = 0.;
        let mut paint_h = 0.;
        if let Ok(w) = paint_w_s.parse::<f64>() {
            paint_w = w;
            paint_h = w / ratio;
        }

        let cnv_w = window().inner_width().unwrap().as_f64().unwrap().round() as u32 - 30;
        let cnv_h = (cnv_w as f64 / ratio).round() as u32;

        canvas.set_width(cnv_w);
        canvas.set_height(cnv_h);

        _ = context.draw_image_with_html_image_element_and_dw_and_dh(&img_html, 0., 0., cnv_w as f64, cnv_h as f64);

        let rect = canvas.get_bounding_client_rect();
        let cnv_x = rect.left();
        let cnv_y = rect.top();

        context.set_font("12px serif");

        for l in lines_vert {
            let x = l as f64 - cnv_x;
            context.begin_path();
            context.move_to(x, 0.);
            context.line_to(x, cnv_h as f64);
            context.stroke();
            let paint_x: f64 = x * paint_w / cnv_w as f64;
            _ = context.fill_text(format!("{:.2}", paint_x).as_str(), x + 2. , 10.);
        }

        for l in lines_hori {
            let y = l as f64 - cnv_y;
            context.begin_path();
            context.move_to(0., y);
            context.line_to(cnv_w as f64, y);
            context.stroke();
            let paint_y: f64 = y * paint_h / cnv_h as f64;
            _ = context.fill_text(format!("{:.2}", paint_y).as_str(), 0. , y - 4.);
        }
        return ratio;
    }
    0.
}