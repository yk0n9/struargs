use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Lit, Meta, Type};

#[proc_macro_derive(Args, attributes(args))]
pub fn struargs(input: TokenStream) -> TokenStream {
    common(input)
}

fn common(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let Data::Struct(stru) = input.data else {
        return syn::Error::new_spanned(
            input,
            "FieldNames can only be derived for structs with named or unnamed fields",
        )
        .to_compile_error()
        .into();
    };

    let mut option_args = vec![];
    for field in stru.fields.iter() {
        let path = match field.ty {
            Type::Path(ref type_path) if type_path.qself.is_none() => &type_path.path,
            _ => continue,
        };
        let Some(last_segment) = path.segments.last() else {
            continue;
        };
        if last_segment.ident != "Option" {
            continue;
        }

        let Some(ref ident) = field.ident else {
            continue;
        };

        let mut ident_arg = format!("--{}", ident.to_string());

        for attr in field.attrs.iter() {
            if attr.path().is_ident("args") {
                if let Ok(Meta::NameValue(ref nv)) = attr.parse_args() {
                    if nv.path.is_ident("rename") {
                        if let Expr::Lit(ref lit) = nv.value {
                            if let Lit::Str(ref s) = lit.lit {
                                ident_arg = format!("--{}", s.value());
                            }
                        }
                    }
                }
            }
        }

        option_args.push(quote! {
            if let Some(ref arg) = self.#ident {
                args.extend([#ident_arg.to_string(), arg.to_string()]);
            }
        });
    }

    let ident = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics ::struargs::Args for #ident #ty_generics #where_clause {
            fn args(&self) -> Vec<String> {
                let mut args = vec![];
                #(#option_args)*
                args
            }
        }
    }
    .into()
}
