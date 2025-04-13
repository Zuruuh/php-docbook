use function::{Function, FunctionDefinition};
use libxml::parser::XmlParseError;
use r#type::TypeHint;

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
        let description =
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
                    node
                        // .map_err(|_| XmlError::MalformedXmlDefinition("return type"))?
                        .get_nodes_as_vec()
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
                return Ok(Function::Alias(description));
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

        Ok(Function::Definition(FunctionDefinition {
            name: title,
            description,
            return_type,
            arguments: function_params,
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
