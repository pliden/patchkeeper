# Test bdelete named

pk init
pk new INITIAL
pk finalize

fail pk bdelete | grep "error: missing required free argument"

grep "ref: refs/heads/main" .git/HEAD
grep $(HEAD) .git/refs/heads/main

pk bnew branch0
pk new A
grep "ref: refs/heads/branch0" .git/HEAD
grep $(HEAD) .git/refs/heads/branch0

fail pk bdelete branch0 | grep "error: cannot delete current branch"

pk bset main
fail pk bdelete branch0 | grep "error: branch has patches and/or properties"

pk bset branch0
pk pop
pk del -n

pk bset main
pk bdelete branch0
fail test -f .git/refs/heads/branch0

# End of file
