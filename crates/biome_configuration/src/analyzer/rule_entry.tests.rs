use super::*;
use crate::analyzer::{RuleWithFixOptions, RuleWithOptions};
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_json_parser::JsonParserOptions;

#[derive(
    Clone,
    Debug,
    Default,
    Deserializable,
    Eq,
    Merge,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
struct PlainOptions {
    zeta: Option<bool>,
    alpha: Option<bool>,
}

/// Options whose value depends on the order of the properties of an object.
#[derive(
    Clone,
    Debug,
    Default,
    Deserializable,
    Eq,
    Merge,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
struct OrderedOptions {
    names: Names,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Names(Vec<String>);

impl Merge for Names {
    fn merge_with(&mut self, other: Self) {
        *self = other;
    }
}

impl Deserializable for Names {
    fn deserialize<V: DeserializableValue>(
        _: &mut dyn DeserializationContext<State = V::State>,
        _: &V,
        _: &str,
    ) -> Option<Self> {
        None
    }
}

impl serde::Serialize for Names {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(self.0.iter().map(|name| (name, true)))
    }
}

impl<'de> serde::Deserialize<'de> for Names {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Names;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("an object")
            }

            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                let mut names = Vec::new();
                while let Some((name, _)) = map.next_entry::<String, bool>()? {
                    names.push(name);
                }
                Ok(Names(names))
            }
        }

        deserializer.deserialize_map(Visitor)
    }
}

/// A group with a rule without fix, a rule with a fix, and a rule whose
/// options are ordered.
struct TestTable;

impl LintGroupTable for TestTable {
    const NAME: &'static str = "test";
    const TYPE_NAME: &'static str = "Test";
    const RULES: &'static [&'static str] = &["noFix", "ordered", "withFix"];
    const RULE_IDENTS: &'static [&'static str] = &["no_fix", "ordered", "with_fix"];
    const HAS_FIX: &'static [bool] = &[false, false, true];

    #[cfg(feature = "schema")]
    const HAS_PRESET_IN_SCHEMA: bool = true;

    #[cfg(feature = "schema")]
    fn rule_schema(
        _: usize,
        _: &mut schemars::SchemaGenerator,
    ) -> (&'static str, schemars::Schema) {
        ("", schemars::json_schema!({}))
    }

    fn default_options(index: usize) -> Box<dyn ErasedRuleOptions> {
        match index {
            1 => Box::new(OrderedOptions::default()),
            _ => Box::new(PlainOptions::default()),
        }
    }

    fn options_from_json(
        index: usize,
        json: &str,
    ) -> Result<Box<dyn ErasedRuleOptions>, serde_json::Error> {
        Ok(match index {
            1 => Box::new(serde_json::from_str::<OrderedOptions>(json)?),
            _ => Box::new(serde_json::from_str::<PlainOptions>(json)?),
        })
    }

    fn deserialize_options<S>(
        index: usize,
        ctx: &mut dyn DeserializationContext<State = S>,
        value: &Box<dyn DeserializableValue<State = S>>,
        name: &str,
    ) -> Option<Box<dyn ErasedRuleOptions>> {
        Some(match index {
            1 => Box::new(OrderedOptions::deserialize(ctx, value, name)?),
            _ => Box::new(PlainOptions::deserialize(ctx, value, name)?),
        })
    }
}

type Group = RuleGroupConfig<TestTable>;

const NO_FIX: RuleSlot<TestTable, PlainOptions, false> = RuleSlot::new(0);
const ORDERED: RuleSlot<TestTable, OrderedOptions, false> = RuleSlot::new(1);
const WITH_FIX: RuleSlot<TestTable, PlainOptions, true> = RuleSlot::new(2);

/// Reads a group the way a configuration file is read, and returns it with
/// the number of diagnostics.
fn read(source: &str) -> (Option<Group>, usize) {
    let (group, diagnostics) =
        deserialize_from_json_str::<Group>(source, JsonParserOptions::default(), "").consume();
    (group, diagnostics.len())
}

fn with_options(
    level: RulePlainConfiguration,
    options: PlainOptions,
) -> RuleConfiguration<PlainOptions> {
    RuleConfiguration::WithOptions(RuleWithOptions { level, options })
}

#[test]
fn reads_a_severity() {
    let (group, diagnostics) = read(r#"{ "noFix": "warn", "recommended": false }"#);
    let group = group.unwrap();

    assert_eq!(diagnostics, 0);
    assert_eq!(group.recommended, Some(false));
    assert_eq!(
        group.rule(NO_FIX),
        Some(&RulePlainConfiguration::Warn.into())
    );
    assert_eq!(group.rule_options(NO_FIX), None);
    assert_eq!(group.rule(WITH_FIX), None);
}

#[test]
fn reads_the_level_the_fix_and_the_options() {
    let (group, diagnostics) = read(
        r#"{ "withFix": { "level": "error", "fix": "unsafe", "options": { "alpha": true } } }"#,
    );
    let group = group.unwrap();
    let rule = group.rule(WITH_FIX).unwrap();

    assert_eq!(diagnostics, 0);
    assert_eq!(rule.level(), RulePlainConfiguration::Error);
    assert_eq!(rule.fix(), Some(FixKind::Unsafe));
    assert_eq!(
        group.rule_options(WITH_FIX),
        Some(&PlainOptions {
            zeta: None,
            alpha: Some(true)
        })
    );
}

#[test]
fn defaults_the_options_of_an_object_without_options() {
    let (group, diagnostics) = read(r#"{ "noFix": { "level": "info" } }"#);

    assert_eq!(diagnostics, 0);
    assert_eq!(
        group.unwrap().rule_options(NO_FIX),
        Some(&PlainOptions::default())
    );
}

#[test]
fn ignores_null() {
    let (group, diagnostics) = read(
        r#"{
            "recommended": null,
            "preset": null,
            "noFix": null,
            "withFix": { "level": "error", "fix": null }
        }"#,
    );
    let group = group.unwrap();

    assert_eq!(diagnostics, 0);
    assert_eq!(group.recommended, None);
    assert_eq!(group.preset, None);
    assert_eq!(group.rule(NO_FIX), None);
    assert_eq!(group.rule(WITH_FIX).unwrap().fix(), None);
}

#[test]
fn reports_an_invalid_level_once_and_leaves_the_rule_unset() {
    let (group, diagnostics) = read(r#"{ "noFix": { "level": "errror" } }"#);

    assert_eq!(diagnostics, 1);
    assert_eq!(group.unwrap().rule(NO_FIX), None);
}

#[test]
fn reports_a_missing_level_and_keeps_the_options() {
    let (group, diagnostics) = read(r#"{ "noFix": { "options": { "zeta": true } } }"#);
    let group = group.unwrap();

    assert_eq!(diagnostics, 1);
    assert_eq!(
        group.rule(NO_FIX).unwrap().level(),
        RulePlainConfiguration::Off
    );
    assert_eq!(
        group.rule_options(NO_FIX),
        Some(&PlainOptions {
            zeta: Some(true),
            alpha: None
        })
    );
}

#[test]
fn reports_unknown_keys() {
    assert_eq!(read(r#"{ "unknown": "error" }"#).1, 1);
    assert_eq!(
        read(r#"{ "noFix": { "level": "error", "unknown": true } }"#).1,
        1
    );
    // Only a rule with a fix accepts `fix`.
    assert_eq!(
        read(r#"{ "noFix": { "level": "error", "fix": "safe" } }"#).1,
        1
    );
}

#[test]
fn serializes_like_it_reads() {
    let mut group = Group {
        recommended: Some(true),
        ..Default::default()
    };
    group.set_rule(NO_FIX, RulePlainConfiguration::Warn);
    group.set_rule(
        WITH_FIX,
        RuleFixConfiguration::WithOptions(RuleWithFixOptions {
            level: RulePlainConfiguration::Error,
            fix: Some(FixKind::Safe),
            options: PlainOptions {
                zeta: Some(true),
                alpha: Some(false),
            },
        }),
    );

    // The properties of the options keep the order of their declaration.
    let json = serde_json::to_string(&group).unwrap();
    assert_eq!(
        json,
        r#"{"recommended":true,"noFix":"warn","withFix":{"level":"error","fix":"safe","options":{"zeta":true,"alpha":false}}}"#
    );

    assert_eq!(serde_json::from_str::<Group>(&json).unwrap(), group);
    assert_eq!(read(&json), (Some(group), 0));
}

#[test]
fn serde_keeps_the_order_of_the_properties_of_the_options() {
    let json = r#"{"ordered":{"level":"error","options":{"names":{"zeta":true,"alpha":true}}}}"#;
    let group: Group = serde_json::from_str(json).unwrap();

    assert_eq!(
        group.rule_options(ORDERED).unwrap().names.0,
        ["zeta", "alpha"]
    );
    assert_eq!(serde_json::to_string(&group).unwrap(), json);
}

#[test]
fn serde_rejects_invalid_rules() {
    assert!(serde_json::from_str::<Group>(r#"{"unknown":"error"}"#).is_err());
    assert!(serde_json::from_str::<Group>(r#"{"noFix":{"options":{}}}"#).is_err());
    assert!(serde_json::from_str::<Group>(r#"{"noFix":{"level":"error","fix":"safe"}}"#).is_err());
    assert_eq!(
        serde_json::from_str::<Group>(r#"{"noFix":null,"preset":null}"#).unwrap(),
        Group::default()
    );
}

#[test]
fn overriding_the_severity_keeps_the_options() {
    let options = PlainOptions {
        zeta: Some(true),
        alpha: None,
    };
    let mut group =
        Group::default().with_rule(NO_FIX, with_options(RulePlainConfiguration::Error, options));
    group.merge_with(Group::default().with_rule(NO_FIX, RulePlainConfiguration::Off));

    assert_eq!(
        group.rule(NO_FIX).unwrap().level(),
        RulePlainConfiguration::Off
    );
    assert_eq!(
        group.rule_options(NO_FIX),
        Some(&PlainOptions {
            zeta: Some(true),
            alpha: None
        })
    );
}

#[test]
fn merging_options_merges_them_and_keeps_the_fix_that_is_set() {
    let mut group = Group::default().with_rule(
        WITH_FIX,
        RuleFixConfiguration::WithOptions(RuleWithFixOptions {
            level: RulePlainConfiguration::Warn,
            fix: Some(FixKind::Unsafe),
            options: PlainOptions {
                zeta: Some(true),
                alpha: None,
            },
        }),
    );
    group.merge_with(Group::default().with_rule(
        WITH_FIX,
        RuleFixConfiguration::WithOptions(RuleWithFixOptions {
            level: RulePlainConfiguration::Error,
            fix: None,
            options: PlainOptions {
                zeta: None,
                alpha: Some(true),
            },
        }),
    ));
    let rule = group.rule(WITH_FIX).unwrap();

    assert_eq!(rule.level(), RulePlainConfiguration::Error);
    assert_eq!(rule.fix(), Some(FixKind::Unsafe));
    assert_eq!(
        group.rule_options(WITH_FIX),
        Some(&PlainOptions {
            zeta: Some(true),
            alpha: Some(true)
        })
    );
}

#[test]
fn a_severity_is_replaced_by_the_configuration_it_is_merged_with() {
    let options = PlainOptions {
        zeta: None,
        alpha: Some(true),
    };
    let mut group = Group::default().with_rule(NO_FIX, RulePlainConfiguration::Error);
    group.merge_with(Group::default().with_rule(
        NO_FIX,
        with_options(RulePlainConfiguration::Warn, options.clone()),
    ));

    assert_eq!(
        group.rule(NO_FIX).unwrap().level(),
        RulePlainConfiguration::Warn
    );
    assert_eq!(group.rule_options(NO_FIX), Some(&options));
}

#[test]
fn merging_leaves_the_other_rules_alone() {
    let mut group = Group::default().with_rule(NO_FIX, RulePlainConfiguration::Error);
    group.merge_with(Group::default().with_rule(WITH_FIX, RulePlainConfiguration::Warn));

    assert_eq!(
        group.rule(NO_FIX).unwrap().level(),
        RulePlainConfiguration::Error
    );
    assert_eq!(
        group.rule(WITH_FIX).unwrap().level(),
        RulePlainConfiguration::Warn
    );
}

#[test]
fn groups_are_equal_whatever_the_rules_that_were_unset() {
    let mut group = Group::default();
    group.set_rule(WITH_FIX, RulePlainConfiguration::Error);
    group.unset_rule(WITH_FIX);

    assert_eq!(group, Group::default());
    assert_ne!(
        group,
        Group::default().with_rule(NO_FIX, RulePlainConfiguration::Off)
    );
}

#[test]
fn a_group_severity_configures_every_rule() {
    let group = Group::from_entry(GroupPlainConfiguration::Warn.into());

    assert_eq!(
        group.rule(NO_FIX).unwrap().level(),
        RulePlainConfiguration::Warn
    );
    assert_eq!(
        group.rule(WITH_FIX).unwrap().level(),
        RulePlainConfiguration::Warn
    );
    assert_eq!(
        group.rule(ORDERED).unwrap().level(),
        RulePlainConfiguration::Warn
    );
}

#[test]
fn serde_rejects_a_property_given_twice() {
    assert!(serde_json::from_str::<Group>(r#"{"noFix":"error","noFix":"warn"}"#).is_err());
    assert!(
        serde_json::from_str::<Group>(r#"{"noFix":{"level":"error","level":"warn"}}"#).is_err()
    );
}

#[test]
fn serde_reads_and_writes_values_like_text() {
    let json = r#"{"noFix":"warn","withFix":{"level":"error","fix":"safe","options":{"zeta":true,"alpha":null}}}"#;
    let group: Group = serde_json::from_str(json).unwrap();
    let value = serde_json::to_value(&group).unwrap();

    assert_eq!(
        value,
        serde_json::from_str::<serde_json::Value>(json).unwrap()
    );
    assert_eq!(serde_json::from_value::<Group>(value).unwrap(), group);
}

#[test]
fn a_group_is_read_behind_a_severity_or_group() {
    use crate::analyzer::SeverityOrGroup;

    let json = r#"{"noFix":{"level":"error","options":{"zeta":true,"alpha":null}}}"#;
    let group: Group = serde_json::from_str(json).unwrap();

    assert_eq!(
        serde_json::from_str::<SeverityOrGroup<Group>>(json).unwrap(),
        SeverityOrGroup::Group(group.clone())
    );
    assert_eq!(
        serde_json::from_str::<SeverityOrGroup<Group>>(r#""warn""#).unwrap(),
        SeverityOrGroup::Plain(GroupPlainConfiguration::Warn)
    );
    assert_eq!(
        serde_json::to_string(&SeverityOrGroup::Group(group.clone())).unwrap(),
        json
    );

    let (read, diagnostics) =
        deserialize_from_json_str::<SeverityOrGroup<Group>>(json, JsonParserOptions::default(), "")
            .consume();
    assert_eq!(diagnostics.len(), 0);
    assert_eq!(read, Some(SeverityOrGroup::Group(group)));
}
