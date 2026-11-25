//! Generated file, do not edit by hand, see `xtask/codegen`

use super::{eslint_eslint, eslint_to_biome};
pub(crate) fn migrate_eslint_any_rule(
    rules: &mut biome_configuration::Rules,
    eslint_name: &str,
    rule_severity: eslint_eslint::Severity,
    options: &eslint_to_biome::MigrationOptions,
    results: &mut eslint_to_biome::MigrationResults,
) -> bool {
    let mapped_rules: &[eslint_to_biome::MappedRule] = match eslint_name {
        "@eslint-react/component-hook-factories" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_COMPONENT_HOOK_FACTORIES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/dom-no-dangerously-set-innerhtml" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::NO_DANGEROUSLY_SET_INNER_HTML
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/dom-no-dangerously-set-innerhtml-with-children" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Security (biome_configuration :: analyzer :: Security :: NO_DANGEROUSLY_SET_INNER_HTML_WITH_CHILDREN . untyped () ,) , is_inspired : false , }]
            }
        }
        "@eslint-react/dom-no-missing-button-type" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_BUTTON_TYPE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/dom-no-missing-iframe-sandbox" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::USE_IFRAME_SANDBOX.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/dom-no-render-return-value" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_RENDER_RETURN_VALUE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/dom-no-script-url" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::NO_SCRIPT_URL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/dom-no-unsafe-iframe-sandbox" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_UNSAFE_IFRAME_SANDBOX.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/dom-no-unsafe-target-blank" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::NO_BLANK_TARGET.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@eslint-react/dom-no-void-elements-with-children" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_VOID_ELEMENTS_WITH_CHILDREN
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/exhaustive-deps" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_EXHAUSTIVE_DEPENDENCIES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/jsx-no-children-prop" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_CHILDREN_PROP.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/jsx-no-leaked-dollar" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_JSX_LEAKED_DOLLAR.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/jsx-no-namespace" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_JSX_NAMESPACE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/jsx-no-useless-fragment" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_FRAGMENTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/naming-convention-context-name" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_REACT_NAMING_CONVENTION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/naming-convention-id-name" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_REACT_NAMING_CONVENTION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/naming-convention-ref-name" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_REACT_NAMING_CONVENTION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/no-array-index-key" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_ARRAY_INDEX_KEY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/no-direct-mutation-state" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_REACT_DIRECT_STATE_MUTATION
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@eslint-react/no-forward-ref" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_REACT_FORWARD_REF.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/no-missing-key" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_JSX_KEY_IN_ITERABLE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/no-nested-component-definitions" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_NESTED_COMPONENT_DEFINITIONS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@eslint-react/rsc-function-definition" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_REACT_ASYNC_SERVER_FUNCTION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@graphql-eslint/description-style" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_CONSISTENT_GRAPHQL_DESCRIPTIONS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@graphql-eslint/input-name" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_INPUT_NAME.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@graphql-eslint/lone-anonymous-operation" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_LONE_ANONYMOUS_OPERATION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@graphql-eslint/lone-executable-definition" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_LONE_EXECUTABLE_DEFINITION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@graphql-eslint/no-anonymous-operations" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_GRAPHQL_NAMED_OPERATIONS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@graphql-eslint/no-duplicate-fields" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DUPLICATE_FIELDS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@graphql-eslint/no-root-type" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_ROOT_TYPE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@graphql-eslint/require-deprecation-date" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_DEPRECATED_DATE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@graphql-eslint/require-deprecation-reason" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_DEPRECATED_REASON.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@graphql-eslint/unique-argument-names" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_DUPLICATE_ARGUMENT_NAMES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@graphql-eslint/unique-enum-value-names" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_DUPLICATE_ENUM_VALUE_NAMES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@graphql-eslint/unique-field-definition-names" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Correctness (biome_configuration :: analyzer :: Correctness :: NO_DUPLICATE_FIELD_DEFINITION_NAMES . untyped () ,) , is_inspired : false , }]
            }
        }
        "@graphql-eslint/unique-input-field-names" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_DUPLICATE_INPUT_FIELD_NAMES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@graphql-eslint/unique-operation-name" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Suspicious (biome_configuration :: analyzer :: Suspicious :: NO_DUPLICATE_GRAPHQL_OPERATION_NAME . untyped () ,) , is_inspired : true , }]
            }
        }
        "@graphql-eslint/unique-variable-names" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_DUPLICATE_VARIABLE_NAMES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@html-eslint/no-accesskey-attrs" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_ACCESS_KEY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@html-eslint/no-aria-hidden-on-focusable" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_ARIA_HIDDEN_ON_FOCUSABLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@html-eslint/no-duplicate-attrs" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_DUPLICATE_ATTRIBUTES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@html-eslint/no-empty-headings" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_HEADING_CONTENT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@html-eslint/no-inline-styles" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_INLINE_STYLES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@html-eslint/no-invalid-role" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_VALID_ARIA_ROLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@html-eslint/no-non-scalable-viewport" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_NON_SCALABLE_VIEWPORT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@html-eslint/no-obsolete-attrs" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_OBSOLETE_ATTRIBUTES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@html-eslint/no-obsolete-tags" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_OBSOLETE_TAGS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@html-eslint/no-positive-tabindex" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_POSITIVE_TABINDEX.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@html-eslint/no-redundant-role" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_REDUNDANT_ROLES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@html-eslint/no-target-blank" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::NO_BLANK_TARGET.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@html-eslint/require-button-type" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_BUTTON_TYPE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@html-eslint/require-frame-title" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_IFRAME_TITLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@html-eslint/require-img-alt" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_ALT_TEXT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@html-eslint/require-lang" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_HTML_LANG.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@html-eslint/require-li-container" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_MISPLACED_LIST_ELEMENTS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@html-eslint/use-baseline" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_BASELINE.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@mysticatea/no-this-in-static" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_THIS_IN_STATIC.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@mysticatea/prefer-for-of" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_FOR_OF.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@next/next/google-font-display" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_GOOGLE_FONT_DISPLAY
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@next/next/google-font-preconnect" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Performance(
                        biome_configuration::analyzer::Performance::USE_GOOGLE_FONT_PRECONNECT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@next/next/inline-script-id" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_INLINE_SCRIPT_ID.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@next/next/next-script-for-ga" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: USE_NEXT_SCRIPT_FOR_GOOGLE_ANALYTICS . untyped () ,) , is_inspired : true , }]
            }
        }
        "@next/next/no-async-client-component" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_NEXT_ASYNC_CLIENT_COMPONENT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@next/next/no-before-interactive-script-outside-document" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Correctness (biome_configuration :: analyzer :: Correctness :: NO_BEFORE_INTERACTIVE_SCRIPT_OUTSIDE_DOCUMENT . untyped () ,) , is_inspired : false , }]
            }
        }
        "@next/next/no-document-import-in-page" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DOCUMENT_IMPORT_IN_PAGE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@next/next/no-head-element" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_HEAD_ELEMENT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@next/next/no-head-import-in-document" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_HEAD_IMPORT_IN_DOCUMENT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@next/next/no-img-element" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Performance(
                        biome_configuration::analyzer::Performance::NO_IMG_ELEMENT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@next/next/no-sync-scripts" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Performance(
                        biome_configuration::analyzer::Performance::NO_SYNC_SCRIPTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@next/next/no-unwanted-polyfillio" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Performance(
                        biome_configuration::analyzer::Performance::NO_UNWANTED_POLYFILLIO
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@react-native/no-deep-imports" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_REACT_NATIVE_DEEP_IMPORTS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@react-native/platform-colors" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: USE_REACT_NATIVE_VALID_PLATFORM_COLORS . untyped () ,) , is_inspired : false , }]
            }
        }
        "@stylistic/jsx-curly-brace-presence" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_CONSISTENT_CURLY_BRACES.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@stylistic/jsx-self-closing-comp" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_SELF_CLOSING_ELEMENTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@tanstack/query/exhaustive-deps" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: USE_TANSTACK_QUERY_EXHAUSTIVE_DEPENDENCIES . untyped () ,) , is_inspired : false , }]
            }
        }
        "@tanstack/query/infinite-query-property-order" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: USE_TANSTACK_QUERY_INFINITE_QUERY_PROPERTY_ORDER . untyped () ,) , is_inspired : false , }]
            }
        }
        "@tanstack/query/mutation-property-order" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: USE_TANSTACK_QUERY_MUTATION_PROPERTY_ORDER . untyped () ,) , is_inspired : false , }]
            }
        }
        "@tanstack/query/no-rest-destructuring" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: NO_TANSTACK_QUERY_REST_DESTRUCTURING . untyped () ,) , is_inspired : false , }]
            }
        }
        "@tanstack/query/no-void-query-fn" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_TANSTACK_QUERY_VOID_QUERY_FN
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@tanstack/query/stable-query-client" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: USE_TANSTACK_QUERY_STABLE_QUERY_CLIENT . untyped () ,) , is_inspired : false , }]
            }
        }
        "@typescript-eslint/adjacent-overload-signatures" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_ADJACENT_OVERLOAD_SIGNATURES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/array-type" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_CONSISTENT_ARRAY_TYPE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/ban-ts-comment" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_TS_IGNORE.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/ban-types" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_BANNED_TYPES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/consistent-indexed-object-style" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_CONSISTENT_INDEXED_OBJECT_STYLE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/consistent-type-definitions" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_CONSISTENT_TYPE_DEFINITIONS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/consistent-type-exports" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_EXPORT_TYPE.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/consistent-type-imports" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_IMPORT_TYPE.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/default-param-last" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_DEFAULT_PARAMETER_LAST.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/dot-notation" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_LITERAL_KEYS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/explicit-function-return-type" => {
            const {
                &[
                    eslint_to_biome::MappedRule {
                        slot: eslint_to_biome::AnyRuleSlot::Nursery(
                            biome_configuration::analyzer::Nursery::USE_EXPLICIT_RETURN_TYPE
                                .untyped(),
                        ),
                        is_inspired: true,
                    },
                    eslint_to_biome::MappedRule {
                        slot: eslint_to_biome::AnyRuleSlot::Nursery(
                            biome_configuration::analyzer::Nursery::USE_EXPLICIT_TYPE.untyped(),
                        ),
                        is_inspired: true,
                    },
                ]
            }
        }
        "@typescript-eslint/explicit-member-accessibility" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_CONSISTENT_MEMBER_ACCESSIBILITY
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/explicit-module-boundary-types" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_EXPLICIT_TYPE.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/max-params" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_MAX_PARAMS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/method-signature-style" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_CONSISTENT_METHOD_SIGNATURES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/naming-convention" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_NAMING_CONVENTION.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/no-array-constructor" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_ARRAY_LITERALS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-array-delete" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_ARRAY_DELETE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-base-to-string" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_BASE_TO_STRING.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-deprecated" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DEPRECATED_IMPORTS.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/no-dupe-class-members" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DUPLICATE_CLASS_MEMBERS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-duplicate-enum-values" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DUPLICATE_ENUM_VALUES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-duplicate-type-constituents" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_DUPLICATE_TYPE_CONSTITUENTS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/no-dynamic-delete" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_DYNAMIC_DELETE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-empty-function" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_EMPTY_BLOCK_STATEMENTS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-empty-interface" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_EMPTY_INTERFACE.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/no-empty-object-type" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_BANNED_TYPES.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/no-explicit-any" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_EXPLICIT_ANY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-extra-non-null-assertion" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_EXTRA_NON_NULL_ASSERTION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-extraneous-class" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_STATIC_ONLY_CLASS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-floating-promises" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_FLOATING_PROMISES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-for-in-array" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_FOR_IN.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/no-implied-eval" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_IMPLIED_EVAL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-inferrable-types" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_INFERRABLE_TYPES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-invalid-void-type" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_CONFUSING_VOID_TYPE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-loop-func" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_LOOP_FUNC.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-loss-of-precision" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_PRECISION_LOSS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-magic-numbers" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_MAGIC_NUMBERS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-meaningless-void-operator" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_MEANINGLESS_VOID_OPERATOR
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-misused-new" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_MISLEADING_INSTANTIATOR
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-misused-promises" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_MISUSED_PROMISES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-misused-spread" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_MISUSED_SPREAD.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-mixed-enums" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_CONSISTENT_ENUM_VALUE_TYPE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-namespace" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_NAMESPACE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-non-null-asserted-optional-chain" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Suspicious (biome_configuration :: analyzer :: Suspicious :: NO_NON_NULL_ASSERTED_OPTIONAL_CHAIN . untyped () ,) , is_inspired : false , }]
            }
        }
        "@typescript-eslint/no-non-null-assertion" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_NON_NULL_ASSERTION.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-redeclare" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_REDECLARE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-redundant-type-constituents" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_REDUNDANT_TYPE_CONSTITUENTS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/no-require-imports" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_COMMON_JS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-restricted-imports" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_RESTRICTED_IMPORTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-restricted-types" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_RESTRICTED_TYPES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-shadow" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_SHADOW.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-this-alias" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_THIS_ALIAS.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/no-unnecessary-condition" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_UNNECESSARY_CONDITIONS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/no-unnecessary-parameter-property-assignment" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: NO_USELESS_PARAMETER_PROPERTY_ASSIGNMENT . untyped () ,) , is_inspired : false , }]
            }
        }
        "@typescript-eslint/no-unnecessary-qualifier" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_USELESS_QUALIFIER.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-unnecessary-template-expression" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Complexity (biome_configuration :: analyzer :: Complexity :: NO_UNNECESSARY_TEMPLATE_EXPRESSION . untyped () ,) , is_inspired : true , }]
            }
        }
        "@typescript-eslint/no-unnecessary-type-assertion" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_USELESS_TYPE_ASSERTION.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-unnecessary-type-constraint" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_TYPE_CONSTRAINT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-unnecessary-type-conversion" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_USELESS_TYPE_CONVERSION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-unsafe-declaration-merging" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_UNSAFE_DECLARATION_MERGING
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-unsafe-function-type" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_BANNED_TYPES.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/no-unsafe-return" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_UNSAFE_RETURN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-unsafe-unary-minus" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_UNSAFE_UNARY_MINUS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-unused-vars" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_UNUSED_VARIABLES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-use-before-define" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Correctness (biome_configuration :: analyzer :: Correctness :: NO_INVALID_USE_BEFORE_DECLARATION . untyped () ,) , is_inspired : false , }]
            }
        }
        "@typescript-eslint/no-useless-constructor" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_CONSTRUCTOR.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-useless-default-assignment" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_USELESS_DEFAULT_ASSIGNMENT
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/no-useless-empty-export" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_EMPTY_EXPORT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-var-requires" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_COMMON_JS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/no-wrapper-object-types" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_BANNED_TYPES.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/only-throw-error" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_THROW_ONLY_ERROR.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/parameter-properties" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_PARAMETER_PROPERTIES.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/prefer-as-const" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_AS_CONST_ASSERTION.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/prefer-enum-initializers" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_ENUM_INITIALIZERS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/prefer-find" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_ARRAY_FIND.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/prefer-for-of" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_FOR_OF.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/prefer-function-type" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_SHORTHAND_FUNCTION_TYPE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/prefer-includes" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_INCLUDES.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/prefer-literal-enum-member" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_LITERAL_ENUM_MEMBERS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/prefer-namespace-keyword" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_NAMESPACE_KEYWORD.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/prefer-nullish-coalescing" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_NULLISH_COALESCING.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/prefer-optional-chain" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_OPTIONAL_CHAIN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/prefer-promise-reject-errors" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_PROMISE_REJECT_ERRORS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/prefer-readonly" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_READONLY_CLASS_PROPERTIES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/prefer-reduce-type-parameter" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_REDUCE_TYPE_PARAMETER.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/prefer-regexp-exec" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_REGEXP_EXEC.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/prefer-string-starts-ends-with" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_STRING_STARTS_ENDS_WITH
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/require-array-sort-compare" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_ARRAY_SORT_COMPARE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/require-await" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_AWAIT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/restrict-plus-operands" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_UNSAFE_PLUS_OPERANDS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/strict-boolean-expressions" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_STRICT_BOOLEAN_EXPRESSIONS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "@typescript-eslint/switch-exhaustiveness-check" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_EXHAUSTIVE_SWITCH_CASES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/triple-slash-reference" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_TRIPLE_SLASH_REFERENCE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/unified-signatures" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_UNIFIED_TYPE_SIGNATURES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "@typescript-eslint/use-await-thenable" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_AWAIT_THENABLE.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "accessor-pairs" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_ACCESSOR_PAIRS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "array-callback-return" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_ITERABLE_CALLBACK_RETURN
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "arrow-body-style" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_CONSISTENT_ARROW_RETURN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/alt-text" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_ALT_TEXT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/anchor-ambiguous-text" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_AMBIGUOUS_ANCHOR_TEXT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/anchor-has-content" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_ANCHOR_CONTENT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/anchor-is-valid" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_VALID_ANCHOR.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/aria-activedescendant-has-tabindex" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: A11y (biome_configuration :: analyzer :: A11y :: USE_ARIA_ACTIVEDESCENDANT_WITH_TABINDEX . untyped () ,) , is_inspired : false , }]
            }
        }
        "astro/jsx-a11y/aria-props" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_VALID_ARIA_PROPS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/aria-proptypes" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_VALID_ARIA_VALUES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/aria-role" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_VALID_ARIA_ROLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/aria-unsupported-elements" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_ARIA_UNSUPPORTED_ELEMENTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/autocomplete-valid" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_VALID_AUTOCOMPLETE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/click-events-have-key-events" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_KEY_WITH_CLICK_EVENTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/control-has-associated-label" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_CONTROL_LABEL.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "astro/jsx-a11y/heading-has-content" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_HEADING_CONTENT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/html-has-lang" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_HTML_LANG.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/iframe-has-title" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_IFRAME_TITLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/img-redundant-alt" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_REDUNDANT_ALT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/interactive-supports-focus" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_FOCUSABLE_INTERACTIVE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/label-has-associated-control" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_LABEL_WITHOUT_CONTROL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/lang" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_VALID_LANG.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/media-has-caption" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_MEDIA_CAPTION.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/mouse-events-have-key-events" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_KEY_WITH_MOUSE_EVENTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/no-access-key" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_ACCESS_KEY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/no-aria-hidden-on-focusable" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_ARIA_HIDDEN_ON_FOCUSABLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/no-autofocus" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_AUTOFOCUS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/no-distracting-elements" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_DISTRACTING_ELEMENTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/no-interactive-element-to-noninteractive-role" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: A11y (biome_configuration :: analyzer :: A11y :: NO_INTERACTIVE_ELEMENT_TO_NONINTERACTIVE_ROLE . untyped () ,) , is_inspired : false , }]
            }
        }
        "astro/jsx-a11y/no-noninteractive-element-interactions" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_NONINTERACTIVE_ELEMENT_INTERACTIONS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/no-noninteractive-element-to-interactive-role" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: A11y (biome_configuration :: analyzer :: A11y :: NO_NONINTERACTIVE_ELEMENT_TO_INTERACTIVE_ROLE . untyped () ,) , is_inspired : false , }]
            }
        }
        "astro/jsx-a11y/no-noninteractive-tabindex" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_NONINTERACTIVE_TABINDEX.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/no-redundant-roles" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_REDUNDANT_ROLES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/no-static-element-interactions" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_STATIC_ELEMENT_INTERACTIONS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/prefer-tag-over-role" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_SEMANTIC_ELEMENTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/role-has-required-aria-props" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_ARIA_PROPS_FOR_ROLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/role-supports-aria-props" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_ARIA_PROPS_SUPPORTED_BY_ROLE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/scope" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_HEADER_SCOPE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/jsx-a11y/tabindex-no-positive" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_POSITIVE_TABINDEX.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/missing-client-only-directive-value" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: USE_ASTRO_CLIENT_ONLY_DIRECTIVE_VALUE . untyped () ,) , is_inspired : true , }]
            }
        }
        "astro/no-conflict-set-directives" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_ASTRO_CONFLICTING_SET_DIRECTIVES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "astro/no-exports-from-components" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_ASTRO_COMPONENT_DEFAULT_EXPORT
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "astro/no-set-html-directive" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_ASTRO_SET_HTML_DIRECTIVE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "barrel-files/avoid-barrel-files" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Performance(
                        biome_configuration::analyzer::Performance::NO_BARREL_FILE.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "barrel-files/avoid-namespace-import" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Performance(
                        biome_configuration::analyzer::Performance::NO_NAMESPACE_IMPORT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "barrel-files/avoid-re-export-all" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Performance(
                        biome_configuration::analyzer::Performance::NO_RE_EXPORT_ALL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "better-tailwindcss/enforce-consistent-class-order" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_TAILWIND_SORTED_CLASSES
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "better-tailwindcss/enforce-consistent-variable-syntax" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_TAILWIND_VARIABLES_SHORTHAND
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "better-tailwindcss/enforce-shorthand-classes" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_TAILWIND_SHORTHAND_CLASSES
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "better-tailwindcss/no-conflicting-classes" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_TAILWIND_CONFLICTING_CLASSES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "better-tailwindcss/no-deprecated-classes" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_TAILWIND_LEGACY_UTILITIES
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "better-tailwindcss/no-restricted-classes" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_TAILWIND_RESTRICTED_CLASSES
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "camelcase" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_NAMING_CONVENTION.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "class-methods-use-this" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_THIS_IN_CLASS_METHODS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "complexity" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Complexity (biome_configuration :: analyzer :: Complexity :: NO_EXCESSIVE_COGNITIVE_COMPLEXITY . untyped () ,) , is_inspired : true , }]
            }
        }
        "constructor-super" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_INVALID_CONSTRUCTOR_SUPER
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "css/font-family-fallbacks" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_GENERIC_FONT_NAMES.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "css/no-duplicate-imports" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DUPLICATE_AT_IMPORT_RULES
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "css/no-duplicate-keyframe-selectors" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Suspicious (biome_configuration :: analyzer :: Suspicious :: NO_DUPLICATE_SELECTORS_KEYFRAME_BLOCK . untyped () ,) , is_inspired : false , }]
            }
        }
        "css/no-empty-blocks" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_EMPTY_BLOCK.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "css/no-important" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_IMPORTANT_STYLES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "css/no-invalid-at-rule-placement" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Correctness (biome_configuration :: analyzer :: Correctness :: NO_INVALID_POSITION_AT_IMPORT_RULE . untyped () ,) , is_inspired : true , }]
            }
        }
        "css/no-invalid-at-rules" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_UNKNOWN_AT_RULES.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "css/no-invalid-named-grid-areas" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_INVALID_GRID_AREAS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "css/no-invalid-properties" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_UNKNOWN_PROPERTY.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "css/no-unmatchable-selectors" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_UNMATCHABLE_ANB_SELECTOR
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "css/selector-complexity" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_EXCESSIVE_SELECTOR_COMPLEXITY
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "css/use-baseline" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_BASELINE.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "css/use-layers" => {
            const {
                &[
                    eslint_to_biome::MappedRule {
                        slot: eslint_to_biome::AnyRuleSlot::Nursery(
                            biome_configuration::analyzer::Nursery::USE_LAYERED_STYLES.untyped(),
                        ),
                        is_inspired: true,
                    },
                    eslint_to_biome::MappedRule {
                        slot: eslint_to_biome::AnyRuleSlot::Nursery(
                            biome_configuration::analyzer::Nursery::USE_NAMED_LAYER.untyped(),
                        ),
                        is_inspired: true,
                    },
                ]
            }
        }
        "curly" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_BLOCK_STATEMENTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "default-case" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_DEFAULT_SWITCH_CLAUSE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "default-case-last" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_DEFAULT_SWITCH_CLAUSE_LAST
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "default-param-last" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_DEFAULT_PARAMETER_LAST.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "dot-notation" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_LITERAL_KEYS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "drizzle/enforce-delete-with-where" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DRIZZLE_DELETE_WITHOUT_WHERE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "drizzle/enforce-update-with-where" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DRIZZLE_UPDATE_WITHOUT_WHERE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "e18e/ban-dependencies" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_RESTRICTED_DEPENDENCIES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "e18e/prefer-array-at" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_AT_INDEX.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "e18e/prefer-date-now" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_DATE_NOW.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "e18e/prefer-exponentiation-operator" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_EXPONENTIATION_OPERATOR.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "e18e/prefer-includes" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_INCLUDES.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "e18e/prefer-object-has-own" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_PROTOTYPE_BUILTINS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "e18e/prefer-spread-syntax" => {
            const {
                &[
                    eslint_to_biome::MappedRule {
                        slot: eslint_to_biome::AnyRuleSlot::Style(
                            biome_configuration::analyzer::Style::USE_OBJECT_SPREAD.untyped(),
                        ),
                        is_inspired: true,
                    },
                    eslint_to_biome::MappedRule {
                        slot: eslint_to_biome::AnyRuleSlot::Style(
                            biome_configuration::analyzer::Style::USE_SPREAD_OVER_APPLY.untyped(),
                        ),
                        is_inspired: true,
                    },
                ]
            }
        }
        "eqeqeq" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DOUBLE_EQUALS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "for-direction" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_VALID_FOR_DIRECTION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "func-name-matching" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_MATCHING_FUNCTION_NAME
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "func-names" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_NAMED_FUNCTION.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "func-style" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_CONSISTENT_FUNCTION_STYLE
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "getter-return" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_GETTER_RETURN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "global-require" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_TOP_LEVEL_REQUIRE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "grouped-accessor-pairs" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_GROUPED_ACCESSOR_PAIRS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "guard-for-in" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_GUARD_FOR_IN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "import-access/eslint-plugin-import-access" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_PRIVATE_IMPORTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "import/consistent-type-specifier-style" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_IMPORT_TYPE.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "import/default" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_UNRESOLVED_DEFAULT_IMPORT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "import/enforce-node-protocol-usage" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_NODEJS_IMPORT_PROTOCOL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "import/exports-last" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_EXPORTS_LAST.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "import/extensions" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_IMPORT_EXTENSIONS.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "import/first" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_IMPORTS_FIRST.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "import/named" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_UNRESOLVED_IMPORTS.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "import/namespace" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VALID_NAMESPACE_IMPORTS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "import/no-commonjs" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_COMMON_JS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "import/no-cycle" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_IMPORT_CYCLES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "import/no-default-export" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_DEFAULT_EXPORT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "import/no-deprecated" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DEPRECATED_IMPORTS.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "import/no-dynamic-require" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_DYNAMIC_REQUIRE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "import/no-extraneous-dependencies" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_UNDECLARED_DEPENDENCIES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "import/no-nodejs-modules" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_NODEJS_MODULES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "import/no-self-import" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_SELF_IMPORT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "init-declarations" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_UNASSIGNED_VARIABLES
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "jest/consistent-test-it" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_CONSISTENT_TEST_IT.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "jest/expect-expect" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_EXPECT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/max-nested-describe" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_EXCESSIVE_NESTED_TEST_SUITES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/no-conditional-expect" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_CONDITIONAL_EXPECT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/no-disabled-tests" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_SKIPPED_TESTS.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "jest/no-done-callback" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_DONE_CALLBACK.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/no-duplicate-hooks" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DUPLICATE_TEST_HOOKS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "jest/no-export" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_EXPORTS_IN_TEST.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "jest/no-focused-tests" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_FOCUSED_TESTS.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "jest/no-hooks" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_TEST_HOOKS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/no-identical-title" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_IDENTICAL_TEST_TITLE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/no-jasmine-globals" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_JASMINE_GLOBALS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/no-standalone-expect" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_MISPLACED_ASSERTION.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "jest/no-test-return-statement" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_RETURN_IN_TEST.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/prefer-called-with" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_CALLED_WITH.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/prefer-equality-matcher" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_EXPECT_EQUALITY_MATCHER
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/prefer-hooks-in-order" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_TEST_HOOKS_IN_ORDER.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/prefer-hooks-on-top" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_TEST_HOOKS_ON_TOP.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/prefer-importing-jest-globals" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_TEST_GLOBALS.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "jest/prefer-lowercase-title" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_LOWERCASE_TEST_TITLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/prefer-spy-on" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_SPY_ON.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/prefer-strict-equal" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_EXPECT_TO_STRICT_EQUAL
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/prefer-to-contain" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_EXPECT_TO_CONTAIN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/prefer-to-have-been-called" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_EXPECT_TO_HAVE_BEEN_CALLED
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/prefer-to-have-been-called-times" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: USE_EXPECT_TO_HAVE_BEEN_CALLED_TIMES . untyped () ,) , is_inspired : false , }]
            }
        }
        "jest/prefer-to-have-length" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_EXPECT_TO_HAVE_LENGTH.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/require-top-level-describe" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_TOP_LEVEL_DESCRIBE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jest/valid-title" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VALID_TEST_TITLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsdoc/no-multi-asterisks" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_SINGLE_JS_DOC_ASTERISK
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "json/no-duplicate-keys" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DUPLICATE_OBJECT_KEYS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "json/no-empty-keys" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_EMPTY_OBJECT_KEYS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "json/no-unnormalized-keys" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_CONSISTENT_OBJECT_KEYS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "json/no-unsafe-values" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_JSON_UNSAFE_VALUES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "json/top-level-interop" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_TOP_LEVEL_LITERALS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/alt-text" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_ALT_TEXT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/anchor-ambiguous-text" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_AMBIGUOUS_ANCHOR_TEXT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/anchor-has-content" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_ANCHOR_CONTENT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/anchor-is-valid" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_VALID_ANCHOR.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/aria-activedescendant-has-tabindex" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: A11y (biome_configuration :: analyzer :: A11y :: USE_ARIA_ACTIVEDESCENDANT_WITH_TABINDEX . untyped () ,) , is_inspired : false , }]
            }
        }
        "jsx-a11y/aria-props" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_VALID_ARIA_PROPS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/aria-proptypes" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_VALID_ARIA_VALUES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/aria-role" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_VALID_ARIA_ROLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/aria-unsupported-elements" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_ARIA_UNSUPPORTED_ELEMENTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/autocomplete-valid" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_VALID_AUTOCOMPLETE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/click-events-have-key-events" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_KEY_WITH_CLICK_EVENTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/control-has-associated-label" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_CONTROL_LABEL.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "jsx-a11y/heading-has-content" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_HEADING_CONTENT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/html-has-lang" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_HTML_LANG.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/iframe-has-title" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_IFRAME_TITLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/img-redundant-alt" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_REDUNDANT_ALT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/interactive-supports-focus" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_FOCUSABLE_INTERACTIVE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/label-has-associated-control" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_LABEL_WITHOUT_CONTROL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/label-has-for" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_LABEL_WITHOUT_CONTROL.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "jsx-a11y/lang" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_VALID_LANG.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/media-has-caption" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_MEDIA_CAPTION.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/mouse-events-have-key-events" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_KEY_WITH_MOUSE_EVENTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/no-access-key" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_ACCESS_KEY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/no-aria-hidden-on-focusable" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_ARIA_HIDDEN_ON_FOCUSABLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/no-autofocus" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_AUTOFOCUS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/no-distracting-elements" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_DISTRACTING_ELEMENTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/no-interactive-element-to-noninteractive-role" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: A11y (biome_configuration :: analyzer :: A11y :: NO_INTERACTIVE_ELEMENT_TO_NONINTERACTIVE_ROLE . untyped () ,) , is_inspired : false , }]
            }
        }
        "jsx-a11y/no-noninteractive-element-interactions" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_NONINTERACTIVE_ELEMENT_INTERACTIONS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/no-noninteractive-element-to-interactive-role" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: A11y (biome_configuration :: analyzer :: A11y :: NO_NONINTERACTIVE_ELEMENT_TO_INTERACTIVE_ROLE . untyped () ,) , is_inspired : false , }]
            }
        }
        "jsx-a11y/no-noninteractive-tabindex" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_NONINTERACTIVE_TABINDEX.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/no-redundant-roles" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_REDUNDANT_ROLES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/no-static-element-interactions" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_STATIC_ELEMENT_INTERACTIONS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/prefer-tag-over-role" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_SEMANTIC_ELEMENTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/role-has-required-aria-props" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_ARIA_PROPS_FOR_ROLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/role-supports-aria-props" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_ARIA_PROPS_SUPPORTED_BY_ROLE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/scope" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_HEADER_SCOPE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "jsx-a11y/tabindex-no-positive" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::NO_POSITIVE_TABINDEX.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "max-classes-per-file" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_EXCESSIVE_CLASSES_PER_FILE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "max-depth" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_EXCESSIVE_NESTED_BLOCKS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "max-lines" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_EXCESSIVE_LINES_PER_FILE.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "max-lines-per-function" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_EXCESSIVE_LINES_PER_FUNCTION
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "max-nested-callbacks" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_EXCESSIVE_NESTED_CALLBACKS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "max-params" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_MAX_PARAMS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "max-statements" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: NO_EXCESSIVE_STATEMENTS_PER_FUNCTION . untyped () ,) , is_inspired : false , }]
            }
        }
        "n/global-require" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_TOP_LEVEL_REQUIRE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "n/no-deprecated-api" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_BUFFER_CONSTRUCTOR.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "n/no-new-require" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_NEW_REQUIRE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "n/no-path-concat" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_PATH_CONCAT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "n/no-process-env" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_PROCESS_ENV.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "n/no-process-exit" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_PROCESS_EXIT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "n/no-sync" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_SYNC_METHODS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "n/prefer-node-protocol" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_NODEJS_IMPORT_PROTOCOL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "new-cap" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_CAPITALIZED_CONSTRUCTORS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-alert" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_ALERT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-array-constructor" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_ARRAY_LITERALS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-async-promise-executor" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_ASYNC_PROMISE_EXECUTOR
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-await-in-loop" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Performance(
                        biome_configuration::analyzer::Performance::NO_AWAIT_IN_LOOPS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-bitwise" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_BITWISE_OPERATORS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-buffer-constructor" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_BUFFER_CONSTRUCTOR.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-caller" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_ARGUMENTS_CALLER_OR_CALLEE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-case-declarations" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_SWITCH_DECLARATIONS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-catch-shadow" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_SHADOW.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-class-assign" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_CLASS_ASSIGN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-compare-neg-zero" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_COMPARE_NEG_ZERO.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-cond-assign" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_ASSIGN_IN_EXPRESSIONS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "no-console" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_CONSOLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-const-assign" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_CONST_ASSIGN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-constant-binary-expression" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_CONSTANT_BINARY_EXPRESSIONS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-constant-condition" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_CONSTANT_CONDITION.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-constructor-return" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_CONSTRUCTOR_RETURN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-continue" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_CONTINUE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-control-regex" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_CONTROL_CHARACTERS_IN_REGEX
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-debugger" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DEBUGGER.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-delete-var" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Performance(
                        biome_configuration::analyzer::Performance::NO_DELETE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-div-regex" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_DIV_REGEX.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-dupe-args" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DUPLICATE_PARAMETERS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-dupe-class-members" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DUPLICATE_CLASS_MEMBERS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-dupe-else-if" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DUPLICATE_ELSE_IF.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-dupe-keys" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DUPLICATE_OBJECT_KEYS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-duplicate-case" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DUPLICATE_CASE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-else-return" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_USELESS_ELSE.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "no-empty" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_EMPTY_BLOCK_STATEMENTS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-empty-character-class" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Correctness (biome_configuration :: analyzer :: Correctness :: NO_EMPTY_CHARACTER_CLASS_IN_REGEX . untyped () ,) , is_inspired : false , }]
            }
        }
        "no-empty-function" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_EMPTY_BLOCK_STATEMENTS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-empty-pattern" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_EMPTY_PATTERN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-empty-static-block" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_EMPTY_BLOCK_STATEMENTS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-eq-null" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_EQUALS_TO_NULL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-eval" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::NO_GLOBAL_EVAL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-ex-assign" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_CATCH_ASSIGN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-extend-native" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_EXTEND_NATIVE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-extra-bind" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_USELESS_BIND.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-extra-boolean-cast" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_EXTRA_BOOLEAN_CAST.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-extra-label" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_LABEL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-fallthrough" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_FALLTHROUGH_SWITCH_CLAUSE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-func-assign" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_FUNCTION_ASSIGN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-global-assign" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_GLOBAL_ASSIGN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-implicit-coercion" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_IMPLICIT_COERCIONS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-implied-eval" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_IMPLIED_EVAL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-import-assign" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_IMPORT_ASSIGN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-inner-declarations" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_INNER_DECLARATIONS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-invalid-this" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_INVALID_THIS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-irregular-whitespace" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_IRREGULAR_WHITESPACE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-label-var" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_LABEL_VAR.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-labels" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_CONFUSING_LABELS.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "no-lone-blocks" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_LONE_BLOCK_STATEMENTS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-lonely-if" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_COLLAPSED_ELSE_IF.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-loop-func" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_LOOP_FUNC.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-loss-of-precision" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_PRECISION_LOSS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-magic-numbers" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_MAGIC_NUMBERS.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "no-misleading-character-class" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_MISLEADING_CHARACTER_CLASS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-multi-assign" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_MULTI_ASSIGN.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "no-multi-str" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_MULTILINE_STRING.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-native-reassign" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_GLOBAL_ASSIGN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-negated-condition" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_NEGATION_ELSE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-negated-in-lhs" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_UNSAFE_NEGATION.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "no-nested-ternary" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_NESTED_TERNARY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-new" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_UNUSED_INSTANTIATION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-new-func" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_IMPLIED_EVAL.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "no-new-native-nonconstructor" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Correctness (biome_configuration :: analyzer :: Correctness :: NO_INVALID_BUILTIN_INSTANTIATION . untyped () ,) , is_inspired : false , }]
            }
        }
        "no-new-object" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_OBJECT_CONSTRUCTOR.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "no-new-require" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_NEW_REQUIRE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-new-symbol" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Correctness (biome_configuration :: analyzer :: Correctness :: NO_INVALID_BUILTIN_INSTANTIATION . untyped () ,) , is_inspired : true , }]
            }
        }
        "no-new-wrappers" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_CONSISTENT_BUILTIN_INSTANTIATION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-nonoctal-decimal-escape" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_NONOCTAL_DECIMAL_ESCAPE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-obj-calls" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_GLOBAL_OBJECT_CALLS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-object-constructor" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_OBJECT_CONSTRUCTOR.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-octal" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_OCTAL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-octal-escape" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_OCTAL_ESCAPE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-param-reassign" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_PARAMETER_ASSIGN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-plusplus" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_INCREMENT_DECREMENT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-process-env" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_PROCESS_ENV.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-process-exit" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_PROCESS_EXIT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-promise-executor-return" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_PROMISE_EXECUTOR_RETURN
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-proto" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_PROTO.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-prototype-builtins" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_PROTOTYPE_BUILTINS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-redeclare" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_REDECLARE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-regex-spaces" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_ADJACENT_SPACES_IN_REGEX
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-restricted-globals" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_RESTRICTED_GLOBALS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-restricted-imports" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_RESTRICTED_IMPORTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-restricted-properties" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_JS_RESTRICTED_PROPERTIES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-return-assign" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_RETURN_ASSIGN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-script-url" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::NO_SCRIPT_URL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-secrets/no-secrets" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::NO_SECRETS.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "no-self-assign" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_SELF_ASSIGN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-self-compare" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_SELF_COMPARE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-sequences" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_COMMA_OPERATOR.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-setter-return" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_SETTER_RETURN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-shadow" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_SHADOW.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-shadow-restricted-names" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_SHADOW_RESTRICTED_NAMES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-sparse-arrays" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_SPARSE_ARRAY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-template-curly-in-string" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_TEMPLATE_CURLY_IN_STRING
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-ternary" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_TERNARY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-this-before-super" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_UNREACHABLE_SUPER.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-throw-literal" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_THROW_ONLY_ERROR.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "no-unassigned-vars" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_UNASSIGNED_VARIABLES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-undef" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_UNDECLARED_VARIABLES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-undef-init" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Complexity (biome_configuration :: analyzer :: Complexity :: NO_USELESS_UNDEFINED_INITIALIZATION . untyped () ,) , is_inspired : false , }]
            }
        }
        "no-underscore-dangle" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_DANGLING_UNDERSCORE.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "no-unmodified-loop-condition" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_UNMODIFIED_LOOP_CONDITION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-unneeded-ternary" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_TERNARY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-unreachable" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_UNREACHABLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-unsafe-finally" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_UNSAFE_FINALLY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-unsafe-negation" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_UNSAFE_NEGATION.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-unsafe-optional-chaining" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_UNSAFE_OPTIONAL_CHAINING
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-unused-expressions" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_UNUSED_EXPRESSIONS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-unused-labels" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_UNUSED_LABELS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-unused-private-class-members" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_UNUSED_PRIVATE_CLASS_MEMBERS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "no-unused-vars" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_UNUSED_VARIABLES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-use-before-define" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Correctness (biome_configuration :: analyzer :: Correctness :: NO_INVALID_USE_BEFORE_DECLARATION . untyped () ,) , is_inspired : false , }]
            }
        }
        "no-useless-assignment" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_USELESS_ASSIGNMENT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-useless-backreference" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_USELESS_REGEX_BACKREFS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-useless-call" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_USELESS_CALL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-useless-catch" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_CATCH.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-useless-computed-key" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_LITERAL_KEYS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-useless-concat" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_STRING_CONCAT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-useless-constructor" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_CONSTRUCTOR.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-useless-escape" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_ESCAPE_IN_REGEX
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-useless-rename" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_RENAME.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-useless-return" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_RETURN.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "no-var" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_VAR.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-void" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_VOID.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "no-with" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_WITH.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "object-shorthand" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_CONSISTENT_OBJECT_DEFINITIONS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "one-var" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_SINGLE_VAR_DECLARATOR.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "operator-assignment" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_SHORTHAND_ASSIGN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "package-json-dependencies/duplicate-dependencies" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DUPLICATE_DEPENDENCIES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "package-json/require-cpu" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_PACKAGE_JSON_CPU.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "package-json/require-engines" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_PACKAGE_JSON_ENGINES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "package-json/unique-dependencies" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DUPLICATE_DEPENDENCIES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "package-json/valid-cpu" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_PACKAGE_JSON_VALID_CPU
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "package-json/valid-engines" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_PACKAGE_JSON_VALID_ENGINES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "package-json/valid-license" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_PACKAGE_JSON_VALID_LICENSE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/expect-expect" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_EXPECT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/missing-playwright-await" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_PLAYWRIGHT_MISSING_AWAIT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/no-conditional-expect" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_CONDITIONAL_EXPECT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/no-element-handle" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_PLAYWRIGHT_ELEMENT_HANDLE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/no-eval" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_PLAYWRIGHT_EVAL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/no-focused-test" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_FOCUSED_TESTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/no-force-option" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_PLAYWRIGHT_FORCE_OPTION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/no-hooks" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_TEST_HOOKS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/no-identical-title" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_IDENTICAL_TEST_TITLE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/no-networkidle" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_PLAYWRIGHT_NETWORKIDLE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/no-nth-methods" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_PLAYWRIGHT_NTH_METHODS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/no-page-pause" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_PLAYWRIGHT_PAGE_PAUSE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/no-skipped-test" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_SKIPPED_TESTS.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "playwright/no-test-return-statement" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_RETURN_IN_TEST.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/no-unsafe-references" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_PLAYWRIGHT_UNSAFE_REFERENCES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/no-useless-await" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_PLAYWRIGHT_USELESS_AWAIT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/no-wait-for-navigation" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Suspicious (biome_configuration :: analyzer :: Suspicious :: NO_PLAYWRIGHT_WAIT_FOR_NAVIGATION . untyped () ,) , is_inspired : false , }]
            }
        }
        "playwright/no-wait-for-selector" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_PLAYWRIGHT_WAIT_FOR_SELECTOR
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/no-wait-for-timeout" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_PLAYWRIGHT_WAIT_FOR_TIMEOUT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/prefer-equality-matcher" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_EXPECT_EQUALITY_MATCHER
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/prefer-hooks-in-order" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_TEST_HOOKS_IN_ORDER.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/prefer-hooks-on-top" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_TEST_HOOKS_ON_TOP.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/prefer-lowercase-title" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_LOWERCASE_TEST_TITLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/prefer-strict-equal" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_EXPECT_TO_STRICT_EQUAL
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/prefer-to-contain" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_EXPECT_TO_CONTAIN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/prefer-to-have-length" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_EXPECT_TO_HAVE_LENGTH.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/require-top-level-describe" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_TOP_LEVEL_DESCRIBE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "playwright/valid-describe-callback" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Correctness (biome_configuration :: analyzer :: Correctness :: USE_PLAYWRIGHT_VALID_DESCRIBE_CALLBACK . untyped () ,) , is_inspired : false , }]
            }
        }
        "prefer-arrow-callback" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_ARROW_FUNCTION.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "prefer-const" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_CONST.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "prefer-destructuring" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_DESTRUCTURING.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "prefer-exponentiation-operator" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_EXPONENTIATION_OPERATOR.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "prefer-named-capture-group" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_NAMED_CAPTURE_GROUP.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "prefer-numeric-literals" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_NUMERIC_LITERALS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "prefer-object-has-own" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_PROTOTYPE_BUILTINS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "prefer-object-spread" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_OBJECT_SPREAD.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "prefer-promise-reject-errors" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_PROMISE_REJECT_ERRORS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "prefer-regex-literals" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_REGEX_LITERALS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "prefer-rest-params" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_ARGUMENTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "prefer-spread" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_SPREAD_OVER_APPLY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "prefer-template" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_TEMPLATE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "preserve-caught-error" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_ERROR_CAUSE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "promise/no-nesting" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_NESTED_PROMISES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "promise/no-return-in-finally" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_RETURN_IN_FINALLY.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "qwik/jsx-a" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_VALID_ANCHOR.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "qwik/jsx-img" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_IMAGE_SIZE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "qwik/jsx-key" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_JSX_KEY_IN_ITERABLE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "qwik/jsx-no-script-url" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::NO_SCRIPT_URL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "qwik/loader-location" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_QWIK_LOADER_LOCATION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "qwik/no-react-props" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_REACT_SPECIFIC_PROPS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "qwik/no-use-visible-task" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_QWIK_USE_VISIBLE_TASK
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "qwik/prefer-classlist" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_QWIK_CLASSLIST.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "qwik/use-method-usage" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_QWIK_METHOD_USAGE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "qwik/valid-lexical-scope" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_QWIK_VALID_LEXICAL_SCOPE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "radix" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_PARSE_INT_RADIX.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-dom/no-dangerously-set-innerhtml" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::NO_DANGEROUSLY_SET_INNER_HTML
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-dom/no-dangerously-set-innerhtml-with-children" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Security (biome_configuration :: analyzer :: Security :: NO_DANGEROUSLY_SET_INNER_HTML_WITH_CHILDREN . untyped () ,) , is_inspired : false , }]
            }
        }
        "react-dom/no-missing-button-type" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_BUTTON_TYPE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-dom/no-missing-iframe-sandbox" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::USE_IFRAME_SANDBOX.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "react-dom/no-render-return-value" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_RENDER_RETURN_VALUE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-dom/no-script-url" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::NO_SCRIPT_URL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-dom/no-unsafe-iframe-sandbox" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_UNSAFE_IFRAME_SANDBOX.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-dom/no-unsafe-target-blank" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::NO_BLANK_TARGET.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "react-dom/no-void-elements-with-children" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_VOID_ELEMENTS_WITH_CHILDREN
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-hooks/component-hook-factories" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_COMPONENT_HOOK_FACTORIES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-hooks/exhaustive-deps" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_EXHAUSTIVE_DEPENDENCIES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-hooks/react-compiler" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_REACT_PROP_ASSIGNMENTS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-hooks/rules-of-hooks" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_HOOK_AT_TOP_LEVEL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-jsx/no-children-prop" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_CHILDREN_PROP.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-jsx/no-leaked-dollar" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_JSX_LEAKED_DOLLAR.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-jsx/no-namespace" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_JSX_NAMESPACE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-jsx/no-useless-fragment" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_FRAGMENTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-naming-convention/context-name" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_REACT_NAMING_CONVENTION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-naming-convention/id-name" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_REACT_NAMING_CONVENTION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-naming-convention/ref-name" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_REACT_NAMING_CONVENTION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-native/no-color-literals" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_REACT_NATIVE_LITERAL_COLORS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-native/no-inline-styles" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_INLINE_STYLES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-native/no-raw-text" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_REACT_NATIVE_RAW_TEXT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-native/split-platform-components" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Correctness (biome_configuration :: analyzer :: Correctness :: USE_REACT_NATIVE_PLATFORM_COMPONENTS . untyped () ,) , is_inspired : true , }]
            }
        }
        "react-prefer-function-component/prefer-function-component" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_REACT_FUNCTION_COMPONENTS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-prefer-function-component/react-prefer-function-component" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_REACT_FUNCTION_COMPONENTS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-refresh/only-export-components" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_COMPONENT_EXPORT_ONLY_MODULES
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "react-rsc/function-definition" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_REACT_ASYNC_SERVER_FUNCTION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-x/component-hook-factories" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_COMPONENT_HOOK_FACTORIES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-x/exhaustive-deps" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_EXHAUSTIVE_DEPENDENCIES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-x/no-array-index-key" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_ARRAY_INDEX_KEY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-x/no-direct-mutation-state" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_REACT_DIRECT_STATE_MUTATION
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "react-x/no-forward-ref" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_REACT_FORWARD_REF.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-x/no-missing-key" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_JSX_KEY_IN_ITERABLE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react-x/no-nested-component-definitions" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_NESTED_COMPONENT_DEFINITIONS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/async-server-action" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_REACT_ASYNC_SERVER_FUNCTION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/button-has-type" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_BUTTON_TYPE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/forbid-elements" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_RESTRICTED_ELEMENTS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/function-component-definition" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: USE_REACT_FUNCTION_COMPONENT_DEFINITION . untyped () ,) , is_inspired : true , }]
            }
        }
        "react/iframe-missing-sandbox" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::USE_IFRAME_SANDBOX.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/jsx-boolean-value" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_IMPLICIT_BOOLEAN.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "react/jsx-curly-brace-presence" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_CONSISTENT_CURLY_BRACES.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "react/jsx-fragments" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_FRAGMENT_SYNTAX.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/jsx-key" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_JSX_KEY_IN_ITERABLE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/jsx-no-bind" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Performance(
                        biome_configuration::analyzer::Performance::NO_JSX_PROPS_BIND.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "react/jsx-no-comment-textnodes" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_COMMENT_TEXT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/jsx-no-duplicate-props" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DUPLICATE_JSX_PROPS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/jsx-no-leaked-render" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_LEAKED_RENDER.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "react/jsx-no-literals" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_JSX_LITERALS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/jsx-no-script-url" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::NO_SCRIPT_URL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/jsx-no-target-blank" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::NO_BLANK_TARGET.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "react/jsx-no-useless-fragment" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_FRAGMENTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/jsx-props-no-spread-multi" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DUPLICATED_SPREAD_PROPS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/no-array-index-key" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_ARRAY_INDEX_KEY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/no-children-prop" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_CHILDREN_PROP.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/no-danger" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::NO_DANGEROUSLY_SET_INNER_HTML
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/no-danger-with-children" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Security (biome_configuration :: analyzer :: Security :: NO_DANGEROUSLY_SET_INNER_HTML_WITH_CHILDREN . untyped () ,) , is_inspired : false , }]
            }
        }
        "react/no-direct-mutation-state" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_REACT_DIRECT_STATE_MUTATION
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "react/no-object-type-as-default-prop" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: NO_REACT_OBJECT_TYPE_AS_DEFAULT_PROP . untyped () ,) , is_inspired : false , }]
            }
        }
        "react/no-render-return-value" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_RENDER_RETURN_VALUE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/no-string-refs" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_REACT_STRING_REFS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/no-unknown-property" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_UNKNOWN_ATTRIBUTE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/self-closing-comp" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_SELF_CLOSING_ELEMENTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "react/void-dom-elements-no-children" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_VOID_ELEMENTS_WITH_CHILDREN
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "regexp/no-useless-backreference" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_USELESS_REGEX_BACKREFS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "regexp/prefer-regexp-exec" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_REGEXP_EXEC.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "require-atomic-updates" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_ATOMIC_UPDATES.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "require-await" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_AWAIT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "require-unicode-regexp" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_UNICODE_REGEX.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "require-yield" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_YIELD.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "shadcn/no-arbitrary-values" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_TAILWIND_ARBITRARY_VALUE
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "shadcn/no-inline-styles" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_INLINE_STYLES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "shadcn/no-raw-colors" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_TAILWIND_RAW_COLORS.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "shadcn/no-restyle" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_TAILWIND_RESTYLED_COMPONENTS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "shadcn/require-static-classes" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_TAILWIND_STATIC_CLASS_STRINGS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "solid/jsx-no-script-url" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::NO_SCRIPT_URL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "solid/no-destructure" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_SOLID_DESTRUCTURED_PROPS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "solid/no-react-specific-props" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_REACT_SPECIFIC_PROPS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "solid/prefer-for" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Performance(
                        biome_configuration::analyzer::Performance::USE_SOLID_FOR_COMPONENT
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "sonarjs/cognitive-complexity" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Complexity (biome_configuration :: analyzer :: Complexity :: NO_EXCESSIVE_COGNITIVE_COMPLEXITY . untyped () ,) , is_inspired : false , }]
            }
        }
        "sonarjs/no-nested-switch" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_NESTED_SWITCH.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "sonarjs/no-nested-template-literals" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_NESTED_TEMPLATE_LITERALS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "sonarjs/prefer-while" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_WHILE.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "strict" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_STRICT_MODE.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "svelte/block-lang" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_CONSISTENT_BLOCK_LANG.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "svelte/button-has-type" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_BUTTON_TYPE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/no-add-event-listener" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_SVELTE_ADD_EVENT_LISTENER
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/no-at-const-tags" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_SVELTE_LEGACY_CONST.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/no-at-debug-tags" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_SVELTE_AT_DEBUG_TAGS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/no-at-html-tags" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_SVELTE_AT_HTML_TAGS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/no-bind-value-on-checkable-inputs" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: NO_SVELTE_BIND_VALUE_ON_CHECKABLE_INPUTS . untyped () ,) , is_inspired : false , }]
            }
        }
        "svelte/no-ignored-unsubscribe" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_SVELTE_IGNORED_UNSUBSCRIBE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/no-inline-styles" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_INLINE_STYLES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/no-inner-declarations" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_INNER_DECLARATIONS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/no-inspect" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_SVELTE_INSPECT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/no-navigation-without-resolve" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_SVELTE_KIT_RESOLVE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/no-object-in-text-mustaches" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_SVELTE_OBJECT_IN_TEXT_MUSTACHES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/no-restricted-html-elements" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_RESTRICTED_ELEMENTS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "svelte/no-store-async" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_SVELTE_ASYNC_STORE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/no-svelte-internal" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_SVELTE_INTERNAL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/no-target-blank" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::NO_BLANK_TARGET.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/no-unnecessary-state-wrap" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_SVELTE_UNNECESSARY_STATE_WRAP
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/prefer-const" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_CONST.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/prefer-derived-over-derived-by" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_SVELTE_USELESS_DERIVED_BY
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/prefer-svelte-reactivity" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_SVELTE_REACTIVE_CLASSES
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "svelte/require-each-key" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_SVELTE_REQUIRE_EACH_KEY
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/require-event-dispatcher-types" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_SVELTE_EVENT_DISPATCHER_TYPES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/require-stores-init" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_SVELTE_STORE_INITIAL_VALUE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "svelte/shorthand-directive" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_SVELTE_SHORTHAND_DIRECTIVE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "symbol-description" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_SYMBOL_DESCRIPTION.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "tailwindcss/classnames-order" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_TAILWIND_SORTED_CLASSES
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "tailwindcss/enforces-shorthand" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_TAILWIND_SHORTHAND_CLASSES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "tailwindcss/no-arbitrary-value" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_TAILWIND_ARBITRARY_VALUE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "turbo/no-undeclared-env-vars" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_UNDECLARED_ENV_VARS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicode-bom" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_CONSISTENT_UNICODE_BOM
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/better-dom-traversing" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_BETTER_DOM_TRAVERSING.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/catch-error-name" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_CATCH_ERROR_NAME.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/consistent-boolean-name" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_CONSISTENT_BOOLEAN_NAME
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/consistent-date-clone" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_USELESS_DATE_GET_TIME.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/consistent-json-file-read" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_CONSISTENT_JSON_FILE_READ
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/consistent-tuple-labels" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_CONSISTENT_TUPLE_LABELS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/error-message" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_ERROR_MESSAGE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/explicit-length-check" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_EXPLICIT_LENGTH_CHECK.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/filename-case" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_FILENAMING_CONVENTION.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/new-for-builtins" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Correctness (biome_configuration :: analyzer :: Correctness :: NO_INVALID_BUILTIN_INSTANTIATION . untyped () ,) , is_inspired : false , }]
            }
        }
        "unicorn/no-accidental-bitwise-operator" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_ACCIDENTAL_BITWISE_OPERATORS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-array-for-each" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_FOR_EACH.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-array-sort-for-min-max" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_ARRAY_SORT_FOR_MIN_MAX.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-async-promise-finally" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_ASYNC_PROMISE_FINALLY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-await-in-promise-methods" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_AWAIT_IN_PROMISE_METHODS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-boolean-sort-comparator" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_BOOLEAN_SORT_COMPARATOR
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-canvas-to-image" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_CANVAS_TO_IMAGE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-confusing-array-splice" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_CONFUSING_ARRAY_SPLICE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-console-spaces" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_CONSOLE_SPACES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-constant-zero-expression" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_ERASING_OPERATION.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/no-document-cookie" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DOCUMENT_COOKIE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-double-comparison" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_DOUBLE_COMPARISON.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-duplicate-loops" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_EAGER_ARRAY_METHODS_IN_LOOPS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-empty-file" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_EMPTY_SOURCE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-for-each" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_ARRAY_FOR_EACH.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/no-for-loop" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_FOR_OF.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-immediate-mutation" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_IMMEDIATE_MUTATION.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/no-instanceof-array" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_IS_ARRAY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-invalid-fetch-options" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_INVALID_FETCH_OPTIONS.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/no-invalid-file-input-accept" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_INVALID_FILE_INPUT_ACCEPT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-invalid-remove-event-listener" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_INVALID_REMOVE_EVENT_LISTENER
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-late-current-target-access" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_LATE_CURRENT_TARGET_ACCESS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/no-late-event-control" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_LATE_EVENT_CONTROL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-lonely-if" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_COLLAPSED_IF.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-magic-array-flat-depth" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_MAGIC_ARRAY_FLAT_DEPTH.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-mismatched-map-key" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_MISMATCHED_MAP_KEY.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/no-negated-array-predicate" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_NEGATED_ARRAY_PREDICATE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-negated-condition" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_NEGATION_ELSE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-negation-in-equality-check" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_NEGATION_IN_EQUALITY_CHECK
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-nested-ternary" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_NESTED_TERNARY.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/no-new-array" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_NEW_ARRAY.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/no-null" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_NULL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-process-exit" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_PROCESS_EXIT.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/no-return-array-push" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_RETURN_ARRAY_PUSH.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-single-promise-in-promise-methods" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: NO_SINGLE_PROMISE_IN_PROMISE_METHODS . untyped () ,) , is_inspired : false , }]
            }
        }
        "unicorn/no-static-only-class" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_STATIC_ONLY_CLASS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-subtraction-comparison" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_SUBTRACTION_COMPARISON.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-thenable" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_THEN_PROPERTY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-this-outside-of-class" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_THIS_OUTSIDE_OF_CLASS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-unnecessary-await" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_USELESS_AWAIT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-unnecessary-nested-ternary" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_USELESS_NESTED_TERNARY.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/no-unnecessary-splice" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_USELESS_SPLICE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-unnecessary-string-trim" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_UNNECESSARY_STRING_TRIM
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-unreadable-for-of-expression" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_UNREADABLE_FOR_OF_EXPRESSION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-unreadable-iife" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_UNREADABLE_IIFE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-unsafe-sqlite-interpolation" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_UNSAFE_SQLITE_INTERPOLATION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-useless-concat" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_STRING_CONCAT
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/no-useless-fallback-in-spread" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_USELESS_FALLBACK_IN_SPREAD
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-useless-length-check" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_USELESS_LENGTH_CHECK.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-useless-recursion" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_USELESS_RECURSION.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-useless-set-construction" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_USELESS_SET_CONSTRUCTION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-useless-spread" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_USELESS_SPREAD.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-useless-switch-case" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_SWITCH_CASE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-useless-undefined" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_UNDEFINED.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-xor-as-exponentiation" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_XOR_AS_EXPONENTIATION.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/no-zero-fractions" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_ZERO_FRACTIONS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/numeric-separators-style" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_NUMERIC_SEPARATORS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-array-flat-map" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_FLAT_MAP.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-array-index-of" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_INDEX_OF.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-array-some" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_ARRAY_SOME.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/prefer-at" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_AT_INDEX.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/prefer-bigint-literals" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_BIGINT_LITERALS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-block-statement-over-iife" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_BLOCK_STATEMENT_OVER_IIFE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-boolean-return" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_SIMPLIFIED_BOOLEAN_RETURN
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-continue" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_EARLY_CONTINUE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-date-now" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_DATE_NOW.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-dom-node-text-content" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Performance(
                        biome_configuration::analyzer::Performance::USE_DOM_NODE_TEXT_CONTENT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-flat-math-min-max" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_FLAT_MATH_MIN_MAX.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-global-this" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_GLOBAL_THIS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-includes" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_INCLUDES.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/prefer-iterator-zip" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_ITERATOR_ZIP.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-math-abs" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_MATH_ABS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-math-constants" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Suspicious (biome_configuration :: analyzer :: Suspicious :: NO_APPROXIMATIVE_NUMERIC_CONSTANT . untyped () ,) , is_inspired : false , }]
            }
        }
        "unicorn/prefer-math-min-max" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_MATH_MIN_MAX.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-math-trunc" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_MATH_TRUNC.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-modern-math-apis" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_MODERN_MATH_APIS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-module" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_GLOBAL_DIRNAME_FILENAME
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/prefer-node-protocol" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_NODEJS_IMPORT_PROTOCOL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-number-properties" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_NUMBER_NAMESPACE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-object-from-entries" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_OBJECT_FROM_ENTRIES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-query-selector" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_DOM_QUERY_SELECTOR.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "unicorn/prefer-regexp-test" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_REGEXP_TEST.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-set-size" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_SET_SIZE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-string-slice" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_SUBSTR.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-string-trim-start-end" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_TRIM_START_END.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-unary-minus" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_UNARY_MINUS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/prefer-while-loop-condition" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_WHILE_LOOP_CONDITION.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unicorn/require-number-to-fixed-digits-argument" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Suspicious (biome_configuration :: analyzer :: Suspicious :: USE_NUMBER_TO_FIXED_DIGITS_ARGUMENT . untyped () ,) , is_inspired : false , }]
            }
        }
        "unicorn/throw-new-error" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_THROW_NEW_ERROR.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unused-imports/no-unused-imports" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_UNUSED_IMPORTS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "unused-imports/no-unused-vars" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_UNUSED_VARIABLES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "use-isnan" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_IS_NAN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "valid-typeof" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_VALID_TYPEOF.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vars-on-top" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_VARS_ON_TOP.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/consistent-test-it" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_CONSISTENT_TEST_IT.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vitest/expect-expect" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_EXPECT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/max-nested-describe" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_EXCESSIVE_NESTED_TEST_SUITES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/no-conditional-expect" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_CONDITIONAL_EXPECT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/no-disabled-tests" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_SKIPPED_TESTS.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vitest/no-done-callback" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_DONE_CALLBACK.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/no-duplicate-hooks" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DUPLICATE_TEST_HOOKS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vitest/no-focused-tests" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_FOCUSED_TESTS.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vitest/no-hooks" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_TEST_HOOKS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/no-identical-title" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_IDENTICAL_TEST_TITLE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/no-standalone-expect" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_MISPLACED_ASSERTION.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vitest/no-test-return-statement" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_RETURN_IN_TEST.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/prefer-called-times" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_EXPECT_TO_HAVE_BEEN_CALLED_ONCE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/prefer-called-with" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_CALLED_WITH.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/prefer-describe-function-title" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_DESCRIBE_FUNCTION_TITLE
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vitest/prefer-equality-matcher" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_EXPECT_EQUALITY_MATCHER
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/prefer-hooks-in-order" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_TEST_HOOKS_IN_ORDER.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/prefer-hooks-on-top" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_TEST_HOOKS_ON_TOP.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/prefer-importing-vitest-globals" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_TEST_GLOBALS.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vitest/prefer-lowercase-title" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_LOWERCASE_TEST_TITLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/prefer-spy-on" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_SPY_ON.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/prefer-strict-equal" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_EXPECT_TO_STRICT_EQUAL
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/prefer-to-contain" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_EXPECT_TO_CONTAIN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/prefer-to-have-been-called-times" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: USE_EXPECT_TO_HAVE_BEEN_CALLED_TIMES . untyped () ,) , is_inspired : false , }]
            }
        }
        "vitest/prefer-to-have-length" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_EXPECT_TO_HAVE_LENGTH.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/require-top-level-describe" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_TOP_LEVEL_DESCRIBE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vitest/valid-title" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VALID_TEST_TITLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/attribute-hyphenation" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_VUE_HYPHENATED_ATTRIBUTES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/block-lang" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_CONSISTENT_BLOCK_LANG.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/block-order" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_BLOCK_ORDER.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/camelcase" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_NAMING_CONVENTION.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/component-api-style" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_VUE_OPTIONS_API.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/component-options-name-casing" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: USE_VUE_REGISTERED_COMPONENT_NAME_CASING . untyped () ,) , is_inspired : true , }]
            }
        }
        "vue/define-emits-declaration" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: USE_VUE_CONSISTENT_DEFINE_EMITS_DECLARATION . untyped () ,) , is_inspired : false , }]
            }
        }
        "vue/define-macros-order" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_VUE_DEFINE_MACROS_ORDER.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/define-props-declaration" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Style (biome_configuration :: analyzer :: Style :: USE_VUE_CONSISTENT_DEFINE_PROPS_DECLARATION . untyped () ,) , is_inspired : false , }]
            }
        }
        "vue/define-props-destructuring" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_DEFINE_PROPS_DESTRUCTURING
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/dot-notation" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::USE_LITERAL_KEYS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/enforce-style-attribute" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::USE_SCOPED_STYLES.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/eqeqeq" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_DOUBLE_EQUALS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/html-button-has-type" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::A11y(
                        biome_configuration::analyzer::A11y::USE_BUTTON_TYPE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/match-component-file-name" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: USE_VUE_MATCHING_COMPONENT_FILE_NAME . untyped () ,) , is_inspired : true , }]
            }
        }
        "vue/max-props" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_EXCESSIVE_PROPS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/multi-word-component-names" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_VUE_MULTI_WORD_COMPONENT_NAMES
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/next-tick-style" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_VUE_NEXT_TICK_PROMISE.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/no-arrow-functions-in-watch" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_VUE_ARROW_FUNC_IN_WATCH
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-async-in-computed-properties" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_ASYNC_COMPUTED.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/no-boolean-default" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_BOOLEAN_DEFAULT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-child-content" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_CHILD_CONTENT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-computed-properties-in-data" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_COMPUTED_PROPERTIES_IN_DATA
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-console" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_CONSOLE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-constant-condition" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_CONSTANT_CONDITION.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-define-model-default" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_DEFINE_MODEL_DEFAULT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-deprecated-data-object-declaration" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_VUE_DATA_OBJECT_DECLARATION
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/no-deprecated-delete-set" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_DELETE_SET
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-deprecated-destroyed-lifecycle" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: NO_VUE_DEPRECATED_DESTROYED_LIFECYCLE . untyped () ,) , is_inspired : false , }]
            }
        }
        "vue/no-deprecated-dollar-listeners-api" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_LISTENERS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-deprecated-dollar-scopedslots-api" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_SCOPED_SLOTS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-deprecated-events-api" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_EVENTS_API
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/no-deprecated-filter" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_FILTER.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-deprecated-functional-template" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: NO_VUE_DEPRECATED_FUNCTIONAL_TEMPLATE . untyped () ,) , is_inspired : false , }]
            }
        }
        "vue/no-deprecated-html-element-is" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_HTML_ELEMENT_IS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-deprecated-inline-template" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_INLINE_TEMPLATE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-deprecated-model-definition" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_MODEL_DEFINITION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-deprecated-props-default-this" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: NO_VUE_DEPRECATED_PROPS_DEFAULT_THIS . untyped () ,) , is_inspired : false , }]
            }
        }
        "vue/no-deprecated-router-link-tag-prop" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: NO_VUE_DEPRECATED_ROUTER_LINK_TAG_PROP . untyped () ,) , is_inspired : false , }]
            }
        }
        "vue/no-deprecated-scope-attribute" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_SCOPE_ATTRIBUTE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-deprecated-slot-attribute" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_SLOT_ATTRIBUTE
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/no-deprecated-slot-scope-attribute" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_SLOT_SCOPE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-deprecated-v-bind-sync" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_V_BIND_SYNC
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-deprecated-v-is" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_V_IS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-deprecated-v-on-native-modifier" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: NO_VUE_DEPRECATED_V_ON_NATIVE_MODIFIER . untyped () ,) , is_inspired : false , }]
            }
        }
        "vue/no-deprecated-v-on-number-modifiers" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_VUE_V_ON_NUMBER_VALUES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-deprecated-vue-config-keycodes" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_DEPRECATED_CONFIG_KEY_CODES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-dupe-keys" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_VUE_DUPLICATE_KEYS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-dupe-v-else-if" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_DUPLICATE_V_ELSE_IF
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-duplicate-attributes" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_DUPLICATE_ATTRIBUTES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-empty-pattern" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_EMPTY_PATTERN.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-export-in-script-setup" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_EXPORT_IN_SCRIPT_SETUP
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-expose-after-await" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_EXPOSE_AFTER_AWAIT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-implicit-coercion" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_IMPLICIT_COERCIONS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-import-compiler-macros" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_VUE_IMPORT_COMPILER_MACROS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-irregular-whitespace" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_IRREGULAR_WHITESPACE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-lifecycle-after-await" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_LIFECYCLE_AFTER_AWAIT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-lone-template" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_LONE_TEMPLATE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-loss-of-precision" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_PRECISION_LOSS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-multiple-objects-in-class" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_MULTIPLE_OBJECTS_IN_CLASS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-multiple-slot-args" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_MULTIPLE_SLOT_ARGS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-mutating-props" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_MUTATING_PROPS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-negated-condition" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_NEGATION_ELSE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-negated-v-if-condition" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_NEGATION_ELSE.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/no-ref-as-operand" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_VUE_REF_AS_OPERAND.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-ref-object-reactivity-loss" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_REF_OBJECT_REACTIVITY_LOSS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/no-required-prop-with-default" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_REQUIRED_PROP_WITH_DEFAULT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-reserved-keys" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_VUE_RESERVED_KEYS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-reserved-props" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_VUE_RESERVED_PROPS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-restricted-call-after-await" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_RESTRICTED_CALL_AFTER_AWAIT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-restricted-html-elements" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_RESTRICTED_ELEMENTS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/no-restricted-v-on" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_RESTRICTED_V_ON.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-root-v-if" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_ROOT_V_IF.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-setup-props-reactivity-loss" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Correctness (biome_configuration :: analyzer :: Correctness :: NO_VUE_SETUP_PROPS_REACTIVITY_LOSS . untyped () ,) , is_inspired : true , }]
            }
        }
        "vue/no-shadow-native-events" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_SHADOW_NATIVE_EVENTS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/no-shared-component-data" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_VUE_DATA_OBJECT_DECLARATION
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/no-side-effects-in-computed-properties" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_SIDE_EFFECTS_IN_COMPUTED
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/no-sparse-arrays" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Suspicious(
                        biome_configuration::analyzer::Suspicious::NO_SPARSE_ARRAY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-template-key" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_TEMPLATE_KEY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-template-shadow" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_TEMPLATE_SHADOW.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-template-target-blank" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Security(
                        biome_configuration::analyzer::Security::NO_BLANK_TARGET.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-textarea-mustache" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_TEXTAREA_MUSTACHE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-this-in-before-route-enter" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_THIS_IN_BEFORE_ROUTE_ENTER
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-undef-directives" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_UNDECLARED_DIRECTIVES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-undef-properties" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_UNDECLARED_PROPERTIES
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/no-unused-emit-declarations" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_UNUSED_EMIT_DECLARATIONS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-unused-refs" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_UNUSED_REFS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-unused-vars" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_UNUSED_VARIABLES.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-use-v-else-with-v-for" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_VUE_V_IF_WITH_V_FOR
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-use-v-if-with-v-for" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::NO_VUE_V_IF_WITH_V_FOR
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-useless-concat" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Complexity(
                        biome_configuration::analyzer::Complexity::NO_USELESS_STRING_CONCAT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-useless-template-attributes" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_USELESS_TEMPLATE_ATTRIBUTES
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-useless-v-bind" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_USELESS_V_BIND.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/no-v-for-template-key-on-child" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_V_FOR_TEMPLATE_KEY_ON_CHILD
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-v-html" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_V_HTML.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-v-text" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_V_TEXT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-v-text-v-html-on-component" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_V_TEXT_V_HTML_ON_COMPONENT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/no-watch-after-await" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_WATCH_AFTER_AWAIT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/object-shorthand" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_CONSISTENT_OBJECT_DEFINITIONS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/one-component-per-file" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_ONE_COMPONENT_PER_FILE
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/prefer-define-options" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_DEFINE_OPTIONS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/prefer-import-from-vue" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_BASE_IMPORT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/prefer-prop-type-boolean-first" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_BOOLEAN_PROP_TYPE_FIRST
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/prefer-separate-static-class" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_SEPARATE_STATIC_CLASS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/prefer-single-event-payload" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_SINGLE_EVENT_PAYLOAD
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/prefer-template" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_TEMPLATE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/prefer-true-attribute-shorthand" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: USE_VUE_CONSISTENT_BOOLEAN_PROP_STYLE . untyped () ,) , is_inspired : false , }]
            }
        }
        "vue/prefer-use-template-ref" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_TEMPLATE_REF.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/prefer-v-model" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_V_MODEL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/prop-name-casing" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_CONSISTENT_PROP_NAME_CASING
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/require-component-is" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_COMPONENT_IS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/require-default-export" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_DEFAULT_EXPORT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/require-default-prop" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_DEFAULT_PROP.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/require-emit-validator" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_EMIT_VALIDATOR.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/require-explicit-emits" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_UNDECLARED_EMITS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/require-explicit-slots" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_UNDECLARED_SLOTS.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/require-macro-variable-name" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_MACRO_VARIABLE_NAME
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/require-prop-type-constructor" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_PROP_TYPE_CONSTRUCTOR
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/require-prop-types" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_TYPED_PROPS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/require-render-return" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_RENDER_RETURN.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/require-slots-as-functions" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_SLOTS_AS_FUNCTIONS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/require-toggle-inside-transition" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_TOGGLE_INSIDE_TRANSITION
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/require-typed-object-prop" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_TYPED_OBJECT_PROPS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/require-typed-ref" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_TYPED_REF.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/require-v-for-key" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_VUE_V_FOR_KEY.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/require-valid-default-prop" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_VALID_DEFAULT_PROP
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/return-in-computed-property" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_COMPUTED_PROPERTY_RETURN
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/return-in-emits-validator" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_EMITS_VALIDATOR_RETURN
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/slot-name-casing" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_SLOT_NAME_CASING.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/this-in-template" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::NO_VUE_THIS_IN_TEMPLATE.untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/use-v-on-exact" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_V_ON_EXACT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/v-bind-style" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_VUE_CONSISTENT_V_BIND_STYLE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/v-for-delimiter-style" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_CONSISTENT_V_FOR_DELIMITER
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/v-on-event-hyphenation" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: USE_VUE_CONSISTENT_EVENT_HYPHENATION . untyped () ,) , is_inspired : false , }]
            }
        }
        "vue/v-on-handler-style" => {
            const {
                & [eslint_to_biome :: MappedRule { slot : eslint_to_biome :: AnyRuleSlot :: Nursery (biome_configuration :: analyzer :: Nursery :: USE_VUE_CONSISTENT_V_ON_HANDLER_STYLE . untyped () ,) , is_inspired : false , }]
            }
        }
        "vue/v-on-style" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::USE_VUE_CONSISTENT_V_ON_STYLE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/v-slot-style" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_CONSISTENT_V_SLOT_STYLE
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-attribute-name" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_VALID_ATTRIBUTE_NAME
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-define-emits" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_VALID_DEFINE_EMITS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-define-options" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_VALID_DEFINE_OPTIONS
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-define-props" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_VALID_DEFINE_PROPS
                            .untyped(),
                    ),
                    is_inspired: true,
                }]
            }
        }
        "vue/valid-next-tick" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_VALID_NEXT_TICK.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-template-root" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_VUE_VALID_TEMPLATE_ROOT
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-v-bind" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_BIND.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-v-cloak" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_CLOAK.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-v-else" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_ELSE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-v-else-if" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_ELSE_IF
                            .untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-v-for" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_FOR.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-v-html" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_HTML.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-v-if" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_IF.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-v-is" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_VALID_V_IS.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-v-memo" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_VALID_V_MEMO.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-v-model" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_VALID_V_MODEL.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-v-on" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_ON.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-v-once" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_ONCE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-v-pre" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_PRE.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-v-show" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_VALID_V_SHOW.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-v-slot" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Nursery(
                        biome_configuration::analyzer::Nursery::USE_VUE_VALID_V_SLOT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "vue/valid-v-text" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Correctness(
                        biome_configuration::analyzer::Correctness::USE_VUE_VALID_V_TEXT.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        "yoda" => {
            const {
                &[eslint_to_biome::MappedRule {
                    slot: eslint_to_biome::AnyRuleSlot::Style(
                        biome_configuration::analyzer::Style::NO_YODA_EXPRESSION.untyped(),
                    ),
                    is_inspired: false,
                }]
            }
        }
        _ => {
            results.add(
                eslint_name,
                eslint_to_biome::RuleMigrationResult::Unsupported,
            );
            return false;
        }
    };
    eslint_to_biome::migrate_mapped_rules(
        rules,
        eslint_name,
        rule_severity,
        options,
        results,
        mapped_rules,
    )
}
