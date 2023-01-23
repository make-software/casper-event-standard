use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote, TokenStreamExt};
use syn::{Data, DataStruct, DeriveInput, Fields, Type};

const EVENT_PREFIX: &str = "event_";

#[proc_macro_derive(Event)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    match derive_event_code(input.into()) {
        Ok(output) => output,
        Err(err) => err.to_compile_error(),
    }
    .into()
}

fn derive_event_code(input: TokenStream2) -> Result<TokenStream2, syn::Error> {
    let input: DeriveInput = syn::parse2(input)?;
    let event_ident = input.ident.clone();
    let fields = extract_fields(input)?;

    let cl_typed_impl = generate_cl_typed_impl(&event_ident);
    let to_bytes_impl = generate_to_bytes_impl(&event_ident, &fields);
    let from_bytes_impl = generate_from_bytes_impl(&event_ident, &fields);
    let event_instance_impl = generate_event_instance_impl(&event_ident, &fields);

    Ok(quote! {
        #cl_typed_impl
        #to_bytes_impl
        #from_bytes_impl
        #event_instance_impl
    })
}

fn generate_cl_typed_impl(event_ident: &Ident) -> TokenStream2 {
    quote! {
        impl casper_event_standard::casper_types::CLTyped for #event_ident {
            fn cl_type() -> casper_event_standard::casper_types::CLType {
                casper_event_standard::casper_types::CLType::Any
            }
        }
    }
}

fn generate_to_bytes_impl(event_ident: &Ident, fields: &[(Ident, Type)]) -> TokenStream2 {
    let name_literal = format_ident!("{EVENT_PREFIX}{event_ident}");
    let name_literal = quote! { stringify!(#name_literal) };
    let append_bytes = fields
        .iter()
        .map(|(ident, _)| ident)
        .flat_map(|ident| quote!(vec.extend(self.#ident.to_bytes()?);))
        .collect::<TokenStream2>();
    let mut sum_serialized_lengths = quote! {
        size += #name_literal.serialized_length();
    };
    sum_serialized_lengths.append_all(
        fields
            .iter()
            .map(|(ident, _)| ident)
            .map(|ident| quote!(size += self.#ident.serialized_length();)),
    );
    quote! {
        impl casper_event_standard::casper_types::bytesrepr::ToBytes for #event_ident {
            fn to_bytes(&self) -> Result<casper_event_standard::alloc::vec::Vec<u8>, casper_event_standard::casper_types::bytesrepr::Error> {
                let mut vec = casper_event_standard::alloc::vec::Vec::with_capacity(self.serialized_length());
                vec.append(&mut #name_literal.to_bytes()?);
                #append_bytes
                Ok(vec)
            }

            fn serialized_length(&self) -> usize {
                let mut size = 0;
                #sum_serialized_lengths
                size
            }
        }
    }
}

fn generate_from_bytes_impl(event_ident: &Ident, fields: &[(Ident, Type)]) -> TokenStream2 {
    let deserialize_fields = fields
        .iter()
        .map(|(ident, _)| ident)
        .map(|ident| quote!(let (#ident, bytes) = FromBytes::from_bytes(bytes)?;))
        .collect::<TokenStream2>();

    let construct_struct = fields
        .iter()
        .map(|(ident, _)| ident)
        .map(|ident| quote! { #ident, })
        .collect::<TokenStream2>();

    quote! {
        impl casper_event_standard::casper_types::bytesrepr::FromBytes for #event_ident {
            fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), casper_event_standard::casper_types::bytesrepr::Error> {
                use casper_event_standard::casper_types::bytesrepr::FromBytes;
                let (_, bytes): (casper_event_standard::alloc::string::String, &[u8]) = FromBytes::from_bytes(bytes)?;
                #deserialize_fields
                let value = #event_ident {
                    #construct_struct
                };
                Ok((value, bytes))
            }
        }
    }
}

fn generate_event_instance_impl(event_ident: &Ident, fields: &[(Ident, Type)]) -> TokenStream2 {
    let schema_elements = fields
        .iter()
        .map(|(ident, ty)| quote! {
            schema.with_elem(stringify!(#ident), <#ty as casper_event_standard::casper_types::CLTyped>::cl_type());
        })
        .collect::<TokenStream2>();

    quote! {
        impl casper_event_standard::EventInstance for #event_ident {
            fn name() -> casper_event_standard::alloc::string::String {
                casper_event_standard::alloc::string::String::from(stringify!(#event_ident))
            }

            fn schema() -> casper_event_standard::Schema {
                let mut schema = casper_event_standard::Schema::new();
                #schema_elements
                schema
            }
        }
    }
}

fn extract_fields(input: DeriveInput) -> Result<Vec<(Ident, Type)>, syn::Error> {
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(named_fields),
            ..
        }) => named_fields
            .named
            .into_iter()
            .map(|f| (f.ident.unwrap(), f.ty))
            .collect::<Vec<_>>(),
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "Expected a struct with named fields.",
            ))
        }
    };
    Ok(fields)
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;

    use crate::derive_event_code;

    #[test]
    fn test_event_code_generation() {
        let input = quote! {
            pub struct Transfer {
                amount: U256,
                from: Key,
                to: Key
            }
        };
        let result = derive_event_code(input).unwrap();
        let expected = quote! {
            impl casper_event_standard::casper_types::CLTyped for Transfer {
                fn cl_type() -> casper_event_standard::casper_types::CLType {
                    casper_event_standard::casper_types::CLType::Any
                }
            }

            impl casper_event_standard::casper_types::bytesrepr::ToBytes for Transfer {
                fn to_bytes(&self) -> Result<casper_event_standard::alloc::vec::Vec<u8>, casper_event_standard::casper_types::bytesrepr::Error> {
                    let mut vec = casper_event_standard::alloc::vec::Vec::with_capacity(self.serialized_length());
                    vec.append(&mut stringify!(event_Transfer).to_bytes()?);
                    vec.extend(self.amount.to_bytes()?);
                    vec.extend(self.from.to_bytes()?);
                    vec.extend(self.to.to_bytes()?);
                    Ok(vec)
                }

                fn serialized_length(&self) -> usize {
                    let mut size = 0;
                    size += stringify!(event_Transfer).serialized_length();
                    size += self.amount.serialized_length();
                    size += self.from.serialized_length();
                    size += self.to.serialized_length();
                    size
                }
            }

            impl casper_event_standard::casper_types::bytesrepr::FromBytes for Transfer {
                fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), casper_event_standard::casper_types::bytesrepr::Error> {
                    use casper_event_standard::casper_types::bytesrepr::FromBytes;
                    let (_, bytes): (casper_event_standard::alloc::string::String, &[u8]) = FromBytes::from_bytes(bytes)?;
                    let (amount, bytes) = FromBytes::from_bytes(bytes)?;
                    let (from, bytes) = FromBytes::from_bytes(bytes)?;
                    let (to, bytes) = FromBytes::from_bytes(bytes)?;
                    let value = Transfer { amount, from, to, };
                    Ok((value, bytes))
                }
            }

            impl casper_event_standard::EventInstance for Transfer {
                fn name() -> casper_event_standard::alloc::string::String {
                    casper_event_standard::alloc::string::String::from(stringify!(Transfer))
                }

                fn schema() -> casper_event_standard::Schema {
                    let mut schema = casper_event_standard::Schema::new();
                    schema.with_elem(stringify!(amount), <U256 as casper_event_standard::casper_types::CLTyped>::cl_type());
                    schema.with_elem(stringify!(from), <Key as casper_event_standard::casper_types::CLTyped>::cl_type());
                    schema.with_elem(stringify!(to), <Key as casper_event_standard::casper_types::CLTyped>::cl_type());
                    schema
                }
            }

        };
        assert_eq_code(result, expected);
    }

    fn assert_eq_code(result: TokenStream, expected: TokenStream) {
        pretty_assertions::assert_eq!(expected.to_string(), result.to_string());
    }
}
