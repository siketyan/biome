use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::YamlFoldedScalar;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFoldedScalar;
impl FormatNodeRule<YamlFoldedScalar> for FormatYamlFoldedScalar {
    fn fmt_fields(&self, node: &YamlFoldedScalar, f: &mut YamlFormatter) -> FormatResult<()> {
        let marker = node.r_angle_token()?;

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
