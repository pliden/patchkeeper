use super::Patches;
use git2::Oid;

impl Patches {
    pub fn is_empty(&self) -> bool {
        let Patches(patches) = self;
        patches.is_empty()
    }

    pub fn contains(&self, oid: Oid) -> bool {
        let Patches(patches) = self;
        patches.contains(&oid)
    }

    pub fn add_top(&mut self, oid: Oid) {
        let Patches(patches) = self;
        patches.push_front(oid);
    }

    pub fn add_bottom(&mut self, oid: Oid) {
        let Patches(patches) = self;
        patches.push_back(oid);
    }

    pub fn replace_top(&mut self, oid: Oid) {
        let Patches(patches) = self;
        let front = patches.front_mut().unwrap();
        *front = oid;
    }

    pub fn remove(&mut self, oid: Oid) -> bool {
        let Patches(patches) = self;
        patches
            .iter()
            .position(|id| id == &oid)
            .map(|i| patches.remove(i).is_some())
            .is_some()
    }

    pub fn remove_all(&mut self) {
        let Patches(patches) = self;
        patches.clear();
    }

    pub fn top(&self) -> Oid {
        let Patches(patches) = self;
        patches.front().copied().unwrap()
    }

    pub fn bottom(&self) -> Oid {
        let Patches(patches) = self;
        patches.back().copied().unwrap()
    }

    pub fn top_as_vec(&self) -> Vec<Oid> {
        let Patches(patches) = self;
        patches.front().copied().iter().copied().collect::<Vec<_>>()
    }

    pub fn bottom_as_vec(&self) -> Vec<Oid> {
        let Patches(patches) = self;
        patches.back().copied().iter().copied().collect::<Vec<_>>()
    }

    pub fn all(&self) -> Vec<Oid> {
        let Patches(patches) = self;
        patches.iter().copied().collect::<Vec<_>>()
    }

    pub fn all_reversed(&self) -> Vec<Oid> {
        let Patches(patches) = self;
        patches.iter().rev().copied().collect::<Vec<_>>()
    }

    pub fn range(&self, oid: Oid) -> Vec<Oid> {
        let Patches(patches) = self;
        patches
            .iter()
            .position(|id| id == &oid)
            .map(|i| patches.range(..i).copied().collect::<Vec<_>>())
            .unwrap_or_default()
    }

    pub fn range_reversed(&self, oid: Oid) -> Vec<Oid> {
        let Patches(patches) = self;
        patches
            .iter()
            .position(|id| id == &oid)
            .map(|i| patches.range(i..).rev().copied().collect::<Vec<_>>())
            .unwrap_or_default()
    }
}
