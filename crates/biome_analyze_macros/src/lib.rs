use proc_macro::TokenStream;

mod group_macro;
mod rule_docs;
mod rule_source_variant_index;

/// Declares an analyzer group by reading rule files from the filesystem.
///
/// This macro scans the filesystem at compile time to discover all rule files
/// in a directory and generates the necessary module declarations and group
/// registration code.
///
/// # Example
///
/// ```ignore
/// use biome_analyze_macros::declare_group_from_fs;
///
/// declare_group_from_fs! {
///     category: "lint",
///     group: "nursery"
/// }
/// ```
///
/// This will:
/// 1. Scan the nursery/ subdirectory for .rs files
/// 2. Generate pub mod declarations for each rule
/// 3. Generate a declare_lint_group! invocation with all discovered rules
#[proc_macro]
pub fn declare_group_from_fs(input: TokenStream) -> TokenStream {
    group_macro::declare_group_from_fs_impl(input)
}

/// Compresses the documentation of a rule into a byte string literal.
///
/// The macro takes the string literals of the `#[doc]` attributes of a rule,
/// one per line of documentation. It joins them with a line break after each
/// one, and expands to the DEFLATE stream of that text.
/// `biome_analyze::RuleDocs` reads the text back.
///
/// # Example
///
/// ```ignore
/// use biome_analyze_macros::compress_rule_docs;
///
/// const DOCS: &[u8] = compress_rule_docs!(" Disallow the use of `debugger`." "");
/// ```
#[proc_macro]
pub fn compress_rule_docs(input: TokenStream) -> TokenStream {
    rule_docs::compress_rule_docs_impl(input)
}

#[proc_macro_derive(RuleSourceVariantIndex)]
pub fn rule_source_variant_index(input: TokenStream) -> TokenStream {
    rule_source_variant_index::rule_source_variant_index_impl(input)
}
