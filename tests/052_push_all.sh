# Test push all

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

pk push -a

assert $(metadata | popped | count) == 0
assert $(metadata | pushed | count) == 3
assert $(HEAD | summary) == C

fail pk push -a | grep "error: nothing to push"

# End of file
