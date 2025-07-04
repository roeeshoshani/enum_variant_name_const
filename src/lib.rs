//! Attribute macro `#[enum_variant_name_const]`
//!
//! It injects a
//! ```rust
//! pub const fn variant_name(&self) -> &'static str
//! ```
//! on the annotated enum, returning the exact identifier of the
//! variant (`"A"`, `"B"` …) at **compile time**.
//!
//! # Example
//! ```
//! use enum_variant_name_const::enum_variant_name_const;
//!
//! #[enum_variant_name_const]
//! enum MyEnum {
//!     A(i32),
//!     B { x: u32, y: u32 },
//! }
//!
//! const NAME: &str = {
//!     let e = MyEnum::B { x: 1, y: 2 };
//!     e.variant_name()
//! };
//!
//! assert_eq!(NAME, "B");
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, ItemEnum, parse_macro_input};

/// Apply on an enum to generate `const fn variant_name(&self) -> &'static str`.
///
/// ```rust
/// use enum_variant_name_const::enum_variant_name_const;
///
/// #[enum_variant_name_const]
/// enum Message<T> {
///     Ping,
///     Data(T),
///     Error { code: u16, msg: String },
/// }
/// ```
#[proc_macro_attribute]
pub fn enum_variant_name_const(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input as an enum.
    let input = parse_macro_input!(item as ItemEnum);
    let enum_ident = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Build one match arm per variant, choosing the correct pattern kind.
    let match_arms = input.variants.iter().map(|v| {
        let ident = &v.ident;
        let pat = match &v.fields {
            Fields::Named(_) => quote! { Self::#ident { .. } },
            Fields::Unnamed(_) => quote! { Self::#ident ( .. ) },
            Fields::Unit => quote! { Self::#ident },
        };
        quote! { #pat => stringify!(#ident), }
    });

    // Compose the expanded code.
    let expanded = quote! {
        #input

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
