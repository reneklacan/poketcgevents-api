use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
    ),
    components(schemas(

    )),
    servers(
        (url = "https://poketcgevents-api.onrender.com/"),
    )
)]
pub struct ApiDoc;
