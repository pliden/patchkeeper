# Test fold next

pk init
pk new INITIAL
pk finalize

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

pk fold -n

assert $(metadata | popped | count) == 1
assert $(metadata | pushed | count) == 1
assert $(HEAD | summary) == A
grep A fileA
grep B fileB
fail test -f fileC

pk fold -n

assert $(metadata | popped | count) == 0
assert $(metadata | pushed | count) == 1
assert $(HEAD | summary) == A
grep A fileA
grep B fileB
grep C fileC

fail pk fold -n | grep "error: nothing to fold"

# End of file
