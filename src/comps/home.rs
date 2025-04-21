use leptos::{prelude::*, task::spawn_local};
use thaw::{Card, Field, FileList, Input, Upload, UploadDragger};
use wasm_bindgen::JsCast;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {

    let (img, set_img) = signal("".to_string());
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

    _ = Effect::watch(
        move || img.get(),
        move |_, _, _| {

            let canvas_ele = document().get_element_by_id("canvas").unwrap();
            let canvas = canvas_ele.dyn_into::<web_sys::HtmlCanvasElement>().ok().unwrap();
            let context = canvas.get_context("2d").unwrap().unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
        
            let img_ele = document().get_element_by_id("img").unwrap();
            let img_html = img_ele.dyn_into::<web_sys::HtmlImageElement>().ok().unwrap();
            
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
                        <div class="home__fields">
                            <Field label="Painting width">
                                <Input value=paint_w />
                            </Field>
                            <Field label="Painting height">
                                <Input value=paint_h />
                            </Field>
                        </div>
                        <Upload custom_request=file_upload >
                            <UploadDragger>"Click or drag a file to this area to upload"</UploadDragger>
                        </Upload>
                    </Card>
                </div>
            </div>
            <div class="home__canvas">
                <canvas id="canvas"></canvas>
                <img id="img" class="img" src=img />
            </div>
        </div>
    }
}
