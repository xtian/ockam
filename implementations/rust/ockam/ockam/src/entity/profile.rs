mod identifier;
pub use identifier::*;

mod change;
pub use change::*;

#[derive(Clone, Debug)]
pub struct Profile {
    pub identifier: ProfileIdentifier,
    pub change_history: ProfileChangeHistory,
}

impl Profile {
    pub fn new() -> Self {
        Profile {
            identifier: ProfileIdentifier::new(),
            change_history: ProfileChangeHistory::new(),
        }
    }

    pub fn apply(&mut self, change_event: ProfileChangeEvent) {
        change_event.apply(self)
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let _profile = Profile::new();
    }
}
