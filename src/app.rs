use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};
use thaw::ConfigProvider;
use crate::comps::home::HomePage;
use crate::comps::notfound::NotFound;


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
