# Test blist

pk bls -h | usage bls
pk blist -h | usage blist

pk init
pk new INITIAL
pk finalize

assert $(pk blist | count) == 1
pk blist | grep "main"

pk bnew branch0
pk bnew branch1
assert $(pk blist | count) == 3
pk blist | grep "main"
pk blist | grep "branch0"
pk blist | grep "branch1"

# End of file
