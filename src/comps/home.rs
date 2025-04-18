use leptos::{prelude::*, task::spawn_local};
use thaw::{FileList, Upload, UploadDragger};

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {

    let (img, set_img) = signal("".to_string());

    let custom_request = move |file_list: FileList| {
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

    view! {
        <h1>"Painter Setup"</h1>
        <div class="ctn">
            <Upload custom_request>
                <UploadDragger>"Click or drag a file to this area to upload"</UploadDragger>
            </Upload>
            <img class="img" src={img}/>
        </div>
    }
}
