use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlBlockContent, YamlFoldedScalar, YamlLiteralScalar};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockContent;
impl FormatNodeRule<YamlBlockContent> for FormatYamlBlockContent {
    fn fmt_fields(&self, node: &YamlBlockContent, f: &mut YamlFormatter) -> FormatResult<()> {
        let value = node.value_token()?;
        let mut text = value.text().to_string();

        if has_strip_indicator(node) {
            text = strip_chomped_block_content(&text).to_string();
        } else if !has_keep_indicator(node) {
            text = clip_chomped_block_content(&text).to_string();
        }

        if let Some(target_indent) = block_content_indent(node) {
            if let Some(normalized) = normalize_block_content_indent(&text, target_indent) {
                return format_block_content_text(&value, &normalized).fmt(f);
            }
        }

        format_block_content_text(&value, &text).fmt(f)
    }
}

fn format_block_content_text<'a>(
    value: &'a biome_yaml_syntax::YamlSyntaxToken,
    text: &'a str,
) -> impl Format<YamlFormatContext> + 'a {
    format_once(move |f| {
        if let Some(rest) = text
            .strip_prefix("\r\n")
            .or_else(|| text.strip_prefix('\n'))
        {
            write!(f, [hard_line_break(), format_token_text(value, rest.to_string())])
        } else {
            format_token_text(value, text.to_string()).fmt(f)
        }
    })
}

fn has_strip_indicator(node: &YamlBlockContent) -> bool {
    let Some(parent) = node.syntax().parent() else {
        return false;
    };

    if let Some(folded) = YamlFoldedScalar::cast(parent.clone()) {
        return folded
            .headers()
            .iter()
            .any(|header| header.as_yaml_block_strip_indicator().is_some());
    }

    if let Some(literal) = YamlLiteralScalar::cast(parent) {
        return literal
            .headers()
            .iter()
            .any(|header| header.as_yaml_block_strip_indicator().is_some());
    }

    false
}

fn has_keep_indicator(node: &YamlBlockContent) -> bool {
    let Some(parent) = node.syntax().parent() else {
        return false;
    };

    if let Some(folded) = YamlFoldedScalar::cast(parent.clone()) {
        return folded
            .headers()
            .iter()
            .any(|header| header.as_yaml_block_keep_indicator().is_some());
    }

    if let Some(literal) = YamlLiteralScalar::cast(parent) {
        return literal
            .headers()
            .iter()
            .any(|header| header.as_yaml_block_keep_indicator().is_some());
    }

    false
}

fn block_content_indent(node: &YamlBlockContent) -> Option<usize> {
    let Some(parent) = node.syntax().parent() else {
        return Some(2);
    };

    let has_indentation = if let Some(folded) = YamlFoldedScalar::cast(parent.clone()) {
        folded.headers().iter().find_map(|header| {
            header
                .as_yaml_indentation_indicator()
                .and_then(|indicator| indicator.indentation_indicator_token().ok())
        })
    } else if let Some(literal) = YamlLiteralScalar::cast(parent) {
        literal.headers().iter().find_map(|header| {
            header
                .as_yaml_indentation_indicator()
                .and_then(|indicator| indicator.indentation_indicator_token().ok())
        })
    } else {
        None
    };

    if has_indentation.is_some() {
        None
    } else {
        Some(2)
    }
}

fn strip_chomped_block_content(text: &str) -> &str {
    let mut end = text.len();

    for line in text.split_inclusive('\n').rev() {
        let line_without_newline = line
            .strip_suffix("\r\n")
            .or_else(|| line.strip_suffix('\n'))
            .unwrap_or(line);

        if line_without_newline.chars().all(|c| c == ' ') {
            end -= line.len();
        } else {
            break;
        }
    }

    &text[..end]
}

fn clip_chomped_block_content(text: &str) -> &str {
    let mut end = text.len();

    for line in text.split_inclusive('\n').rev() {
        let line_without_newline = line
            .strip_suffix("\r\n")
            .or_else(|| line.strip_suffix('\n'))
            .unwrap_or(line);

        if line_without_newline.chars().all(|c| c == ' ') {
            end -= line.len();
        } else {
            break;
        }
    }

    &text[..end]
}

fn normalize_block_content_indent(text: &str, target_indent: usize) -> Option<String> {
    let mut min_indent = usize::MAX;

    for line in text.lines().skip(1) {
        if line.is_empty() {
            continue;
        }

        min_indent = min_indent.min(line.len() - line.trim_start_matches(' ').len());
    }

    if min_indent == usize::MAX || min_indent == target_indent {
        return None;
    }

    let mut normalized = String::with_capacity(text.len());

    for (index, line) in text.split_inclusive('\n').enumerate() {
        if index == 0 || line == "\n" || line == "\r\n" {
            normalized.push_str(line);
        } else if let Some(empty_line) = normalize_empty_block_content_line(line, min_indent) {
            normalized.push_str(empty_line);
        } else if min_indent < target_indent {
            normalized.push_str(&" ".repeat(target_indent - min_indent));
            normalized.push_str(line);
        } else {
            normalized.push_str(line.get(min_indent - target_indent..).unwrap_or(line));
        }
    }

    Some(normalized)
}

fn normalize_empty_block_content_line(line: &str, min_indent: usize) -> Option<&str> {
    let content = line.strip_suffix("\r\n").or_else(|| line.strip_suffix('\n'))?;

    if content.len() == min_indent && content.chars().all(|c| c == ' ') {
        return line.get(content.len()..);
    }

    None
}
