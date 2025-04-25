use leptos::{logging::log, prelude::*, task::spawn_local};
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
    _ = Effect::watch(move || img.get(), move |_, _, _| {
            draw_cnv(lines_vert.get_untracked(), lines_hori.get_untracked(), paint_w.get_untracked(), paint_h.get_untracked());
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
            log!("{0}", e.y());
            set_lines_hori.update(|l| l.push(e.y()));
            set_draw_hori.set(false);
        }
        if isDraw {
            draw_cnv(lines_vert.get_untracked(), lines_hori.get_untracked(), paint_w.get_untracked(), paint_h.get_untracked());
        }
    };

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
                        // <span>"Picture ratio : " {ratio}</span>
                        <div class="home__fields">
                            <Field label="Painting width">
                                <Input value=paint_w />
                            </Field>
                            <Field label="Painting height">
                                <Input value=paint_h />
                            </Field>
                        </div>
                        <div class="home__fields">
                            <Button icon=icondata::IoAdd on_click=move |_| set_draw_vert.set(true)>"Vertical line"</Button>
                            <Button icon=icondata::IoAdd on_click=move |_| set_draw_hori.set(true)>"Horizontal line"</Button>
                        </div>
                    </Card>
                </div>
            </div>
            <div>
                <canvas id="canvas" on:mousedown=move |e| cnv_click(e)></canvas>
                <img id="img" class="img" src=img />
            </div>
        </div>
    }
}
