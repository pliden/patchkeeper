# Test bnew

pk bn -h | usage bn
pk bnew -h | usage bnew

pk init
pk new INITIAL
pk finalize

grep "ref: refs/heads/main" .git/HEAD
grep $(HEAD) .git/refs/heads/main

pk bnew branch0
grep "ref: refs/heads/branch0" .git/HEAD
pk new A0
grep $(HEAD) .git/refs/heads/branch0

pk bnew branch1
grep "ref: refs/heads/branch1" .git/HEAD
pk new A1
grep $(HEAD) .git/refs/heads/branch1

assert $(cat .git/refs/heads/main) != $(cat .git/refs/heads/branch0)
assert $(cat .git/refs/heads/main) != $(cat .git/refs/heads/branch1)
assert $(cat .git/refs/heads/branch0) != $(cat .git/refs/heads/branch1)

# End of file
