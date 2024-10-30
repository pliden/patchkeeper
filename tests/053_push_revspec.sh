# Test push revspec

pk init
pk new INITIAL
pk finalize

fail pk push $(HEAD) | grep "error: cannot push non-popped commit"
fail pk push INVALID | grep "error: revspec 'INVALID' not found"

pk new A
pk new B
pk new C
pk new D
pk new E
pk pop -a

assert $(metadata | popped | count) == 5
assert $(metadata | pushed | count) == 0
assert $(HEAD | summary) == INITIAL

pk push $(metadata | popped | bottom)

assert $(metadata | popped | count) == 4
assert $(metadata | pushed | count) == 1
assert $(HEAD | summary) == A

pk push $(metadata | popped | top)

assert $(metadata | popped | count) == 0
assert $(metadata | pushed | count) == 5
assert $(HEAD | summary) == E

# End of file
