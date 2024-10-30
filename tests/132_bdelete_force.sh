# Test bdelete force

pk init
pk new INITIAL
pk finalize

grep "ref: refs/heads/main" .git/HEAD
grep $(HEAD) .git/refs/heads/main

pk bnew branch0
pk new A
grep "ref: refs/heads/branch0" .git/HEAD
grep $(HEAD) .git/refs/heads/branch0

fail pk bdelete branch0 | grep "error: cannot delete current branch"

pk bset main
pk bdelete -f branch0

fail test -f .git/refs/heads/branch0

# End of file
