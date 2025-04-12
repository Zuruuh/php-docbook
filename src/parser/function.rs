use super::r#type::TypeHint;
use std::fmt;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: String,
    pub return_type: TypeHint,
    pub arguments: Vec<Parameter>,
}

impl fmt::Display for FunctionDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", self.name)?;
        for (i, arg) in self.arguments.iter().enumerate() {
            write!(
                f,
                "{}{}{} {}${}{}",
                (i > 0).then(|| ", ").unwrap_or_default(),
                arg.attributes
                    .iter()
                    .map(|attr| format!("{attr} "))
                    .collect::<String>(),
                arg.r#type,
                arg.repeat.then(|| "...").unwrap_or_default(),
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

#[derive(Debug, Default)]
pub struct Parameter {
    pub name: String,
    pub r#type: TypeHint,
    pub repeat: bool,
    pub default_value: Option<String>,
    pub attributes: Vec<String>,
}
