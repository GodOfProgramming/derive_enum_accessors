use proc_macro_error::{abort, emit_warning, proc_macro_error};
use proc_macro2::Ident;
use quote::quote;
use std::collections::{BTreeSet, HashMap};
use syn::{Data, DeriveInput, Generics, Type, parse_macro_input};

struct ItemType {
    key: String,
    value: Type,
}

impl PartialEq for ItemType {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl Eq for ItemType {}

impl PartialOrd for ItemType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ItemType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

#[proc_macro_error]
#[proc_macro_derive(EnumFieldAccessors)]
pub fn enum_field_accessors(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let info = parse_macro_input!(stream as DeriveInput);

    let Data::Enum(e) = info.data else {
        abort!(info.ident, "This derive must be used on an enum");
    };

    let variants = e.variants;

    // all variant names
    let mut all_variant_names = BTreeSet::new();

    // field names -> which variants they belong to
    let mut variant_fields = HashMap::<_, BTreeSet<_>>::new();

    // field names -> what type they are
    let mut field_types = HashMap::<_, BTreeSet<_>>::new();

    for v in variants {
        all_variant_names.insert(v.ident.clone());

        if let syn::Fields::Named(fields) = v.fields {
            for field in fields.named {
                let ident = field.ident.expect("A named field should have a name");
                let field_type = &field.ty;

                let entry = variant_fields.entry(ident.clone()).or_default();
                entry.insert(v.ident.clone());

                let entry = field_types.entry(ident).or_default();
                entry.insert(ItemType {
                    key: quote! {#field_type}.to_string(),
                    value: field_type.clone(),
                });
            }
        } else {
            emit_warning!(
                v.fields,
                "No fields will be generated for this type of enum"
            );
        }
    }

    let mut accessors = Vec::new();

    for (field_name, field_variants) in &variant_fields {
        let Some(field_type) = field_types.get(field_name) else {
            abort!(field_name, "Found a field name with no type");
        };

        if field_type.len() != 1 {
            emit_warning!(
                field_name,
                "Field type is not unique and will not have any accessors"
            );
            continue;
        }

        let field_type = field_type
            .iter()
            .next()
            .expect("The length was checked in the previous expression");

        let is_common_name = *field_variants == all_variant_names;

        let t = &field_type.value;

        let (return_type, return_type_mut, return_value) = if is_common_name {
            (quote! { & #t }, quote! { &mut #t }, quote! { #field_name })
        } else {
            (
                quote! { Option<& #t > },
                quote! { Option<&mut #t > },
                quote! { Some(#field_name) },
            )
        };

        let field_name_mut = Ident::new(&format!("{field_name}_mut"), field_name.span());

        let matches = all_variant_names
            .iter()
            .map(|vn| {
                if field_variants.contains(vn) {
                    quote! {
                      Self:: #vn { #field_name, .. } => #return_value,
                    }
                } else {
                    quote! {
                      Self:: #vn { .. } => None,
                    }
                }
            })
            .collect::<Vec<_>>();

        let field_accessors = quote! {
          pub fn #field_name (&self) -> #return_type {
            match self {
              #(#matches)*
            }
          }

          pub fn #field_name_mut (&mut self) -> #return_type_mut {
            match self {
              #(#matches)*
            }
          }
        };

        accessors.push(field_accessors);
    }

    let ident = info.ident;
    let Generics {
        lt_token,
        params,
        gt_token,
        where_clause,
    } = info.generics;

    quote! {
      impl #lt_token #params #gt_token #ident #lt_token #params #gt_token #where_clause {
        #(#accessors)*
      }
    }
    .into()
}
