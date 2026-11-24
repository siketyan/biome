/// Options shared by every rule that has nothing to configure.
#[derive(
    Default,
    Clone,
    Debug,
    serde::Deserialize,
    biome_deserialize_macros::Deserializable,
    biome_deserialize_macros::Merge,
    Eq,
    PartialEq,
    serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct EmptyRuleOptions {}
