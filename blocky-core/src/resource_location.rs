use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResourceLocationError {
    #[error("non [a-z0-9_.-] character in namespace of location")]
    Parse,
    #[error("non [a-z0-9_.-] character in namespace of location")]
    InvalidNamespace,
    #[error("non [a-z0-9/._-] character in path of location")]
    InvalidPath,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResourceLocation {
    namespace: String,
    path: String,
}

impl ResourceLocation {
    pub fn new<S: Into<String>>(namespace: S, path: S) -> Result<Self, ResourceLocationError> {
        let mut namespace = namespace.into();
        let path = path.into();

        if namespace.is_empty() {
            namespace.push_str("minecraft");
        }

        if !Self::is_valid_namespace(&namespace) {
            Err(ResourceLocationError::InvalidNamespace)
        } else if !Self::is_valid_path(&path) {
            Err(ResourceLocationError::InvalidPath)
        } else {
            Ok(Self { namespace, path })
        }
    }

    pub fn namespace(&self) -> &str {
        self.namespace.as_ref()
    }

    pub fn path(&self) -> &str {
        self.path.as_ref()
    }

    fn is_valid_namespace(namespace: &str) -> bool {
        if namespace.is_empty() {
            false
        } else {
            for chr in namespace.chars() {
                if match chr {
                    '0'..='9' | 'a'..='z' | '_' | '-' | '.' => false,
                    _ => true,
                } {
                    return false;
                }
            }

            true
        }
    }

    fn is_valid_path(path: &str) -> bool {
        if path.is_empty() {
            false
        } else {
            for chr in path.chars() {
                if match chr {
                    '0'..='9' | 'a'..='z' | '_' | '-' | '/' | '.' => false,
                    _ => true,
                } {
                    return false;
                }
            }

            true
        }
    }
}

impl fmt::Display for ResourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

impl FromStr for ResourceLocation {
    type Err = ResourceLocationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (namespace, path) = match s.find('c') {
            Some(i) => s.split_at(i),
            None => ("minecraft", s),
        };

        Self::new(namespace, path)
    }
}
