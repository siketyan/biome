use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use biome_formatter::separated::TrailingSeparator;
use biome_yaml_syntax::YamlFlowSequenceEntryList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowSequenceEntryList;
impl FormatRule<YamlFlowSequenceEntryList> for FormatYamlFlowSequenceEntryList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlFlowSequenceEntryList, f: &mut YamlFormatter) -> FormatResult<()> {
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
