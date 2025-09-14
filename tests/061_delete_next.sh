# Test delete next

pk init
pk new INITIAL
pk finalize

pk new A
pk new B
pk new C
pk pop
pk pop

assert $(metadata | popped | count) == 2
assert $(metadata | pushed | count) == 1
assert $(HEAD | summary) == A

pk delete

assert $(metadata | popped | count) == 1
assert $(metadata | pushed | count) == 1
assert $(HEAD | summary) == A

pk delete

assert $(metadata | popped | count) == 0
assert $(metadata | pushed | count) == 1
assert $(HEAD | summary) == A

fail pk delete | grep "error: nothing to delete"

# End of file
