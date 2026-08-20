//! Table-driven deserialization of struct-like maps.
//!
//! The [biome_deserialize_macros::Deserializable] derive used to expand the
//! whole member loop — key matching, unknown-key reporting, required-key
//! tracking — into every struct's visitor, which monomorphized the same
//! machinery once per struct. Instead, the derive now emits a table of
//! [DeserializableStructField] entries and calls [deserialize_struct_fields],
//! so the loop is compiled once. Only the per-field assignment functions are
//! generated per struct, and the struct travels as `&mut dyn Any` so that the
//! driver itself is not generic.

use std::any::Any;

use biome_diagnostics::Severity;

use crate::{
    Deserializable, DeserializableValue, DeserializationContext, DeserializationDiagnostic, Text,
    TextRange,
};

/// How a deprecated field is reported.
#[derive(Clone, Copy)]
pub enum DeserializableFieldDeprecation {
    /// The key is deprecated; the message is appended to the diagnostic as a note.
    Message(&'static str),
    /// The key is deprecated in favor of the given key path.
    UseInstead(&'static str),
}

/// One field of a struct in the table passed to [deserialize_struct_fields].
pub struct DeserializableStructField {
    /// The key under which the field is (de)serialized.
    pub key: &'static str,
    /// Deserializes the member value into the field of `target`, which is the
    /// struct being deserialized. Returns whether deserialization succeeded.
    pub deserialize: fn(
        ctx: &mut dyn DeserializationContext,
        value: &Box<dyn DeserializableValue>,
        key_text: &Text,
        target: &mut dyn Any,
    ) -> bool,
    /// Abort the whole struct when this field fails to deserialize.
    pub bail_on_error: bool,
    /// For a required field, its index in the `required_keys` slice passed to
    /// [deserialize_struct_fields].
    pub required_index: Option<u8>,
    /// Set when the field is deprecated.
    pub deprecation: Option<DeserializableFieldDeprecation>,
}

/// What [deserialize_struct_fields] does with keys that match no field.
pub enum UnknownKeyPolicy {
    /// Report an error diagnostic listing the allowed keys.
    Deny(&'static [&'static str]),
    /// Like [Self::Deny], but with warning severity.
    Warn(&'static [&'static str]),
    /// Silently ignore unknown keys.
    Allow,
    /// Collect unknown members through this function
    /// (`#[deserializable(rest)]`).
    Rest(
        fn(
            ctx: &mut dyn DeserializationContext,
            key_text: Text,
            value: &Box<dyn DeserializableValue>,
            target: &mut dyn Any,
        ),
    ),
}

/// Deserializes the members of a map into `target` according to `fields`.
///
/// Returns whether the struct deserialized successfully; on `false` the
/// caller should return `None`. `target` must be the struct type the field
/// table was generated for.
#[expect(clippy::too_many_arguments)]
pub fn deserialize_struct_fields(
    ctx: &mut dyn DeserializationContext,
    members: &mut dyn ExactSizeIterator<
        Item = Option<(Box<dyn DeserializableValue>, Box<dyn DeserializableValue>)>,
    >,
    range: TextRange,
    name: &str,
    target: &mut dyn Any,
    fields: &[DeserializableStructField],
    required_keys: &'static [&'static str],
    unknown_key_policy: UnknownKeyPolicy,
    validator: Option<
        fn(
            ctx: &mut dyn DeserializationContext,
            target: &mut dyn Any,
            name: &str,
            range: TextRange,
        ) -> bool,
    >,
) -> bool {
    debug_assert!(required_keys.len() <= 64);
    let mut seen_required = 0u64;
    for member in members {
        let Some((key, value)) = member else {
            continue;
        };
        let Some(key_text) = Text::deserialize(ctx, &key, "") else {
            continue;
        };
        match fields.iter().find(|field| field.key == key_text.text()) {
            Some(field) => {
                if (field.deserialize)(ctx, &value, &key_text, target) {
                    match field.deprecation {
                        Some(DeserializableFieldDeprecation::Message(message)) => {
                            ctx.report(
                                DeserializationDiagnostic::new_deprecated(
                                    key_text.text(),
                                    value.range(),
                                )
                                .with_note(message),
                            );
                        }
                        Some(DeserializableFieldDeprecation::UseInstead(path)) => {
                            ctx.report(DeserializationDiagnostic::new_deprecated_use_instead(
                                &key_text,
                                key.range(),
                                path,
                            ));
                        }
                        None => {}
                    }
                    if let Some(index) = field.required_index {
                        seen_required |= 1 << index;
                    }
                } else if field.bail_on_error {
                    return false;
                }
            }
            None => match unknown_key_policy {
                UnknownKeyPolicy::Deny(allowed_keys) => {
                    ctx.report(DeserializationDiagnostic::new_unknown_key(
                        key_text.text(),
                        key.range(),
                        allowed_keys,
                    ));
                }
                UnknownKeyPolicy::Warn(allowed_keys) => {
                    ctx.report(
                        DeserializationDiagnostic::new_unknown_key(
                            key_text.text(),
                            key.range(),
                            allowed_keys,
                        )
                        .with_custom_severity(Severity::Warning),
                    );
                }
                UnknownKeyPolicy::Allow => {}
                UnknownKeyPolicy::Rest(collect) => {
                    collect(ctx, key_text, &value, target);
                }
            },
        }
    }
    for (index, key) in required_keys.iter().enumerate() {
        if seen_required & (1 << index) == 0 {
            ctx.report(DeserializationDiagnostic::new_missing_key(
                key,
                range,
                required_keys,
            ));
        }
    }
    if let Some(validate) = validator {
        if !validate(ctx, target, name, range) {
            return false;
        }
    }
    true
}
