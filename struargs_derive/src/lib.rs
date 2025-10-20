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

    let mut rets = vec![];
    for field in stru.fields.iter() {
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

        if is_option(&field.ty) {
            rets.push(quote! {
                if let Some(ref arg) = self.#ident {
                    args.extend([#ident_arg.to_string(), arg.to_string()]);
                }
            });
        } else {
            rets.push(quote! {
                args.extend([#ident_arg.to_string(), self.#ident.to_string()]);
            });
        }
    }

    let ident = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics ::struargs::Args for #ident #ty_generics #where_clause {
            fn args(&self) -> Vec<String> {
                let mut args = vec![];
                #(#rets)*
                args
            }
        }
    }
    .into()
}

fn is_option(ty: &Type) -> bool {
    let path = match ty {
        Type::Path(type_path) if type_path.qself.is_none() => &type_path.path,
        _ => return false,
    };
    let Some(last_segment) = path.segments.last() else {
        return false;
    };
    last_segment.ident == "Option"
}
