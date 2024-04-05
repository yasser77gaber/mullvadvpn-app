//! This `proc-macro` crate exports the [`Intersection`] derive macro, see the trait documentation
//! for more information.
extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(UnwrapProto)]
pub fn unwrap_proto_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    inner::derive(input).into()
}

mod inner {

    use proc_macro2::TokenStream;
    use quote::{format_ident, quote, TokenStreamExt};
    use syn::{spanned::Spanned, DeriveInput, Error, Type};

    pub(crate) fn derive(input: DeriveInput) -> TokenStream {
        if let syn::Data::Struct(data) = &input.data {
            derive_for_struct(&input, data).unwrap_or_else(Error::into_compile_error)
        } else {
            syn::Error::new(
                input.span(),
                "Deriving `Intersection` is only supported for structs",
            )
            .into_compile_error()
        }
    }

    pub(crate) fn derive_for_struct(
        input: &DeriveInput,
        data: &syn::DataStruct,
    ) -> syn::Result<TokenStream> {
        // println!("Deriving on:\n{}", quote! {#input});
        // let ident = &input.ident;
        // println!("Deriving on:\n{}", quote! {#ident});

        let struct_name = &input.ident;
        let generated_struct_name = format_ident!("{}Unwrapped", struct_name);

        let mut out_fields = quote! {};
        let mut field_conversions = quote! {};

        for field in &data.fields {
            let Some(name) = &field.ident else {
                return Err(syn::Error::new(
                    field.span(),
                    "Tuple structs are not currently supported",
                ));
            };

            println!("Field: {:#?}", field.ty);

            if let Type::Path(path) = &field.ty {
                println!("inner path: {:#?}", path.path);
                let path_segment = path.path.segments.last().expect("Typ should exist");
                if path_segment.ident == "Option" {
                    match &path_segment.arguments {
                        syn::PathArguments::AngleBracketed(wrapped_arg) => {
                            let arg = wrapped_arg.args.first().unwrap();
                            out_fields.append_all(quote! {
                                #name: #arg,
                            });
                            field_conversions.append_all(quote! {
                                #name: other.#name.ok_or(())?,
                            });
                        }
                        _ => unimplemented!(),
                    }

                    println!("Field was an option")
                } else {
                    out_fields.append_all(quote! {
                        #name: #path,
                    });

                    field_conversions.append_all(quote! {
                        #name: other.#name,
                    });
                }
            }
        }

        Ok(quote! {
            #[derive(Debug, PartialEq, Eq)]
            struct #generated_struct_name {
                #out_fields
            }

            impl TryFrom<#struct_name> for #generated_struct_name {
                type Error = ();

                fn try_from(other: #struct_name) -> Result<Self, ()> {
                    Ok(Self {
                        #field_conversions
                    })
                }
            }
        })
    }
}
