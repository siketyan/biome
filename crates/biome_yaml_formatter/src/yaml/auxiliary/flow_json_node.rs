use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::YamlFlowJsonNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowJsonNode;
impl FormatNodeRule<YamlFlowJsonNode> for FormatYamlFlowJsonNode {
    fn fmt_fields(&self, node: &YamlFlowJsonNode, f: &mut YamlFormatter) -> FormatResult<()> {
        let properties = node.properties();

        write!(f, [properties.format()])?;

        if !properties.is_empty() {
            write!(f, [space()])?;
        }

        write!(f, [node.content().format()])
    }
}
