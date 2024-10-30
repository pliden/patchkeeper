# Test push next

pk init
pk new INITIAL
pk finalize

pk new A
pk new B
pk new C
pk pop -a

assert $(metadata | popped | count) == 3
assert $(metadata | pushed | count) == 0
assert $(HEAD | summary) == INITIAL

pk push

assert $(metadata | popped | count) == 2
assert $(metadata | pushed | count) == 1
assert $(HEAD | summary) == A

pk push

assert $(metadata | popped | count) == 1
assert $(metadata | pushed | count) == 2
assert $(HEAD | summary) == B

pk push

assert $(metadata | popped | count) == 0
assert $(metadata | pushed | count) == 3
assert $(HEAD | summary) == C

fail pk push | grep "error: nothing to push"

# End of file
