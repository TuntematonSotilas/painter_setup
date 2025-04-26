use leptos::{prelude::*, task::spawn_local};
use thaw::{Button, Card, Field, FileList, Input, Upload, UploadDragger};
use web_sys::MouseEvent;
use crate::services::canvas::draw_cnv;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {

    let (img, set_img) = signal("".to_string());
    //let (ratio, set_ratio) = signal(0.);
    let (draw_vert, set_draw_vert) = signal(false);
    let (draw_hori, set_draw_hori) = signal(false);
    let (lines_vert, set_lines_vert) = signal(Vec::<i32>::new());
    let (lines_hori, set_lines_hori) = signal(Vec::<i32>::new());

    let paint_w = RwSignal::new("".to_string());
    let (paint_h, set_paint_h) = signal(0.);
    let (ratio, set_ratio) = signal(0.);

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
    
    let img_loaded = move || {
        let rat = draw_cnv(lines_vert.get_untracked(), lines_hori.get_untracked(), paint_w.get_untracked());
        set_ratio.set(rat);
    };

    // Watch paint_w change to set the canvas
    _ = Effect::watch(move || paint_w.get(), move |pw, _, _| {
        if let Ok(w) = pw.parse::<f64>() {
            set_paint_h.set(w / ratio.get_untracked());
        }
    }, true);

    let cnv_click = move |e: MouseEvent| {
        let mut isDraw = false;
        if draw_vert.get() {
            isDraw = true;
            set_lines_vert.update(|l| l.push(e.x()));
            set_draw_vert.set(false);
        }
        if draw_hori.get() {
            isDraw = true;
            set_lines_hori.update(|l| l.push(e.y()));
            set_draw_hori.set(false);
        }
        if isDraw {
            draw_cnv(lines_vert.get_untracked(), lines_hori.get_untracked(), paint_w.get_untracked());
        }
    };

    let clear_lines = move || {
        set_lines_hori.set(Vec::new());
        set_lines_vert.set(Vec::new());
        draw_cnv(lines_vert.get_untracked(), lines_hori.get_untracked(), paint_w.get_untracked());
    };

    view! {
        <div class="home">
            <div class="home__header">
                <div class="home__title">
                    <Card>
                        <h3>"Painter Setup"</h3>
                        <div class="home__fields">
                            <Field label="Painting width">
                                <Input value=paint_w />
                            </Field>
                            <Field label="Painting height">
                                {move || format!("{:.2}", paint_h.get())}
                            </Field>
                        </div>
                    </Card>
                </div>
                <div class="home__form">
                    <Card>
                        <Upload custom_request=file_upload >
                            <UploadDragger>"Click or drag a file to this area to upload"</UploadDragger>
                        </Upload>
                        <div class="home__fields">
                            <Button icon=icondata::BiVerticalBottomRegular on_click=move |_| set_draw_vert.set(true)>"Vertical line"</Button>
                            <Button icon=icondata::BiHorizontalLeftRegular on_click=move |_| set_draw_hori.set(true)>"Horizontal line"</Button>
                            <Button icon=icondata::MdiBroom on_click=move |_| clear_lines()>"Clear lines"</Button>
                        </div>
                    </Card>
                </div>
            </div>
            <div>
                <canvas id="canvas" on:mousedown=cnv_click></canvas>
                <img id="img" class="img" src=img on:load=move |_| img_loaded() />
            </div>
        </div>
    }
}
