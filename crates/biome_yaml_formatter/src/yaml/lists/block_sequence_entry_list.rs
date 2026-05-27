use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_yaml_syntax::YamlBlockSequenceEntryList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockSequenceEntryList;
impl FormatRule<YamlBlockSequenceEntryList> for FormatYamlBlockSequenceEntryList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlBlockSequenceEntryList, f: &mut YamlFormatter) -> FormatResult<()> {
        let mut join = f.join_with(hard_line_break());

        for entry in node.iter() {
            join.entry(&format_args![
                format_once(|f| {
                    let has_raw_leading_comments = entry.syntax().has_leading_comments();
                    let has_placed_leading_comments =
                        f.comments().has_leading_comments(entry.syntax());

                    if get_lines_before(entry.syntax()) > 1
                        && (!has_raw_leading_comments || has_placed_leading_comments)
                    {
                        write!(f, [empty_line()])
                    } else {
                        Ok(())
                    }
                }),
                entry.format()
            ]);
        }

        join.finish()
    }
}
