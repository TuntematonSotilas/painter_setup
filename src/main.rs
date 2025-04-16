#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::prelude::*;
    use leptos::config::get_configuration;
    use leptos_meta::MetaTags;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use painter_setup::app::*;
    use thaw::ssr::SSRMountStyleProvider;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);

        let leptos_options = &conf.leptos_options;

        let site_root = leptos_options.site_root.clone().to_string();

        println!("listening on http://{}", &addr);

        App::new()
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", &site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(routes, {
                
                let opts = leptos_options.clone();

                move || {
                    
                    let opta = opts.clone();
                    let optb = opts.clone();

                    view! {
                        <SSRMountStyleProvider>
                            <!DOCTYPE html>
                            <html lang="en">
                                <head>
                                    <meta charset="utf-8"/>
                                    <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                    <AutoReload options=opta />
                                    <HydrationScripts options=optb />
                                    <MetaTags/>
                                </head>
                                <body>
                                    <App/>
                                </body>
                            </html>
                        </SSRMountStyleProvider>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.clone()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // if no SSR : nothing
}

