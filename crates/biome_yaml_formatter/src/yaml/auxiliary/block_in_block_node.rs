use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{AnyYamlBlockInBlockContent, YamlBlockInBlockNode};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockInBlockNode;
impl FormatNodeRule<YamlBlockInBlockNode> for FormatYamlBlockInBlockNode {
    fn fmt_fields(&self, node: &YamlBlockInBlockNode, f: &mut YamlFormatter) -> FormatResult<()> {
        let properties = node.properties();

        write!(f, [properties.format()])?;

        let has_property_trailing_comments = properties
            .iter()
            .any(|property| f.comments().has_trailing_comments(property.syntax()));

        if !properties.is_empty()
            && matches!(
                node.content(),
                Ok(AnyYamlBlockInBlockContent::YamlBlockMapping(_)
                    | AnyYamlBlockInBlockContent::YamlBlockSequence(_))
            )
            || has_property_trailing_comments
        {
            write!(f, [hard_line_break()])?;
        } else if !properties.is_empty() {
            write!(f, [space()])?;
        }

        write!(f, [node.content().format()])
    }
}
