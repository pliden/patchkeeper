# Test unhide revspec

pk init
pk new INITIAL
pk finalize

fail pk unhide | grep "error: missing option"
fail pk unhide $(HEAD) | grep "error: cannot unhide non-hidden commit"
fail pk unhide INVALID | grep "error: revspec 'INVALID' not found"

pk new A
pk new B
pk new C
pk new D
pk new E
pk pop -a
pk hide -a

assert $(metadata | hidden | count) == 5
assert $(metadata | popped | count) == 0
assert $(metadata | pushed | count) == 0
assert $(metadata | hidden | top | summary) == E
assert $(metadata | hidden | bottom | summary) == A

pk unhide $(metadata | hidden | top 2 | bottom)

assert $(metadata | hidden | count) == 4
assert $(metadata | popped | count) == 1
assert $(metadata | pushed | count) == 0
assert $(metadata | popped | bottom | summary) == D

pk unhide $(metadata | hidden)

assert $(metadata | hidden | count) == 0
assert $(metadata | popped | count) == 5
assert $(metadata | pushed | count) == 0
assert $(metadata | popped | top | summary) == A
assert $(metadata | popped | bottom | summary) == D

# End of file
