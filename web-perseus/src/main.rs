mod templates;

#[macro_use]
extern crate lazy_static;

use perseus::{
    i18n::TranslationsManager, server::ServerProps, stores::MutableStore, Html, PerseusApp,
};

#[cfg(not(target_arch = "wasm32"))]
pub async fn dflt_server<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    props: ServerProps<M, T>,
    (host, port): (String, u16),
) {
    use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
    use futures::{executor::block_on, Future};
    use perseus_actix_web::configurer;
    use std::time::Duration;

    async fn proxy(req) -> HttpResponse {
        HttpResponse::Ok().body("Hello")
    }

    HttpServer::new(move || {
        App::new()
            .service(web::resource("/api/{tail:.*}").to(proxy))
            .configure(block_on(configurer(props.clone())))
    })
    .bind((host, port))
    .expect(
        "Couldn't bind to given address. Maybe something is already running on the selected port?",
    )
    .run()
    .await
    .expect("Server failed.")
}

#[perseus::main(dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new().template(crate::templates::index::get_template)
}
