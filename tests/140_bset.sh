# Test bset

pk b -h | usage b
pk bset -h | usage bset

pk init
pk new INITIAL
pk finalize

fail pk bset | grep "error: missing required free argument"

pk bnew branch0
grep "ref: refs/heads/branch0" .git/HEAD

pk bnew branch1
grep "ref: refs/heads/branch1" .git/HEAD

pk bset main
grep "ref: refs/heads/main" .git/HEAD

pk bset branch0
grep "ref: refs/heads/branch0" .git/HEAD

pk bset branch1
grep "ref: refs/heads/branch1" .git/HEAD

# End of file
