use {Service};

use quote::{ToTokens, Tokens};
use syn;

/// Generate the service implementations
pub fn generate(ast: &syn::File, services: &[Service]) -> String {
    // Tokens representing the output
    let mut tokens = ast.into_tokens();

    for service in services {
        let ty = &service.self_ty;

        let mut routes_fn = Tokens::new();
        let mut dispatch_fn = Tokens::new();

        for (i, route) in service.routes.iter().enumerate() {
            let ident = &route.ident;
            let method = route.method.as_ref().unwrap().to_tokens();
            let path = route.path_lit.as_ref().unwrap();

            routes_fn.append_all(quote! {
                routes.push(
                    Route::new(
                        Destination::new(#i),
                        Condition::new(#method, #path)));
            });

            dispatch_fn.append_all(quote! {
                #i => {
                    let response = self.#ident()
                        .into_response()
                        .map(|response| {
                            response.map(|body| {
                                // TODO: Log error
                                let body = body.map_err(|_| ::tower_web::Error::Internal);
                                Box::new(body) as Self::Body
                            })
                        });

                    Box::new(response) as Self::Future
                }
            });
        }

        // Define `Resource` on the struct.
        tokens.append_all(quote! {
            impl ::tower_web::Resource for #ty {
                type Body = ::tower_web::codegen::BoxBody;
                type Future = ::tower_web::codegen::BoxResponse<Self::Body>;

                fn routes(&self) -> ::tower_web::routing::RouteSet {
                    use ::tower_web::routing::{Route, RouteSet, Destination, Condition};

                    let mut routes = RouteSet::new();
                    #routes_fn
                    routes
                }

                fn dispatch(&mut self,
                            route: &::tower_web::routing::Match,
                            request: ::tower_web::codegen::http::Request<()>)
                    -> Self::Future
                {
                    use ::tower_web::IntoResponse;
                    use ::tower_web::codegen::bytes::Bytes;
                    use ::tower_web::codegen::futures::{future, stream, Future, Stream};

                    drop(request);

                    match route.destination().id() {
                        #dispatch_fn
                        _ => panic!("invalid route destination"),
                    }
                }
            }

            impl<U> ::tower_web::resource::Chain<U> for #ty {
                type Resource = (Self, U);

                fn chain(self, other: U) -> Self::Resource {
                    (self, other)
                }
            }
        });
    }

    tokens.to_string()
}