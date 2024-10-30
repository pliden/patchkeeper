use super::Branch;

impl Branch {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
            && self.hidden.is_empty()
            && self.popped.is_empty()
            && self.pushed.is_empty()
    }
}
