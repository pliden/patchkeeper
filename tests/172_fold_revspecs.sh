# Test fold revspecs

pk init
pk new INITIAL
pk finalize

fail pk fold | grep "error: missing option"
fail pk fold $(HEAD) | grep "error: cannot fold non-popped commit"
fail pk fold INVALID | grep "error: revspec 'INVALID' not found"

pk new A
echo A > fileA
git add fileA
pk refresh

pk new B
echo B > fileB
git add fileB
pk refresh

pk new C
echo C > fileC
git add fileC
pk refresh

grep A fileA
grep B fileB
grep C fileC

pk pop
fail test -f fileC

pk pop
fail test -f fileB

assert $(metadata | popped | count) == 2
assert $(metadata | pushed | count) == 1
assert $(HEAD | summary) == A

pk fold $(metadata | popped)

assert $(metadata | popped | count) == 0
assert $(metadata | pushed | count) == 1
assert $(HEAD | summary) == A
grep A fileA
grep B fileB
grep C fileC

# End of file
