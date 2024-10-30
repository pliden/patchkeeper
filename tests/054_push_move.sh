# Test push move

pk init
pk new INITIAL
pk finalize

fail pk push -m | grep "error: missing argument to option"
fail pk push -m $(HEAD) | grep "error: cannot move non-popped commit"
fail pk push -m INVALID | grep "error: revspec 'INVALID' not found"

pk new A
pk new B
pk new C
pk pop -a

assert $(metadata | popped | count) == 3
assert $(metadata | pushed | count) == 0
assert $(HEAD | summary) == INITIAL

pk push -m $(metadata | popped | top 2 | bottom)

assert $(metadata | popped | count) == 2
assert $(metadata | pushed | count) == 1
assert $(HEAD | summary) == B

pk push -m $(metadata | popped | top)

assert $(metadata | popped | count) == 1
assert $(metadata | pushed | count) == 2
assert $(HEAD | summary) == C

pk push -m $(metadata | popped | top)

assert $(metadata | popped | count) == 0
assert $(metadata | pushed | count) == 3
assert $(HEAD | summary) == A

# End of file
