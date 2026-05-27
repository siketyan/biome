use crate::AsFormat;
use crate::prelude::YamlFormatContext;
use crate::{FormatYamlSyntaxToken, YamlFormatter};
use biome_formatter::comments::SourceComment;
use biome_formatter::formatter::Formatter;
use biome_formatter::prelude::{empty_line, format_with, hard_line_break, syntax_token_cow_slice};
use biome_formatter::trivia::FormatToken;
use biome_formatter::{Buffer, Format, FormatResult, write};
use biome_rowan::TextSize;
use biome_yaml_syntax::{YamlLanguage, YamlSyntaxToken};
use std::borrow::Cow;

pub(crate) struct FormatRemoved<'a> {
    token: &'a YamlSyntaxToken,
}

pub(crate) fn format_removed(token: &YamlSyntaxToken) -> FormatRemoved<'_> {
    FormatRemoved { token }
}

impl<'a> Format<YamlFormatContext> for FormatRemoved<'a> {
    fn fmt(&self, f: &mut Formatter<YamlFormatContext>) -> FormatResult<()> {
        FormatYamlSyntaxToken.format_removed(self.token, f)
    }
}

pub(crate) struct FormatTrimmedToken<'a> {
    token: &'a YamlSyntaxToken,
}

pub(crate) fn format_trimmed_token(token: &YamlSyntaxToken) -> FormatTrimmedToken<'_> {
    FormatTrimmedToken { token }
}

pub(crate) fn format_block_scalar_marker(token: &YamlSyntaxToken) -> FormatTokenText<'_> {
    let text = token.text_trimmed().trim();
    let text = if let Some(comment_start) = text.find('#') {
        format!(
            "{} {}",
            text[..comment_start].trim_end(),
            text[comment_start..].trim_start()
        )
    } else {
        text.to_string()
    };

    format_token_text(token, text)
}

pub(crate) struct FormatTokenText<'a> {
    token: &'a YamlSyntaxToken,
    text: String,
}

pub(crate) fn format_token_text(token: &YamlSyntaxToken, text: String) -> FormatTokenText<'_> {
    FormatTokenText { token, text }
}

impl Format<YamlFormatContext> for FormatTokenText<'_> {
    fn fmt(&self, f: &mut Formatter<YamlFormatContext>) -> FormatResult<()> {
        FormatYamlSyntaxToken.format_replaced(
            self.token,
            &syntax_token_cow_slice(
                Cow::Owned(self.text.clone()),
                self.token,
                self.token.text_trimmed_range().start(),
            ),
            f,
        )
    }
}

impl Format<YamlFormatContext> for FormatTrimmedToken<'_> {
    fn fmt(&self, f: &mut Formatter<YamlFormatContext>) -> FormatResult<()> {
        let token_text = self.token.text_trimmed();
        let trimmed_text = token_text.trim();

        if trimmed_text.contains(['\n', '\r']) {
            self.token.format().fmt(f)
        } else {
            let trimmed_start = token_text
                .len()
                .saturating_sub(token_text.trim_start().len());
            let start =
                self.token.text_trimmed_range().start() + TextSize::from(trimmed_start as u32);

            FormatYamlSyntaxToken.format_replaced(
                self.token,
                &syntax_token_cow_slice(Cow::Owned(trimmed_text.to_string()), self.token, start),
                f,
            )
        }
    }
}

pub(crate) fn on_skipped(token: &YamlSyntaxToken, f: &mut YamlFormatter) -> FormatResult<()> {
    FormatYamlSyntaxToken.format_skipped_token_trivia(token, f)
}

pub(crate) fn on_removed(token: &YamlSyntaxToken, f: &mut YamlFormatter) -> FormatResult<()> {
    FormatYamlSyntaxToken.format_removed(token, f)
}

pub(crate) fn format_yaml_dangling_comments(
    comments: &[SourceComment<YamlLanguage>],
) -> impl Format<YamlFormatContext> + '_ {
    format_with(move |f| {
        let mut previous_comment = false;

        for comment in comments {
            if previous_comment {
                if comment.lines_before() > 1 {
                    write!(f, [empty_line()])?;
                } else {
                    write!(f, [hard_line_break()])?;
                }
            }

            write!(f, [comment.piece().as_piece()])?;
            comment.mark_formatted();
            previous_comment = true;
        }

        Ok(())
    })
}
