opub struct ResponseAttributes {
    inner: HashMap<&'static str, &'static str>,
}

impl ResponeAttributes {
    pub fn new(inner: HashMap<&'static str, &'static str>) -> ResponseAttributes {
        ResponseAttributes { inner }
    }

    pub fn attribute(&self, key: &'static str) -> Option<&'static str> {
        inner.get(key)
    }
}

pub struct SerializeContext {
    attributes: ResponseAttributes,
    resource_name: &'static str,
}

impl SerializeContext {
    pub fn new(resource_name: &'static str, attributes: ResponseAttributes) -> SerializeContext {
        SerializeContext {
            attributes,
            resource_name,
        }
    }

    /// Returns the module in which the `impl_web!` was
    pub fn resource_impl_module(&self) -> &'static str {
        // TODO:
        ""
    }

    /// Returns the type name, for example:
    ///
    /// ```
    /// impl_web! {
    ///     impl HelloWorld { ... }
    /// }
    ///
    /// serialize_context.resource_name() // "HelloWorld"
    /// ```
    ///
    ///
    fn resource_name(&self) -> &'static str {
        self.resource_name
    }

    /// Returns a reference to the original request (without the body).
    // fn request(&self) -> &http::Request<()> {
    //     //TODO:

    // }

    /// Returns an application configuration type
    // pub fn config<T: 'static>(&self) -> Option<&T> {
    //     // TODO:
    // }

    /// Returns the `#[web(...)]` attributes set on response types defined with
    /// `#[derive(Response)]`
    ///
    /// This allows access to the attributes at runtime (i.e. not in the macro).
    pub fn response_attributes(&self) -> ResponseAttributes {
        self.attributes
    }
}
