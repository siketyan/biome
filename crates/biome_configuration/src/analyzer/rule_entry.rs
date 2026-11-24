//! Table-driven storage for the configuration of the rules of a lint group.
//!
//! A group stores one optional [`RuleEntry`] per rule, indexed by the position
//! of the rule in [`LintGroupTable::RULES`]. The options of a rule are stored
//! behind [`ErasedRuleOptions`], so the code that reads, writes, compares and
//! merges a group is compiled once per group instead of once per rule.

use super::{
    GroupPlainConfiguration, RuleConfiguration, RuleFixConfiguration, RulePlainConfiguration,
};
use crate::analyzer::presets::PresetConfig;
use biome_analyze::FixKind;
use biome_analyze::options::RuleOptions;
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableTypes, DeserializableValue,
    DeserializationContext, DeserializationDiagnostic, DeserializationVisitor, MapMembers, Merge,
    Text, TextRange,
};
use serde::ser::SerializeMap;
use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

/// Options of a rule, with the concrete type erased.
pub trait ErasedRuleOptions: Any + Send + Sync + Debug {
    fn clone_box(&self) -> Box<dyn ErasedRuleOptions>;
    /// Returns `false` when `other` holds a different concrete type.
    fn eq_dyn(&self, other: &dyn ErasedRuleOptions) -> bool;
    /// Merges `other` into `self`. Does nothing when `other` holds a different
    /// concrete type.
    fn merge_dyn(&mut self, other: Box<dyn ErasedRuleOptions>);
    fn to_json(&self) -> SerializedOptions;
    fn to_rule_options(&self, fix: Option<FixKind>) -> RuleOptions;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T> ErasedRuleOptions for T
where
    T: Any + Send + Sync + Debug + Clone + PartialEq + Merge + serde::Serialize,
{
    fn clone_box(&self) -> Box<dyn ErasedRuleOptions> {
        Box::new(self.clone())
    }

    fn eq_dyn(&self, other: &dyn ErasedRuleOptions) -> bool {
        other
            .as_any()
            .downcast_ref::<T>()
            .is_some_and(|other| self == other)
    }

    fn merge_dyn(&mut self, other: Box<dyn ErasedRuleOptions>) {
        if let Ok(other) = other.into_any().downcast::<T>() {
            self.merge_with(*other);
        }
    }

    fn to_json(&self) -> SerializedOptions {
        serde_json::to_string(self)
            .ok()
            .and_then(|json| serde_json::from_str(&json).ok())
            .unwrap_or(SerializedOptions::Null)
    }

    fn to_rule_options(&self, fix: Option<FixKind>) -> RuleOptions {
        RuleOptions::new(self.clone(), fix)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

/// The options of a rule in a form that any serializer can write.
///
/// Unlike [serde_json::Value], it keeps the properties of an object in the
/// order in which the options serialize them.
pub enum SerializedOptions {
    Null,
    Bool(bool),
    Number(serde_json::Number),
    String(String),
    Array(Vec<Self>),
    Object(Vec<(String, Self)>),
}

impl serde::Serialize for SerializedOptions {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Null => serializer.serialize_unit(),
            Self::Bool(value) => serializer.serialize_bool(*value),
            Self::Number(value) => value.serialize(serializer),
            Self::String(value) => serializer.serialize_str(value),
            Self::Array(items) => serializer.collect_seq(items),
            Self::Object(members) => {
                let mut map = serializer.serialize_map(Some(members.len()))?;
                for (key, value) in members {
                    map.serialize_entry(key, value)?;
                }
                map.end()
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for SerializedOptions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SerializedOptions;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a JSON value")
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(SerializedOptions::Null)
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
                Ok(SerializedOptions::Bool(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
                Ok(SerializedOptions::Number(value.into()))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
                Ok(SerializedOptions::Number(value.into()))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
                Ok(serde_json::Number::from_f64(value)
                    .map_or(SerializedOptions::Null, SerializedOptions::Number))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
                Ok(SerializedOptions::String(value.to_string()))
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut seq: A,
            ) -> Result<Self::Value, A::Error> {
                let mut items = Vec::new();
                while let Some(item) = seq.next_element()? {
                    items.push(item);
                }
                Ok(SerializedOptions::Array(items))
            }

            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                let mut members = Vec::new();
                while let Some(member) = map.next_entry()? {
                    members.push(member);
                }
                Ok(SerializedOptions::Object(members))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

/// Configuration of a single lint rule.
///
/// `options` is `None` when the rule is configured with a bare severity, and
/// `Some` when it's configured with an object. `fix` is only meaningful in the
/// latter form.
#[derive(Debug, Default)]
pub struct RuleEntry {
    level: RulePlainConfiguration,
    fix: Option<FixKind>,
    options: Option<Box<dyn ErasedRuleOptions>>,
}

impl RuleEntry {
    pub fn new_with_options<T: ErasedRuleOptions>(
        level: RulePlainConfiguration,
        fix: Option<FixKind>,
        options: T,
    ) -> Self {
        Self {
            level,
            fix,
            options: Some(Box::new(options)),
        }
    }

    pub fn is_disabled(&self) -> bool {
        matches!(self.level, RulePlainConfiguration::Off)
    }

    pub fn is_enabled(&self) -> bool {
        !self.is_disabled()
    }

    pub fn level(&self) -> RulePlainConfiguration {
        self.level
    }

    pub fn set_level(&mut self, level: RulePlainConfiguration) {
        self.level = level;
    }

    pub fn fix(&self) -> Option<FixKind> {
        self.fix
    }

    /// Returns the options when the rule is configured with an object and the
    /// options are of type `T`.
    pub fn options<T: 'static>(&self) -> Option<&T> {
        self.options.as_ref()?.as_any().downcast_ref()
    }

    /// Mutable counterpart of [`Self::options`].
    pub fn options_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.options.as_mut()?.as_any_mut().downcast_mut()
    }

    pub fn get_options(&self) -> Option<RuleOptions> {
        self.options
            .as_ref()
            .map(|options| options.to_rule_options(self.fix))
    }
}

impl Clone for RuleEntry {
    fn clone(&self) -> Self {
        Self {
            level: self.level,
            fix: self.fix,
            options: self.options.as_ref().map(|options| options.clone_box()),
        }
    }
}

impl PartialEq for RuleEntry {
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level
            && self.fix == other.fix
            && match (&self.options, &other.options) {
                (None, None) => true,
                (Some(this), Some(other)) => this.eq_dyn(other.as_ref()),
                _ => false,
            }
    }
}

impl Eq for RuleEntry {}

// Overriding the severity doesn't override the options.
impl Merge for RuleEntry {
    fn merge_with(&mut self, other: Self) {
        let Some(options) = self.options.as_mut() else {
            *self = other;
            return;
        };
        self.level = other.level;
        if let Some(other_options) = other.options {
            self.fix = other.fix.or(self.fix);
            options.merge_dyn(other_options);
        }
    }
}

impl From<RulePlainConfiguration> for RuleEntry {
    fn from(level: RulePlainConfiguration) -> Self {
        Self {
            level,
            fix: None,
            options: None,
        }
    }
}

impl From<GroupPlainConfiguration> for RuleEntry {
    fn from(value: GroupPlainConfiguration) -> Self {
        Self::from(RulePlainConfiguration::from(value))
    }
}

impl<T: Default + Merge + ErasedRuleOptions> From<RuleConfiguration<T>> for RuleEntry {
    fn from(value: RuleConfiguration<T>) -> Self {
        match value {
            RuleConfiguration::Plain(level) => level.into(),
            RuleConfiguration::WithOptions(rule) => {
                Self::new_with_options(rule.level, None, rule.options)
            }
        }
    }
}

impl<T: Default + Merge + ErasedRuleOptions> From<RuleFixConfiguration<T>> for RuleEntry {
    fn from(value: RuleFixConfiguration<T>) -> Self {
        match value {
            RuleFixConfiguration::Plain(level) => level.into(),
            RuleFixConfiguration::WithOptions(rule) => {
                Self::new_with_options(rule.level, rule.fix, rule.options)
            }
        }
    }
}

impl serde::Serialize for RuleEntry {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let Some(options) = &self.options else {
            return self.level.serialize(serializer);
        };
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("level", &self.level)?;
        if let Some(fix) = &self.fix {
            map.serialize_entry("fix", fix)?;
        }
        map.serialize_entry("options", &options.to_json())?;
        map.end()
    }
}

/// Static description of the rules of a lint group.
///
/// `RULES`, `RULE_IDENTS` and `HAS_FIX` have the same length and are indexed by
/// the position of the rule in the group. `RULES` is sorted.
pub trait LintGroupTable: 'static {
    const NAME: &'static str;
    /// Names of the rules, as written in the configuration file.
    const RULES: &'static [&'static str];
    /// Names of the rules in snake case.
    const RULE_IDENTS: &'static [&'static str];
    const HAS_FIX: &'static [bool];

    fn default_options(index: usize) -> Box<dyn ErasedRuleOptions>;

    fn options_from_json(
        index: usize,
        value: serde_json::Value,
    ) -> Result<Box<dyn ErasedRuleOptions>, serde_json::Error>;

    #[expect(
        clippy::borrowed_box,
        reason = "`Deserializable::deserialize` requires a sized value, which the box is and the trait object isn't"
    )]
    fn deserialize_options<S>(
        index: usize,
        ctx: &mut dyn DeserializationContext<State = S>,
        value: &Box<dyn DeserializableValue<State = S>>,
        name: &str,
    ) -> Option<Box<dyn ErasedRuleOptions>>;
}

/// Configuration of all the rules of the lint group described by `G`.
pub struct RuleGroupConfig<G> {
    pub recommended: Option<bool>,
    pub preset: Option<PresetConfig>,
    /// Indexed by the position of the rule in the group. Can be shorter than
    /// the number of rules: the missing entries are unset.
    rules: Vec<Option<RuleEntry>>,
    table: PhantomData<fn() -> G>,
}

impl<G> Default for RuleGroupConfig<G> {
    fn default() -> Self {
        Self {
            recommended: None,
            preset: None,
            rules: Vec::new(),
            table: PhantomData,
        }
    }
}

impl<G> Clone for RuleGroupConfig<G> {
    fn clone(&self) -> Self {
        Self {
            recommended: self.recommended,
            preset: self.preset.clone(),
            rules: self.rules.clone(),
            table: PhantomData,
        }
    }
}

impl<G> PartialEq for RuleGroupConfig<G> {
    fn eq(&self, other: &Self) -> bool {
        let len = self.rules.len().max(other.rules.len());
        self.recommended == other.recommended
            && self.preset == other.preset
            && (0..len).all(|index| self.get_at(index) == other.get_at(index))
    }
}

impl<G> Eq for RuleGroupConfig<G> {}

impl<G> RuleGroupConfig<G> {
    pub fn get_at(&self, index: usize) -> Option<&RuleEntry> {
        self.rules.get(index)?.as_ref()
    }

    pub fn slot_at(&mut self, index: usize) -> &mut Option<RuleEntry> {
        if self.rules.len() <= index {
            self.rules.resize_with(index + 1, || None);
        }
        &mut self.rules[index]
    }

    /// Returns the configured rules with their position in the group.
    pub fn entries(&self) -> impl Iterator<Item = (usize, &RuleEntry)> {
        self.rules
            .iter()
            .enumerate()
            .filter_map(|(index, entry)| Some((index, entry.as_ref()?)))
    }
}

impl<G: LintGroupTable> RuleGroupConfig<G> {
    /// Returns a group where every rule is configured with `entry`.
    pub fn from_entry(entry: RuleEntry) -> Self {
        Self {
            recommended: None,
            preset: None,
            rules: vec![Some(entry); G::RULES.len()],
            table: PhantomData,
        }
    }

    fn index_of_ident(ident: &str) -> usize {
        G::RULE_IDENTS
            .iter()
            .position(|candidate| *candidate == ident)
            .unwrap_or_else(|| panic!("the group {} has no rule {ident}", G::NAME))
    }

    /// Returns the configuration of the rule named `name` in the configuration file.
    pub fn get(&self, name: &str) -> Option<&RuleEntry> {
        self.get_at(G::RULES.binary_search(&name).ok()?)
    }

    /// Returns the configuration of the rule whose snake case name is `ident`.
    ///
    /// Panics when the group has no such rule.
    pub fn rule(&self, ident: &str) -> Option<&RuleEntry> {
        self.get_at(Self::index_of_ident(ident))
    }

    /// Returns the slot of the rule whose snake case name is `ident`.
    ///
    /// Panics when the group has no such rule.
    pub fn rule_slot(&mut self, ident: &str) -> &mut Option<RuleEntry> {
        self.slot_at(Self::index_of_ident(ident))
    }

    /// Configures the rule whose snake case name is `ident`.
    ///
    /// Panics when the group has no such rule.
    pub fn set_rule(&mut self, ident: &str, entry: impl Into<RuleEntry>) {
        *self.rule_slot(ident) = Some(entry.into());
    }

    /// Returns the group with the rule whose snake case name is `ident`
    /// configured with `entry`.
    ///
    /// Panics when the group has no such rule.
    pub fn with_rule(mut self, ident: &str, entry: impl Into<RuleEntry>) -> Self {
        self.set_rule(ident, entry);
        self
    }

    /// Returns the configuration of the rule whose snake case name is `ident`,
    /// configuring the rule with its default configuration when it's unset.
    ///
    /// Panics when the group has no such rule.
    pub fn rule_mut(&mut self, ident: &str) -> &mut RuleEntry {
        self.rule_slot(ident).get_or_insert_with(RuleEntry::default)
    }
}

impl<G: LintGroupTable> Debug for RuleGroupConfig<G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct(G::NAME);
        debug.field("recommended", &self.recommended);
        debug.field("preset", &self.preset);
        for (index, entry) in self.entries() {
            debug.field(G::RULE_IDENTS[index], entry);
        }
        debug.finish()
    }
}

impl<G> Merge for RuleGroupConfig<G> {
    fn merge_with(&mut self, other: Self) {
        self.recommended.merge_with(other.recommended);
        self.preset.merge_with(other.preset);
        for (index, entry) in other.rules.into_iter().enumerate() {
            if entry.is_some() {
                self.slot_at(index).merge_with(entry);
            }
        }
    }
}

impl<G: LintGroupTable> serde::Serialize for RuleGroupConfig<G> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if let Some(recommended) = &self.recommended {
            map.serialize_entry("recommended", recommended)?;
        }
        if let Some(preset) = &self.preset {
            map.serialize_entry("preset", preset)?;
        }
        for (index, entry) in self.entries() {
            map.serialize_entry(G::RULES[index], entry)?;
        }
        map.end()
    }
}

impl<'de, G: LintGroupTable> serde::Deserialize<'de> for RuleGroupConfig<G> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let map = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
        Self::from_json(map).map_err(serde::de::Error::custom)
    }
}

impl<G: LintGroupTable> RuleGroupConfig<G> {
    fn from_json(map: serde_json::Map<String, serde_json::Value>) -> Result<Self, String> {
        let mut result = Self::default();
        for (key, value) in map {
            match key.as_str() {
                "recommended" => {
                    result.recommended =
                        serde_json::from_value(value).map_err(|error| error.to_string())?;
                }
                "preset" => {
                    result.preset =
                        serde_json::from_value(value).map_err(|error| error.to_string())?;
                }
                name => {
                    let Ok(index) = G::RULES.binary_search(&name) else {
                        return Err(format!("unknown field `{name}`"));
                    };
                    if !value.is_null() {
                        *result.slot_at(index) = Some(Self::entry_from_json(index, value)?);
                    }
                }
            }
        }
        Ok(result)
    }

    fn entry_from_json(index: usize, value: serde_json::Value) -> Result<RuleEntry, String> {
        let members = match value {
            serde_json::Value::Object(members) => members,
            value => {
                let level: RulePlainConfiguration =
                    serde_json::from_value(value).map_err(|error| error.to_string())?;
                return Ok(level.into());
            }
        };
        let mut level = None;
        let mut fix = None;
        let mut options = None;
        for (key, value) in members {
            match key.as_str() {
                "level" => {
                    level = Some(serde_json::from_value(value).map_err(|error| error.to_string())?);
                }
                "fix" if G::HAS_FIX[index] => {
                    fix = serde_json::from_value(value).map_err(|error| error.to_string())?;
                }
                "options" => {
                    options = Some(
                        G::options_from_json(index, value).map_err(|error| error.to_string())?,
                    );
                }
                name => return Err(format!("unknown field `{name}`")),
            }
        }
        Ok(RuleEntry {
            level: level.ok_or("missing field `level`")?,
            fix,
            options: Some(options.unwrap_or_else(|| G::default_options(index))),
        })
    }
}

impl<G: LintGroupTable> Deserializable for RuleGroupConfig<G> {
    fn deserialize<V: DeserializableValue>(
        ctx: &mut dyn DeserializationContext<State = V::State>,
        value: &V,
        name: &str,
    ) -> Option<Self> {
        struct Visitor<G>(PhantomData<fn() -> G>);
        impl<S, G: LintGroupTable> DeserializationVisitor<S> for Visitor<G> {
            type Output = RuleGroupConfig<G>;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;
            fn visit_map(
                self,
                ctx: &mut dyn DeserializationContext<State = S>,
                members: &mut MapMembers<'_, S>,
                _range: TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                let mut result = RuleGroupConfig::<G>::default();
                for (key, value) in members.flatten() {
                    let Some(key_text) = Text::deserialize(ctx, &key, "") else {
                        continue;
                    };
                    let key_text = key_text.text();
                    match key_text {
                        "recommended" => {
                            if let Some(value) = Deserializable::deserialize(ctx, &value, key_text)
                            {
                                result.recommended = Some(value);
                            }
                        }
                        "preset" => {
                            if let Some(value) = Deserializable::deserialize(ctx, &value, key_text)
                            {
                                result.preset = Some(value);
                            }
                        }
                        rule_name => {
                            let Ok(index) = G::RULES.binary_search(&rule_name) else {
                                let allowed_keys: Vec<_> = ["recommended", "preset"]
                                    .into_iter()
                                    .chain(G::RULES.iter().copied())
                                    .collect();
                                ctx.report(DeserializationDiagnostic::new_unknown_key(
                                    rule_name,
                                    key.range(),
                                    &allowed_keys,
                                ));
                                continue;
                            };
                            if let Some(entry) = deserialize_entry::<G, S>(index, ctx, &value) {
                                *result.slot_at(index) = Some(entry);
                            }
                        }
                    }
                }
                Some(result)
            }
        }

        value.deserialize(ctx, Visitor(PhantomData), name)
    }
}

#[expect(
    clippy::borrowed_box,
    reason = "`Deserializable::deserialize` requires a sized value, which the box is and the trait object isn't"
)]
fn deserialize_entry<G: LintGroupTable, S>(
    index: usize,
    ctx: &mut dyn DeserializationContext<State = S>,
    value: &Box<dyn DeserializableValue<State = S>>,
) -> Option<RuleEntry> {
    struct Visitor<G>(usize, PhantomData<fn() -> G>);
    impl<S, G: LintGroupTable> DeserializationVisitor<S> for Visitor<G> {
        type Output = RuleEntry;
        const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;
        fn visit_map(
            self,
            ctx: &mut dyn DeserializationContext<State = S>,
            members: &mut MapMembers<'_, S>,
            range: TextRange,
            _name: &str,
        ) -> Option<Self::Output> {
            let index = self.0;
            let mut level = None;
            let mut fix = None;
            let mut options = None;
            for (key, value) in members.flatten() {
                let Some(key_text) = Text::deserialize(ctx, &key, "") else {
                    continue;
                };
                let key_text = key_text.text();
                match key_text {
                    "level" => level = Deserializable::deserialize(ctx, &value, key_text),
                    "fix" if G::HAS_FIX[index] => {
                        fix = Deserializable::deserialize(ctx, &value, key_text);
                    }
                    "options" => {
                        options = G::deserialize_options(index, ctx, &value, key_text);
                    }
                    unknown_key => {
                        let allowed_keys: &[&str] = if G::HAS_FIX[index] {
                            &["level", "fix", "options"]
                        } else {
                            &["level", "options"]
                        };
                        ctx.report(DeserializationDiagnostic::new_unknown_key(
                            unknown_key,
                            key.range(),
                            allowed_keys,
                        ));
                    }
                }
            }
            let Some(level) = level else {
                ctx.report(DeserializationDiagnostic::new_missing_key(
                    "level",
                    range,
                    &["level"],
                ));
                return None;
            };
            Some(RuleEntry {
                level,
                fix,
                options: Some(options.unwrap_or_else(|| G::default_options(index))),
            })
        }
    }

    let name = G::RULES[index];
    if value.visitable_type(ctx)? == DeserializableType::Str {
        RulePlainConfiguration::deserialize(ctx, value, name).map(RuleEntry::from)
    } else {
        value.deserialize(ctx, Visitor::<G>(index, PhantomData), name)
    }
}

#[cfg(feature = "schema")]
impl<G: LintGroupTable> schemars::JsonSchema for RuleGroupConfig<G> {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(G::NAME)
    }

    // ponytail: placeholder schema, the per-rule properties need to be generated from the group table.
    fn json_schema(_generator: &mut schemars::generate::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type": "object" })
    }
}
