# Test pop finalized

pk init
pk new INITIAL
pk finalize

fail pk pop -f | grep "error: cannot pop initial commit"

pk new A
fail pk pop -f | grep "error: cannot have pushed commits"
pk finalize

assert $(metadata | pushed | count) == 0
assert $(HEAD | summary) == A

pk pop -f

assert $(metadata | popped | count) == 1
assert $(metadata | popped | bottom | summary) == A
assert $(HEAD | summary) == INITIAL

# End of file
