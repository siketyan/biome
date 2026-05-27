use crate::prelude::*;
use biome_formatter::{FormatOptions, write};
use biome_yaml_syntax::YamlRoot;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlRoot;
impl FormatNodeRule<YamlRoot> for FormatYamlRoot {
    fn fmt_fields(&self, node: &YamlRoot, f: &mut YamlFormatter) -> FormatResult<()> {
        let documents = node.documents();
        let eof_token = node.eof_token()?;

        write!(f, [documents.format(), format_removed(&eof_token)])?;

        if f.options().trailing_newline().value() {
            write!(f, [hard_line_break()])
        } else {
            Ok(())
        }
    }
}
