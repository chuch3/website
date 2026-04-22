use crate::context::Context;
use http_bytes::http::{Error, Response, Uri, header, response::Builder};
use httparse::{Request, Status};
use mime;
use minijinja::{Environment, context};
use new_mime_guess;
use std::fs;

pub fn build_get_response(
    ctx: &Context,
    req: Request,
    status: Status<usize>,
) -> Result<Response<Vec<u8>>, Error> {
    let mut res_builder = Builder::new();

    let req_path = { if let Some(path) = req.path { path } else { "" } };

    dbg!(req_path);

    let uri = req_path.parse::<Uri>().unwrap();

    let (uri_path, query) = (uri.path(), uri.query());

    dbg!(uri_path, query);

    let path_list: Vec<_> = uri_path.split("/").skip(1).collect();

    dbg!(&path_list);

    let (template, page_context) = match *path_list.get(0).unwrap_or(&"") {
        "" => ("base.html", context! {}),
        "static" => {
            let mime_type = new_mime_guess::from_path(uri_path).first_or(mime::TEXT_HTML);

            dbg!(&mime_type);

            if let Ok(file) = fs::read(&uri_path[1..]) {
                // MIME is just for standard format, sending the static resource is
                // enough for the html page.
                return res_builder
                    .header(header::CONTENT_TYPE, mime_type.essence_str())
                    .header(header::CONTENT_LENGTH, file.len())
                    .body(file);
            } else {
                return res_builder.status(404).body(vec![]);
            }
        }
        _ => ("error.html", context! {}),
    };

    // Page context holds the `Context` struct to access its values. Attribute values has to be imported beforehand during instantiation.
    let jinja_ctx = context! {
        page_context => page_context,
        uri_path => uri_path,
        // err => err, // should be taken from the query parameters
    };

    dbg!(&jinja_ctx);

    let body = jinja_to_bytes(template, jinja_ctx, &ctx.jinja_env).unwrap();

    res_builder
        .header(header::CONTENT_TYPE, mime::TEXT_HTML.essence_str())
        .header(header::CONTENT_LENGTH, body.len())
        .body(body)
}

fn jinja_to_bytes<'a>(
    template: &str,
    jinja_ctx: minijinja::Value,
    jinja_env: &Environment<'a>,
) -> Result<Vec<u8>, minijinja::Error> {
    jinja_env
        .get_template(template)
        .and_then(|t| t.render(jinja_ctx))
        .map(|s| s.into_bytes())
}

pub fn cont() -> Result<Response<Vec<u8>>, Error> {
    // Empty body with continue status code
    Builder::new().status(100).body(vec![])
}
