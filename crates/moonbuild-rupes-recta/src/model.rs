// moon: The build system and package manager for MoonBit.
// Copyright (C) 2024 International Digital Economy Academy
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// For inquiries, you can contact us via e-mail at jichuruanjian@idea.edu.cn.

slotmap::new_key_type! {
    /// An unique identifier pointing to a package currently discovered from imported modules.
    pub struct PackageId;
}

/// Represents the overall action of this build tool call
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum RunAction {
    Build,
    Bundle,
    Check,
    Test,
}

/// Represents the actions performed on a single build target.
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TargetAction {
    Check,
    Build,
    BuildCStubs,
    LinkCore,
    MakeExecutable,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum TargetKind {
    Source,
    WhiteboxTest,
    BlackboxTest,

    // TODO: do we really need to specify inline tests as a separate target kind,
    // or should it be just `Source` with tests enabled?
    InlineTest,
    /// This is the subpackage designed originally for breaking cycles in
    /// `moonbitlang/core`. It's expected to be used sparingly.
    SubPackage,
}

/// Represents a single compile target that may be separately checked, built,
/// linked, etc.
#[derive(Clone, PartialEq, Eq, Hash, Copy, PartialOrd, Ord)]
pub struct BuildTarget {
    pub package: PackageId,
    pub kind: TargetKind,
    // TODO: Target backend need to be added here!
}

impl std::fmt::Debug for BuildTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}@{:?}", self.package, self.kind)
    }
}

impl PackageId {
    pub fn build_target(self, kind: TargetKind) -> BuildTarget {
        BuildTarget {
            package: self,
            kind,
        }
    }
}
