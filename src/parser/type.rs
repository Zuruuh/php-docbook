use std::fmt;

use libxml::tree::Node;

#[derive(Debug, Clone)]
pub enum TypeHint {
    Regular(String),
    Union(UnionTypeHint),
}

impl fmt::Display for TypeHint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
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

impl From<Node> for TypeHint {
    fn from(value: Node) -> Self {
        let children = value.get_child_elements();
        if children.is_empty() {
            return TypeHint::Regular(value.get_content());
        }

        // Assuming we are working with a union type
        let mut first_type = Option::<TypeHint>::None;
        let mut union_type = Option::<UnionTypeHint>::None;
        for child in children {
            let type_hint = Self::from(child);

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
        }

        TypeHint::Union(union_type.unwrap())
    }
}
