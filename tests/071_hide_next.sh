# Test hide next

pk init
pk new INITIAL
pk finalize

pk new A
pk new B
pk new C
pk pop
pk pop

assert $(metadata | hidden | count) == 0
assert $(metadata | popped | count) == 2
assert $(metadata | pushed | count) == 1
assert $(HEAD | summary) == A

pk hide -n

assert $(metadata | hidden | count) == 1
assert $(metadata | popped | count) == 1
assert $(metadata | pushed | count) == 1
assert $(HEAD | summary) == A

pk hide -n

assert $(metadata | hidden | count) == 2
assert $(metadata | popped | count) == 0
assert $(metadata | pushed | count) == 1
assert $(HEAD | summary) == A

fail pk hide -n | grep "error: nothing to hide"

# End of file
