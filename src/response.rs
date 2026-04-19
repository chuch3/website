use crate::context::Context;
use http_bytes::{
    Response,
    http::{Uri, header, response::Builder},
};
use httparse::{Request, Status};
use minijinja::{Environment, context};

pub fn build_get_response(ctx: &Context, req: Request, status: Status<usize>) {
    let res_builder = Builder::new();

    let req_path = { if let Some(path) = req.path { path } else { "" } };

    dbg!(req_path);

    let uri = req_path.parse::<Uri>().unwrap();

    // Query is not needed for basic web browsing
    let (uri_path, query) = (uri.path(), uri.query());

    dbg!(uri_path, query);

    // let (uri_path, _) = req_path.split_once("?").unwrap_or((req_path, ""));

    let split_path: Vec<_> = uri_path.split("/").collect();

    // TODO: Add mock template to test response and minijinja
    let (template, page_context) = match *split_path.get(0).unwrap_or(&"") {
        "" => ("response.html", context! {}),
        _ => ("error.html", context! {}),
    };

    // Page context holds the `Context` struct to access its values.
    // Its attribute values has to be imported beforehand during instantiation.
    let jinja_context = context! {
        page_context => page_context,
        uri_path => uri_path,
        // err => err,
    };

    let body = to_bytes(template, jinja_context, ctx);

    // TODO: Create context module with jinja environment and context
    //
    // - Get template, render the context for template and then convert result to bytes

    //let body = ;

    //res_builder.header(header::CONTENT_TYPE, mime::TEXT_HTML).body()
}

fn to_bytes(template: &str, jinja_context: minijinja::Value, ctx: &Context) {
    todo!()
}
