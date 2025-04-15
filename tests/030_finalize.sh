# Test finalize

pk fin -h | usage fin
pk finalize -h | usage finalize

pk init

fail pk finalize | grep "error: reference 'refs/heads/main' not found"

pk new 0
assert $(metadata | pushed | count) == 1

pk finalize
assert $(metadata | pushed | count) == 0

pk new 1
pk new 2
pk new 3
assert $(metadata | pushed | count) == 3

pk finalize
assert $(metadata | pushed | count) == 0

fail pk finalize | grep "error: nothing to finalize"

# End of file
