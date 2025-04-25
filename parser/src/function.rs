use std::fmt;

use serde::{Deserialize, Serialize};

use super::{text::TextNode, r#type::TypeHint};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Function {
    Definition(FunctionDefinition),
    Alias(String),
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Function::Definition(function_definition) => function_definition.fmt(f),
            Function::Alias(string) => string.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub short_description: String,
    pub return_type: TypeHint,
    pub arguments: Vec<Parameter>,
    pub description: Vec<TextNode>,
}

impl fmt::Display for FunctionDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", self.name)?;
        for (i, arg) in self.arguments.iter().enumerate() {
            write!(
                f,
                "{}{}{} {}${}{}",
                (i > 0).then_some(", ").unwrap_or_default(),
                {
                    use std::fmt::Write as _;

                    arg.attributes.iter().fold(String::new(), |mut f, attr| {
                        let _ = write!(f, "{attr} ");
                        f
                    })
                },
                arg.r#type,
                arg.repeat.then_some("...").unwrap_or_default(),
                arg.name,
                arg.default_value
                    .as_ref()
                    .map(|value| format!(" = {value}"))
                    .unwrap_or_default(),
            )?;
        }

        write!(f, "): {};", self.return_type)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub r#type: TypeHint,
    pub repeat: bool,
    pub default_value: Option<String>,
    pub attributes: Vec<String>,
}
