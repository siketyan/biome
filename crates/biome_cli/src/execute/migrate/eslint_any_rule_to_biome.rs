//! Generated file, do not edit by hand, see `xtask/codegen`

use super::{eslint_eslint, eslint_to_biome};
pub(crate) fn migrate_eslint_any_rule(
    rules: &mut biome_configuration::Rules,
    eslint_name: &str,
    rule_severity: eslint_eslint::Severity,
    options: &eslint_to_biome::MigrationOptions,
    results: &mut eslint_to_biome::MigrationResults,
) -> bool {
    match eslint_name {
        "@eslint-react/component-hook-factories" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_COMPONENT_HOOK_FACTORIES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/dom-no-dangerously-set-innerhtml" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::NO_DANGEROUSLY_SET_INNER_HTML);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/dom-no-dangerously-set-innerhtml-with-children" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group . unwrap_group_as_mut () . rule_mut (biome_configuration :: analyzer :: Security :: NO_DANGEROUSLY_SET_INNER_HTML_WITH_CHILDREN) ;
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/dom-no-missing-button-type" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_BUTTON_TYPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/dom-no-missing-iframe-sandbox" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::USE_IFRAME_SANDBOX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/dom-no-render-return-value" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_RENDER_RETURN_VALUE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/dom-no-script-url" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::NO_SCRIPT_URL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/dom-no-unsafe-iframe-sandbox" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_UNSAFE_IFRAME_SANDBOX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/dom-no-unsafe-target-blank" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::NO_BLANK_TARGET);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/dom-no-void-elements-with-children" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_VOID_ELEMENTS_WITH_CHILDREN,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/exhaustive-deps" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_EXHAUSTIVE_DEPENDENCIES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/jsx-no-children-prop" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_CHILDREN_PROP);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/jsx-no-leaked-dollar" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_JSX_LEAKED_DOLLAR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/jsx-no-namespace" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_JSX_NAMESPACE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/jsx-no-useless-fragment" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_FRAGMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/naming-convention-context-name" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_REACT_NAMING_CONVENTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/naming-convention-id-name" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_REACT_NAMING_CONVENTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/naming-convention-ref-name" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_REACT_NAMING_CONVENTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/no-array-index-key" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_ARRAY_INDEX_KEY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/no-direct-mutation-state" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_REACT_DIRECT_STATE_MUTATION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/no-forward-ref" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_REACT_FORWARD_REF);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/no-missing-key" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_JSX_KEY_IN_ITERABLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/no-nested-component-definitions" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_NESTED_COMPONENT_DEFINITIONS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/rsc-function-definition" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::USE_REACT_ASYNC_SERVER_FUNCTION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/description-style" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Style::USE_CONSISTENT_GRAPHQL_DESCRIPTIONS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/input-name" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_INPUT_NAME);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/lone-anonymous-operation" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_LONE_ANONYMOUS_OPERATION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/lone-executable-definition" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_LONE_EXECUTABLE_DEFINITION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/no-anonymous-operations" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_GRAPHQL_NAMED_OPERATIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/no-duplicate-fields" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_FIELDS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/no-root-type" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_ROOT_TYPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/require-deprecation-date" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::USE_DEPRECATED_DATE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/require-deprecation-reason" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_DEPRECATED_REASON);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/unique-argument-names" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_DUPLICATE_ARGUMENT_NAMES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/unique-enum-value-names" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_DUPLICATE_ENUM_VALUE_NAMES,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/unique-field-definition-names" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_DUPLICATE_FIELD_DEFINITION_NAMES,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/unique-input-field-names" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_DUPLICATE_INPUT_FIELD_NAMES,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/unique-operation-name" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Suspicious::NO_DUPLICATE_GRAPHQL_OPERATION_NAME,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/unique-variable-names" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_DUPLICATE_VARIABLE_NAMES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-accesskey-attrs" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_ACCESS_KEY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-aria-hidden-on-focusable" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_ARIA_HIDDEN_ON_FOCUSABLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-duplicate-attrs" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_DUPLICATE_ATTRIBUTES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-empty-headings" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_HEADING_CONTENT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-inline-styles" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_INLINE_STYLES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-invalid-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_VALID_ARIA_ROLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-non-scalable-viewport" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_NON_SCALABLE_VIEWPORT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-obsolete-attrs" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_OBSOLETE_ATTRIBUTES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-obsolete-tags" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_OBSOLETE_TAGS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-positive-tabindex" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_POSITIVE_TABINDEX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-redundant-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_REDUNDANT_ROLES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-target-blank" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::NO_BLANK_TARGET);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/require-button-type" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_BUTTON_TYPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/require-frame-title" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_IFRAME_TITLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/require-img-alt" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_ALT_TEXT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/require-lang" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_HTML_LANG);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/require-li-container" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_MISPLACED_LIST_ELEMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/use-baseline" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::USE_BASELINE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@mysticatea/no-this-in-static" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_THIS_IN_STATIC);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@mysticatea/prefer-for-of" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_FOR_OF);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/google-font-display" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::USE_GOOGLE_FONT_DISPLAY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/google-font-preconnect" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Performance::USE_GOOGLE_FONT_PRECONNECT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/inline-script-id" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_INLINE_SCRIPT_ID);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/next-script-for-ga" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_NEXT_SCRIPT_FOR_GOOGLE_ANALYTICS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/no-async-client-component" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_NEXT_ASYNC_CLIENT_COMPONENT,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/no-before-interactive-script-outside-document" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group . unwrap_group_as_mut () . rule_mut (biome_configuration :: analyzer :: Correctness :: NO_BEFORE_INTERACTIVE_SCRIPT_OUTSIDE_DOCUMENT) ;
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/no-document-import-in-page" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DOCUMENT_IMPORT_IN_PAGE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/no-head-element" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_HEAD_ELEMENT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/no-head-import-in-document" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_HEAD_IMPORT_IN_DOCUMENT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/no-img-element" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Performance::NO_IMG_ELEMENT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/no-sync-scripts" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Performance::NO_SYNC_SCRIPTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/no-unwanted-polyfillio" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Performance::NO_UNWANTED_POLYFILLIO);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@react-native/no-deep-imports" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_REACT_NATIVE_DEEP_IMPORTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@react-native/platform-colors" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_REACT_NATIVE_VALID_PLATFORM_COLORS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@stylistic/jsx-curly-brace-presence" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_CONSISTENT_CURLY_BRACES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@stylistic/jsx-self-closing-comp" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_SELF_CLOSING_ELEMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@tanstack/query/exhaustive-deps" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_TANSTACK_QUERY_EXHAUSTIVE_DEPENDENCIES,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@tanstack/query/infinite-query-property-order" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group . unwrap_group_as_mut () . rule_mut (biome_configuration :: analyzer :: Nursery :: USE_TANSTACK_QUERY_INFINITE_QUERY_PROPERTY_ORDER) ;
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@tanstack/query/mutation-property-order" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_TANSTACK_QUERY_MUTATION_PROPERTY_ORDER,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@tanstack/query/no-rest-destructuring" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_TANSTACK_QUERY_REST_DESTRUCTURING,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@tanstack/query/no-void-query-fn" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_TANSTACK_QUERY_VOID_QUERY_FN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@tanstack/query/stable-query-client" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_TANSTACK_QUERY_STABLE_QUERY_CLIENT,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/adjacent-overload-signatures" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Suspicious::USE_ADJACENT_OVERLOAD_SIGNATURES,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/array-type" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_CONSISTENT_ARRAY_TYPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/ban-ts-comment" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_TS_IGNORE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/ban-types" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_BANNED_TYPES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/consistent-indexed-object-style" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_CONSISTENT_INDEXED_OBJECT_STYLE,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/consistent-type-definitions" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_CONSISTENT_TYPE_DEFINITIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/consistent-type-exports" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_EXPORT_TYPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/consistent-type-imports" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_IMPORT_TYPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/default-param-last" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_DEFAULT_PARAMETER_LAST);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/dot-notation" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_LITERAL_KEYS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/explicit-function-return-type" => {
            let mut migrated = false;
            {
                let mut blocked = false;
                if !options.include_inspired {
                    results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                    blocked = true;
                }
                if !options.include_nursery {
                    results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                    blocked = true;
                }
                if !blocked {
                    let group = rules.nursery.get_or_insert_with(Default::default);
                    let rule = group
                        .unwrap_group_as_mut()
                        .rule_mut(biome_configuration::analyzer::Nursery::USE_EXPLICIT_RETURN_TYPE);
                    rule.set_level(rule.level().max(rule_severity.into()));
                    migrated = true;
                }
            }
            {
                let mut blocked = false;
                if !options.include_inspired {
                    results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                    blocked = true;
                }
                if !options.include_nursery {
                    results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                    blocked = true;
                }
                if !blocked {
                    let group = rules.nursery.get_or_insert_with(Default::default);
                    let rule = group
                        .unwrap_group_as_mut()
                        .rule_mut(biome_configuration::analyzer::Nursery::USE_EXPLICIT_TYPE);
                    rule.set_level(rule.level().max(rule_severity.into()));
                    migrated = true;
                }
            }
            if !migrated {
                return false;
            }
        }
        "@typescript-eslint/explicit-member-accessibility" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Style::USE_CONSISTENT_MEMBER_ACCESSIBILITY,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/explicit-module-boundary-types" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_EXPLICIT_TYPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/max-params" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_MAX_PARAMS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/method-signature-style" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_CONSISTENT_METHOD_SIGNATURES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/naming-convention" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_NAMING_CONVENTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-array-constructor" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_ARRAY_LITERALS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-array-delete" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_ARRAY_DELETE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-base-to-string" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_BASE_TO_STRING);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-deprecated" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DEPRECATED_IMPORTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-dupe-class-members" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_CLASS_MEMBERS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-duplicate-enum-values" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_ENUM_VALUES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-duplicate-type-constituents" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_DUPLICATE_TYPE_CONSTITUENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-dynamic-delete" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_DYNAMIC_DELETE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-empty-function" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_EMPTY_BLOCK_STATEMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-empty-interface" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_EMPTY_INTERFACE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-empty-object-type" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_BANNED_TYPES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-explicit-any" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_EXPLICIT_ANY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-extra-non-null-assertion" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_EXTRA_NON_NULL_ASSERTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-extraneous-class" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_STATIC_ONLY_CLASS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-floating-promises" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_FLOATING_PROMISES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-for-in-array" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_FOR_IN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-implied-eval" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_IMPLIED_EVAL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-inferrable-types" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_INFERRABLE_TYPES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-invalid-void-type" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_CONFUSING_VOID_TYPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-loop-func" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_LOOP_FUNC);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-loss-of-precision" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_PRECISION_LOSS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-magic-numbers" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_MAGIC_NUMBERS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-meaningless-void-operator" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_MEANINGLESS_VOID_OPERATOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-misused-new" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_MISLEADING_INSTANTIATOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-misused-promises" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_MISUSED_PROMISES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-misused-spread" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_MISUSED_SPREAD);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-mixed-enums" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_CONSISTENT_ENUM_VALUE_TYPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-namespace" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_NAMESPACE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-non-null-asserted-optional-chain" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Suspicious::NO_NON_NULL_ASSERTED_OPTIONAL_CHAIN,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-non-null-assertion" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_NON_NULL_ASSERTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-redeclare" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_REDECLARE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-redundant-type-constituents" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_REDUNDANT_TYPE_CONSTITUENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-require-imports" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_COMMON_JS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-restricted-imports" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_RESTRICTED_IMPORTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-restricted-types" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_RESTRICTED_TYPES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-shadow" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_SHADOW);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-this-alias" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_THIS_ALIAS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unnecessary-condition" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_UNNECESSARY_CONDITIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unnecessary-parameter-property-assignment" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_USELESS_PARAMETER_PROPERTY_ASSIGNMENT,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unnecessary-qualifier" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_USELESS_QUALIFIER);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unnecessary-template-expression" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Complexity::NO_UNNECESSARY_TEMPLATE_EXPRESSION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unnecessary-type-assertion" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_USELESS_TYPE_ASSERTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unnecessary-type-constraint" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_TYPE_CONSTRAINT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unnecessary-type-conversion" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_USELESS_TYPE_CONVERSION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unsafe-declaration-merging" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_UNSAFE_DECLARATION_MERGING);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unsafe-function-type" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_BANNED_TYPES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unsafe-return" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_UNSAFE_RETURN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unsafe-unary-minus" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_UNSAFE_UNARY_MINUS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unused-vars" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNUSED_VARIABLES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-use-before-define" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_INVALID_USE_BEFORE_DECLARATION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-useless-constructor" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_CONSTRUCTOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-useless-default-assignment" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_USELESS_DEFAULT_ASSIGNMENT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-useless-empty-export" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_EMPTY_EXPORT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-var-requires" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_COMMON_JS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-wrapper-object-types" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_BANNED_TYPES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/only-throw-error" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_THROW_ONLY_ERROR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/parameter-properties" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_PARAMETER_PROPERTIES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-as-const" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_AS_CONST_ASSERTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-enum-initializers" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_ENUM_INITIALIZERS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-find" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_ARRAY_FIND);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-for-of" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_FOR_OF);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-function-type" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_SHORTHAND_FUNCTION_TYPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-includes" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_INCLUDES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-literal-enum-member" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_LITERAL_ENUM_MEMBERS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-namespace-keyword" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::USE_NAMESPACE_KEYWORD);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-nullish-coalescing" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_NULLISH_COALESCING);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-optional-chain" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_OPTIONAL_CHAIN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-promise-reject-errors" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_PROMISE_REJECT_ERRORS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-readonly" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_READONLY_CLASS_PROPERTIES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-reduce-type-parameter" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_REDUCE_TYPE_PARAMETER);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-regexp-exec" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_REGEXP_EXEC);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-string-starts-ends-with" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_STRING_STARTS_ENDS_WITH);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/require-array-sort-compare" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::USE_ARRAY_SORT_COMPARE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/require-await" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::USE_AWAIT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/restrict-plus-operands" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_UNSAFE_PLUS_OPERANDS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/strict-boolean-expressions" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_STRICT_BOOLEAN_EXPRESSIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/switch-exhaustiveness-check" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_EXHAUSTIVE_SWITCH_CASES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/triple-slash-reference" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_TRIPLE_SLASH_REFERENCE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/unified-signatures" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_UNIFIED_TYPE_SIGNATURES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/use-await-thenable" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_AWAIT_THENABLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "accessor-pairs" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_ACCESSOR_PAIRS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "array-callback-return" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::USE_ITERABLE_CALLBACK_RETURN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "arrow-body-style" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_CONSISTENT_ARROW_RETURN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/alt-text" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_ALT_TEXT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/anchor-ambiguous-text" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_AMBIGUOUS_ANCHOR_TEXT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/anchor-has-content" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_ANCHOR_CONTENT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/anchor-is-valid" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_VALID_ANCHOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/aria-activedescendant-has-tabindex" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::A11y::USE_ARIA_ACTIVEDESCENDANT_WITH_TABINDEX,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_VALID_ARIA_PROPS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/aria-proptypes" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_VALID_ARIA_VALUES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/aria-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_VALID_ARIA_ROLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/aria-unsupported-elements" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_ARIA_UNSUPPORTED_ELEMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/autocomplete-valid" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_VALID_AUTOCOMPLETE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/click-events-have-key-events" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_KEY_WITH_CLICK_EVENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/control-has-associated-label" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_CONTROL_LABEL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/heading-has-content" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_HEADING_CONTENT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/html-has-lang" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_HTML_LANG);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/iframe-has-title" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_IFRAME_TITLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/img-redundant-alt" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_REDUNDANT_ALT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/interactive-supports-focus" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_FOCUSABLE_INTERACTIVE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/label-has-associated-control" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_LABEL_WITHOUT_CONTROL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/lang" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_VALID_LANG);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/media-has-caption" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_MEDIA_CAPTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/mouse-events-have-key-events" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_KEY_WITH_MOUSE_EVENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-access-key" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_ACCESS_KEY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-aria-hidden-on-focusable" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_ARIA_HIDDEN_ON_FOCUSABLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-autofocus" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_AUTOFOCUS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-distracting-elements" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_DISTRACTING_ELEMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-interactive-element-to-noninteractive-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::A11y::NO_INTERACTIVE_ELEMENT_TO_NONINTERACTIVE_ROLE,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-noninteractive-element-interactions" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::A11y::NO_NONINTERACTIVE_ELEMENT_INTERACTIONS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-noninteractive-element-to-interactive-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::A11y::NO_NONINTERACTIVE_ELEMENT_TO_INTERACTIVE_ROLE,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-noninteractive-tabindex" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_NONINTERACTIVE_TABINDEX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-redundant-roles" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_REDUNDANT_ROLES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-static-element-interactions" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_STATIC_ELEMENT_INTERACTIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/prefer-tag-over-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_SEMANTIC_ELEMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/role-has-required-aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_ARIA_PROPS_FOR_ROLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/role-supports-aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_ARIA_PROPS_SUPPORTED_BY_ROLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/scope" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_HEADER_SCOPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/tabindex-no-positive" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_POSITIVE_TABINDEX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/missing-client-only-directive-value" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_ASTRO_CLIENT_ONLY_DIRECTIVE_VALUE,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/no-conflict-set-directives" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_ASTRO_CONFLICTING_SET_DIRECTIVES,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/no-exports-from-components" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_ASTRO_COMPONENT_DEFAULT_EXPORT,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/no-set-html-directive" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_ASTRO_SET_HTML_DIRECTIVE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "barrel-files/avoid-barrel-files" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Performance::NO_BARREL_FILE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "barrel-files/avoid-namespace-import" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Performance::NO_NAMESPACE_IMPORT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "barrel-files/avoid-re-export-all" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Performance::NO_RE_EXPORT_ALL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "better-tailwindcss/enforce-consistent-class-order" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_TAILWIND_SORTED_CLASSES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "better-tailwindcss/enforce-consistent-variable-syntax" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_TAILWIND_VARIABLES_SHORTHAND);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "better-tailwindcss/enforce-shorthand-classes" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_TAILWIND_SHORTHAND_CLASSES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "better-tailwindcss/no-conflicting-classes" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_TAILWIND_CONFLICTING_CLASSES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "better-tailwindcss/no-deprecated-classes" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_TAILWIND_LEGACY_UTILITIES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "better-tailwindcss/no-restricted-classes" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_TAILWIND_RESTRICTED_CLASSES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "camelcase" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_NAMING_CONVENTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "class-methods-use-this" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_THIS_IN_CLASS_METHODS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "complexity" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Complexity::NO_EXCESSIVE_COGNITIVE_COMPLEXITY,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "constructor-super" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_INVALID_CONSTRUCTOR_SUPER);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/font-family-fallbacks" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_GENERIC_FONT_NAMES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/no-duplicate-imports" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_AT_IMPORT_RULES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/no-duplicate-keyframe-selectors" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Suspicious::NO_DUPLICATE_SELECTORS_KEYFRAME_BLOCK,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/no-empty-blocks" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_EMPTY_BLOCK);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/no-important" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_IMPORTANT_STYLES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/no-invalid-at-rule-placement" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_INVALID_POSITION_AT_IMPORT_RULE,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/no-invalid-at-rules" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_UNKNOWN_AT_RULES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/no-invalid-named-grid-areas" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_INVALID_GRID_AREAS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/no-invalid-properties" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNKNOWN_PROPERTY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/no-unmatchable-selectors" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNMATCHABLE_ANB_SELECTOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/selector-complexity" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_EXCESSIVE_SELECTOR_COMPLEXITY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/use-baseline" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::USE_BASELINE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/use-layers" => {
            let mut migrated = false;
            {
                let mut blocked = false;
                if !options.include_inspired {
                    results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                    blocked = true;
                }
                if !options.include_nursery {
                    results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                    blocked = true;
                }
                if !blocked {
                    let group = rules.nursery.get_or_insert_with(Default::default);
                    let rule = group
                        .unwrap_group_as_mut()
                        .rule_mut(biome_configuration::analyzer::Nursery::USE_LAYERED_STYLES);
                    rule.set_level(rule.level().max(rule_severity.into()));
                    migrated = true;
                }
            }
            {
                let mut blocked = false;
                if !options.include_inspired {
                    results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                    blocked = true;
                }
                if !options.include_nursery {
                    results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                    blocked = true;
                }
                if !blocked {
                    let group = rules.nursery.get_or_insert_with(Default::default);
                    let rule = group
                        .unwrap_group_as_mut()
                        .rule_mut(biome_configuration::analyzer::Nursery::USE_NAMED_LAYER);
                    rule.set_level(rule.level().max(rule_severity.into()));
                    migrated = true;
                }
            }
            if !migrated {
                return false;
            }
        }
        "curly" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_BLOCK_STATEMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "default-case" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_DEFAULT_SWITCH_CLAUSE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "default-case-last" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Suspicious::USE_DEFAULT_SWITCH_CLAUSE_LAST,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "default-param-last" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_DEFAULT_PARAMETER_LAST);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "dot-notation" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_LITERAL_KEYS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "drizzle/enforce-delete-with-where" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Suspicious::NO_DRIZZLE_DELETE_WITHOUT_WHERE,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "drizzle/enforce-update-with-where" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Suspicious::NO_DRIZZLE_UPDATE_WITHOUT_WHERE,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "e18e/ban-dependencies" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_RESTRICTED_DEPENDENCIES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "e18e/prefer-array-at" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_AT_INDEX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "e18e/prefer-date-now" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_DATE_NOW);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "e18e/prefer-exponentiation-operator" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_EXPONENTIATION_OPERATOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "e18e/prefer-includes" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_INCLUDES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "e18e/prefer-object-has-own" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_PROTOTYPE_BUILTINS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "e18e/prefer-spread-syntax" => {
            let mut migrated = false;
            {
                let mut blocked = false;
                if !options.include_inspired {
                    results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                    blocked = true;
                }
                if !blocked {
                    let group = rules.style.get_or_insert_with(Default::default);
                    let rule = group
                        .unwrap_group_as_mut()
                        .rule_mut(biome_configuration::analyzer::Style::USE_OBJECT_SPREAD);
                    rule.set_level(rule.level().max(rule_severity.into()));
                    migrated = true;
                }
            }
            {
                let mut blocked = false;
                if !options.include_inspired {
                    results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                    blocked = true;
                }
                if !blocked {
                    let group = rules.style.get_or_insert_with(Default::default);
                    let rule = group
                        .unwrap_group_as_mut()
                        .rule_mut(biome_configuration::analyzer::Style::USE_SPREAD_OVER_APPLY);
                    rule.set_level(rule.level().max(rule_severity.into()));
                    migrated = true;
                }
            }
            if !migrated {
                return false;
            }
        }
        "eqeqeq" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DOUBLE_EQUALS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "for-direction" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_VALID_FOR_DIRECTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "func-name-matching" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_MATCHING_FUNCTION_NAME);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "func-names" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_NAMED_FUNCTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "func-style" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_CONSISTENT_FUNCTION_STYLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "getter-return" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::USE_GETTER_RETURN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "global-require" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_TOP_LEVEL_REQUIRE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "grouped-accessor-pairs" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_GROUPED_ACCESSOR_PAIRS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "guard-for-in" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::USE_GUARD_FOR_IN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import-access/eslint-plugin-import-access" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_PRIVATE_IMPORTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/consistent-type-specifier-style" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_IMPORT_TYPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/default" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_UNRESOLVED_DEFAULT_IMPORT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/enforce-node-protocol-usage" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_NODEJS_IMPORT_PROTOCOL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/exports-last" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_EXPORTS_LAST);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/extensions" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_IMPORT_EXTENSIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/first" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_IMPORTS_FIRST);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/named" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNRESOLVED_IMPORTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/namespace" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VALID_NAMESPACE_IMPORTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-commonjs" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_COMMON_JS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-cycle" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_IMPORT_CYCLES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-default-export" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_DEFAULT_EXPORT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-deprecated" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DEPRECATED_IMPORTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-dynamic-require" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_DYNAMIC_REQUIRE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-extraneous-dependencies" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNDECLARED_DEPENDENCIES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-nodejs-modules" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_NODEJS_MODULES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-self-import" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_SELF_IMPORT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "init-declarations" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_UNASSIGNED_VARIABLES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/consistent-test-it" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_CONSISTENT_TEST_IT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/expect-expect" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::USE_EXPECT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/max-nested-describe" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Complexity::NO_EXCESSIVE_NESTED_TEST_SUITES,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-conditional-expect" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_CONDITIONAL_EXPECT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-disabled-tests" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_SKIPPED_TESTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-done-callback" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_DONE_CALLBACK);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-duplicate-hooks" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_TEST_HOOKS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-export" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_EXPORTS_IN_TEST);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-focused-tests" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_FOCUSED_TESTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-hooks" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_TEST_HOOKS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-identical-title" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_IDENTICAL_TEST_TITLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-jasmine-globals" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_JASMINE_GLOBALS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-standalone-expect" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_MISPLACED_ASSERTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-test-return-statement" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_RETURN_IN_TEST);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/prefer-called-with" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_CALLED_WITH);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/prefer-equality-matcher" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_EXPECT_EQUALITY_MATCHER);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/prefer-hooks-in-order" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_TEST_HOOKS_IN_ORDER);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/prefer-hooks-on-top" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_TEST_HOOKS_ON_TOP);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/prefer-importing-jest-globals" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_TEST_GLOBALS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/prefer-lowercase-title" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_LOWERCASE_TEST_TITLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/prefer-spy-on" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_SPY_ON);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/prefer-strict-equal" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_EXPECT_TO_STRICT_EQUAL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/prefer-to-contain" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_EXPECT_TO_CONTAIN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/prefer-to-have-been-called" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_EXPECT_TO_HAVE_BEEN_CALLED);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/prefer-to-have-been-called-times" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_EXPECT_TO_HAVE_BEEN_CALLED_TIMES,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/prefer-to-have-length" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_EXPECT_TO_HAVE_LENGTH);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/require-top-level-describe" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_TOP_LEVEL_DESCRIBE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/valid-title" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VALID_TEST_TITLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsdoc/no-multi-asterisks" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_SINGLE_JS_DOC_ASTERISK);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "json/no-duplicate-keys" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_OBJECT_KEYS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "json/no-empty-keys" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_EMPTY_OBJECT_KEYS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "json/no-unnormalized-keys" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_CONSISTENT_OBJECT_KEYS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "json/no-unsafe-values" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_JSON_UNSAFE_VALUES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "json/top-level-interop" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_TOP_LEVEL_LITERALS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/alt-text" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_ALT_TEXT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/anchor-ambiguous-text" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_AMBIGUOUS_ANCHOR_TEXT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/anchor-has-content" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_ANCHOR_CONTENT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/anchor-is-valid" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_VALID_ANCHOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/aria-activedescendant-has-tabindex" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::A11y::USE_ARIA_ACTIVEDESCENDANT_WITH_TABINDEX,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_VALID_ARIA_PROPS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/aria-proptypes" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_VALID_ARIA_VALUES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/aria-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_VALID_ARIA_ROLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/aria-unsupported-elements" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_ARIA_UNSUPPORTED_ELEMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/autocomplete-valid" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_VALID_AUTOCOMPLETE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/click-events-have-key-events" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_KEY_WITH_CLICK_EVENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/control-has-associated-label" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_CONTROL_LABEL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/heading-has-content" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_HEADING_CONTENT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/html-has-lang" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_HTML_LANG);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/iframe-has-title" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_IFRAME_TITLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/img-redundant-alt" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_REDUNDANT_ALT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/interactive-supports-focus" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_FOCUSABLE_INTERACTIVE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/label-has-associated-control" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_LABEL_WITHOUT_CONTROL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/label-has-for" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_LABEL_WITHOUT_CONTROL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/lang" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_VALID_LANG);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/media-has-caption" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_MEDIA_CAPTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/mouse-events-have-key-events" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_KEY_WITH_MOUSE_EVENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-access-key" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_ACCESS_KEY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-aria-hidden-on-focusable" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_ARIA_HIDDEN_ON_FOCUSABLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-autofocus" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_AUTOFOCUS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-distracting-elements" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_DISTRACTING_ELEMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-interactive-element-to-noninteractive-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::A11y::NO_INTERACTIVE_ELEMENT_TO_NONINTERACTIVE_ROLE,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-noninteractive-element-interactions" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::A11y::NO_NONINTERACTIVE_ELEMENT_INTERACTIONS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-noninteractive-element-to-interactive-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::A11y::NO_NONINTERACTIVE_ELEMENT_TO_INTERACTIVE_ROLE,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-noninteractive-tabindex" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_NONINTERACTIVE_TABINDEX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-redundant-roles" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_REDUNDANT_ROLES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-static-element-interactions" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_STATIC_ELEMENT_INTERACTIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/prefer-tag-over-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_SEMANTIC_ELEMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/role-has-required-aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_ARIA_PROPS_FOR_ROLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/role-supports-aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_ARIA_PROPS_SUPPORTED_BY_ROLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/scope" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_HEADER_SCOPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/tabindex-no-positive" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::NO_POSITIVE_TABINDEX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "max-classes-per-file" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_EXCESSIVE_CLASSES_PER_FILE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "max-depth" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_EXCESSIVE_NESTED_BLOCKS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "max-lines" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_EXCESSIVE_LINES_PER_FILE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "max-lines-per-function" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Complexity::NO_EXCESSIVE_LINES_PER_FUNCTION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "max-nested-callbacks" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_EXCESSIVE_NESTED_CALLBACKS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "max-params" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_MAX_PARAMS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "max-statements" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_EXCESSIVE_STATEMENTS_PER_FUNCTION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "n/global-require" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_TOP_LEVEL_REQUIRE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "n/no-deprecated-api" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_BUFFER_CONSTRUCTOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "n/no-new-require" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_NEW_REQUIRE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "n/no-path-concat" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_PATH_CONCAT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "n/no-process-env" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_PROCESS_ENV);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "n/no-process-exit" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_PROCESS_EXIT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "n/no-sync" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_SYNC_METHODS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "n/prefer-node-protocol" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_NODEJS_IMPORT_PROTOCOL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "new-cap" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_CAPITALIZED_CONSTRUCTORS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-alert" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_ALERT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-array-constructor" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_ARRAY_LITERALS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-async-promise-executor" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_ASYNC_PROMISE_EXECUTOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-await-in-loop" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Performance::NO_AWAIT_IN_LOOPS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-bitwise" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_BITWISE_OPERATORS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-buffer-constructor" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_BUFFER_CONSTRUCTOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-caller" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_ARGUMENTS_CALLER_OR_CALLEE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-case-declarations" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_SWITCH_DECLARATIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-catch-shadow" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_SHADOW);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-class-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_CLASS_ASSIGN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-compare-neg-zero" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_COMPARE_NEG_ZERO);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-cond-assign" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_ASSIGN_IN_EXPRESSIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-console" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_CONSOLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-const-assign" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_CONST_ASSIGN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-constant-binary-expression" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Suspicious::NO_CONSTANT_BINARY_EXPRESSIONS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-constant-condition" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_CONSTANT_CONDITION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-constructor-return" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_CONSTRUCTOR_RETURN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-continue" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_CONTINUE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-control-regex" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Suspicious::NO_CONTROL_CHARACTERS_IN_REGEX,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-debugger" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DEBUGGER);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-delete-var" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Performance::NO_DELETE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-div-regex" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_DIV_REGEX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-dupe-args" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_PARAMETERS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-dupe-class-members" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_CLASS_MEMBERS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-dupe-else-if" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_ELSE_IF);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-dupe-keys" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_OBJECT_KEYS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-duplicate-case" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_CASE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-else-return" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_USELESS_ELSE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-empty" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_EMPTY_BLOCK_STATEMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-empty-character-class" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_EMPTY_CHARACTER_CLASS_IN_REGEX,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-empty-function" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_EMPTY_BLOCK_STATEMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-empty-pattern" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_EMPTY_PATTERN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-empty-static-block" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_EMPTY_BLOCK_STATEMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-eq-null" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_EQUALS_TO_NULL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-eval" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::NO_GLOBAL_EVAL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-ex-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_CATCH_ASSIGN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-extend-native" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_EXTEND_NATIVE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-extra-bind" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_USELESS_BIND);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-extra-boolean-cast" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_EXTRA_BOOLEAN_CAST);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-extra-label" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_LABEL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-fallthrough" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_FALLTHROUGH_SWITCH_CLAUSE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-func-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_FUNCTION_ASSIGN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-global-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_GLOBAL_ASSIGN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-implicit-coercion" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_IMPLICIT_COERCIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-implied-eval" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_IMPLIED_EVAL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-import-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_IMPORT_ASSIGN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-inner-declarations" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_INNER_DECLARATIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-invalid-this" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_INVALID_THIS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-irregular-whitespace" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_IRREGULAR_WHITESPACE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-label-var" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_LABEL_VAR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-labels" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_CONFUSING_LABELS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-lone-blocks" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Complexity::NO_USELESS_LONE_BLOCK_STATEMENTS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-lonely-if" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_COLLAPSED_ELSE_IF);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-loop-func" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_LOOP_FUNC);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-loss-of-precision" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_PRECISION_LOSS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-magic-numbers" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_MAGIC_NUMBERS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-misleading-character-class" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_MISLEADING_CHARACTER_CLASS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-multi-assign" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_MULTI_ASSIGN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-multi-str" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_MULTILINE_STRING);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-native-reassign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_GLOBAL_ASSIGN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-negated-condition" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_NEGATION_ELSE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-negated-in-lhs" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_UNSAFE_NEGATION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-nested-ternary" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_NESTED_TERNARY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-new" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNUSED_INSTANTIATION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-new-func" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_IMPLIED_EVAL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-new-native-nonconstructor" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_INVALID_BUILTIN_INSTANTIATION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-new-object" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_OBJECT_CONSTRUCTOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-new-require" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_NEW_REQUIRE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-new-symbol" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_INVALID_BUILTIN_INSTANTIATION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-new-wrappers" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Style::USE_CONSISTENT_BUILTIN_INSTANTIATION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-nonoctal-decimal-escape" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_NONOCTAL_DECIMAL_ESCAPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-obj-calls" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_GLOBAL_OBJECT_CALLS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-object-constructor" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_OBJECT_CONSTRUCTOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-octal" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_OCTAL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-octal-escape" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_OCTAL_ESCAPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-param-reassign" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_PARAMETER_ASSIGN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-plusplus" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_INCREMENT_DECREMENT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-process-env" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_PROCESS_ENV);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-process-exit" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_PROCESS_EXIT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-promise-executor-return" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_PROMISE_EXECUTOR_RETURN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-proto" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_PROTO);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-prototype-builtins" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_PROTOTYPE_BUILTINS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-redeclare" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_REDECLARE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-regex-spaces" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_ADJACENT_SPACES_IN_REGEX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-restricted-globals" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_RESTRICTED_GLOBALS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-restricted-imports" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_RESTRICTED_IMPORTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-restricted-properties" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_JS_RESTRICTED_PROPERTIES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-return-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_RETURN_ASSIGN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-script-url" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::NO_SCRIPT_URL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-secrets/no-secrets" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::NO_SECRETS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-self-assign" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_SELF_ASSIGN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-self-compare" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_SELF_COMPARE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-sequences" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_COMMA_OPERATOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-setter-return" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_SETTER_RETURN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-shadow" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_SHADOW);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-shadow-restricted-names" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_SHADOW_RESTRICTED_NAMES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-sparse-arrays" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_SPARSE_ARRAY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-template-curly-in-string" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_TEMPLATE_CURLY_IN_STRING);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-ternary" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_TERNARY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-this-before-super" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNREACHABLE_SUPER);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-throw-literal" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_THROW_ONLY_ERROR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unassigned-vars" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_UNASSIGNED_VARIABLES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-undef" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNDECLARED_VARIABLES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-undef-init" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Complexity::NO_USELESS_UNDEFINED_INITIALIZATION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-underscore-dangle" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_DANGLING_UNDERSCORE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unmodified-loop-condition" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_UNMODIFIED_LOOP_CONDITION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unneeded-ternary" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_TERNARY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unreachable" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNREACHABLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unsafe-finally" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNSAFE_FINALLY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unsafe-negation" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_UNSAFE_NEGATION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unsafe-optional-chaining" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNSAFE_OPTIONAL_CHAINING);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unused-expressions" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_UNUSED_EXPRESSIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unused-labels" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNUSED_LABELS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unused-private-class-members" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_UNUSED_PRIVATE_CLASS_MEMBERS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unused-vars" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNUSED_VARIABLES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-use-before-define" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_INVALID_USE_BEFORE_DECLARATION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-assignment" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_USELESS_ASSIGNMENT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-backreference" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_USELESS_REGEX_BACKREFS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-call" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_USELESS_CALL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-catch" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_CATCH);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-computed-key" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_LITERAL_KEYS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-concat" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_STRING_CONCAT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-constructor" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_CONSTRUCTOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-escape" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_ESCAPE_IN_REGEX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-rename" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_RENAME);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-return" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_RETURN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-var" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_VAR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-void" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_VOID);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-with" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_WITH);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "object-shorthand" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_CONSISTENT_OBJECT_DEFINITIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "one-var" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_SINGLE_VAR_DECLARATOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "operator-assignment" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_SHORTHAND_ASSIGN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "package-json-dependencies/duplicate-dependencies" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_DEPENDENCIES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "package-json/require-cpu" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_PACKAGE_JSON_CPU);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "package-json/require-engines" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_PACKAGE_JSON_ENGINES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "package-json/unique-dependencies" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_DEPENDENCIES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "package-json/valid-cpu" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_PACKAGE_JSON_VALID_CPU);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "package-json/valid-engines" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_PACKAGE_JSON_VALID_ENGINES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "package-json/valid-license" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_PACKAGE_JSON_VALID_LICENSE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/expect-expect" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::USE_EXPECT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/missing-playwright-await" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_PLAYWRIGHT_MISSING_AWAIT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-conditional-expect" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_CONDITIONAL_EXPECT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-element-handle" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_PLAYWRIGHT_ELEMENT_HANDLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-eval" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_PLAYWRIGHT_EVAL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-focused-test" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_FOCUSED_TESTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-force-option" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_PLAYWRIGHT_FORCE_OPTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-hooks" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_TEST_HOOKS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-identical-title" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_IDENTICAL_TEST_TITLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-networkidle" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_PLAYWRIGHT_NETWORKIDLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-nth-methods" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_PLAYWRIGHT_NTH_METHODS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-page-pause" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_PLAYWRIGHT_PAGE_PAUSE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-skipped-test" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_SKIPPED_TESTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-test-return-statement" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_RETURN_IN_TEST);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-unsafe-references" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_PLAYWRIGHT_UNSAFE_REFERENCES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-useless-await" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_PLAYWRIGHT_USELESS_AWAIT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-wait-for-navigation" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Suspicious::NO_PLAYWRIGHT_WAIT_FOR_NAVIGATION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-wait-for-selector" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_PLAYWRIGHT_WAIT_FOR_SELECTOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-wait-for-timeout" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Suspicious::NO_PLAYWRIGHT_WAIT_FOR_TIMEOUT,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/prefer-equality-matcher" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_EXPECT_EQUALITY_MATCHER);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/prefer-hooks-in-order" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_TEST_HOOKS_IN_ORDER);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/prefer-hooks-on-top" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_TEST_HOOKS_ON_TOP);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/prefer-lowercase-title" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_LOWERCASE_TEST_TITLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/prefer-strict-equal" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_EXPECT_TO_STRICT_EQUAL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/prefer-to-contain" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_EXPECT_TO_CONTAIN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/prefer-to-have-length" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_EXPECT_TO_HAVE_LENGTH);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/require-top-level-describe" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_TOP_LEVEL_DESCRIBE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/valid-describe-callback" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::USE_PLAYWRIGHT_VALID_DESCRIBE_CALLBACK,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-arrow-callback" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_ARROW_FUNCTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-const" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_CONST);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-destructuring" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_DESTRUCTURING);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-exponentiation-operator" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_EXPONENTIATION_OPERATOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-named-capture-group" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_NAMED_CAPTURE_GROUP);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-numeric-literals" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_NUMERIC_LITERALS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-object-has-own" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_PROTOTYPE_BUILTINS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-object-spread" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_OBJECT_SPREAD);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-promise-reject-errors" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_PROMISE_REJECT_ERRORS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-regex-literals" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_REGEX_LITERALS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-rest-params" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_ARGUMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-spread" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_SPREAD_OVER_APPLY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-template" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_TEMPLATE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "preserve-caught-error" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_ERROR_CAUSE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "promise/no-nesting" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_NESTED_PROMISES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "promise/no-return-in-finally" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_RETURN_IN_FINALLY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/jsx-a" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_VALID_ANCHOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/jsx-img" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_IMAGE_SIZE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/jsx-key" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_JSX_KEY_IN_ITERABLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/jsx-no-script-url" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::NO_SCRIPT_URL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/loader-location" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_QWIK_LOADER_LOCATION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/no-react-props" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_REACT_SPECIFIC_PROPS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/no-use-visible-task" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_QWIK_USE_VISIBLE_TASK);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/prefer-classlist" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_QWIK_CLASSLIST);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/use-method-usage" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_QWIK_METHOD_USAGE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/valid-lexical-scope" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_QWIK_VALID_LEXICAL_SCOPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "radix" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_PARSE_INT_RADIX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-dom/no-dangerously-set-innerhtml" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::NO_DANGEROUSLY_SET_INNER_HTML);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-dom/no-dangerously-set-innerhtml-with-children" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group . unwrap_group_as_mut () . rule_mut (biome_configuration :: analyzer :: Security :: NO_DANGEROUSLY_SET_INNER_HTML_WITH_CHILDREN) ;
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-dom/no-missing-button-type" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_BUTTON_TYPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-dom/no-missing-iframe-sandbox" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::USE_IFRAME_SANDBOX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-dom/no-render-return-value" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_RENDER_RETURN_VALUE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-dom/no-script-url" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::NO_SCRIPT_URL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-dom/no-unsafe-iframe-sandbox" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_UNSAFE_IFRAME_SANDBOX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-dom/no-unsafe-target-blank" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::NO_BLANK_TARGET);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-dom/no-void-elements-with-children" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_VOID_ELEMENTS_WITH_CHILDREN,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-hooks/component-hook-factories" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_COMPONENT_HOOK_FACTORIES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-hooks/exhaustive-deps" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_EXHAUSTIVE_DEPENDENCIES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-hooks/react-compiler" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_REACT_PROP_ASSIGNMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-hooks/rules-of-hooks" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_HOOK_AT_TOP_LEVEL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-jsx/no-children-prop" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_CHILDREN_PROP);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-jsx/no-leaked-dollar" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_JSX_LEAKED_DOLLAR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-jsx/no-namespace" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_JSX_NAMESPACE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-jsx/no-useless-fragment" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_FRAGMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-naming-convention/context-name" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_REACT_NAMING_CONVENTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-naming-convention/id-name" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_REACT_NAMING_CONVENTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-naming-convention/ref-name" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_REACT_NAMING_CONVENTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-native/no-color-literals" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_REACT_NATIVE_LITERAL_COLORS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-native/no-inline-styles" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_INLINE_STYLES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-native/no-raw-text" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_REACT_NATIVE_RAW_TEXT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-native/split-platform-components" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::USE_REACT_NATIVE_PLATFORM_COMPONENTS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-prefer-function-component/prefer-function-component" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_REACT_FUNCTION_COMPONENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-prefer-function-component/react-prefer-function-component" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_REACT_FUNCTION_COMPONENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-refresh/only-export-components" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_COMPONENT_EXPORT_ONLY_MODULES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-rsc/function-definition" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::USE_REACT_ASYNC_SERVER_FUNCTION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-x/component-hook-factories" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_COMPONENT_HOOK_FACTORIES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-x/exhaustive-deps" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_EXHAUSTIVE_DEPENDENCIES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-x/no-array-index-key" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_ARRAY_INDEX_KEY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-x/no-direct-mutation-state" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_REACT_DIRECT_STATE_MUTATION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-x/no-forward-ref" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_REACT_FORWARD_REF);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-x/no-missing-key" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_JSX_KEY_IN_ITERABLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-x/no-nested-component-definitions" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_NESTED_COMPONENT_DEFINITIONS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/async-server-action" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::USE_REACT_ASYNC_SERVER_FUNCTION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/button-has-type" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_BUTTON_TYPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/forbid-elements" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_RESTRICTED_ELEMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/function-component-definition" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_REACT_FUNCTION_COMPONENT_DEFINITION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/iframe-missing-sandbox" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::USE_IFRAME_SANDBOX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-boolean-value" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_IMPLICIT_BOOLEAN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-curly-brace-presence" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_CONSISTENT_CURLY_BRACES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-fragments" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_FRAGMENT_SYNTAX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-key" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_JSX_KEY_IN_ITERABLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-bind" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Performance::NO_JSX_PROPS_BIND);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-comment-textnodes" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_COMMENT_TEXT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-duplicate-props" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_JSX_PROPS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-leaked-render" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_LEAKED_RENDER);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-literals" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_JSX_LITERALS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-script-url" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::NO_SCRIPT_URL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-target-blank" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::NO_BLANK_TARGET);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-useless-fragment" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_FRAGMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-props-no-spread-multi" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATED_SPREAD_PROPS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-array-index-key" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_ARRAY_INDEX_KEY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-children-prop" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_CHILDREN_PROP);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-danger" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::NO_DANGEROUSLY_SET_INNER_HTML);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-danger-with-children" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group . unwrap_group_as_mut () . rule_mut (biome_configuration :: analyzer :: Security :: NO_DANGEROUSLY_SET_INNER_HTML_WITH_CHILDREN) ;
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-direct-mutation-state" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_REACT_DIRECT_STATE_MUTATION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-object-type-as-default-prop" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_REACT_OBJECT_TYPE_AS_DEFAULT_PROP,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-render-return-value" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_RENDER_RETURN_VALUE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-string-refs" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_REACT_STRING_REFS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-unknown-property" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_UNKNOWN_ATTRIBUTE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/self-closing-comp" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_SELF_CLOSING_ELEMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/void-dom-elements-no-children" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_VOID_ELEMENTS_WITH_CHILDREN,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "regexp/no-useless-backreference" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_USELESS_REGEX_BACKREFS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "regexp/prefer-regexp-exec" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_REGEXP_EXEC);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "require-atomic-updates" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_ATOMIC_UPDATES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "require-await" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::USE_AWAIT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "require-unicode-regexp" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_UNICODE_REGEX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "require-yield" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_YIELD);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "shadcn/no-arbitrary-values" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_TAILWIND_ARBITRARY_VALUE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "shadcn/no-inline-styles" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_INLINE_STYLES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "shadcn/no-raw-colors" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_TAILWIND_RAW_COLORS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "shadcn/no-restyle" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_TAILWIND_RESTYLED_COMPONENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "shadcn/require-static-classes" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_TAILWIND_STATIC_CLASS_STRINGS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "solid/jsx-no-script-url" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::NO_SCRIPT_URL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "solid/no-destructure" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_SOLID_DESTRUCTURED_PROPS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "solid/no-react-specific-props" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_REACT_SPECIFIC_PROPS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "solid/prefer-for" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Performance::USE_SOLID_FOR_COMPONENT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "sonarjs/cognitive-complexity" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Complexity::NO_EXCESSIVE_COGNITIVE_COMPLEXITY,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "sonarjs/no-nested-switch" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_NESTED_SWITCH);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "sonarjs/no-nested-template-literals" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_NESTED_TEMPLATE_LITERALS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "sonarjs/prefer-while" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_WHILE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "strict" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::USE_STRICT_MODE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/block-lang" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_CONSISTENT_BLOCK_LANG);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/button-has-type" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_BUTTON_TYPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-add-event-listener" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_SVELTE_ADD_EVENT_LISTENER);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-at-const-tags" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_SVELTE_LEGACY_CONST);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-at-debug-tags" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_SVELTE_AT_DEBUG_TAGS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-at-html-tags" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_SVELTE_AT_HTML_TAGS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-bind-value-on-checkable-inputs" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_SVELTE_BIND_VALUE_ON_CHECKABLE_INPUTS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-ignored-unsubscribe" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_SVELTE_IGNORED_UNSUBSCRIBE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-inline-styles" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_INLINE_STYLES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-inner-declarations" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_INNER_DECLARATIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-inspect" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_SVELTE_INSPECT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-navigation-without-resolve" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_SVELTE_KIT_RESOLVE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-object-in-text-mustaches" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_SVELTE_OBJECT_IN_TEXT_MUSTACHES,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-restricted-html-elements" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_RESTRICTED_ELEMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-store-async" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_SVELTE_ASYNC_STORE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-svelte-internal" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_SVELTE_INTERNAL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-target-blank" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::NO_BLANK_TARGET);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-unnecessary-state-wrap" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_SVELTE_UNNECESSARY_STATE_WRAP);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/prefer-const" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_CONST);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/prefer-derived-over-derived-by" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_SVELTE_USELESS_DERIVED_BY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/prefer-svelte-reactivity" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_SVELTE_REACTIVE_CLASSES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/require-each-key" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_SVELTE_REQUIRE_EACH_KEY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/require-event-dispatcher-types" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_SVELTE_EVENT_DISPATCHER_TYPES,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/require-stores-init" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_SVELTE_STORE_INITIAL_VALUE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/shorthand-directive" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_SVELTE_SHORTHAND_DIRECTIVE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "symbol-description" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_SYMBOL_DESCRIPTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "tailwindcss/classnames-order" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_TAILWIND_SORTED_CLASSES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "tailwindcss/enforces-shorthand" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_TAILWIND_SHORTHAND_CLASSES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "tailwindcss/no-arbitrary-value" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_TAILWIND_ARBITRARY_VALUE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "turbo/no-undeclared-env-vars" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_UNDECLARED_ENV_VARS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicode-bom" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_CONSISTENT_UNICODE_BOM);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/better-dom-traversing" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_BETTER_DOM_TRAVERSING);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/catch-error-name" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_CATCH_ERROR_NAME);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/consistent-boolean-name" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_CONSISTENT_BOOLEAN_NAME);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/consistent-date-clone" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_USELESS_DATE_GET_TIME);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/consistent-json-file-read" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_CONSISTENT_JSON_FILE_READ);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/consistent-tuple-labels" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_CONSISTENT_TUPLE_LABELS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/error-message" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::USE_ERROR_MESSAGE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/explicit-length-check" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_EXPLICIT_LENGTH_CHECK);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/filename-case" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_FILENAMING_CONVENTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/new-for-builtins" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_INVALID_BUILTIN_INSTANTIATION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-accidental-bitwise-operator" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_ACCIDENTAL_BITWISE_OPERATORS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-array-for-each" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_FOR_EACH);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-array-sort-for-min-max" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_ARRAY_SORT_FOR_MIN_MAX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-async-promise-finally" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_ASYNC_PROMISE_FINALLY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-await-in-promise-methods" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_AWAIT_IN_PROMISE_METHODS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-boolean-sort-comparator" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_BOOLEAN_SORT_COMPARATOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-canvas-to-image" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_CANVAS_TO_IMAGE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-confusing-array-splice" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_CONFUSING_ARRAY_SPLICE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-console-spaces" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_CONSOLE_SPACES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-constant-zero-expression" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_ERASING_OPERATION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-document-cookie" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DOCUMENT_COOKIE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-double-comparison" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_DOUBLE_COMPARISON);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-duplicate-loops" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_EAGER_ARRAY_METHODS_IN_LOOPS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-empty-file" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_EMPTY_SOURCE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-for-each" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_ARRAY_FOR_EACH);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-for-loop" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_FOR_OF);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-immediate-mutation" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_IMMEDIATE_MUTATION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-instanceof-array" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::USE_IS_ARRAY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-invalid-fetch-options" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_INVALID_FETCH_OPTIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-invalid-file-input-accept" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_INVALID_FILE_INPUT_ACCEPT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-invalid-remove-event-listener" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_INVALID_REMOVE_EVENT_LISTENER);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-late-current-target-access" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_LATE_CURRENT_TARGET_ACCESS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-late-event-control" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_LATE_EVENT_CONTROL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-lonely-if" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_COLLAPSED_IF);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-magic-array-flat-depth" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_MAGIC_ARRAY_FLAT_DEPTH);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-mismatched-map-key" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_MISMATCHED_MAP_KEY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-negated-array-predicate" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_NEGATED_ARRAY_PREDICATE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-negated-condition" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_NEGATION_ELSE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-negation-in-equality-check" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_NEGATION_IN_EQUALITY_CHECK);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-nested-ternary" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_NESTED_TERNARY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-new-array" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_NEW_ARRAY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-null" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_NULL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-process-exit" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_PROCESS_EXIT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-return-array-push" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_RETURN_ARRAY_PUSH);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-single-promise-in-promise-methods" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_SINGLE_PROMISE_IN_PROMISE_METHODS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-static-only-class" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_STATIC_ONLY_CLASS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-subtraction-comparison" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_SUBTRACTION_COMPARISON);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-thenable" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_THEN_PROPERTY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-this-outside-of-class" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_THIS_OUTSIDE_OF_CLASS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-unnecessary-await" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_USELESS_AWAIT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-unnecessary-nested-ternary" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_USELESS_NESTED_TERNARY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-unnecessary-splice" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_USELESS_SPLICE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-unnecessary-string-trim" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_UNNECESSARY_STRING_TRIM);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-unreadable-for-of-expression" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_UNREADABLE_FOR_OF_EXPRESSION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-unreadable-iife" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_UNREADABLE_IIFE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-unsafe-sqlite-interpolation" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_UNSAFE_SQLITE_INTERPOLATION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-useless-concat" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_STRING_CONCAT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-useless-fallback-in-spread" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_USELESS_FALLBACK_IN_SPREAD);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-useless-length-check" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_USELESS_LENGTH_CHECK);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-useless-recursion" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_USELESS_RECURSION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-useless-set-construction" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_USELESS_SET_CONSTRUCTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-useless-spread" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_USELESS_SPREAD);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-useless-switch-case" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_SWITCH_CASE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-useless-undefined" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_UNDEFINED);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-xor-as-exponentiation" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_XOR_AS_EXPONENTIATION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-zero-fractions" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_ZERO_FRACTIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/numeric-separators-style" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_NUMERIC_SEPARATORS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-array-flat-map" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_FLAT_MAP);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-array-index-of" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_INDEX_OF);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-array-some" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_ARRAY_SOME);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-at" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_AT_INDEX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-bigint-literals" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_BIGINT_LITERALS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-block-statement-over-iife" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_BLOCK_STATEMENT_OVER_IIFE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-boolean-return" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_SIMPLIFIED_BOOLEAN_RETURN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-continue" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_EARLY_CONTINUE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-date-now" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_DATE_NOW);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-dom-node-text-content" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Performance::USE_DOM_NODE_TEXT_CONTENT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-flat-math-min-max" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_FLAT_MATH_MIN_MAX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-global-this" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_GLOBAL_THIS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-includes" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_INCLUDES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-iterator-zip" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_ITERATOR_ZIP);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-math-abs" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_MATH_ABS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-math-constants" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Suspicious::NO_APPROXIMATIVE_NUMERIC_CONSTANT,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-math-min-max" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_MATH_MIN_MAX);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-math-trunc" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_MATH_TRUNC);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-modern-math-apis" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_MODERN_MATH_APIS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-module" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_GLOBAL_DIRNAME_FILENAME);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-node-protocol" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_NODEJS_IMPORT_PROTOCOL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-number-properties" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_NUMBER_NAMESPACE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-object-from-entries" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_OBJECT_FROM_ENTRIES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-query-selector" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_DOM_QUERY_SELECTOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-regexp-test" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_REGEXP_TEST);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-set-size" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_SET_SIZE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-string-slice" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_SUBSTR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-string-trim-start-end" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_TRIM_START_END);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-unary-minus" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_UNARY_MINUS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-while-loop-condition" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_WHILE_LOOP_CONDITION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/require-number-to-fixed-digits-argument" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Suspicious::USE_NUMBER_TO_FIXED_DIGITS_ARGUMENT,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/throw-new-error" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_THROW_NEW_ERROR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unused-imports/no-unused-imports" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNUSED_IMPORTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unused-imports/no-unused-vars" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNUSED_VARIABLES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "use-isnan" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_IS_NAN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "valid-typeof" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_VALID_TYPEOF);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vars-on-top" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_VARS_ON_TOP);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/consistent-test-it" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_CONSISTENT_TEST_IT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/expect-expect" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::USE_EXPECT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/max-nested-describe" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Complexity::NO_EXCESSIVE_NESTED_TEST_SUITES,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-conditional-expect" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_CONDITIONAL_EXPECT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-disabled-tests" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_SKIPPED_TESTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-done-callback" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_DONE_CALLBACK);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-duplicate-hooks" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DUPLICATE_TEST_HOOKS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-focused-tests" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_FOCUSED_TESTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-hooks" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_TEST_HOOKS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-identical-title" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_IDENTICAL_TEST_TITLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-standalone-expect" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_MISPLACED_ASSERTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-test-return-statement" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_RETURN_IN_TEST);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-called-times" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_EXPECT_TO_HAVE_BEEN_CALLED_ONCE,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-called-with" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_CALLED_WITH);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-describe-function-title" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_DESCRIBE_FUNCTION_TITLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-equality-matcher" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_EXPECT_EQUALITY_MATCHER);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-hooks-in-order" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_TEST_HOOKS_IN_ORDER);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-hooks-on-top" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_TEST_HOOKS_ON_TOP);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-importing-vitest-globals" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_TEST_GLOBALS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-lowercase-title" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_LOWERCASE_TEST_TITLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-spy-on" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_SPY_ON);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-strict-equal" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_EXPECT_TO_STRICT_EQUAL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-to-contain" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_EXPECT_TO_CONTAIN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-to-have-been-called-times" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_EXPECT_TO_HAVE_BEEN_CALLED_TIMES,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-to-have-length" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_EXPECT_TO_HAVE_LENGTH);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/require-top-level-describe" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_TOP_LEVEL_DESCRIBE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/valid-title" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VALID_TEST_TITLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/attribute-hyphenation" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_VUE_HYPHENATED_ATTRIBUTES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/block-lang" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_CONSISTENT_BLOCK_LANG);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/block-order" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_BLOCK_ORDER);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/camelcase" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_NAMING_CONVENTION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/component-api-style" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_VUE_OPTIONS_API);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/component-options-name-casing" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_VUE_REGISTERED_COMPONENT_NAME_CASING,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/define-emits-declaration" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_VUE_CONSISTENT_DEFINE_EMITS_DECLARATION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/define-macros-order" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_VUE_DEFINE_MACROS_ORDER);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/define-props-declaration" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Style::USE_VUE_CONSISTENT_DEFINE_PROPS_DECLARATION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/define-props-destructuring" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_VUE_DEFINE_PROPS_DESTRUCTURING,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/dot-notation" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::USE_LITERAL_KEYS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/enforce-style-attribute" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::USE_SCOPED_STYLES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/eqeqeq" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_DOUBLE_EQUALS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/html-button-has-type" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::A11y::USE_BUTTON_TYPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/match-component-file-name" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_VUE_MATCHING_COMPONENT_FILE_NAME,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/max-props" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_EXCESSIVE_PROPS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/multi-word-component-names" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_VUE_MULTI_WORD_COMPONENT_NAMES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/next-tick-style" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_VUE_NEXT_TICK_PROMISE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-arrow-functions-in-watch" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_VUE_ARROW_FUNC_IN_WATCH);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-async-in-computed-properties" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_ASYNC_COMPUTED);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-boolean-default" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_BOOLEAN_DEFAULT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-child-content" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_CHILD_CONTENT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-computed-properties-in-data" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_VUE_COMPUTED_PROPERTIES_IN_DATA,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-console" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_CONSOLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-constant-condition" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_CONSTANT_CONDITION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-define-model-default" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_DEFINE_MODEL_DEFAULT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-data-object-declaration" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_VUE_DATA_OBJECT_DECLARATION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-delete-set" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_DELETE_SET);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-destroyed-lifecycle" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_DESTROYED_LIFECYCLE,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-dollar-listeners-api" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_LISTENERS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-dollar-scopedslots-api" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_SCOPED_SLOTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-events-api" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_EVENTS_API);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-filter" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_FILTER);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-functional-template" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_FUNCTIONAL_TEMPLATE,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-html-element-is" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_HTML_ELEMENT_IS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-inline-template" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_INLINE_TEMPLATE,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-model-definition" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_MODEL_DEFINITION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-props-default-this" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_PROPS_DEFAULT_THIS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-router-link-tag-prop" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_ROUTER_LINK_TAG_PROP,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-scope-attribute" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_SCOPE_ATTRIBUTE,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-slot-attribute" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_SLOT_ATTRIBUTE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-slot-scope-attribute" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_SLOT_SCOPE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-v-bind-sync" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_V_BIND_SYNC);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-v-is" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_V_IS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-v-on-native-modifier" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_V_ON_NATIVE_MODIFIER,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-v-on-number-modifiers" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_VUE_V_ON_NUMBER_VALUES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-vue-config-keycodes" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_CONFIG_KEY_CODES,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-dupe-keys" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_VUE_DUPLICATE_KEYS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-dupe-v-else-if" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_DUPLICATE_V_ELSE_IF);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-duplicate-attributes" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_DUPLICATE_ATTRIBUTES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-empty-pattern" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_EMPTY_PATTERN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-export-in-script-setup" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_EXPORT_IN_SCRIPT_SETUP);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-expose-after-await" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_EXPOSE_AFTER_AWAIT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-implicit-coercion" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_IMPLICIT_COERCIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-import-compiler-macros" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_VUE_IMPORT_COMPILER_MACROS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-irregular-whitespace" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_IRREGULAR_WHITESPACE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-lifecycle-after-await" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_LIFECYCLE_AFTER_AWAIT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-lone-template" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_LONE_TEMPLATE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-loss-of-precision" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_PRECISION_LOSS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-multiple-objects-in-class" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_MULTIPLE_OBJECTS_IN_CLASS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-multiple-slot-args" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_MULTIPLE_SLOT_ARGS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-mutating-props" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_MUTATING_PROPS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-negated-condition" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_NEGATION_ELSE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-negated-v-if-condition" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_NEGATION_ELSE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-ref-as-operand" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_VUE_REF_AS_OPERAND);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-ref-object-reactivity-loss" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_VUE_REF_OBJECT_REACTIVITY_LOSS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-required-prop-with-default" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_VUE_REQUIRED_PROP_WITH_DEFAULT,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-reserved-keys" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_VUE_RESERVED_KEYS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-reserved-props" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_VUE_RESERVED_PROPS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-restricted-call-after-await" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_VUE_RESTRICTED_CALL_AFTER_AWAIT,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-restricted-html-elements" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_RESTRICTED_ELEMENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-restricted-v-on" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_RESTRICTED_V_ON);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-root-v-if" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_ROOT_V_IF);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-setup-props-reactivity-loss" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_VUE_SETUP_PROPS_REACTIVITY_LOSS,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-shadow-native-events" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_SHADOW_NATIVE_EVENTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-shared-component-data" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Correctness::NO_VUE_DATA_OBJECT_DECLARATION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-side-effects-in-computed-properties" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_SIDE_EFFECTS_IN_COMPUTED);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-sparse-arrays" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Suspicious::NO_SPARSE_ARRAY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-template-key" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_TEMPLATE_KEY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-template-shadow" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_TEMPLATE_SHADOW);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-template-target-blank" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Security::NO_BLANK_TARGET);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-textarea-mustache" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_TEXTAREA_MUSTACHE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-this-in-before-route-enter" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_VUE_THIS_IN_BEFORE_ROUTE_ENTER,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-undef-directives" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_UNDECLARED_DIRECTIVES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-undef-properties" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_UNDECLARED_PROPERTIES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-unused-emit-declarations" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_UNUSED_EMIT_DECLARATIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-unused-refs" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_UNUSED_REFS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-unused-vars" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_UNUSED_VARIABLES);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-use-v-else-with-v-for" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_VUE_V_IF_WITH_V_FOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-use-v-if-with-v-for" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::NO_VUE_V_IF_WITH_V_FOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-useless-concat" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Complexity::NO_USELESS_STRING_CONCAT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-useless-template-attributes" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_VUE_USELESS_TEMPLATE_ATTRIBUTES,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-useless-v-bind" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_USELESS_V_BIND);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-v-for-template-key-on-child" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_VUE_V_FOR_TEMPLATE_KEY_ON_CHILD,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-v-html" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_V_HTML);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-v-text" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_V_TEXT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-v-text-v-html-on-component" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::NO_VUE_V_TEXT_V_HTML_ON_COMPONENT,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-watch-after-await" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_WATCH_AFTER_AWAIT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/object-shorthand" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_CONSISTENT_OBJECT_DEFINITIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/one-component-per-file" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_ONE_COMPONENT_PER_FILE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/prefer-define-options" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_DEFINE_OPTIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/prefer-import-from-vue" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_BASE_IMPORT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/prefer-prop-type-boolean-first" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_BOOLEAN_PROP_TYPE_FIRST);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/prefer-separate-static-class" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_SEPARATE_STATIC_CLASS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/prefer-single-event-payload" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_SINGLE_EVENT_PAYLOAD);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/prefer-template" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_TEMPLATE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/prefer-true-attribute-shorthand" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_VUE_CONSISTENT_BOOLEAN_PROP_STYLE,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/prefer-use-template-ref" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_TEMPLATE_REF);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/prefer-v-model" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_V_MODEL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/prop-name-casing" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_VUE_CONSISTENT_PROP_NAME_CASING,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-component-is" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_COMPONENT_IS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-default-export" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_DEFAULT_EXPORT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-default-prop" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_DEFAULT_PROP);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-emit-validator" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_EMIT_VALIDATOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-explicit-emits" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_UNDECLARED_EMITS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-explicit-slots" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_UNDECLARED_SLOTS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-macro-variable-name" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_MACRO_VARIABLE_NAME);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-prop-type-constructor" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_PROP_TYPE_CONSTRUCTOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-prop-types" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_TYPED_PROPS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-render-return" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_RENDER_RETURN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-slots-as-functions" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_SLOTS_AS_FUNCTIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-toggle-inside-transition" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_TOGGLE_INSIDE_TRANSITION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-typed-object-prop" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_TYPED_OBJECT_PROPS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-typed-ref" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_TYPED_REF);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-v-for-key" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_VUE_V_FOR_KEY);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-valid-default-prop" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_VALID_DEFAULT_PROP);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/return-in-computed-property" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_COMPUTED_PROPERTY_RETURN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/return-in-emits-validator" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_EMITS_VALIDATOR_RETURN);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/slot-name-casing" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_SLOT_NAME_CASING);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/this-in-template" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::NO_VUE_THIS_IN_TEMPLATE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/use-v-on-exact" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_V_ON_EXACT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/v-bind-style" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_VUE_CONSISTENT_V_BIND_STYLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/v-for-delimiter-style" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_VUE_CONSISTENT_V_FOR_DELIMITER,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/v-on-event-hyphenation" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_VUE_CONSISTENT_EVENT_HYPHENATION,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/v-on-handler-style" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut(
                biome_configuration::analyzer::Nursery::USE_VUE_CONSISTENT_V_ON_HANDLER_STYLE,
            );
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/v-on-style" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::USE_VUE_CONSISTENT_V_ON_STYLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/v-slot-style" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_CONSISTENT_V_SLOT_STYLE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-attribute-name" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_VALID_ATTRIBUTE_NAME);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-define-emits" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_VALID_DEFINE_EMITS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-define-options" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_VALID_DEFINE_OPTIONS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-define-props" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_VALID_DEFINE_PROPS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-next-tick" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_VALID_NEXT_TICK);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-template-root" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_VUE_VALID_TEMPLATE_ROOT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-bind" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_BIND);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-cloak" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_CLOAK);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-else" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_ELSE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-else-if" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_ELSE_IF);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-for" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_FOR);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-html" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_HTML);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-if" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_IF);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-is" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_VALID_V_IS);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-memo" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_VALID_V_MEMO);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-model" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_VALID_V_MODEL);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-on" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_ON);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-once" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_ONCE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-pre" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_PRE);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-show" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_VALID_V_SHOW);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-slot" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Nursery::USE_VUE_VALID_V_SLOT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-text" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_TEXT);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "yoda" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut(biome_configuration::analyzer::Style::NO_YODA_EXPRESSION);
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        _ => {
            results.add(
                eslint_name,
                eslint_to_biome::RuleMigrationResult::Unsupported,
            );
            return false;
        }
    }
    results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Migrated);
    true
}
