use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use biome_formatter::separated::TrailingSeparator;
use biome_yaml_syntax::YamlFlowMapEntryList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowMapEntryList;
impl FormatRule<YamlFlowMapEntryList> for FormatYamlFlowMapEntryList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlFlowMapEntryList, f: &mut YamlFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_soft_line();

        for (element, formatted) in node
            .elements()
            .zip(node.format_separated(",", TrailingSeparator::Allowed))
        {
            if let Ok(element) = element.node() {
                join.entry(element.syntax(), &formatted);
            }
        }

        join.finish()
    }
}
