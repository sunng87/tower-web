use syn;
use proc_macro2::TokenStream;

#[derive(Debug)]
pub(crate) struct Arg {
    /// The function argument index
    pub index: usize,

    /// The index in the extract tuple
    pub extract_index: Option<usize>,

    /// Argument identifier, i.e., the variable name.
    pub ident: Option<String>,

    /// The index of the path binding the identifier matches.
    pub param: Option<usize>,

    /// The argument type
    pub ty: syn::Type,
}

impl Arg {
    /// Create a new, regular, argument.
    pub fn new(index: usize,
               extract_index: usize,
               ident: String,
               param: Option<usize>,
               ty: syn::Type) -> Arg
    {
        Arg {
            index,
            extract_index: Some(extract_index),
            ident: Some(ident),
            param,
            ty,
        }
    }

    /// The argument is formatted in a way that cannot be interpretted.
    pub fn ty_only(index: usize,
                   extract_index: usize,
                   ty: syn::Type) -> Arg
    {
        Arg {
            index,
            extract_index: Some(extract_index),
            ty,
            ident: None,
            param: None,
        }
    }

    pub fn request_stream(index: usize, ty: syn::Type) -> Arg {
        Arg {
            index,
            extract_index: None,
            ty,
            ident: None,
            param: None,
        }
    }

    pub fn is_extracted(&self) -> bool {
        self.extract_index.is_some()
    }

    pub fn extract_index(&self) -> usize {
        self.extract_index.unwrap()
    }

    /// Generate a call site for the argument
    pub fn new_callsite(&self) -> TokenStream {
        assert!(self.extract_index.is_some());

        if let Some(idx) = self.param {
            quote! { __tw::codegen::CallSite::new_param(#idx) }
        } else if let Some(ref ident) = self.ident {
            match &ident[..] {
                "query_string" => quote! { __tw::codegen::CallSite::new_query_string() },
                "body" => quote! { __tw::codegen::CallSite::new_body() },
                header => {
                    let header = ::header::arg_to_header_name(header);
                    let header = header.as_str();

                    quote! { __tw::codegen::CallSite::new_header(#header) }
                }
            }
        } else {
            quote! { __tw::codegen::CallSite::new_unknown() }
        }
    }
}
