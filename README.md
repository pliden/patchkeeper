__NOTE! PatchKeeper version 2.0 is still work in progress__

# PatchKeeper

[![Build & Test](https://github.com/pliden/patchkeeper/actions/workflows/build-and-test.yaml/badge.svg)](https://github.com/pliden/patchkeeper/actions/workflows/build-and-test.yaml)

PatchKeeper (`pk`) is a [Git](https://git-scm.com/) command-line tool, with
first-class support for stack based patch management. It helps you manage
patches (amend, push, pop, fold, split, reorder, graft, etc.) with little
effort, something that can be tedious and cumbersome to do using the standard
`git(1)` command.

PatchKeeper operates on normal Git repositories, and all managed patches are
normal Git commits. It can therefore be used either as a replacement of, or as
a compliment to, the standard `git(1)` command and other tools that operate on
Git repositories.

## Usage

```
Usage: pk [options] <command> [command options]

Options:
  -r, --repo <path>       Path to repository
  -h, --help              Print help message

Commands:
  init                    Initialize repository
  clone                   Clone repository
  fetch                   Fetch remote commit(s)
  pull                    Pull remote commit(s)
  bnew, bn                New branch
  bset, b                 Set branch
  brename, br             Rename branch
  bdelete, bd             Delete branch
  blist, bls, bl          List branches
  bhide                   Hide branch
  bunhide                 Unhide branch
  new, n                  New commit
  delete, del             Delete commit
  refresh, r              Refresh commit
  message, msg, m         Set commit message
  finalize, fin           Finalize commit(s)
  add, a                  Add file(s)
  remove, rm              Remove file(s)
  move, mv                Move file(s)
  include, i              Include file(s) in commit
  exclude, x              Exclude file(s) from commit
  push, pu                Push commit
  pop, po                 Pop commit
  fold                    Fold commit
  hide                    Hide commit
  unhide                  Unhide commit
  list, ls, l             List commits
  resolve, res            Resolve merge conflict
  reset                   Reset head
  show, s                 Show commit
  version                 Show version
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
