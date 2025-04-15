# Test hide revspecs

pk init
pk new INITIAL
pk finalize

fail pk hide | grep "error: missing option"
fail pk hide $(HEAD) | grep "error: cannot hide non-popped commit"

pk new A
pk new B
pk new C
pk new D
pk new E

fail pk hide $(HEAD) | grep "error: cannot hide non-popped commit"

pk pop -a

assert $(metadata | hidden | count) == 0
assert $(metadata | popped | count) == 5
assert $(metadata | pushed | count) == 0
assert $(metadata | popped | bottom | summary) == A

pk hide $(metadata | popped | bottom)

assert $(metadata | hidden | count) == 1
assert $(metadata | popped | count) == 4
assert $(metadata | pushed | count) == 0
assert $(metadata | popped | bottom | summary) == B

pk hide $(metadata | popped)

assert $(metadata | hidden | count) == 5
assert $(metadata | popped | count) == 0
assert $(metadata | pushed | count) == 0

# End of file
