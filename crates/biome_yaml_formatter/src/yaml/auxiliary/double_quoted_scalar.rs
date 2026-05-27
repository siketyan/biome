use crate::prelude::*;
use biome_yaml_syntax::YamlDoubleQuotedScalar;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlDoubleQuotedScalar;
impl FormatNodeRule<YamlDoubleQuotedScalar> for FormatYamlDoubleQuotedScalar {
    fn fmt_fields(&self, node: &YamlDoubleQuotedScalar, f: &mut YamlFormatter) -> FormatResult<()> {
        let token = node.value_token()?;
        let value = token.text_trimmed();

        if let Some(inner) = value.strip_prefix('"').and_then(|text| text.strip_suffix('"')) {
            let only_escaped_double_quotes = inner.contains("\\\"")
                && !inner
                    .replace("\\\"", "")
                    .contains(['\n', '\r', '\\']);

            if only_escaped_double_quotes {
                let inner = inner.replace("\\\"", "\"").replace('\'', "''");
                return format_token_text(&token, std::format!("'{inner}'")).fmt(f);
            }
        }

        format_trimmed_token(&token).fmt(f)
    }
}
