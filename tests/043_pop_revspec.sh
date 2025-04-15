# Test pop revspec

pk init
pk new INITIAL
pk finalize

fail pk pop $(HEAD) | grep "error: cannot pop non-pushed commit"
fail pk pop INVALID | grep "error: revspec 'INVALID' not found"

pk new A
pk new B
pk new C
pk new D
pk new E

assert $(metadata | popped | count) == 0
assert $(metadata | pushed | count) == 5

pk pop $(metadata | pushed | top 2 | bottom)

assert $(metadata | popped | count) == 1
assert $(metadata | pushed | count) == 4
assert $(HEAD | summary) == D

pk pop $(metadata | pushed | bottom)

assert $(metadata | popped | count) == 4
assert $(metadata | pushed | count) == 1
assert $(HEAD | summary) == A

# End of file
