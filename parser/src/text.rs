use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::r#type::TypeHint;

/// TODO: Intern some strings here (constants mostly)
#[derive(Display, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TextNode {
    Text(String),
    BoldText(String),
    ItalicText(String),
    Subtitle(String),
    Function(String),
    Constant(String),
    Parameter(String),
    Classname(String),
    InterfaceName(String),
    Literal(String),
    Filename(String),
    Type(TypeHint),
    Code(String),
    Link(String),
    Note(String),
    Inset(String),
    HtmlTag(String),
    InlineCode(String),
    InlinePhpCode(String),
    /// Countable::count
    MethodName(String),
    /// TODO: actually implement this
    Table(String),
    Xref(String),
    Warning(String),
    None,
}
