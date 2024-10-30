use super::Branch;
use super::Branches;

impl Branches {
    pub fn contains(&self, name: &str) -> bool {
        let Branches(branches) = self;
        branches.borrow().contains_key(name)
    }

    pub fn acquire(&self, name: &str) -> Branch {
        let Branches(branches) = self;
        branches
            .borrow_mut()
            .remove(name)
            .unwrap_or(Branch::new(name))
    }

    pub fn release(&self, branch: Branch) {
        let Branches(branches) = self;
        branches.borrow_mut().insert(branch.name.clone(), branch);
    }

    pub fn rename(&self, old_name: &str, new_name: &str) {
        let Branches(branches) = self;
        let mut branch = branches
            .borrow_mut()
            .remove(old_name)
            .unwrap_or(Branch::new(old_name));
        branch.name = new_name.to_string();
        branches.borrow_mut().insert(branch.name.clone(), branch);
    }
}
