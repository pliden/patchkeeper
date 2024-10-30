# Test unhide all

pk init
pk new INITIAL
pk finalize

pk new A
pk new B
pk new C
pk pop -a
pk hide -a

assert $(metadata | hidden | count) == 3
assert $(metadata | popped | count) == 0
assert $(metadata | pushed | count) == 0
assert $(metadata | hidden | top | summary) == C
assert $(metadata | hidden | bottom | summary) == A

pk unhide -a

assert $(metadata | hidden | count) == 0
assert $(metadata | popped | count) == 3
assert $(metadata | pushed | count) == 0
assert $(metadata | popped | top | summary) == C
assert $(metadata | popped | bottom | summary) == A

fail pk unhide -a | grep "error: nothing to unhide"

# End of file
