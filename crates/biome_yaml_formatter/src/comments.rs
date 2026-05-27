use biome_diagnostics::category;
use biome_formatter::comments::{
    CommentKind, CommentPlacement, CommentStyle, CommentTextPosition, Comments, DecoratedComment,
    SourceComment,
};
use biome_formatter::formatter::Formatter;
use biome_formatter::{FormatResult, FormatRule, write};
use biome_rowan::{AstNode, SyntaxNode, SyntaxToken, SyntaxTriviaPieceComments, TextSize};
use biome_suppression::{SuppressionKind, parse_suppression_comment};
use biome_yaml_syntax::{
    AnyYamlBlockInBlockContent, AnyYamlBlockNode, YamlAnchorProperty,
    YamlBlockMapExplicitEntry, YamlBlockMapImplicitEntry, YamlBlockMapping, YamlBlockSequence,
    YamlBlockSequenceEntry, YamlDocument, YamlLanguage, YamlRoot, YamlTagProperty,
};

use crate::prelude::*;

pub type YamlComments = Comments<YamlLanguage>;

#[derive(Default)]
pub struct FormatYamlLeadingComment;

impl FormatRule<SourceComment<YamlLanguage>> for FormatYamlLeadingComment {
    type Context = YamlFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<YamlLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        write!(f, [comment.piece().as_piece()])
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct YamlCommentStyle;

impl CommentStyle for YamlCommentStyle {
    type Language = YamlLanguage;

    fn is_suppression(text: &str) -> bool {
        parse_suppression_comment(text)
            .filter_map(Result::ok)
            .filter(|suppression| suppression.kind == SuppressionKind::Classic)
            .flat_map(|suppression| suppression.categories)
            .any(|(key, ..)| key == category!("format"))
    }

    fn is_global_suppression(text: &str) -> bool {
        parse_suppression_comment(text)
            .filter_map(Result::ok)
            .filter(|suppression| suppression.kind == SuppressionKind::All)
            .flat_map(|suppression| suppression.categories)
            .any(|(key, ..)| key == category!("format"))
    }

    fn get_comment_kind(_comment: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind {
        CommentKind::Line
    }

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        handle_global_suppression(comment)
            .or_else(place_end_of_line_entry_value_comment)
            .or_else(place_block_collection_end_comment)
            .or_else(place_own_line_entry_value_comment)
    }
}

fn place_end_of_line_entry_value_comment(
    comment: DecoratedComment<YamlLanguage>,
) -> CommentPlacement<YamlLanguage> {
    if comment.text_position() != CommentTextPosition::EndOfLine {
        return CommentPlacement::Default(comment);
    }

    if is_property(comment.enclosing_node()) || comment.preceding_node().is_some_and(is_property) {
        return CommentPlacement::Default(comment);
    }

    let Some(following) = comment.following_node() else {
        return CommentPlacement::Default(comment);
    };

    let Some(entry) = nearest_block_entry(comment.enclosing_node()) else {
        return CommentPlacement::Default(comment);
    };

    if entry_value_contains_node(&entry, following) {
        return CommentPlacement::dangling(entry, comment);
    }

    CommentPlacement::Default(comment)
}

fn place_own_line_entry_value_comment(
    comment: DecoratedComment<YamlLanguage>,
) -> CommentPlacement<YamlLanguage> {
    if comment.text_position() != CommentTextPosition::OwnLine {
        return CommentPlacement::Default(comment);
    }

    if let Some(following) = comment.following_node() {
        let Some(preceding) = comment.preceding_node() else {
            return CommentPlacement::Default(comment);
        };
        let Some(comment_indent) = comment_indent(&comment) else {
            return CommentPlacement::Default(comment);
        };

        if YamlBlockMapping::can_cast(comment.enclosing_node().kind())
            && is_block_entry(preceding)
            && comment_indent < node_column(preceding)
        {
            return CommentPlacement::dangling(comment.enclosing_node().clone(), comment);
        }

        if comment_indent > node_column(following) && is_block_entry(preceding) {
            if let Some(last_entry) = last_sequence_entry(preceding) {
                return CommentPlacement::dangling(last_entry, comment);
            }

            return CommentPlacement::dangling(preceding.clone(), comment);
        }

        if let Some(last_entry) = last_sequence_entry(preceding) {
            if comment_indent > node_column(preceding) {
                return CommentPlacement::dangling(last_entry, comment);
            }
        }

        if let Some(enclosing) = nearest_block_entry(comment.enclosing_node()) {
            if YamlBlockMapExplicitEntry::can_cast(enclosing.kind())
                && comment_indent > node_column(&enclosing)
            {
                return CommentPlacement::dangling(enclosing, comment);
            }
        }

        return CommentPlacement::Default(comment);
    }

    if let (Some(preceding), Some(comment_indent)) =
        (comment.preceding_node(), comment_indent(&comment))
    {
        if let Some(collection) = block_collection_value(preceding)
            && YamlBlockSequence::can_cast(collection.kind())
            && comment_indent > node_column(&collection)
            && let Some(last_entry) = last_sequence_entry(preceding)
        {
            return CommentPlacement::dangling(last_entry, comment);
        }

        if YamlDocument::can_cast(preceding.kind())
            && comment_indent > node_column(preceding)
            && let Some(last_entry) = last_sequence_entry(preceding)
        {
            return CommentPlacement::dangling(last_entry, comment);
        }

        if comment_indent > node_column(preceding) && is_block_entry(preceding) {
            return CommentPlacement::dangling(preceding.clone(), comment);
        }
    }

    if let Some(enclosing) = nearest_block_entry(comment.enclosing_node()) {
        return CommentPlacement::dangling(enclosing, comment);
    }

    CommentPlacement::Default(comment)
}

fn last_sequence_entry(node: &SyntaxNode<YamlLanguage>) -> Option<SyntaxNode<YamlLanguage>> {
    let sequence = YamlBlockSequence::cast(node.clone())
        .or_else(|| {
            let collection = block_collection_value(node)?;
            YamlBlockSequence::cast(collection)
        })
        .or_else(|| {
            let document = YamlDocument::cast(node.clone())?;
            let collection = block_node_collection(document.node()?)?;
            YamlBlockSequence::cast(collection)
        })?;
    Some(sequence.entries().into_iter().last()?.into_syntax())
}

fn nearest_block_entry(node: &SyntaxNode<YamlLanguage>) -> Option<SyntaxNode<YamlLanguage>> {
    node.ancestors().find(is_block_entry)
}

fn is_block_entry(node: &SyntaxNode<YamlLanguage>) -> bool {
    YamlBlockMapImplicitEntry::can_cast(node.kind())
        || YamlBlockMapExplicitEntry::can_cast(node.kind())
        || YamlBlockSequenceEntry::can_cast(node.kind())
}

fn is_property(node: &SyntaxNode<YamlLanguage>) -> bool {
    YamlAnchorProperty::can_cast(node.kind()) || YamlTagProperty::can_cast(node.kind())
}

fn entry_value_contains_node(
    entry: &SyntaxNode<YamlLanguage>,
    node: &SyntaxNode<YamlLanguage>,
) -> bool {
    let Some(value) = entry_value(entry) else {
        return false;
    };

    value
        .into_syntax()
        .text_trimmed_range()
        .contains_range(node.text_trimmed_range())
}

fn place_block_collection_end_comment(
    comment: DecoratedComment<YamlLanguage>,
) -> CommentPlacement<YamlLanguage> {
    if comment.text_position() != CommentTextPosition::OwnLine {
        return CommentPlacement::Default(comment);
    }

    let Some(preceding) = comment.preceding_node() else {
        return CommentPlacement::Default(comment);
    };

    if comment.lines_before() != 1 {
        return CommentPlacement::Default(comment);
    }

    let Some(comment_indent) = comment_indent(&comment) else {
        return CommentPlacement::Default(comment);
    };

    let Some(following) = comment.following_node() else {
        return CommentPlacement::Default(comment);
    };

    let sibling_column = node_column(following);

    if comment_indent <= sibling_column {
        return CommentPlacement::Default(comment);
    }

    if let Some(collection) = block_collection_value(preceding) {
        if comment_indent > node_column(&collection) {
            return CommentPlacement::Default(comment);
        }

        return CommentPlacement::trailing(collection, comment);
    }

    CommentPlacement::Default(comment)
}

fn comment_indent(comment: &DecoratedComment<YamlLanguage>) -> Option<usize> {
    let token = comment.following_token()?;
    trivia_comment_indent(token, comment.piece().text_range())
}

fn trivia_comment_indent(token: &SyntaxToken<YamlLanguage>, comment_range: biome_rowan::TextRange) -> Option<usize> {
    let mut column = 0;

    for piece in token.leading_trivia().pieces() {
        if piece.text_range() == comment_range {
            return Some(column);
        }

        if piece.is_newline() {
            column = 0;
        } else {
            column += piece.text().len();
        }
    }

    None
}

fn node_column(node: &SyntaxNode<YamlLanguage>) -> usize {
    node.first_token().map_or(0, |token| {
        let mut column = 0;

        for piece in token.leading_trivia().pieces() {
            if piece.is_newline() {
                column = 0;
            } else {
                column += piece.text().len();
            }
        }

        column
    })
}

fn block_collection_value(node: &SyntaxNode<YamlLanguage>) -> Option<SyntaxNode<YamlLanguage>> {
    block_node_collection(entry_value(node)?)
}

fn entry_value(node: &SyntaxNode<YamlLanguage>) -> Option<AnyYamlBlockNode> {
    if let Some(entry) = YamlBlockMapImplicitEntry::cast(node.clone()) {
        return entry.value();
    }

    if let Some(entry) = YamlBlockMapExplicitEntry::cast(node.clone()) {
        return entry.value();
    }

    if let Some(entry) = YamlBlockSequenceEntry::cast(node.clone()) {
        return entry.value();
    }

    None
}

fn block_node_collection(value: AnyYamlBlockNode) -> Option<SyntaxNode<YamlLanguage>> {
    let block = value.as_yaml_block_in_block_node()?;
    match block.content().ok()? {
        AnyYamlBlockInBlockContent::YamlBlockMapping(mapping) => Some(mapping.into_syntax()),
        AnyYamlBlockInBlockContent::YamlBlockSequence(sequence) => Some(sequence.into_syntax()),
        _ => None,
    }
}

fn handle_global_suppression(
    comment: DecoratedComment<YamlLanguage>,
) -> CommentPlacement<YamlLanguage> {
    let node = comment.enclosing_node();

    if node.text_range_with_trivia().start() == TextSize::from(0) {
        let has_global_suppression = node.first_leading_trivia().is_some_and(|trivia| {
            trivia
                .pieces()
                .filter(|piece| piece.is_comments())
                .any(|piece| YamlCommentStyle::is_global_suppression(piece.text()))
        });
        let root = node.ancestors().find_map(YamlRoot::cast);
        if let Some(root) = root
            && has_global_suppression
        {
            return CommentPlacement::leading(root.syntax().clone(), comment);
        }
    }

    CommentPlacement::Default(comment)
}
