use super::Source;
use git2::Oid;

impl Source {
    pub fn id(&self) -> Option<Oid> {
        let Source(source) = self;
        source.borrow().as_ref().copied()
    }

    pub fn set_id(&self, id: Oid) {
        let Source(source) = self;
        source.borrow_mut().replace(id);
    }
}
