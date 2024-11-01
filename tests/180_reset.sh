# Test reset

pk reset -h | usage reset

pk init
pk new INITIAL
pk finalize

fail pk reset | grep "error: missing required free argument"
fail pk reset INVALID | grep "error: revspec 'INVALID' not found"

pk bnew branch0
pk new A

pk bset main
assert $(HEAD | summary) == INITIAL

pk reset branch0
assert $(HEAD | summary) == A

# End of file
