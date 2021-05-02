use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::*;

#[proc_macro_attribute]
pub fn format(_meta: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let mut errs = Vec::new();
    let Signature {
        ident,
        inputs,
        output,
        ..
    } = input_fn.sig;
    if let ReturnType::Type(_, output) = output {
        if output.to_token_stream().to_string() != "bool" {
            errs.push(syn::Error::new(ident.span(), "Must return bool"));
        } 
    } else {
        errs.push(syn::Error::new(ident.span(), "Must return bool"));
    };

    if inputs.len() != 1 {
        errs.push(syn::Error::new(ident.span(), "Must accept only a &str agrument"));
    }
    
    let arg = inputs.first();
    match &arg {
        Some(FnArg::Typed(pat)) => 
        {
            match pat.pat.as_ref() {
                Pat::Ident(_) | Pat::Wild(_) => (),
                _ => errs.push(syn::Error::new(ident.span(),"Must accept only a &str argument"))
            }
        },
        _ => errs.push(syn::Error::new(ident.span(),"Must accept only a &str argument")),
    };
    if !errs.is_empty() {
        let mut err = errs.first().unwrap().clone();
        for e in errs.iter() {
            err.combine(e.clone());
        }
        return err.to_compile_error().into();
    }
    
    let vis = &input_fn.vis;
    let body = &input_fn.block;
    let custom_t_name = ident.to_string() + "_t";
    let custom_t_name = Ident::new(&custom_t_name,ident.span());
    let ident_name = ident.to_string();
    TokenStream::from(quote! {
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        #vis struct #custom_t_name;

        impl ::jsonschema_custom_validator::CustomFormat for #custom_t_name
        {
            const NAME : &'static str = #ident_name;
            fn is_valid(&self,#arg) -> bool 
            #body
        }
        #[allow(non_upper_case_globals)]
        const #ident : #custom_t_name = #custom_t_name {};

        impl ToString for #custom_t_name {
            fn to_string(&self) -> String {
                format!("format: {}", <Self as ::jsonschema_custom_validator::CustomFormat>::NAME).to_string()
            }
        }
    })
}


#[cfg(test)]
mod tests {
    #[test]
    fn macro_tests() {
        let t = trybuild::TestCases::new();
        t.pass("test_cases/valid_macro_use.rs");
        t.compile_fail("test_cases/invalid_macro_use_arg.rs");
        t.compile_fail("test_cases/invalid_macro_use_return.rs");
    }
}