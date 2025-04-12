use std::fmt::Display;

use libxml::{parser::XmlParseError, tree::Node};

#[derive(Debug, thiserror::Error)]
pub enum XmlError {
    #[error(transparent)]
    ParseError(XmlParseError),
    #[error("Could not register XML namespace ?")]
    NamespaceRegistrationError,
    #[error("Could not initialize the XML XPath Context")]
    XPathInitializationError,
    #[error("Could not evaluate an XPath expression")]
    XPathEvaluationError,
}

#[derive(Default)]
pub struct XmlParser {
    parser: libxml::parser::Parser,
}

impl XmlParser {
    pub fn parse_function(&self, content: Vec<u8>) -> Result<Function, XmlError> {
        let doc = self
            .parser
            .parse_string(content)
            .map_err(XmlError::ParseError)?;

        let xpath =
            libxml::xpath::Context::new(&doc).map_err(|_| XmlError::XPathInitializationError)?;
        xpath
            .register_namespace("d", "http://docbook.org/ns/docbook")
            .map_err(|_| XmlError::NamespaceRegistrationError)?;
        let title = get_string_from_xpath(&xpath, "//d:refentry/d:refnamediv/d:refname")?;
        let description = get_string_from_xpath(&xpath, "//d:refentry/d:refnamediv/d:refpurpose")?;

        let return_type = {
            let return_type_node = xpath
                .evaluate(r#"//d:refentry/d:refsect1[@role="description"]/d:methodsynopsis/d:type"#)
                .unwrap()
                .get_nodes_as_vec()
                .into_iter()
                .next()
                .unwrap();

            parse_type_recursive(return_type_node)
        };

        let method_params = {
            let method_param_nodes = xpath
            .evaluate(
                r#"//d:refentry/d:refsect1[@role="description"]/d:methodsynopsis/d:methodparam"#,
            )
            .unwrap()
            .get_nodes_as_vec();

            let mut parameters = Vec::<Parameter>::new();

            for method in method_param_nodes {
                let mut r#type = Option::<TypeHint>::None;
                let mut name = Option::<String>::None;
                let repeat = method
                    .get_attribute("rep")
                    .map(|value| value.as_str() == "repeat")
                    .unwrap_or_default();
                for child in method.get_child_elements() {
                    match child.get_name().as_str() {
                        "type" => {
                            r#type = Some(parse_type_recursive(child));
                        }
                        "parameter" => {
                            name = Some(child.get_content());
                        }
                        name => todo!("Unhandled case for <methodparam><{name}>..."),
                    };
                }

                match (r#type, name) {
                    (Some(r#type), Some(name)) => {
                        parameters.push(Parameter {
                            name,
                            r#type,
                            repeat,
                        });
                    }
                    (r#type, name) => {
                        todo!("Unhandled case where either {type:?} or {name:?} is unset?");
                    }
                }
            }

            parameters
        };

        Ok(Function {
            name: title,
            description,
            return_type,
            arguments: method_params,
        })
    }
}

fn get_string_from_xpath(xpath: &libxml::xpath::Context, path: &str) -> Result<String, XmlError> {
    xpath
        .evaluate(path)
        .map_err(|_| XmlError::XPathEvaluationError)
        .map(|object| object.get_nodes_as_str().join(""))
}

fn parse_type_recursive(node: Node) -> TypeHint {
    let children = node.get_child_elements();
    if children.is_empty() {
        return TypeHint::Regular(node.get_content());
    }

    // Assuming we are working with a union type
    let mut first_type = Option::<TypeHint>::None;
    let mut union_type = Option::<UnionTypeHint>::None;
    for child in children {
        let type_hint = parse_type_recursive(child);

        if let Some(first_type) = &first_type {
            if let Some(union_type) = &mut union_type {
                let right_type = union_type.right.clone();
                union_type.right = Box::new(TypeHint::Union(UnionTypeHint {
                    left: right_type,
                    right: Box::new(type_hint),
                }));
            } else {
                union_type = Some(UnionTypeHint {
                    left: Box::new(first_type.clone()),
                    right: Box::new(type_hint),
                });
            }
        } else {
            first_type = Some(type_hint);
        }

        //         }));
        //     }
        // };
    }

    TypeHint::Union(union_type.unwrap())
}

// pub struct Page {
//     content
// }

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub return_type: TypeHint,
    pub arguments: Vec<Parameter>,
}

#[derive(Debug, Clone)]
pub enum TypeHint {
    Regular(String),
    Union(UnionTypeHint),
}

impl Display for TypeHint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeHint::Regular(regular) => write!(f, "{regular}"),
            TypeHint::Union(union_type_hint) => {
                write!(f, "{}|{}", union_type_hint.left, union_type_hint.right)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnionTypeHint {
    left: Box<TypeHint>,
    right: Box<TypeHint>,
}

impl Default for TypeHint {
    fn default() -> Self {
        Self::Regular("mixed".to_string())
    }
}

#[derive(Debug, Default)]
pub struct Parameter {
    pub name: String,
    pub r#type: TypeHint,
    pub repeat: bool,
}
