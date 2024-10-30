# <picture><img src="doc/git-icon.svg" width="70"></picture> PatchKeeper

[![Build & Test](https://github.com/pliden/patchkeeper/actions/workflows/build-and-test.yaml/badge.svg)](https://github.com/pliden/patchkeeper/actions/workflows/build-and-test.yaml)

PatchKeeper (`pk`) is an alternative [Git](https://git-scm.com/) command-line
interface, with first-class support for managing patches.

PatchKeeper operates on normal git repositories, and all managed patches are
normal git commits. It can therefore be used either as a replacement of, or as
a compliment to, the standard `git(1)` command.

## Commands

|Command|Description|
|-------|-----------|
|`pk init`|Initialize repository|
|`pk new`|Create new commit|
|`...`|...|

## Example

```
$ mkdir myproject
$ pk init                       # Initialize repository
$ pk new "Initial commit"       # Create a new commit
$ pk add file.txt               # Add file.txt
$ pk refresh                    # Update current commit
$ pk pop
...
```

## Build from source
Requires [Rust](https://www.rust-lang.org/) to be installed.
```
$ git clone https://github.com/pliden/patchkeeper
$ cd patchkeeper
$ ./build.sh    # Build only
<or>
$ ./test.sh     # Build and test
```
