use function::{Function, FunctionDefinition};
use libxml::{parser::XmlParseError, tree::NodeType};
use r#type::{DescriptionNode, TypeHint};

pub mod function;
pub mod r#type;

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
    #[error("Could not find the xml representation of the {0}")]
    MalformedXmlDefinition(&'static str),
    #[error("Could not read the xml file")]
    IOError(std::io::Error),
}

#[derive(Default)]
pub struct XmlParser {
    parser: libxml::parser::Parser,
}

impl XmlParser {
    fn get_string_from_xpath(
        xpath: &libxml::xpath::Context,
        path: &str,
    ) -> Result<String, XmlError> {
        xpath
            .evaluate(path)
            .map_err(|_| XmlError::XPathEvaluationError)
            .map(|object| object.get_nodes_as_str().join(""))
    }

    pub fn parse_function<Bytes: AsRef<[u8]>>(&self, content: Bytes) -> Result<Function, XmlError> {
        let doc = self
            .parser
            .parse_string(content)
            .map_err(XmlError::ParseError)?;

        let xpath =
            libxml::xpath::Context::new(&doc).map_err(|_| XmlError::XPathInitializationError)?;
        xpath
            .register_namespace("d", "http://docbook.org/ns/docbook")
            .map_err(|_| XmlError::NamespaceRegistrationError)?;

        let title = Self::get_string_from_xpath(&xpath, "//d:refentry/d:refnamediv/d:refname")?;
        let short_description =
            Self::get_string_from_xpath(&xpath, "//d:refentry/d:refnamediv/d:refpurpose")?;

        let return_type = {
            xpath
                .evaluate(r#"//d:refentry/d:refsect1[@role="description"]/d:methodsynopsis/d:type"#)
                .ok()
                .and_then(|node| {
                    if node.get_number_of_nodes() == 0 {
                        None
                    } else {
                        Some(node)
                    }
                })
                .map(|node| {
                    node.get_nodes_as_vec()
                        .into_iter()
                        .next()
                        .ok_or(XmlError::MalformedXmlDefinition("return type"))
                })
                .transpose()?
                .map(TypeHint::from)
        };

        let return_type = match return_type {
            Some(return_type) => return_type,
            None => {
                return Ok(Function::Alias(short_description));
            }
        };

        let function_params = {
            let function_param_nodes = xpath
                .evaluate(
                    r#"/d:refentry/d:refsect1[@role="description"]/d:methodsynopsis/d:methodparam"#,
                )
                .map_err(|_| XmlError::MalformedXmlDefinition("function parameters"))?
                .get_nodes_as_vec();

            let mut parameters = Vec::<function::Parameter>::new();

            for method in function_param_nodes {
                let mut r#type = Option::<TypeHint>::None;
                let mut name = Option::<String>::None;
                let mut default_value = Option::<String>::None;
                let mut attributes = Vec::<String>::new();

                let repeat = method
                    .get_attribute("rep")
                    .map(|value| value.as_str() == "repeat")
                    .unwrap_or_default();

                for child in method.get_child_elements() {
                    match child.get_name().as_str() {
                        "type" => {
                            r#type = Some(TypeHint::from(child));
                        }
                        "parameter" => {
                            name = Some(child.get_content());
                        }
                        "initializer" => {
                            default_value = Some(child.get_content());
                        }
                        "modifier"
                            if child
                                .get_attribute("role")
                                .map(|role| role.as_str() == "attribute")
                                .unwrap_or_default() =>
                        {
                            attributes.push(child.get_content());
                        }
                        name => todo!("Unhandled case for <methodparam><{name}>..."),
                    };
                }

                match (r#type, name) {
                    (r#type, Some(name)) => {
                        parameters.push(function::Parameter {
                            name,
                            r#type: r#type.unwrap_or_default(),
                            repeat,
                            default_value,
                            attributes,
                        });
                    }
                    (r#type, name) => {
                        todo!("Unhandled case where either {type:?} or {name:?} is unset?");
                    }
                }
            }

            parameters
        };

        let description = {
            let description_node = xpath
                .evaluate(r#"/d:refentry/d:refsect1[@role="description"]/d:para"#)
                .map(|node| node.get_nodes_as_vec().into_iter().next())
                .unwrap_or_default()
                .or_else(|| {
                    xpath
                        .evaluate(r#"/d:refentry/d:refsect1[@role="description"]/d:simpara"#)
                        .map(|node| node.get_nodes_as_vec().into_iter().next())
                        .unwrap_or_default()
                });

            let mut description = Vec::<DescriptionNode>::new();

            for node in description_node
                .map(|node| node.get_child_nodes())
                .unwrap_or_default()
            {
                let content = node.get_content();
                let text_node = match node.get_name().as_str() {
                    "text" => DescriptionNode::Text(content),
                    "function" => DescriptionNode::Function(content),
                    "constant" => DescriptionNode::Constant(content),
                    "parameter" | "varname" => DescriptionNode::Parameter(content),
                    "classname" => DescriptionNode::Classname(content),
                    "interfacename" => DescriptionNode::InterfaceName(content),
                    "literal" => DescriptionNode::Literal(content),
                    "filename" => DescriptionNode::Filename(content),
                    "type" => DescriptionNode::Type(TypeHint::from(node)),
                    "programlisting" => DescriptionNode::Code(content),
                    "link" => DescriptionNode::Link(content),
                    "methodname" => DescriptionNode::MethodName(content),
                    "table" => DescriptionNode::Table(content),
                    "xref" => DescriptionNode::Xref(node.get_attribute("linkend").unwrap_or_default()),
                    "return.falseforfailure" => DescriptionNode::Text("false on failure".to_string()),
                    // wtf ?
                    "return.success" => {
                        DescriptionNode::Text("Returns true on success or false on failure".to_string())
                    }
                    "emphasis"
                        if node
                            .get_attribute("role")
                            .map(|role| &role == "bold")
                            .unwrap_or_default() =>
                    {
                        DescriptionNode::BoldText(content)
                    }
                    "command" => DescriptionNode::BoldText(content),
                    "emphasis" if node.get_attribute("role").is_none() => {
                        DescriptionNode::ItalicText(content)
                    }
                    // TODO: implement this (html equivalent of <ul>, with <li> being <listitem>)
                    "itemizedlist" | "simplelist" => DescriptionNode::Text(content),
                    // TODO: actually implement this (Like show full text on hover ?)
                    "acronym" | "abbrev" => DescriptionNode::Text(content),
                    "style.oop" | "style.procedural" => DescriptionNode::Subtitle(content),
                    "note" => DescriptionNode::Note(content),
                    "screen" => DescriptionNode::Inset(content),
                    "tag" => DescriptionNode::HtmlTag(content),
                    "php.ini" => DescriptionNode::InlineCode("php.ini".to_string()),
                    "code" | "userinput" => DescriptionNode::InlinePhpCode(content),
                    "quote" => DescriptionNode::ItalicText(format!(r#""{content}""#)),
                    "superscript" => DescriptionNode::ItalicText(format!("^{content}")),
                    // TODO: Find a solution one day maybe ? No clue if possible though
                    "subscript" => DescriptionNode::ItalicText(format!("⋁{content}")),
                    "warn.undocumented.func" => DescriptionNode::Warning(
                        "This function is currently not documented; only its argument list is available.".to_string()
                    ),
                    // TODO: Handle correctly :pray:
                    // Example at doc-en/reference/stream/functions/stream-context-set-option.xml
                    "methodsynopsis" => DescriptionNode::None,
                    _ if node.get_type() == Some(NodeType::EntityRefNode) => {DescriptionNode::None},

                    name => todo!("Unhandled text node {name}"),
                };

                description.push(text_node);
            }

            description
        };

        Ok(Function::Definition(FunctionDefinition {
            name: title,
            short_description,
            return_type,
            arguments: function_params,
            description,
        }))
    }
}

#[cfg(test)]
mod test {
    use crate::parser::XmlParser;

    #[tokio::test]
    pub async fn smoke_test_function_parsing() -> Result<(), Box<dyn std::error::Error>> {
        let parser = XmlParser::default();
        for file in glob::glob("./.data/**/functions/**/*.xml")? {
            let file = file?;
            println!("Parsing file {}", &file.to_string_lossy());
            let function = parser.parse_function(tokio::fs::read(file).await?)?;
            println!("{function}");
        }

        Ok(())
    }

    #[tokio::test]
    pub async fn test_alias_function_definition() -> Result<(), Box<dyn std::error::Error>> {
        let parser = XmlParser::default();
        let key_exists_definition =
            tokio::fs::read("./.data/doc-en/reference/array/functions/key-exists.xml").await?;
        let function = parser.parse_function(key_exists_definition)?;
        println!("{function}");

        Ok(())
    }
}
