use leptos::{logging::log, prelude::*};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};
use thaw::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <ConfigProvider>
            // injects a stylesheet into the document <head>
            // id=leptos means cargo-leptos will hot-reload this stylesheet
            <Stylesheet id="leptos" href="/pkg/painter_setup.css"/>

            // sets the document title
            <Title text="Painter Setup"/>

            // content for this welcome page
            <Router>
                <main>
                    <Routes fallback=move || "Not found.">
                        <Route path=StaticSegment("") view=HomePage/>
                        <Route path=WildcardSegment("any") view=NotFound/>
                    </Routes>
                </main>
            </Router>
        </ConfigProvider>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {

    let custom_request = move |file_list: FileList| {
        let len = file_list.length();
        log!("{0}", len);
    };

    view! {
        <h1>"Painter Setup"</h1>
        <div class="ctn">
            <Upload custom_request>
                <UploadDragger>"Click or drag a file to this area to upload"</UploadDragger>
            </Upload>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
