# Test pop next

pk init
pk new INITIAL
fail pk pop | grep "error: cannot pop initial commit"
pk finalize

pk new A
pk new B
pk new C

assert $(metadata | popped | count) == 0
assert $(metadata | pushed | count) == 3
assert $(HEAD | summary) == C

pk pop

assert $(metadata | popped | count) == 1
assert $(metadata | pushed | count) == 2
assert $(HEAD | summary) == B

pk pop

assert $(metadata | popped | count) == 2
assert $(metadata | pushed | count) == 1
assert $(HEAD | summary) == A

pk pop

assert $(metadata | popped | count) == 3
assert $(metadata | pushed | count) == 0
assert $(HEAD | summary) == INITIAL

fail pk pop | grep "error: nothing to pop"

# End of file
