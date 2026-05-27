use crate::prelude::*;
use biome_yaml_syntax::YamlSingleQuotedScalar;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlSingleQuotedScalar;
impl FormatNodeRule<YamlSingleQuotedScalar> for FormatYamlSingleQuotedScalar {
    fn fmt_fields(&self, node: &YamlSingleQuotedScalar, f: &mut YamlFormatter) -> FormatResult<()> {
        let token = node.value_token()?;
        let value = token.text_trimmed();

        if let Some(inner) = value.strip_prefix('\'').and_then(|text| text.strip_suffix('\'')) {
            let unescaped = inner.replace("''", "'");
            if inner.contains(['\n', '\r', '\\']) || unescaped.contains('"') {
                format_trimmed_token(&token).fmt(f)
            } else {
                format_token_text(&token, std::format!("\"{unescaped}\"")).fmt(f)
            }
        } else {
            format_trimmed_token(&token).fmt(f)
        }
    }
}
