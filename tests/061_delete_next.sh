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

pk delete -n

assert $(metadata | popped | count) == 1
assert $(metadata | pushed | count) == 1
assert $(HEAD | summary) == A

pk delete -n

assert $(metadata | popped | count) == 0
assert $(metadata | pushed | count) == 1
assert $(HEAD | summary) == A

fail pk delete -n | grep "error: nothing to delete"

# End of file
