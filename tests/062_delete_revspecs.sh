# Test delete revspecs

pk init
pk new INITIAL
pk finalize

fail pk delete | grep "error: missing option"
fail pk delete $(HEAD) | grep "error: cannot delete non-popped commit"

pk new A
pk new B
pk new C
pk new D
pk new E

fail pk delete $(HEAD) | grep "error: cannot delete non-popped commit"

pk pop -a

assert $(metadata | popped | count) == 5
assert $(metadata | pushed | count) == 0
assert $(metadata | popped | bottom | summary) == A

pk delete $(metadata | popped | bottom)

assert $(metadata | popped | count) == 4
assert $(metadata | pushed | count) == 0
assert $(metadata | popped | bottom | summary) == B

pk delete $(metadata | popped)

assert $(metadata | popped | count) == 0
assert $(metadata | pushed | count) == 0

# End of file
