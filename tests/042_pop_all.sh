# Test pop all

pk init
pk new INITIAL
fail pk pop -a | grep "error: cannot pop initial commit"
pk finalize

pk new A
pk new B
pk new C

assert $(metadata | popped | count) == 0
assert $(metadata | pushed | count) == 3

pk pop -a

assert $(metadata | popped | count) == 3
assert $(metadata | pushed | count) == 0
assert $(HEAD | summary) == INITIAL

fail pk pop -a | grep "error: nothing to pop"

# End of file
