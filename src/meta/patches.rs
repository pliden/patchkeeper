use std::ops::Bound;
use std::ops::RangeBounds;

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

    pub fn top_vec(&self) -> Vec<Oid> {
        let Patches(patches) = self;
        patches.front().copied().iter().copied().collect::<Vec<_>>()
    }

    pub fn bottom_vec(&self) -> Vec<Oid> {
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

    fn range_iter<R: RangeBounds<Oid>>(&self, range: R) -> impl DoubleEndedIterator<Item = &Oid> {
        let Patches(patches) = self;
        match (range.start_bound(), range.end_bound()) {
            (Bound::Unbounded, Bound::Excluded(oid)) => {
                let end = patches.iter().position(|id| id == oid).unwrap();
                patches.range(..end)
            }
            (Bound::Included(oid), Bound::Unbounded) => {
                let start = patches.iter().position(|id| id == oid).unwrap();
                patches.range(start..)
            }
            (_, _) => panic!("unsupported range"),
        }
    }

    pub fn range<R: RangeBounds<Oid>>(&self, range: R) -> Vec<Oid> {
        self.range_iter(range).copied().collect::<Vec<_>>()
    }

    pub fn range_reversed<R: RangeBounds<Oid>>(&self, range: R) -> Vec<Oid> {
        self.range_iter(range).copied().rev().collect::<Vec<_>>()
    }
}
