use function::Function;
use libxml::parser::XmlParseError;
use r#type::TypeHint;

mod function;
mod r#type;

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

            TypeHint::from(return_type_node)
        };

        let method_params = {
            let method_param_nodes = xpath
                .evaluate(
                    r#"/d:refentry/d:refsect1[@role="description"]/d:methodsynopsis/d:methodparam"#,
                )
                .unwrap()
                .get_nodes_as_vec();

            let mut parameters = Vec::<function::Parameter>::new();

            for method in method_param_nodes {
                let mut r#type = Option::<TypeHint>::None;
                let mut name = Option::<String>::None;
                let mut default_value = Option::<String>::None;

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
                        name => todo!("Unhandled case for <methodparam><{name}>..."),
                    };
                }

                match (r#type, name) {
                    (Some(r#type), Some(name)) => {
                        parameters.push(function::Parameter {
                            name,
                            r#type,
                            repeat,
                            default_value,
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
