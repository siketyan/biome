//! Generated file, do not edit by hand, see `xtask/codegen`

use super::{eslint_to_biome, stylelint_to_biome};
pub(crate) fn migrate_stylelint_any_rule(
    rules: &mut biome_configuration::Rules,
    stylelint_name: &str,
    rule_level: biome_configuration::RulePlainConfiguration,
    options: &eslint_to_biome::MigrationOptions,
    results: &mut stylelint_to_biome::StylelintMigrationResults,
) -> bool {
    match stylelint_name {
        "at-rule-no-unknown" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_UNKNOWN_AT_RULES);
            rule.set_level(rule.level().max(rule_level));
        }
        "block-no-empty" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_EMPTY_BLOCK);
            rule.set_level(rule.level().max(rule_level));
        }
        "color-no-hex" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_HEX_COLORS);
            rule.set_level(rule.level().max(rule_level));
        }
        "csstools/use-logical" => {
            if !options.include_nursery {
                results.add(
                    stylelint_name,
                    eslint_to_biome::RuleMigrationResult::Nursery,
                );
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_LOGICAL_PROPERTIES);
            rule.set_level(rule.level().max(rule_level));
        }
        "custom-property-no-missing-var-function" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_MISSING_VAR_FUNCTION);
            rule.set_level(rule.level().max(rule_level));
        }
        "declaration-block-no-duplicate-custom-properties" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Suspicious::NO_DUPLICATE_CUSTOM_PROPERTIES,
            );
            rule.set_level(rule.level().max(rule_level));
        }
        "declaration-block-no-duplicate-properties" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_PROPERTIES);
            rule.set_level(rule.level().max(rule_level));
        }
        "declaration-block-no-shorthand-property-overrides" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Suspicious::NO_SHORTHAND_PROPERTY_OVERRIDES,
            );
            rule.set_level(rule.level().max(rule_level));
        }
        "declaration-no-important" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_IMPORTANT_STYLES);
            rule.set_level(rule.level().max(rule_level));
        }
        "font-family-no-duplicate-names" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_FONT_NAMES);
            rule.set_level(rule.level().max(rule_level));
        }
        "font-family-no-missing-generic-family-keyword" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_GENERIC_FONT_NAMES);
            rule.set_level(rule.level().max(rule_level));
        }
        "font-weight-notation" => {
            if !options.include_nursery {
                results.add(
                    stylelint_name,
                    eslint_to_biome::RuleMigrationResult::Nursery,
                );
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_CONSISTENT_FONT_WEIGHT_NOTATION,
            );
            rule.set_level(rule.level().max(rule_level));
        }
        "function-linear-gradient-no-nonstandard-direction" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_INVALID_DIRECTION_IN_LINEAR_GRADIENT,
            );
            rule.set_level(rule.level().max(rule_level));
        }
        "function-no-unknown" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNKNOWN_FUNCTION);
            rule.set_level(rule.level().max(rule_level));
        }
        "keyframe-block-no-duplicate-selectors" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Suspicious::NO_DUPLICATE_SELECTORS_KEYFRAME_BLOCK,
            );
            rule.set_level(rule.level().max(rule_level));
        }
        "keyframe-declaration-no-important" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_IMPORTANT_IN_KEYFRAME);
            rule.set_level(rule.level().max(rule_level));
        }
        "length-zero-no-unit" => {
            if !options.include_nursery {
                results.add(
                    stylelint_name,
                    eslint_to_biome::RuleMigrationResult::Nursery,
                );
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_ZERO_LENGTH_UNIT);
            rule.set_level(rule.level().max(rule_level));
        }
        "liberty/use-logical-spec" => {
            if !options.include_nursery {
                results.add(
                    stylelint_name,
                    eslint_to_biome::RuleMigrationResult::Nursery,
                );
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_LOGICAL_PROPERTIES);
            rule.set_level(rule.level().max(rule_level));
        }
        "lightness-notation" => {
            if !options.include_nursery {
                results.add(
                    stylelint_name,
                    eslint_to_biome::RuleMigrationResult::Nursery,
                );
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_CONSISTENT_LIGHTNESS_NOTATION,
            );
            rule.set_level(rule.level().max(rule_level));
        }
        "max-nesting-depth" => {
            if !options.include_nursery {
                results.add(
                    stylelint_name,
                    eslint_to_biome::RuleMigrationResult::Nursery,
                );
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_EXCESSIVE_NESTED_BLOCKS);
            rule.set_level(rule.level().max(rule_level));
        }
        "media-feature-name-no-unknown" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_UNKNOWN_MEDIA_FEATURE_NAME,
            );
            rule.set_level(rule.level().max(rule_level));
        }
        "media-type-no-deprecated" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DEPRECATED_MEDIA_TYPE);
            rule.set_level(rule.level().max(rule_level));
        }
        "named-grid-areas-no-invalid" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_INVALID_GRID_AREAS);
            rule.set_level(rule.level().max(rule_level));
        }
        "no-descending-specificity" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_DESCENDING_SPECIFICITY);
            rule.set_level(rule.level().max(rule_level));
        }
        "no-duplicate-at-import-rules" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_AT_IMPORT_RULES);
            rule.set_level(rule.level().max(rule_level));
        }
        "no-duplicate-selectors" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_SELECTORS);
            rule.set_level(rule.level().max(rule_level));
        }
        "no-empty-source" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_EMPTY_SOURCE);
            rule.set_level(rule.level().max(rule_level));
        }
        "no-invalid-position-at-import-rule" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_INVALID_POSITION_AT_IMPORT_RULE,
            );
            rule.set_level(rule.level().max(rule_level));
        }
        "no-irregular-whitespace" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_IRREGULAR_WHITESPACE);
            rule.set_level(rule.level().max(rule_level));
        }
        "property-no-unknown" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNKNOWN_PROPERTY);
            rule.set_level(rule.level().max(rule_level));
        }
        "scss/at-if-no-null" => {
            if !options.include_inspired {
                results.add(
                    stylelint_name,
                    eslint_to_biome::RuleMigrationResult::Inspired,
                );
                return false;
            }
            if !options.include_nursery {
                results.add(
                    stylelint_name,
                    eslint_to_biome::RuleMigrationResult::Nursery,
                );
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_SCSS_NULL_COMPARISON);
            rule.set_level(rule.level().max(rule_level));
        }
        "scss/dollar-variable-no-missing-interpolation" => {
            if !options.include_inspired {
                results.add(
                    stylelint_name,
                    eslint_to_biome::RuleMigrationResult::Inspired,
                );
                return false;
            }
            if !options.include_nursery {
                results.add(
                    stylelint_name,
                    eslint_to_biome::RuleMigrationResult::Nursery,
                );
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_MISSING_SCSS_INTERPOLATION);
            rule.set_level(rule.level().max(rule_level));
        }
        "selector-anb-no-unmatchable" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNMATCHABLE_ANB_SELECTOR);
            rule.set_level(rule.level().max(rule_level));
        }
        "selector-max-class" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_EXCESSIVE_SELECTOR_CLASSES);
            rule.set_level(rule.level().max(rule_level));
        }
        "selector-pseudo-class-no-unknown" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNKNOWN_PSEUDO_CLASS);
            rule.set_level(rule.level().max(rule_level));
        }
        "selector-pseudo-element-no-unknown" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNKNOWN_PSEUDO_ELEMENT);
            rule.set_level(rule.level().max(rule_level));
        }
        "selector-type-no-unknown" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNKNOWN_TYPE_SELECTOR);
            rule.set_level(rule.level().max(rule_level));
        }
        "time-min-milliseconds" => {
            if !options.include_nursery {
                results.add(
                    stylelint_name,
                    eslint_to_biome::RuleMigrationResult::Nursery,
                );
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_SHORT_TIME_VALUES);
            rule.set_level(rule.level().max(rule_level));
        }
        "unit-no-unknown" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNKNOWN_UNIT);
            rule.set_level(rule.level().max(rule_level));
        }
        _ => {
            results.add(
                stylelint_name,
                eslint_to_biome::RuleMigrationResult::Unsupported,
            );
            return false;
        }
    }
    results.add(
        stylelint_name,
        eslint_to_biome::RuleMigrationResult::Migrated,
    );
    true
}
