# Test push backout

pk init
pk new INITIAL
pk finalize

fail pk push -b | grep "error: missing argument to option"
fail pk push -b INVALID | grep "error: revspec 'INVALID' not found"

pk new A
pk new B
pk new C
pk pop -a

assert $(metadata | popped | count) == 3
assert $(metadata | pushed | count) == 0
assert $(HEAD | summary) == INITIAL

pk push -b $(metadata | popped | top 2 | bottom)

assert $(metadata | popped | count) == 3
assert $(metadata | pushed | count) == 1
assert "$(HEAD | summary)" == "Backout: B"

# End of file
