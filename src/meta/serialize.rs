use anyhow::anyhow;
use anyhow::bail;
use anyhow::Error;
use anyhow::Result;
use git2::Oid;
use itertools::Itertools;
use std::fmt;
use std::str;

use super::Branch;
use super::Branches;
use super::Metadata;
use super::Properties;

const LINE_MODIFIER_PROPERTY: &str = "%";
const LINE_MODIFIER_BRANCH: &str = "@";
const LINE_MODIFIER_PATCH_HIDDEN: &str = "#";
const LINE_MODIFIER_PATCH_POPPED: &str = "-";
const LINE_MODIFIER_PATCH_PUSHED: &str = "+";

enum Line<'a> {
    PropertyFlag(&'a str),
    PropertyValue(&'a str, &'a str),
    Branch(&'a str),
    PatchHidden(&'a str),
    PatchPopped(&'a str),
    PatchPushed(&'a str),
    Error(&'a str),
    None,
}

impl<'a> Line<'a> {
    fn from_str(line: &'a str) -> Self {
        let line = line.trim();

        let (modifier, value) = match (line.get(..1), line.get(1..)) {
            (Some(modifier), Some(value)) => (modifier, value),
            (None, None) => return Line::None,
            (_, _) => return Line::Error(line),
        };

        match modifier {
            LINE_MODIFIER_PROPERTY => {
                match value
                    .split_once(' ')
                    .map(|(name, value)| (name.trim(), value.trim()))
                {
                    Some((name, value)) => Line::PropertyValue(name, value),
                    _ => Line::PropertyFlag(value),
                }
            }
            LINE_MODIFIER_BRANCH => Line::Branch(value),
            LINE_MODIFIER_PATCH_HIDDEN => Line::PatchHidden(value),
            LINE_MODIFIER_PATCH_POPPED => Line::PatchPopped(value),
            LINE_MODIFIER_PATCH_PUSHED => Line::PatchPushed(value),
            _ => Line::Error(line),
        }
    }
}

impl str::FromStr for Metadata {
    type Err = Error;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let mut meta = Metadata::default();
        let mut branch: Option<Branch> = None;

        for (nr, line) in data.lines().enumerate() {
            let properties = match branch.as_mut() {
                Some(branch) => &mut branch.properties,
                _ => &mut meta.properties,
            };

            match Line::from_str(line) {
                Line::PropertyFlag(name) => {
                    if properties.contains(name) {
                        bail!("duplicate property in metadata (line {nr}, '{name}')");
                    }

                    properties.set_flag(name);
                }
                Line::PropertyValue(name, value) => {
                    if properties.contains(name) {
                        bail!("duplicate property in metadata (line {nr}, '{name}')");
                    }

                    properties.set(name, value.to_string())
                }
                Line::Branch(name) => {
                    if let Some(branch) = branch {
                        meta.branches.release(branch);
                    }

                    if meta.branches.contains(name) {
                        bail!("duplicate branch in metadata (line {nr}, '{name}')");
                    }

                    branch = Some(Branch::new(name));
                }
                Line::PatchHidden(id) => {
                    let branch = branch
                        .as_mut()
                        .ok_or(anyhow!("orphaned patch in metadata (line {nr}, '{id}')"))?;

                    branch.hidden.add_bottom(Oid::from_str(id)?);
                }
                Line::PatchPopped(id) => {
                    let branch = branch
                        .as_mut()
                        .ok_or(anyhow!("orphaned patch in metadata (line {nr}, '{id}')"))?;

                    branch.popped.add_bottom(Oid::from_str(id)?);
                }
                Line::PatchPushed(id) => {
                    let branch = branch
                        .as_mut()
                        .ok_or(anyhow!("orphaned patch in metadata (line {nr}, '{id}')"))?;

                    branch.pushed.add_bottom(Oid::from_str(id)?);
                }
                Line::Error(line) => {
                    bail!("syntax error in metadata (line {nr}, '{line}')")
                }
                Line::None => {}
            };
        }

        if let Some(branch) = branch {
            meta.branches.release(branch);
        }

        Ok(meta)
    }
}

impl<'a> fmt::Display for Line<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Line::PropertyFlag(name) => {
                writeln!(f, "{LINE_MODIFIER_PROPERTY}{name}")?;
            }
            Line::PropertyValue(name, value) => {
                writeln!(f, "{LINE_MODIFIER_PROPERTY}{name} {value}")?;
            }
            Line::Branch(name) => {
                writeln!(f, "\n{LINE_MODIFIER_BRANCH}{name}")?;
            }
            Line::PatchHidden(id) => {
                writeln!(f, "{LINE_MODIFIER_PATCH_HIDDEN}{id}")?;
            }
            Line::PatchPopped(id) => {
                writeln!(f, "{LINE_MODIFIER_PATCH_POPPED}{id}")?;
            }
            Line::PatchPushed(id) => {
                writeln!(f, "{LINE_MODIFIER_PATCH_PUSHED}{id}")?;
            }
            _ => panic!("unexpected line variant"),
        }

        Ok(())
    }
}

impl fmt::Display for Properties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Properties(properties) = self;
        for (name, value) in properties.borrow().iter().sorted_by_key(|(name, _)| *name) {
            let property = match value {
                Some(value) => Line::PropertyValue(name, value),
                _ => Line::PropertyFlag(name),
            };

            write!(f, "{}", property)?;
        }

        Ok(())
    }
}

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.properties)?;
        for id in &self.hidden.all() {
            write!(f, "{}", Line::PatchHidden(&format!("{id}")))?;
        }
        for id in &self.popped.all() {
            write!(f, "{}", Line::PatchPopped(&format!("{id}")))?;
        }
        for id in &self.pushed.all() {
            write!(f, "{}", Line::PatchPushed(&format!("{id}")))?;
        }
        Ok(())
    }
}

impl fmt::Display for Branches {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Branches(branches) = self;
        for (name, branch) in branches
            .borrow()
            .iter()
            .filter(|(_, branch)| !branch.is_empty())
            .sorted_by_key(|(name, _)| *name)
        {
            write!(f, "{}", Line::Branch(name))?;
            write!(f, "{}", branch)?;
        }

        Ok(())
    }
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.properties)?;
        write!(f, "{}", self.branches)?;
        Ok(())
    }
}
