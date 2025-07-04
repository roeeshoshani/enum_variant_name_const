//! `#[derive(EnumVariantNameConst)]`
//!
//! Adds
//! ```ignore
//! pub const fn variant_name(&self) -> &'static str
//! ```
//! to the enum, returning the precise identifier of the variant
//! (“A”, “B”, …) and usable in `const` contexts.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// Derive macro that injects a
///
/// ```ignore
/// pub const fn variant_name(&self) -> &'static str
/// ```
///
/// on the annotated enum.
///
/// # Example
/// ```
/// use enum_variant_name_const::EnumVariantNameConst;
///
/// #[derive(EnumVariantNameConst)]
/// enum MyEnum {
///     A(i32),
///     B { x: u32, y: u32 },
/// }
///
/// const NAME: &str = MyEnum::B { x: 1, y: 2 }.variant_name();
/// assert_eq!(NAME, "B");
/// ```
#[proc_macro_derive(EnumVariantNameConst)]
pub fn enum_variant_name_const_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let enum_ident = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Ensure we were applied to an enum.
    let data_enum = match &input.data {
        Data::Enum(data) => data,
        _ => {
            return syn::Error::new_spanned(
                enum_ident,
                "`EnumVariantNameConst` can only be derived for enums",
            )
            .to_compile_error()
            .into();
        }
    };

    // One match arm per variant, with the correct pattern shape.
    let match_arms = data_enum.variants.iter().map(|v| {
        let ident = &v.ident;
        let pat = match &v.fields {
            Fields::Named(_) => quote! { Self::#ident { .. } },
            Fields::Unnamed(_) => quote! { Self::#ident ( .. ) },
            Fields::Unit => quote! { Self::#ident },
        };
        quote! { #pat => stringify!(#ident), }
    });

    let expanded = quote! {
        impl #impl_generics #enum_ident #ty_generics #where_clause {
            /// Compile-time string with the variant’s identifier.
            #[inline(always)]
            pub const fn variant_name(&self) -> &'static str {
                match self {
                    #( #match_arms )*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
