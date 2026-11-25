// Options shared by every lint rule that has nothing to configure.
//
// This isn't a doc comment because the JSON schema would repeat it as the
// description of the options of each of these rules.
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
