use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::events::search,
    ),
    components(schemas(

    )),
    servers(
        (url = "http://localhost:4400/"),
        (url = "https://poketcgevents-api.onrender.com/"),
    )
)]
pub struct ApiDoc;
