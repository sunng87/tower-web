use super::{Context, Response, Serializer};
use error;
use util::BufStream;

use bytes::Bytes;
use http::header;
use serde::Serialize;

#[derive(Debug)]
pub struct ModelAndView<T: Serialize> {
    pub view: String,
    pub model: T,
}

const TEXT_HTML: &'static str = "text/html";

impl<T: Serialize> Response for ModelAndView<T> {
    type Buf = <Self::Body as BufStream>::Item;
    type Body = error::Map<Bytes>;

    fn into_http<S>(self, context: &Context<S>) -> http::Response<Self::Body>
    where
        S: Serializer,
    {
        let content_type = context
            .content_type_header()
            .map(|header| header.clone())
            .unwrap_or_else(|| header::HeaderValue::from_static(TEXT_HTML));

        // TODO: render the model into view
        // TODO: get view provider from context

        http::Response::builder()
            .status(200)
            .header(header::CONTENT_TYPE, content_type)
            .body(error::Map::new(Bytes::from("RENDER_RESULT_HERE")))
            .unwrap()
    }
}
