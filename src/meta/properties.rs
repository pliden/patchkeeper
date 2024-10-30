use super::Properties;
use anyhow::bail;
use anyhow::Result;
use std::str;

impl Properties {
    pub fn is_empty(&self) -> bool {
        let Properties(properties) = self;
        properties.borrow().is_empty()
    }

    pub fn contains(&self, name: &str) -> bool {
        let Properties(properties) = self;
        properties.borrow().get(name).is_some()
    }

    pub fn get_flag(&self, name: &str) -> Result<bool> {
        let Properties(properties) = self;
        match properties.borrow().get(name) {
            Some(value) => match value {
                Some(_) => bail!("property '{name}' is a value, not a flag"),
                _ => Ok(true),
            },
            _ => Ok(false),
        }
    }

    pub fn set_flag(&self, name: &str) {
        let Properties(properties) = self;
        let name = name.to_string();
        let value = None;
        properties.borrow_mut().insert(name, value);
    }

    pub fn get<T: str::FromStr>(&self, name: &str) -> Result<Option<T>> {
        let Properties(properties) = self;
        match properties.borrow().get(name) {
            Some(value) => match value {
                Some(value) => match T::from_str(value) {
                    Ok(value) => Ok(Some(value)),
                    _ => bail!("property '{name}' is not parseable"),
                },
                _ => bail!("property '{name}' is a flag, not a value"),
            },
            _ => Ok(None),
        }
    }

    pub fn set<T: ToString>(&self, name: &str, value: T) {
        let Properties(properties) = self;
        let name = name.to_string();
        let value = Some(value.to_string());
        properties.borrow_mut().insert(name, value);
    }

    pub fn remove(&self, name: &str) {
        let Properties(properties) = self;
        properties.borrow_mut().remove(name);
    }
}
