use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_fs::normalize_path;
use biome_js_syntax::{AnyJsImportLike, JsSyntaxKind, JsSyntaxToken, inner_string_text};
use biome_package::{PackageJson, TsConfigJson};
use biome_rowan::BatchMutationExt;
use biome_rule_options::use_consistent_import_paths::UseConsistentImportPathsOptions;
use camino::{Utf8Path, Utf8PathBuf};

use crate::{JsRuleAction, services::module_graph::ResolvedImports};

declare_lint_rule! {
    /// Enforce consistent import paths by preferring aliases for distant modules and relative paths for nearby modules.
    ///
    /// When a project defines aliases in `tsconfig.json` or `package.json`, parent-relative imports can become noisy and harder to scan.
    /// This rule prefers configured aliases for imports that traverse parent directories, while keeping nearby imports relative when they stay within the current directory tree.
    ///
    /// Biome gives precedence to `tsconfig.json` path aliases. If no TypeScript alias matches, Biome falls back to `package.json#imports`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,file=tsconfig.json
    /// {
    ///   "compilerOptions": {
    ///     "paths": {
    ///       "@/*": ["./src/*"]
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=src/features/main.ts
    /// import { Button } from "../ui/button.ts";
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=src/features/main.ts
    /// export { Button } from "@/features/button.ts";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=src/features/main.ts
    /// import { Button } from "@/ui/button.ts";
    /// ```
    ///
    /// ```ts,file=src/features/main.ts
    /// import { buttonStyles } from "./button/styles.ts";
    /// ```
    pub UseConsistentImportPaths {
        version: "next",
        name: "useConsistentImportPaths",
        language: "js",
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
        domains: &[RuleDomain::Project],
    }
}


#[derive(Debug)]
pub struct RuleState {
    module_name: JsSyntaxToken,
    replacement: String,
    kind: ReplacementKind,
}

#[derive(Clone, Copy, Debug)]
pub enum ReplacementKind {
    Alias,
    Relative,
}

impl Rule for UseConsistentImportPaths {
    type Query = ResolvedImports<AnyJsImportLike>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = UseConsistentImportPathsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if !node.is_static_import() || node.is_in_ts_module_declaration() {
            return None;
        }

        let module_info = ctx.module_info_for_path(ctx.file_path())?;
        let import_path = module_info.get_import_path_by_js_node(node)?;
        let resolved_path = import_path.as_path()?;

        let module_name = node.module_name_token()?;
        let module_text = inner_string_text(&module_name);
        let (specifier, suffix) = split_specifier_suffix(module_text.text());

        if is_relative_parent_specifier(specifier) {
            let replacement = preferred_alias_for_path(ctx, resolved_path, suffix)?;
            return Some(RuleState {
                module_name,
                replacement,
                kind: ReplacementKind::Alias,
            });
        }

        if is_relative_current_specifier(specifier) {
            return None;
        }

        if !is_alias_specifier(ctx, specifier) {
            return None;
        }

        let replacement = relative_specifier_for_path(ctx.file_path(), resolved_path, suffix)?;
        if !is_relative_current_specifier(&replacement) {
            return None;
        }

        Some(RuleState {
            module_name,
            replacement,
            kind: ReplacementKind::Relative,
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state.kind {
            ReplacementKind::Alias => RuleDiagnostic::new(
                rule_category!(),
                state.module_name.text_trimmed_range(),
                markup! {
                    "Use the configured import alias instead of a parent-relative import path."
                },
            )
            .note(markup! {
                "Aliases keep distant imports shorter and make module boundaries easier to scan."
            }),
            ReplacementKind::Relative => RuleDiagnostic::new(
                rule_category!(),
                state.module_name.text_trimmed_range(),
                markup! {
                    "Use a relative import path for nearby modules."
                },
            )
            .note(markup! {
                "Nearby imports are easier to read when they stay relative to the current file."
            }),
        };

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let quote = (*state.module_name.text_trimmed().as_bytes().first()?) as char;
        let new_module_name = JsSyntaxToken::new_detached(
            JsSyntaxKind::JS_STRING_LITERAL,
            &format!("{quote}{}{quote}", state.replacement),
            [],
            [],
        );

        let mut mutation = ctx.root().begin();
        mutation.replace_token(state.module_name.clone(), new_module_name);

        let message = match state.kind {
            ReplacementKind::Alias => markup! { "Use the configured import alias." },
            ReplacementKind::Relative => markup! { "Use a nearby relative import path." },
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message.to_owned(),
            mutation,
        ))
    }
}

fn preferred_alias_for_path(
    ctx: &RuleContext<UseConsistentImportPaths>,
    resolved_path: &Utf8Path,
    suffix: &str,
) -> Option<String> {
    tsconfig_alias_for_path(ctx.file_path(), resolved_path, ctx).or_else(|| {
        package_import_alias_for_path(ctx.file_path(), resolved_path, ctx)
    })
    .map(|specifier| format!("{specifier}{suffix}"))
}

fn is_alias_specifier(ctx: &RuleContext<UseConsistentImportPaths>, specifier: &str) -> bool {
    specifier.starts_with('#')
        || ctx
            .project_layout()
            .query_tsconfig_for_path(ctx.file_path(), |tsconfig| tsconfig.matches_path_alias(specifier))
            .unwrap_or(false)
}

fn tsconfig_alias_for_path(
    file_path: &Utf8Path,
    resolved_path: &Utf8Path,
    ctx: &RuleContext<UseConsistentImportPaths>,
) -> Option<String> {
    ctx.project_layout()
        .query_tsconfig_for_path(file_path, |tsconfig| alias_from_tsconfig(tsconfig, resolved_path))
        .flatten()
}

fn alias_from_tsconfig(tsconfig: &TsConfigJson, resolved_path: &Utf8Path) -> Option<String> {
    let paths = tsconfig.compiler_options.paths.as_ref()?;
    let base = &tsconfig.compiler_options.paths_base;

    for (alias, targets) in paths {
        for target in targets {
            if let Some(specifier) = build_alias_from_mapping(alias, target, base, resolved_path) {
                return Some(specifier);
            }
        }
    }

    None
}

fn package_import_alias_for_path(
    file_path: &Utf8Path,
    resolved_path: &Utf8Path,
    ctx: &RuleContext<UseConsistentImportPaths>,
) -> Option<String> {
    let (package_path, manifest) = ctx.project_layout().find_node_manifest_for_path(file_path)?;
    alias_from_package_imports(&manifest, &package_path, resolved_path)
}

fn alias_from_package_imports(
    manifest: &PackageJson,
    package_path: &Utf8Path,
    resolved_path: &Utf8Path,
) -> Option<String> {
    let imports = manifest.imports.as_ref()?.as_object()?;

    for (key, value) in imports.iter() {
        let target = value.as_string()?;
        let target = target.as_ref();
        if !target.starts_with("./") {
            continue;
        }

        if let Some(specifier) = build_alias_from_mapping(key.as_str(), target, package_path, resolved_path)
        {
            return Some(specifier);
        }
    }

    None
}

fn build_alias_from_mapping(
    alias: &str,
    target: &str,
    base: &Utf8Path,
    resolved_path: &Utf8Path,
) -> Option<String> {
    let normalized_target = strip_query_and_fragment(target);

    match (alias.split_once('*'), normalized_target.split_once('*')) {
        (Some((alias_prefix, alias_suffix)), Some((target_prefix, target_suffix))) => {
            let target_prefix_path = normalize_mapping_target(base, target_prefix);
            let resolved = resolved_path.as_str();
            let prefix = target_prefix_path.as_str();
            let suffix = target_suffix;

            if !resolved.starts_with(prefix) || !resolved.ends_with(suffix) {
                return None;
            }
            if resolved.len() < prefix.len() + suffix.len() {
                return None;
            }

            let middle = resolved[prefix.len()..resolved.len() - suffix.len()]
                .strip_prefix('/')
                .unwrap_or(&resolved[prefix.len()..resolved.len() - suffix.len()]);
            Some(format!("{alias_prefix}{middle}{alias_suffix}"))
        }
        (None, None) => {
            let target_path = normalize_mapping_target(base, normalized_target);
            (target_path == resolved_path).then(|| alias.to_string())
        }
        _ => None,
    }
}

fn normalize_mapping_target(base: &Utf8Path, target: &str) -> Utf8PathBuf {
    let target = target.strip_prefix("./").unwrap_or(target);
    normalize_path(&base.join(target))
}

fn relative_specifier_for_path(
    from_file: &Utf8Path,
    to_file: &Utf8Path,
    suffix: &str,
) -> Option<String> {
    let from_dir = from_file.parent()?;
    let mut from_components = from_dir.components();
    let mut to_components = to_file.components();

    loop {
        match (from_components.clone().next(), to_components.clone().next()) {
            (Some(from), Some(to)) if from == to => {
                from_components.next();
                to_components.next();
            }
            _ => break,
        }
    }

    let mut relative = String::new();
    let mut has_parent = false;
    for _ in from_components {
        relative.push_str("../");
        has_parent = true;
    }

    let remainder: Vec<_> = to_components.map(|component| component.as_str()).collect();
    if remainder.is_empty() {
        if !has_parent {
            relative.push_str("./");
        }
    } else if !has_parent {
        relative.push_str("./");
        relative.push_str(&remainder.join("/"));
    } else {
        relative.push_str(&remainder.join("/"));
    }

    Some(format!("{relative}{suffix}"))
}

fn split_specifier_suffix(specifier: &str) -> (&str, &str) {
    specifier
        .find(['?', '#'])
        .map(|index| specifier.split_at(index))
        .unwrap_or((specifier, ""))
}

fn strip_query_and_fragment(specifier: &str) -> &str {
    split_specifier_suffix(specifier).0
}

fn is_relative_parent_specifier(specifier: &str) -> bool {
    specifier == ".." || specifier.starts_with("../")
}

fn is_relative_current_specifier(specifier: &str) -> bool {
    specifier == "." || specifier.starts_with("./")
}
