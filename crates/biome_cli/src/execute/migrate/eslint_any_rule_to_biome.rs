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
                .rule_mut("no_component_hook_factories");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/dom-no-dangerously-set-innerhtml" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_dangerously_set_inner_html");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/dom-no-dangerously-set-innerhtml-with-children" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_dangerously_set_inner_html_with_children");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/dom-no-missing-button-type" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_button_type");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/dom-no-missing-iframe-sandbox" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_iframe_sandbox");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/dom-no-render-return-value" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_render_return_value");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/dom-no-script-url" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_script_url");
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
                .rule_mut("no_unsafe_iframe_sandbox");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/dom-no-unsafe-target-blank" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_blank_target");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/dom-no-void-elements-with-children" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_void_elements_with_children");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/exhaustive-deps" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_exhaustive_dependencies");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/jsx-no-children-prop" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_children_prop");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/jsx-no-leaked-dollar" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_jsx_leaked_dollar");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/jsx-no-namespace" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_jsx_namespace");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/jsx-no-useless-fragment" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_useless_fragments");
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
                .rule_mut("use_react_naming_convention");
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
                .rule_mut("use_react_naming_convention");
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
                .rule_mut("use_react_naming_convention");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/no-array-index-key" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_array_index_key");
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
                .rule_mut("no_react_direct_state_mutation");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/no-forward-ref" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_react_forward_ref");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/no-missing-key" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_jsx_key_in_iterable");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/no-nested-component-definitions" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_nested_component_definitions");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/rsc-function-definition" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_react_async_server_function");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/description-style" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_consistent_graphql_descriptions");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/input-name" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_input_name");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/lone-anonymous-operation" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_lone_anonymous_operation");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/lone-executable-definition" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_lone_executable_definition");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/no-anonymous-operations" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_graphql_named_operations");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/no-duplicate-fields" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_duplicate_fields");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/no-root-type" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_root_type");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/require-deprecation-date" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_deprecated_date");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/require-deprecation-reason" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_deprecated_reason");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/unique-argument-names" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicate_argument_names");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/unique-enum-value-names" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicate_enum_value_names");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/unique-field-definition-names" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicate_field_definition_names");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/unique-input-field-names" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicate_input_field_names");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/unique-operation-name" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicate_graphql_operation_name");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@graphql-eslint/unique-variable-names" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicate_variable_names");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-accesskey-attrs" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_access_key");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-aria-hidden-on-focusable" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_aria_hidden_on_focusable");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-duplicate-attrs" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicate_attributes");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-empty-headings" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_heading_content");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-inline-styles" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_inline_styles");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-invalid-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_valid_aria_role");
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
                .rule_mut("no_non_scalable_viewport");
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
                .rule_mut("no_obsolete_attributes");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-obsolete-tags" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_obsolete_tags");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-positive-tabindex" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_positive_tabindex");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-redundant-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_redundant_roles");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/no-target-blank" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_blank_target");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/require-button-type" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_button_type");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/require-frame-title" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_iframe_title");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/require-img-alt" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_alt_text");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/require-lang" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_html_lang");
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
                .rule_mut("no_misplaced_list_elements");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@html-eslint/use-baseline" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_baseline");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@mysticatea/no-this-in-static" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_this_in_static");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@mysticatea/prefer-for-of" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_for_of");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/google-font-display" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_google_font_display");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/google-font-preconnect" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_google_font_preconnect");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/inline-script-id" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_inline_script_id");
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
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_next_script_for_google_analytics");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/no-async-client-component" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_next_async_client_component");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/no-before-interactive-script-outside-document" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_before_interactive_script_outside_document");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/no-document-import-in-page" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_document_import_in_page");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/no-head-element" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_head_element");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/no-head-import-in-document" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_head_import_in_document");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/no-img-element" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_img_element");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/no-sync-scripts" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_sync_scripts");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/next/no-unwanted-polyfillio" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_unwanted_polyfillio");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@react-native/no-deep-imports" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_react_native_deep_imports");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@react-native/platform-colors" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_react_native_valid_platform_colors");
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
                .rule_mut("use_consistent_curly_braces");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@stylistic/jsx-self-closing-comp" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_self_closing_elements");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@tanstack/query/exhaustive-deps" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_tanstack_query_exhaustive_dependencies");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@tanstack/query/infinite-query-property-order" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_tanstack_query_infinite_query_property_order");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@tanstack/query/mutation-property-order" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_tanstack_query_mutation_property_order");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@tanstack/query/no-rest-destructuring" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_tanstack_query_rest_destructuring");
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
                .rule_mut("no_tanstack_query_void_query_fn");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@tanstack/query/stable-query-client" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_tanstack_query_stable_query_client");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/adjacent-overload-signatures" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_adjacent_overload_signatures");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/array-type" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_consistent_array_type");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/ban-ts-comment" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_ts_ignore");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/ban-types" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_banned_types");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/consistent-indexed-object-style" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_consistent_indexed_object_style");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/consistent-type-definitions" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_consistent_type_definitions");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/consistent-type-exports" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_export_type");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/consistent-type-imports" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_import_type");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/default-param-last" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_default_parameter_last");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/dot-notation" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_literal_keys");
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
                        .rule_mut("use_explicit_return_type");
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
                    let rule = group.unwrap_group_as_mut().rule_mut("use_explicit_type");
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
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_consistent_member_accessibility");
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
            let rule = group.unwrap_group_as_mut().rule_mut("use_explicit_type");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/max-params" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_max_params");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/method-signature-style" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_consistent_method_signatures");
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
                .rule_mut("use_naming_convention");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-array-constructor" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_array_literals");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-array-delete" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_array_delete");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-base-to-string" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_base_to_string");
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
                .rule_mut("no_deprecated_imports");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-dupe-class-members" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicate_class_members");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-duplicate-enum-values" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicate_enum_values");
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
                .rule_mut("no_duplicate_type_constituents");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-dynamic-delete" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_dynamic_delete");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-empty-function" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_empty_block_statements");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-empty-interface" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_empty_interface");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-empty-object-type" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_banned_types");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-explicit-any" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_explicit_any");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-extra-non-null-assertion" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_extra_non_null_assertion");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-extraneous-class" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_static_only_class");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-floating-promises" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_floating_promises");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-for-in-array" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_for_in");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-implied-eval" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_implied_eval");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-inferrable-types" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_inferrable_types");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-invalid-void-type" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_confusing_void_type");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-loop-func" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_loop_func");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-loss-of-precision" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_precision_loss");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-magic-numbers" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_magic_numbers");
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
                .rule_mut("no_meaningless_void_operator");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-misused-new" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_misleading_instantiator");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-misused-promises" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_misused_promises");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-misused-spread" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_misused_spread");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-mixed-enums" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_consistent_enum_value_type");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-namespace" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_namespace");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-non-null-asserted-optional-chain" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_non_null_asserted_optional_chain");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-non-null-assertion" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_non_null_assertion");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-redeclare" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_redeclare");
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
                .rule_mut("no_redundant_type_constituents");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-require-imports" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_common_js");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-restricted-imports" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_restricted_imports");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-restricted-types" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_restricted_types");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-shadow" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_shadow");
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
                .rule_mut("no_useless_this_alias");
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
                .rule_mut("no_unnecessary_conditions");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unnecessary-parameter-property-assignment" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_useless_parameter_property_assignment");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unnecessary-qualifier" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_useless_qualifier");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unnecessary-template-expression" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_unnecessary_template_expression");
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
                .rule_mut("no_useless_type_assertion");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unnecessary-type-constraint" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_useless_type_constraint");
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
                .rule_mut("no_useless_type_conversion");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unsafe-declaration-merging" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_unsafe_declaration_merging");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unsafe-function-type" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_banned_types");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unsafe-return" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_unsafe_return");
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
                .rule_mut("no_unsafe_unary_minus");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unused-vars" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_unused_variables");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-use-before-define" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_invalid_use_before_declaration");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-useless-constructor" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_useless_constructor");
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
                .rule_mut("no_useless_default_assignment");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-useless-empty-export" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_useless_empty_export");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-var-requires" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_common_js");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-wrapper-object-types" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_banned_types");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/only-throw-error" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_throw_only_error");
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
                .rule_mut("no_parameter_properties");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-as-const" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_as_const_assertion");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-enum-initializers" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_enum_initializers");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-find" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_array_find");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-for-of" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_for_of");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-function-type" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_shorthand_function_type");
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
            let rule = group.unwrap_group_as_mut().rule_mut("use_includes");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-literal-enum-member" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_literal_enum_members");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-namespace-keyword" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_namespace_keyword");
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
                .rule_mut("use_nullish_coalescing");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-optional-chain" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_optional_chain");
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
                .rule_mut("use_promise_reject_errors");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-readonly" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_readonly_class_properties");
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
                .rule_mut("use_reduce_type_parameter");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-regexp-exec" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_regexp_exec");
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
                .rule_mut("use_string_starts_ends_with");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/require-array-sort-compare" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_array_sort_compare");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/require-await" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_await");
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
                .rule_mut("no_unsafe_plus_operands");
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
                .rule_mut("use_strict_boolean_expressions");
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
                .rule_mut("use_exhaustive_switch_cases");
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
                .rule_mut("no_triple_slash_reference");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/unified-signatures" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_unified_type_signatures");
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
            let rule = group.unwrap_group_as_mut().rule_mut("use_await_thenable");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "accessor-pairs" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_accessor_pairs");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "array-callback-return" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_iterable_callback_return");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "arrow-body-style" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_consistent_arrow_return");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/alt-text" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_alt_text");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/anchor-ambiguous-text" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_ambiguous_anchor_text");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/anchor-has-content" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_anchor_content");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/anchor-is-valid" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_valid_anchor");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/aria-activedescendant-has-tabindex" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_aria_activedescendant_with_tabindex");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_valid_aria_props");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/aria-proptypes" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_valid_aria_values");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/aria-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_valid_aria_role");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/aria-unsupported-elements" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_aria_unsupported_elements");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/autocomplete-valid" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_valid_autocomplete");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/click-events-have-key-events" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_key_with_click_events");
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
            let rule = group.unwrap_group_as_mut().rule_mut("use_control_label");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/heading-has-content" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_heading_content");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/html-has-lang" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_html_lang");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/iframe-has-title" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_iframe_title");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/img-redundant-alt" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_redundant_alt");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/interactive-supports-focus" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_focusable_interactive");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/label-has-associated-control" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_label_without_control");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/lang" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_valid_lang");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/media-has-caption" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_media_caption");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/mouse-events-have-key-events" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_key_with_mouse_events");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-access-key" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_access_key");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-aria-hidden-on-focusable" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_aria_hidden_on_focusable");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-autofocus" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_autofocus");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-distracting-elements" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_distracting_elements");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-interactive-element-to-noninteractive-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_interactive_element_to_noninteractive_role");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-noninteractive-element-interactions" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_noninteractive_element_interactions");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-noninteractive-element-to-interactive-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_noninteractive_element_to_interactive_role");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-noninteractive-tabindex" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_noninteractive_tabindex");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-redundant-roles" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_redundant_roles");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/no-static-element-interactions" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_static_element_interactions");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/prefer-tag-over-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_semantic_elements");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/role-has-required-aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_aria_props_for_role");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/role-supports-aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_aria_props_supported_by_role");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/scope" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_header_scope");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/jsx-a11y/tabindex-no-positive" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_positive_tabindex");
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
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_astro_client_only_directive_value");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "astro/no-conflict-set-directives" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_astro_conflicting_set_directives");
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
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_astro_component_default_export");
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
                .rule_mut("no_astro_set_html_directive");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "barrel-files/avoid-barrel-files" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_barrel_file");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "barrel-files/avoid-namespace-import" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_namespace_import");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "barrel-files/avoid-re-export-all" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_re_export_all");
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
                .rule_mut("use_tailwind_sorted_classes");
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
                .rule_mut("use_tailwind_variables_shorthand");
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
                .rule_mut("use_tailwind_shorthand_classes");
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
                .rule_mut("no_tailwind_conflicting_classes");
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
                .rule_mut("no_tailwind_legacy_utilities");
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
                .rule_mut("no_tailwind_restricted_classes");
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
                .rule_mut("use_naming_convention");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "class-methods-use-this" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_this_in_class_methods");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "complexity" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_excessive_cognitive_complexity");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "constructor-super" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_invalid_constructor_super");
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
                .rule_mut("use_generic_font_names");
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
                .rule_mut("no_duplicate_at_import_rules");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/no-duplicate-keyframe-selectors" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicate_selectors_keyframe_block");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/no-empty-blocks" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_empty_block");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/no-important" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_important_styles");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/no-invalid-at-rule-placement" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_invalid_position_at_import_rule");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/no-invalid-at-rules" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_unknown_at_rules");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/no-invalid-named-grid-areas" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_invalid_grid_areas");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/no-invalid-properties" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_unknown_property");
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
                .rule_mut("no_unmatchable_anb_selector");
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
                .rule_mut("no_excessive_selector_complexity");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "css/use-baseline" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_baseline");
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
                    let rule = group.unwrap_group_as_mut().rule_mut("use_layered_styles");
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
                    let rule = group.unwrap_group_as_mut().rule_mut("use_named_layer");
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
            let rule = group.unwrap_group_as_mut().rule_mut("use_block_statements");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "default-case" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_default_switch_clause");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "default-case-last" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_default_switch_clause_last");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "default-param-last" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_default_parameter_last");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "dot-notation" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_literal_keys");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "drizzle/enforce-delete-with-where" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_drizzle_delete_without_where");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "drizzle/enforce-update-with-where" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_drizzle_update_without_where");
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
                .rule_mut("no_restricted_dependencies");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "e18e/prefer-array-at" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_at_index");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "e18e/prefer-date-now" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_date_now");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "e18e/prefer-exponentiation-operator" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_exponentiation_operator");
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
            let rule = group.unwrap_group_as_mut().rule_mut("use_includes");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "e18e/prefer-object-has-own" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_prototype_builtins");
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
                    let rule = group.unwrap_group_as_mut().rule_mut("use_object_spread");
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
                        .rule_mut("use_spread_over_apply");
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
            let rule = group.unwrap_group_as_mut().rule_mut("no_double_equals");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "for-direction" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_valid_for_direction");
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
                .rule_mut("use_matching_function_name");
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
            let rule = group.unwrap_group_as_mut().rule_mut("use_named_function");
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
                .rule_mut("use_consistent_function_style");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "getter-return" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_getter_return");
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
                .rule_mut("use_top_level_require");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "grouped-accessor-pairs" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_grouped_accessor_pairs");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "guard-for-in" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_guard_for_in");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import-access/eslint-plugin-import-access" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_private_imports");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/consistent-type-specifier-style" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_import_type");
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
                .rule_mut("no_unresolved_default_import");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/enforce-node-protocol-usage" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_nodejs_import_protocol");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/exports-last" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_exports_last");
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
                .rule_mut("use_import_extensions");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/first" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_imports_first");
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
                .rule_mut("no_unresolved_imports");
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
                .rule_mut("use_valid_namespace_imports");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-commonjs" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_common_js");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-cycle" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_import_cycles");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-default-export" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_default_export");
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
                .rule_mut("no_deprecated_imports");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-dynamic-require" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_dynamic_require");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-extraneous-dependencies" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_undeclared_dependencies");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-nodejs-modules" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_nodejs_modules");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-self-import" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_self_import");
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
                .rule_mut("no_unassigned_variables");
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
                .rule_mut("use_consistent_test_it");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/expect-expect" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_expect");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/max-nested-describe" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_excessive_nested_test_suites");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-conditional-expect" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_conditional_expect");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-disabled-tests" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_skipped_tests");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-done-callback" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_done_callback");
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
                .rule_mut("no_duplicate_test_hooks");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-export" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_exports_in_test");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-focused-tests" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_focused_tests");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-hooks" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_test_hooks");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-identical-title" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_identical_test_title");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-jasmine-globals" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_jasmine_globals");
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
                .rule_mut("no_misplaced_assertion");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-test-return-statement" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_return_in_test");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/prefer-called-with" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_called_with");
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
                .rule_mut("use_expect_equality_matcher");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/prefer-hooks-in-order" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_test_hooks_in_order");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/prefer-hooks-on-top" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_test_hooks_on_top");
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
            let rule = group.unwrap_group_as_mut().rule_mut("no_test_globals");
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
                .rule_mut("use_lowercase_test_title");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/prefer-spy-on" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_spy_on");
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
                .rule_mut("use_expect_to_strict_equal");
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
                .rule_mut("use_expect_to_contain");
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
                .rule_mut("use_expect_to_have_been_called");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/prefer-to-have-been-called-times" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_expect_to_have_been_called_times");
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
                .rule_mut("use_expect_to_have_length");
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
                .rule_mut("use_top_level_describe");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/valid-title" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_valid_test_title");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsdoc/no-multi-asterisks" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_single_js_doc_asterisk");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "json/no-duplicate-keys" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicate_object_keys");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "json/no-empty-keys" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_empty_object_keys");
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
                .rule_mut("use_consistent_object_keys");
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
                .rule_mut("no_json_unsafe_values");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "json/top-level-interop" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_top_level_literals");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/alt-text" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_alt_text");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/anchor-ambiguous-text" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_ambiguous_anchor_text");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/anchor-has-content" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_anchor_content");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/anchor-is-valid" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_valid_anchor");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/aria-activedescendant-has-tabindex" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_aria_activedescendant_with_tabindex");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_valid_aria_props");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/aria-proptypes" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_valid_aria_values");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/aria-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_valid_aria_role");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/aria-unsupported-elements" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_aria_unsupported_elements");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/autocomplete-valid" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_valid_autocomplete");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/click-events-have-key-events" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_key_with_click_events");
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
            let rule = group.unwrap_group_as_mut().rule_mut("use_control_label");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/heading-has-content" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_heading_content");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/html-has-lang" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_html_lang");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/iframe-has-title" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_iframe_title");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/img-redundant-alt" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_redundant_alt");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/interactive-supports-focus" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_focusable_interactive");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/label-has-associated-control" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_label_without_control");
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
                .rule_mut("no_label_without_control");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/lang" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_valid_lang");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/media-has-caption" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_media_caption");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/mouse-events-have-key-events" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_key_with_mouse_events");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-access-key" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_access_key");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-aria-hidden-on-focusable" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_aria_hidden_on_focusable");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-autofocus" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_autofocus");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-distracting-elements" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_distracting_elements");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-interactive-element-to-noninteractive-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_interactive_element_to_noninteractive_role");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-noninteractive-element-interactions" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_noninteractive_element_interactions");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-noninteractive-element-to-interactive-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_noninteractive_element_to_interactive_role");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-noninteractive-tabindex" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_noninteractive_tabindex");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-redundant-roles" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_redundant_roles");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-static-element-interactions" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_static_element_interactions");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/prefer-tag-over-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_semantic_elements");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/role-has-required-aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_aria_props_for_role");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/role-supports-aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_aria_props_supported_by_role");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/scope" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_header_scope");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/tabindex-no-positive" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_positive_tabindex");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "max-classes-per-file" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_excessive_classes_per_file");
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
                .rule_mut("no_excessive_nested_blocks");
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
                .rule_mut("no_excessive_lines_per_file");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "max-lines-per-function" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_excessive_lines_per_function");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "max-nested-callbacks" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_excessive_nested_callbacks");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "max-params" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_max_params");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "max-statements" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_excessive_statements_per_function");
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
                .rule_mut("use_top_level_require");
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
                .rule_mut("no_buffer_constructor");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "n/no-new-require" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_new_require");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "n/no-path-concat" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_path_concat");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "n/no-process-env" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_process_env");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "n/no-process-exit" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_process_exit");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "n/no-sync" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_sync_methods");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "n/prefer-node-protocol" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_nodejs_import_protocol");
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
                .rule_mut("use_capitalized_constructors");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-alert" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_alert");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-array-constructor" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_array_literals");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-async-promise-executor" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_async_promise_executor");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-await-in-loop" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_await_in_loops");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-bitwise" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_bitwise_operators");
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
                .rule_mut("no_buffer_constructor");
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
                .rule_mut("no_arguments_caller_or_callee");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-case-declarations" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_switch_declarations");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-catch-shadow" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_shadow");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-class-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_class_assign");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-compare-neg-zero" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_compare_neg_zero");
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
                .rule_mut("no_assign_in_expressions");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-console" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_console");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-const-assign" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_const_assign");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-constant-binary-expression" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_constant_binary_expressions");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-constant-condition" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_constant_condition");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-constructor-return" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_constructor_return");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-continue" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_continue");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-control-regex" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_control_characters_in_regex");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-debugger" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_debugger");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-delete-var" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_delete");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-div-regex" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_div_regex");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-dupe-args" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicate_parameters");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-dupe-class-members" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicate_class_members");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-dupe-else-if" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_duplicate_else_if");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-dupe-keys" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicate_object_keys");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-duplicate-case" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_duplicate_case");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-else-return" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_useless_else");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-empty" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_empty_block_statements");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-empty-character-class" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_empty_character_class_in_regex");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-empty-function" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_empty_block_statements");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-empty-pattern" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_empty_pattern");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-empty-static-block" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_empty_block_statements");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-eq-null" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_equals_to_null");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-eval" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_global_eval");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-ex-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_catch_assign");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-extend-native" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_extend_native");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-extra-bind" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_useless_bind");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-extra-boolean-cast" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_extra_boolean_cast");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-extra-label" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_useless_label");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-fallthrough" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_fallthrough_switch_clause");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-func-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_function_assign");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-global-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_global_assign");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-implicit-coercion" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_implicit_coercions");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-implied-eval" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_implied_eval");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-import-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_import_assign");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-inner-declarations" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_inner_declarations");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-invalid-this" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_invalid_this");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-irregular-whitespace" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_irregular_whitespace");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-label-var" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_label_var");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-labels" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_confusing_labels");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-lone-blocks" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_useless_lone_block_statements");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-lonely-if" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_collapsed_else_if");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-loop-func" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_loop_func");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-loss-of-precision" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_precision_loss");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-magic-numbers" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_magic_numbers");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-misleading-character-class" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_misleading_character_class");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-multi-assign" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_multi_assign");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-multi-str" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_multiline_string");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-native-reassign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_global_assign");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-negated-condition" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_negation_else");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-negated-in-lhs" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_unsafe_negation");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-nested-ternary" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_nested_ternary");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-new" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_unused_instantiation");
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
            let rule = group.unwrap_group_as_mut().rule_mut("no_implied_eval");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-new-native-nonconstructor" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_invalid_builtin_instantiation");
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
                .rule_mut("no_object_constructor");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-new-require" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_new_require");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-new-symbol" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_invalid_builtin_instantiation");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-new-wrappers" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_consistent_builtin_instantiation");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-nonoctal-decimal-escape" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_nonoctal_decimal_escape");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-obj-calls" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_global_object_calls");
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
                .rule_mut("no_object_constructor");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-octal" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_octal");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-octal-escape" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_octal_escape");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-param-reassign" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_parameter_assign");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-plusplus" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_increment_decrement");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-process-env" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_process_env");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-process-exit" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_process_exit");
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
                .rule_mut("no_promise_executor_return");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-proto" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_proto");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-prototype-builtins" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_prototype_builtins");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-redeclare" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_redeclare");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-regex-spaces" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_adjacent_spaces_in_regex");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-restricted-globals" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_restricted_globals");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-restricted-imports" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_restricted_imports");
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
                .rule_mut("no_js_restricted_properties");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-return-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_return_assign");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-script-url" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_script_url");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-secrets/no-secrets" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_secrets");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-self-assign" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_self_assign");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-self-compare" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_self_compare");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-sequences" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_comma_operator");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-setter-return" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_setter_return");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-shadow" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_shadow");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-shadow-restricted-names" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_shadow_restricted_names");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-sparse-arrays" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_sparse_array");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-template-curly-in-string" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_template_curly_in_string");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-ternary" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_ternary");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-this-before-super" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_unreachable_super");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-throw-literal" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_throw_only_error");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unassigned-vars" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_unassigned_variables");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-undef" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_undeclared_variables");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-undef-init" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_useless_undefined_initialization");
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
                .rule_mut("no_dangling_underscore");
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
                .rule_mut("no_unmodified_loop_condition");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unneeded-ternary" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_useless_ternary");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unreachable" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_unreachable");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unsafe-finally" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_unsafe_finally");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unsafe-negation" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_unsafe_negation");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unsafe-optional-chaining" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_unsafe_optional_chaining");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unused-expressions" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_unused_expressions");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unused-labels" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_unused_labels");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unused-private-class-members" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_unused_private_class_members");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unused-vars" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_unused_variables");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-use-before-define" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_invalid_use_before_declaration");
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
                .rule_mut("no_useless_assignment");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-backreference" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_useless_regex_backrefs");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-call" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_useless_call");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-catch" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_useless_catch");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-computed-key" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_literal_keys");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-concat" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_useless_string_concat");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-constructor" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_useless_constructor");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-escape" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_useless_escape_in_regex");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-rename" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_useless_rename");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-return" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_useless_return");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-var" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_var");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-void" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_void");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-with" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_with");
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
                .rule_mut("use_consistent_object_definitions");
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
                .rule_mut("use_single_var_declarator");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "operator-assignment" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_shorthand_assign");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "package-json-dependencies/duplicate-dependencies" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicate_dependencies");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "package-json/require-cpu" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_package_json_cpu");
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
                .rule_mut("use_package_json_engines");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "package-json/unique-dependencies" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicate_dependencies");
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
                .rule_mut("use_package_json_valid_cpu");
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
                .rule_mut("use_package_json_valid_engines");
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
                .rule_mut("use_package_json_valid_license");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/expect-expect" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_expect");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/missing-playwright-await" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_playwright_missing_await");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-conditional-expect" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_conditional_expect");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-element-handle" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_playwright_element_handle");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-eval" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_playwright_eval");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-focused-test" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_focused_tests");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-force-option" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_playwright_force_option");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-hooks" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_test_hooks");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-identical-title" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_identical_test_title");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-networkidle" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_playwright_networkidle");
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
                .rule_mut("no_playwright_nth_methods");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-page-pause" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_playwright_page_pause");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-skipped-test" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_skipped_tests");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-test-return-statement" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_return_in_test");
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
                .rule_mut("no_playwright_unsafe_references");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-useless-await" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_playwright_useless_await");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-wait-for-navigation" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_playwright_wait_for_navigation");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-wait-for-selector" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_playwright_wait_for_selector");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/no-wait-for-timeout" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_playwright_wait_for_timeout");
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
                .rule_mut("use_expect_equality_matcher");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/prefer-hooks-in-order" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_test_hooks_in_order");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/prefer-hooks-on-top" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_test_hooks_on_top");
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
                .rule_mut("use_lowercase_test_title");
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
                .rule_mut("use_expect_to_strict_equal");
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
                .rule_mut("use_expect_to_contain");
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
                .rule_mut("use_expect_to_have_length");
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
                .rule_mut("use_top_level_describe");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "playwright/valid-describe-callback" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_playwright_valid_describe_callback");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-arrow-callback" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_arrow_function");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-const" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_const");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-destructuring" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_destructuring");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-exponentiation-operator" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_exponentiation_operator");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-named-capture-group" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_named_capture_group");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-numeric-literals" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_numeric_literals");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-object-has-own" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_prototype_builtins");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-object-spread" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_object_spread");
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
                .rule_mut("use_promise_reject_errors");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-regex-literals" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_regex_literals");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-rest-params" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_arguments");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-spread" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_spread_over_apply");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-template" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_template");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "preserve-caught-error" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_error_cause");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "promise/no-nesting" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_nested_promises");
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
            let rule = group.unwrap_group_as_mut().rule_mut("no_return_in_finally");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/jsx-a" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_valid_anchor");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/jsx-img" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_image_size");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/jsx-key" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_jsx_key_in_iterable");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/jsx-no-script-url" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_script_url");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/loader-location" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_qwik_loader_location");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/no-react-props" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_react_specific_props");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/no-use-visible-task" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_qwik_use_visible_task");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/prefer-classlist" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_qwik_classlist");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/use-method-usage" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_qwik_method_usage");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/valid-lexical-scope" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_qwik_valid_lexical_scope");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "radix" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_parse_int_radix");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-dom/no-dangerously-set-innerhtml" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_dangerously_set_inner_html");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-dom/no-dangerously-set-innerhtml-with-children" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_dangerously_set_inner_html_with_children");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-dom/no-missing-button-type" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_button_type");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-dom/no-missing-iframe-sandbox" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_iframe_sandbox");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-dom/no-render-return-value" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_render_return_value");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-dom/no-script-url" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_script_url");
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
                .rule_mut("no_unsafe_iframe_sandbox");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-dom/no-unsafe-target-blank" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_blank_target");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-dom/no-void-elements-with-children" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_void_elements_with_children");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-hooks/component-hook-factories" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_component_hook_factories");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-hooks/exhaustive-deps" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_exhaustive_dependencies");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-hooks/react-compiler" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_react_prop_assignments");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-hooks/rules-of-hooks" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_hook_at_top_level");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-jsx/no-children-prop" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_children_prop");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-jsx/no-leaked-dollar" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_jsx_leaked_dollar");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-jsx/no-namespace" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_jsx_namespace");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-jsx/no-useless-fragment" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_useless_fragments");
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
                .rule_mut("use_react_naming_convention");
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
                .rule_mut("use_react_naming_convention");
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
                .rule_mut("use_react_naming_convention");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-native/no-color-literals" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_react_native_literal_colors");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-native/no-inline-styles" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_inline_styles");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-native/no-raw-text" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_react_native_raw_text");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-native/split-platform-components" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_react_native_platform_components");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-prefer-function-component/prefer-function-component" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_react_function_components");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-prefer-function-component/react-prefer-function-component" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_react_function_components");
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
                .rule_mut("use_component_export_only_modules");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-rsc/function-definition" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_react_async_server_function");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-x/component-hook-factories" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_component_hook_factories");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-x/exhaustive-deps" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_exhaustive_dependencies");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-x/no-array-index-key" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_array_index_key");
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
                .rule_mut("no_react_direct_state_mutation");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-x/no-forward-ref" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_react_forward_ref");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-x/no-missing-key" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_jsx_key_in_iterable");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-x/no-nested-component-definitions" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_nested_component_definitions");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/async-server-action" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_react_async_server_function");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/button-has-type" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_button_type");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/forbid-elements" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_restricted_elements");
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
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_react_function_component_definition");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/iframe-missing-sandbox" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_iframe_sandbox");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-boolean-value" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_implicit_boolean");
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
                .rule_mut("use_consistent_curly_braces");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-fragments" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_fragment_syntax");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-key" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_jsx_key_in_iterable");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-bind" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_jsx_props_bind");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-comment-textnodes" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_comment_text");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-duplicate-props" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicate_jsx_props");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-leaked-render" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_leaked_render");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-literals" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_jsx_literals");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-script-url" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_script_url");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-target-blank" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_blank_target");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-useless-fragment" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_useless_fragments");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-props-no-spread-multi" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicated_spread_props");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-array-index-key" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_array_index_key");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-children-prop" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_children_prop");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-danger" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_dangerously_set_inner_html");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-danger-with-children" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_dangerously_set_inner_html_with_children");
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
                .rule_mut("no_react_direct_state_mutation");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-object-type-as-default-prop" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_react_object_type_as_default_prop");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-render-return-value" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_render_return_value");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-string-refs" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_react_string_refs");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-unknown-property" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_unknown_attribute");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/self-closing-comp" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_self_closing_elements");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/void-dom-elements-no-children" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_void_elements_with_children");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "regexp/no-useless-backreference" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_useless_regex_backrefs");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "regexp/prefer-regexp-exec" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_regexp_exec");
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
            let rule = group.unwrap_group_as_mut().rule_mut("use_atomic_updates");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "require-await" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_await");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "require-unicode-regexp" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_unicode_regex");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "require-yield" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_yield");
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
                .rule_mut("no_tailwind_arbitrary_value");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "shadcn/no-inline-styles" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_inline_styles");
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
                .rule_mut("no_tailwind_raw_colors");
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
                .rule_mut("no_tailwind_restyled_components");
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
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_tailwind_static_class_strings");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "solid/jsx-no-script-url" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_script_url");
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
                .rule_mut("no_solid_destructured_props");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "solid/no-react-specific-props" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_react_specific_props");
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
                .rule_mut("use_solid_for_component");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "sonarjs/cognitive-complexity" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_excessive_cognitive_complexity");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "sonarjs/no-nested-switch" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_nested_switch");
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
                .rule_mut("no_nested_template_literals");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "sonarjs/prefer-while" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_while");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "strict" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_strict_mode");
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
                .rule_mut("use_consistent_block_lang");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/button-has-type" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_button_type");
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
                .rule_mut("no_svelte_add_event_listener");
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
                .rule_mut("no_svelte_legacy_const");
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
                .rule_mut("no_svelte_at_debug_tags");
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
                .rule_mut("no_svelte_at_html_tags");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-bind-value-on-checkable-inputs" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_svelte_bind_value_on_checkable_inputs");
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
                .rule_mut("no_svelte_ignored_unsubscribe");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-inline-styles" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_inline_styles");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-inner-declarations" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_inner_declarations");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-inspect" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_svelte_inspect");
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
                .rule_mut("use_svelte_kit_resolve");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-object-in-text-mustaches" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_svelte_object_in_text_mustaches");
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
                .rule_mut("no_restricted_elements");
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
                .rule_mut("no_svelte_async_store");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-svelte-internal" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_svelte_internal");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/no-target-blank" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_blank_target");
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
                .rule_mut("no_svelte_unnecessary_state_wrap");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/prefer-const" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_const");
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
                .rule_mut("no_svelte_useless_derived_by");
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
                .rule_mut("use_svelte_reactive_classes");
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
                .rule_mut("use_svelte_require_each_key");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "svelte/require-event-dispatcher-types" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_svelte_event_dispatcher_types");
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
                .rule_mut("use_svelte_store_initial_value");
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
                .rule_mut("use_svelte_shorthand_directive");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "symbol-description" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_symbol_description");
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
                .rule_mut("use_tailwind_sorted_classes");
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
                .rule_mut("use_tailwind_shorthand_classes");
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
                .rule_mut("no_tailwind_arbitrary_value");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "turbo/no-undeclared-env-vars" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_undeclared_env_vars");
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
                .rule_mut("use_consistent_unicode_bom");
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
                .rule_mut("use_better_dom_traversing");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/catch-error-name" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_catch_error_name");
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
                .rule_mut("use_consistent_boolean_name");
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
                .rule_mut("no_useless_date_get_time");
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
                .rule_mut("use_consistent_json_file_read");
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
                .rule_mut("use_consistent_tuple_labels");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/error-message" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_error_message");
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
                .rule_mut("use_explicit_length_check");
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
                .rule_mut("use_filenaming_convention");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/new-for-builtins" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_invalid_builtin_instantiation");
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
                .rule_mut("no_accidental_bitwise_operators");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-array-for-each" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_for_each");
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
                .rule_mut("no_array_sort_for_min_max");
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
                .rule_mut("no_async_promise_finally");
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
                .rule_mut("no_await_in_promise_methods");
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
                .rule_mut("no_boolean_sort_comparator");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-canvas-to-image" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_canvas_to_image");
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
                .rule_mut("no_confusing_array_splice");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-console-spaces" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_console_spaces");
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
            let rule = group.unwrap_group_as_mut().rule_mut("no_erasing_operation");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-document-cookie" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_document_cookie");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-double-comparison" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_double_comparison");
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
                .rule_mut("no_eager_array_methods_in_loops");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-empty-file" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_empty_source");
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
            let rule = group.unwrap_group_as_mut().rule_mut("no_array_for_each");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-for-loop" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_for_of");
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
                .rule_mut("no_immediate_mutation");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-instanceof-array" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_is_array");
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
                .rule_mut("no_invalid_fetch_options");
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
                .rule_mut("no_invalid_file_input_accept");
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
                .rule_mut("no_invalid_remove_event_listener");
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
                .rule_mut("no_late_current_target_access");
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
                .rule_mut("no_late_event_control");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-lonely-if" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_collapsed_if");
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
                .rule_mut("no_magic_array_flat_depth");
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
                .rule_mut("no_mismatched_map_key");
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
                .rule_mut("no_negated_array_predicate");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-negated-condition" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_negation_else");
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
                .rule_mut("no_negation_in_equality_check");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-nested-ternary" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_nested_ternary");
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
            let rule = group.unwrap_group_as_mut().rule_mut("no_new_array");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-null" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_null");
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
            let rule = group.unwrap_group_as_mut().rule_mut("no_process_exit");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-return-array-push" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_return_array_push");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-single-promise-in-promise-methods" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_single_promise_in_promise_methods");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-static-only-class" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_static_only_class");
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
                .rule_mut("no_subtraction_comparison");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-thenable" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_then_property");
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
                .rule_mut("no_this_outside_of_class");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-unnecessary-await" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_useless_await");
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
                .rule_mut("no_useless_nested_ternary");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-unnecessary-splice" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_useless_splice");
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
                .rule_mut("no_unnecessary_string_trim");
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
                .rule_mut("no_unreadable_for_of_expression");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-unreadable-iife" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_unreadable_iife");
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
                .rule_mut("no_unsafe_sqlite_interpolation");
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
                .rule_mut("no_useless_string_concat");
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
                .rule_mut("no_useless_fallback_in_spread");
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
                .rule_mut("no_useless_length_check");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-useless-recursion" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_useless_recursion");
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
                .rule_mut("no_useless_set_construction");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-useless-spread" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_useless_spread");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-useless-switch-case" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_useless_switch_case");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-useless-undefined" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_useless_undefined");
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
                .rule_mut("no_xor_as_exponentiation");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-zero-fractions" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_zero_fractions");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/numeric-separators-style" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_numeric_separators");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-array-flat-map" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_flat_map");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-array-index-of" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_index_of");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-array-some" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_array_some");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-at" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_at_index");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-bigint-literals" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_bigint_literals");
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
                .rule_mut("use_block_statement_over_iife");
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
                .rule_mut("use_simplified_boolean_return");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-continue" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_early_continue");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-date-now" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_date_now");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-dom-node-text-content" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_dom_node_text_content");
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
                .rule_mut("use_flat_math_min_max");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-global-this" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_global_this");
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
            let rule = group.unwrap_group_as_mut().rule_mut("use_includes");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-iterator-zip" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_iterator_zip");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-math-abs" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_math_abs");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-math-constants" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_approximative_numeric_constant");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-math-min-max" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_math_min_max");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-math-trunc" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_math_trunc");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-modern-math-apis" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_modern_math_apis");
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
                .rule_mut("no_global_dirname_filename");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-node-protocol" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_nodejs_import_protocol");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-number-properties" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_number_namespace");
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
                .rule_mut("use_object_from_entries");
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
                .rule_mut("use_dom_query_selector");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-regexp-test" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_regexp_test");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-set-size" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_set_size");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-string-slice" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_substr");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-string-trim-start-end" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_trim_start_end");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-unary-minus" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_unary_minus");
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
                .rule_mut("use_while_loop_condition");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/require-number-to-fixed-digits-argument" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_number_to_fixed_digits_argument");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/throw-new-error" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_throw_new_error");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unused-imports/no-unused-imports" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_unused_imports");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unused-imports/no-unused-vars" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_unused_variables");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "use-isnan" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_is_nan");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "valid-typeof" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_valid_typeof");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vars-on-top" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vars_on_top");
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
                .rule_mut("use_consistent_test_it");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/expect-expect" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_expect");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/max-nested-describe" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_excessive_nested_test_suites");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-conditional-expect" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_conditional_expect");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-disabled-tests" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_skipped_tests");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-done-callback" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_done_callback");
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
                .rule_mut("no_duplicate_test_hooks");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-focused-tests" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_focused_tests");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-hooks" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_test_hooks");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-identical-title" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_identical_test_title");
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
                .rule_mut("no_misplaced_assertion");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-test-return-statement" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_return_in_test");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-called-times" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_expect_to_have_been_called_once");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-called-with" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_called_with");
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
                .rule_mut("use_describe_function_title");
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
                .rule_mut("use_expect_equality_matcher");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-hooks-in-order" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_test_hooks_in_order");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-hooks-on-top" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_test_hooks_on_top");
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
            let rule = group.unwrap_group_as_mut().rule_mut("no_test_globals");
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
                .rule_mut("use_lowercase_test_title");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-spy-on" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_spy_on");
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
                .rule_mut("use_expect_to_strict_equal");
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
                .rule_mut("use_expect_to_contain");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/prefer-to-have-been-called-times" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_expect_to_have_been_called_times");
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
                .rule_mut("use_expect_to_have_length");
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
                .rule_mut("use_top_level_describe");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/valid-title" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_valid_test_title");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/attribute-hyphenation" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_vue_hyphenated_attributes");
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
                .rule_mut("use_consistent_block_lang");
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
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_block_order");
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
                .rule_mut("use_naming_convention");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/component-api-style" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_vue_options_api");
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
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_vue_registered_component_name_casing");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/define-emits-declaration" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_vue_consistent_define_emits_declaration");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/define-macros-order" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_vue_define_macros_order");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/define-props-declaration" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_vue_consistent_define_props_declaration");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/define-props-destructuring" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_vue_define_props_destructuring");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/dot-notation" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_literal_keys");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/enforce-style-attribute" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_scoped_styles");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/eqeqeq" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_double_equals");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/html-button-has-type" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_button_type");
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
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_vue_matching_component_file_name");
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
                .rule_mut("no_vue_excessive_props");
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
                .rule_mut("use_vue_multi_word_component_names");
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
                .rule_mut("use_vue_next_tick_promise");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-arrow-functions-in-watch" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_arrow_func_in_watch");
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
                .rule_mut("no_vue_async_computed");
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
                .rule_mut("no_vue_boolean_default");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-child-content" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_vue_child_content");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-computed-properties-in-data" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_computed_properties_in_data");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-console" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_console");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-constant-condition" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_constant_condition");
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
                .rule_mut("no_vue_define_model_default");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-data-object-declaration" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_data_object_declaration");
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
                .rule_mut("no_vue_deprecated_delete_set");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-destroyed-lifecycle" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_deprecated_destroyed_lifecycle");
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
                .rule_mut("no_vue_deprecated_listeners");
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
                .rule_mut("no_vue_deprecated_scoped_slots");
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
                .rule_mut("no_vue_deprecated_events_api");
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
                .rule_mut("no_vue_deprecated_filter");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-functional-template" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_deprecated_functional_template");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-html-element-is" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_deprecated_html_element_is");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-inline-template" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_deprecated_inline_template");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-model-definition" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_deprecated_model_definition");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-props-default-this" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_deprecated_props_default_this");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-router-link-tag-prop" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_deprecated_router_link_tag_prop");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-scope-attribute" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_deprecated_scope_attribute");
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
                .rule_mut("no_vue_deprecated_slot_attribute");
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
                .rule_mut("no_vue_deprecated_slot_scope");
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
                .rule_mut("no_vue_deprecated_v_bind_sync");
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
                .rule_mut("no_vue_deprecated_v_is");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-v-on-native-modifier" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_deprecated_v_on_native_modifier");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-v-on-number-modifiers" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_v_on_number_values");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-vue-config-keycodes" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_deprecated_config_key_codes");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-dupe-keys" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_duplicate_keys");
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
                .rule_mut("no_vue_duplicate_v_else_if");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-duplicate-attributes" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_duplicate_attributes");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-empty-pattern" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_empty_pattern");
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
                .rule_mut("no_vue_export_in_script_setup");
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
                .rule_mut("no_vue_expose_after_await");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-implicit-coercion" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_implicit_coercions");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-import-compiler-macros" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_import_compiler_macros");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-irregular-whitespace" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_irregular_whitespace");
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
                .rule_mut("no_vue_lifecycle_after_await");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-lone-template" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_vue_lone_template");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-loss-of-precision" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_precision_loss");
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
                .rule_mut("no_vue_multiple_objects_in_class");
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
                .rule_mut("no_vue_multiple_slot_args");
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
                .rule_mut("no_vue_mutating_props");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-negated-condition" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_negation_else");
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
            let rule = group.unwrap_group_as_mut().rule_mut("no_vue_negation_else");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-ref-as-operand" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_ref_as_operand");
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
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_ref_object_reactivity_loss");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-required-prop-with-default" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_required_prop_with_default");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-reserved-keys" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_vue_reserved_keys");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-reserved-props" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_reserved_props");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-restricted-call-after-await" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_restricted_call_after_await");
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
                .rule_mut("no_restricted_elements");
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
                .rule_mut("no_vue_restricted_v_on");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-root-v-if" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_vue_root_v_if");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-setup-props-reactivity-loss" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_setup_props_reactivity_loss");
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
                .rule_mut("no_vue_shadow_native_events");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-shared-component-data" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_data_object_declaration");
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
                .rule_mut("no_vue_side_effects_in_computed");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-sparse-arrays" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_sparse_array");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-template-key" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_vue_template_key");
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
                .rule_mut("no_vue_template_shadow");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-template-target-blank" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_blank_target");
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
                .rule_mut("no_vue_textarea_mustache");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-this-in-before-route-enter" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_this_in_before_route_enter");
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
                .rule_mut("no_vue_undeclared_directives");
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
                .rule_mut("no_vue_undeclared_properties");
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
                .rule_mut("no_vue_unused_emit_declarations");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-unused-refs" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_vue_unused_refs");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-unused-vars" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_unused_variables");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-use-v-else-with-v-for" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_v_if_with_v_for");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-use-v-if-with-v-for" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_v_if_with_v_for");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-useless-concat" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_useless_string_concat");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-useless-template-attributes" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_useless_template_attributes");
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
                .rule_mut("no_vue_useless_v_bind");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-v-for-template-key-on-child" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_v_for_template_key_on_child");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-v-html" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_vue_v_html");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-v-text" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_vue_v_text");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-v-text-v-html-on-component" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("no_vue_v_text_v_html_on_component");
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
                .rule_mut("no_vue_watch_after_await");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/object-shorthand" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_consistent_object_definitions");
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
                .rule_mut("use_vue_one_component_per_file");
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
                .rule_mut("use_vue_define_options");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/prefer-import-from-vue" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_base_import");
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
                .rule_mut("use_vue_boolean_prop_type_first");
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
                .rule_mut("use_vue_separate_static_class");
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
                .rule_mut("use_vue_single_event_payload");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/prefer-template" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_template");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/prefer-true-attribute-shorthand" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_vue_consistent_boolean_prop_style");
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
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_template_ref");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/prefer-v-model" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_v_model");
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
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_vue_consistent_prop_name_casing");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-component-is" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_component_is");
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
                .rule_mut("use_vue_default_export");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-default-prop" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_default_prop");
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
                .rule_mut("use_vue_emit_validator");
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
                .rule_mut("no_vue_undeclared_emits");
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
                .rule_mut("no_vue_undeclared_slots");
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
                .rule_mut("use_vue_macro_variable_name");
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
                .rule_mut("use_vue_prop_type_constructor");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-prop-types" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_typed_props");
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
                .rule_mut("use_vue_render_return");
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
                .rule_mut("use_vue_slots_as_functions");
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
                .rule_mut("use_vue_toggle_inside_transition");
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
                .rule_mut("use_vue_typed_object_props");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-typed-ref" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_typed_ref");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/require-v-for-key" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_v_for_key");
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
                .rule_mut("use_vue_valid_default_prop");
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
                .rule_mut("use_vue_computed_property_return");
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
                .rule_mut("use_vue_emits_validator_return");
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
                .rule_mut("use_vue_slot_name_casing");
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
                .rule_mut("no_vue_this_in_template");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/use-v-on-exact" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_v_on_exact");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/v-bind-style" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_vue_consistent_v_bind_style");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/v-for-delimiter-style" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_vue_consistent_v_for_delimiter");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/v-on-event-hyphenation" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_vue_consistent_event_hyphenation");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/v-on-handler-style" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_vue_consistent_v_on_handler_style");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/v-on-style" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_vue_consistent_v_on_style");
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
                .rule_mut("use_vue_consistent_v_slot_style");
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
                .rule_mut("use_vue_valid_attribute_name");
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
                .rule_mut("use_vue_valid_define_emits");
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
                .rule_mut("use_vue_valid_define_options");
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
                .rule_mut("use_vue_valid_define_props");
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
                .rule_mut("use_vue_valid_next_tick");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-template-root" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_vue_valid_template_root");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-bind" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_valid_v_bind");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-cloak" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_vue_valid_v_cloak");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-else" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_valid_v_else");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-else-if" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .rule_mut("use_vue_valid_v_else_if");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-for" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_valid_v_for");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-html" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_valid_v_html");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-if" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_valid_v_if");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-is" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_valid_v_is");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-memo" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_valid_v_memo");
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
                .rule_mut("use_vue_valid_v_model");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-on" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_valid_v_on");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-once" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_valid_v_once");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-pre" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_valid_v_pre");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-show" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_valid_v_show");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-slot" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_valid_v_slot");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/valid-v-text" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("use_vue_valid_v_text");
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "yoda" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.unwrap_group_as_mut().rule_mut("no_yoda_expression");
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
