extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Variant};

#[proc_macro_derive(ToValue)]
pub fn to_value_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let to_value_impl = match input.data {
        Data::Struct(data) => to_value_struct_impl(name, data.fields),
        Data::Enum(data) => to_value_enum_impl(name, data.variants),
        Data::Union(_) => panic!("ToValueBehavior cannot be derived for unions"),
    };

    let expanded = quote! {
        #to_value_impl
    };

    TokenStream::from(expanded)
}

fn to_value_struct_impl(name: syn::Ident, fields: Fields) -> proc_macro2::TokenStream {
    let field_transforms = match fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| {
                let name = field.ident.as_ref().unwrap();
                let field_name = format!("{}", name);
                quote! {
                    map.insert(#field_name.to_string(), self.#name.clone().into());
                }
            })
            .collect::<Vec<_>>(),
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, _field)| {
                let index = syn::Index::from(index);
                quote! {
                    Value::from(self.#index.clone())
                }
            })
            .collect::<Vec<_>>(),
        Fields::Unit => {
            return quote! {
                impl ToValueBehavior for #name {
                    fn to_value(&self) -> Value {
                        Value::Null
                    }
                }
            }
        }
    };

    quote! {
        impl ToValueBehavior for #name {
            fn to_value(&self) -> Value {
                let mut map: HashMap<String, Value>= std::collections::HashMap::new();
                #(#field_transforms)*
                Value::from(map)
            }
        }
    }
}

fn to_value_enum_impl(
    name: syn::Ident,
    variants: syn::punctuated::Punctuated<Variant, syn::Token![,]>,
) -> proc_macro2::TokenStream {
    let variant_transforms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let fields = match &variant.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|field| {
                    let name = field.ident.as_ref().unwrap();
                    let field_name_string = name.to_string();
                    quote! {
                        map.insert(#field_name_string.to_string(), (&self.#name).to_value());
                    }
                })
                .collect::<Vec<_>>(),
            Fields::Unnamed(fields) => fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(index, _field)| {
                    let index = syn::Index::from(index);
                    quote! {
                        <_ as ToValueBehavior>::to_value(&self.#index)
                    }
                })
                .collect::<Vec<_>>(),
            Fields::Unit => {
                return quote! {
                    Value::String(stringify!(#variant_name).to_owned())
                }
            }
        };

        quote! {
            #name::#variant_name { #(#fields),* } => Value::Object(vec![
                ("type".to_owned(), Value::String(stringify!(#variant_name).to_owned())),
                #(#fields),*
            ].into_iter().collect()),
        }
    });

    quote! {
        impl ToValueBehavior for #name {
            fn to_value(&self) -> Value {
                match self {
                    #(#variant_transforms)*
                }
            }
        }
    }
}

#[proc_macro_derive(FromValue)]
pub fn from_value_derive(input: TokenStream) -> TokenStream {
    // Parse a `DeriveInput` AST from the input tokens.
    let ast = parse_macro_input!(input as DeriveInput);

    // Get the name and fields of the struct being derived.
    let struct_name = &ast.ident;
    let struct_fields = match ast.data {
        Data::Struct(data_struct) => data_struct.fields,
        _ => panic!("Can only derive FromValueBehavior for a struct."),
    };

    // Define a new implementation of the `FromValueBehavior` trait for the struct.
    let mut field_names = Vec::new();
    let mut field_types = Vec::new();
    let mut from_value_exprs = Vec::new();
    if let Fields::Named(fields) = struct_fields {
        for field in fields.named.iter() {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            field_names.push(field_name.clone());
            field_types.push(field_type.clone());
            from_value_exprs.push(quote! {
                #field_name: {
                    let item = map.get(stringify!(#field_name)).unwrap().clone();
                    <#field_type as FromValueBehavior>::from_value(item).unwrap()
                }
            });
        }
    } else {
        panic!("Can only derive FromValueBehavior for a struct with named fields.");
    }

    let expanded = quote! {
        impl FromValueBehavior for #struct_name {
            type Item = Self;

            fn from_value(value: Value) -> Option<Self> {
                if let Value::Object(map) = value {
                    Some(
                        Self {
                            #(#from_value_exprs),*
                        }
                    )
                } else {
                    None
                }
            }
        }
    };

    // Return the generated code as a `TokenStream`.
    TokenStream::from(expanded)
}

#[proc_macro_derive(ToJson)]
pub fn to_json_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let gen = quote! {
        impl ToJsonBehavior for #name {
            fn to_json(&self) -> String {
                self.to_value().to_string()
            }
        }
    };

    gen.into()
}


#[proc_macro_derive(ToYaml)]
pub fn to_yaml_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl ToYamlBehavior for #name {
            fn to_yaml(&self) -> String {
                let value = self.to_value();
                value.to_yaml()
            }
        }
    };

    TokenStream::from(expanded)
}
