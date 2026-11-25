#[macro_export]
macro_rules! declare_transformation {
    ( $( #[doc = $doc:literal] )+ $vis:vis $id:ident {
        version: $version:literal,
        name: $name:tt,
        language: $language:literal,
        $( $key:ident: $value:expr, )*
    } ) => {
        $( #[doc = $doc] )*
        $vis enum $id {}

        impl ::biome_analyze::RuleMeta for $id {
            type Group = $crate::registry::TransformationGroup;
            const METADATA: ::biome_analyze::RuleMetadata =
                ::biome_analyze::RuleMetadata::new(
                    $version,
                    $name,
                    ::biome_analyze::RuleDocs::from_compressed(::biome_analyze::compress_rule_docs!( $( $doc )* )),
                    $language,
                );
        }
    };
}
