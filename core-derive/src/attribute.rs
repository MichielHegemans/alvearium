use virtue::prelude::*;
use virtue::utils::{parse_tagged_attribute, ParsedAttribute};

pub struct ContainerAttributes {
    pub crate_name: String,
    pub bounds: Option<(String, Literal)>,
    pub encode_bounds: Option<(String, Literal)>,
}

impl Default for ContainerAttributes {
    fn default() -> Self {
        Self {
            crate_name: "::alvearium_core".to_string(),
            bounds: None,
            encode_bounds: None,
        }
    }
}

impl FromAttribute for ContainerAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let attributes = match parse_tagged_attribute(group, "hive_encode")? {
            Some(body) => body,
            None => return Ok(None),
        };

        let mut result = Self::default();

        for attribute in attributes {
            match attribute {
                ParsedAttribute::Property(key, val) if key.to_string() == "crate" => {
                    let val_string = val.to_string();
                    if val_string.starts_with('"') && val_string.ends_with('"') {
                        result.crate_name = val_string[1..val_string.len() - 1].to_string();
                    } else {
                        return Err(Error::custom_at("Should be a literal str", val.span()));
                    }
                }
                _ => {}
            }
        }

        Ok(Some(result))
    }
}
