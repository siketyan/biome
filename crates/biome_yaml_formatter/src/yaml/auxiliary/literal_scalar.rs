use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::YamlLiteralScalar;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlLiteralScalar;
impl FormatNodeRule<YamlLiteralScalar> for FormatYamlLiteralScalar {
    fn fmt_fields(&self, node: &YamlLiteralScalar, f: &mut YamlFormatter) -> FormatResult<()> {
        let marker = node.bitwise_or_token()?;

        write!(
            f,
            [
                format_block_scalar_marker(&marker),
                maybe_space(marker.has_trailing_comments()),
                node.headers().format(),
                node.content().format()
            ]
        )
    }
}
