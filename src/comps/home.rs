use leptos::{logging::log, prelude::*, task::spawn_local};
use thaw::{Card, Field, FileList, Input, Upload, UploadDragger};
use wasm_bindgen::JsCast;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {

    let (img, set_img) = signal("".to_string());
    let (ratio, set_ratio) = signal(0.);

    let paint_w = RwSignal::new("".to_string());
    let paint_h = RwSignal::new("".to_string());

    let file_upload = move |file_list: FileList| {
        let opt_file = file_list.get(0);
        if let Some(file) = opt_file {
            let blob_gloo = gloo_file::Blob::from(file);
            spawn_local(async move {
                let res = gloo_file::futures::read_as_data_url(&blob_gloo).await;
                if let Ok(data) = res {
                    set_img.set(data);
                }
            });
        }
    };

    // Watch image change to set the canvas
    _ = Effect::watch(
        move || img.get(),
        move |_, _, _| {

            let canvas_ele = document().get_element_by_id("canvas").unwrap();
            let canvas = canvas_ele.dyn_into::<web_sys::HtmlCanvasElement>().ok().unwrap();
            let context = canvas.get_context("2d").unwrap().unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
        
            let img_ele = document().get_element_by_id("img").unwrap();
            let img_html = img_ele.dyn_into::<web_sys::HtmlImageElement>().ok().unwrap();
            
            let img_w = img_html.width();
            let img_h = img_html.height();

            log!("{0},{1}", img_w, img_h);

            let ratio = img_w as f64 / img_h as f64;
            
            set_ratio.set(ratio);
            
            let win_w = window().inner_width().unwrap().as_f64().unwrap().round() as u32 - 20;
            let cnv_h = (img_w as f64 / ratio).round() as u32;

            canvas.set_width(win_w);
            canvas.set_height(cnv_h);

            _ = context.draw_image_with_html_image_element(&img_html, 0., 0.);
           
        },
        true,
    );

    view! {
        <div class="home">
            <div class="home__header">
                <div class="home__title">
                    <Card>
                        <h3>"Painter Setup"</h3>
                        "An application to calculate photo ratios for a painter canvas"
                    </Card>
                </div>
                <div class="home__form">
                    <Card>
                        <Upload custom_request=file_upload >
                            <UploadDragger>"Click or drag a file to this area to upload"</UploadDragger>
                        </Upload>
                        <span>"Picture ratio : " {ratio}</span>
                        <div class="home__fields">
                            <Field label="Painting width">
                                <Input value=paint_w />
                            </Field>
                            <Field label="Painting height">
                                <Input value=paint_h />
                            </Field>
                        </div>
                    </Card>
                </div>
            </div>
            <div>
                <canvas id="canvas" ></canvas>
                <img id="img" class="img" src=img />
            </div>
        </div>
    }
}
