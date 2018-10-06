use super::{Context, Response, Serializer};
use error;
use util::BufStream;

use bytes::Bytes;
use http::header;
use serde::Serialize;

/// A generic view data as response
#[derive(Debug)]
pub struct ModelAndView<T: Serialize> {
    view: String,
    model: T,
}

const TEXT_HTML: &'static str = "text/html";

impl<T: Serialize> ModelAndView<T> {
    /// Create a ModelAndBView data.
    pub fn new(model: T, view: String) -> ModelAndView<T> {
        ModelAndView { view, model }
    }
}

impl<T: Serialize> Response for ModelAndView<T> {
    type Buf = <Self::Body as BufStream>::Item;
    type Body = error::Map<Bytes>;

    fn into_http<S>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, ::Error>
    where
        S: Serializer,
    {
        let content_type = context
            .content_type_header()
            .map(|header| header.clone())
            .unwrap_or_else(|| header::HeaderValue::from_static(TEXT_HTML));

        let serialize_context = context.serializer_context();
        let render_result = context
            .serialize(&self.model, &serialize_context)
            .unwrap();

        let response = http::Response::builder()
            .status(200)
            .header(header::CONTENT_TYPE, content_type)
            .body(error::Map::new(render_result))
            .unwrap();

        Ok(response)
    }
}
