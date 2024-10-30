# Test new

pk n -h | usage n
pk new -h | usage new

pk init
assert $(metadata | revision) == 1
assert $(metadata | pushed | count) == 0

pk new 0
assert $(metadata | revision) == 2
assert $(metadata | pushed | count) == 1

pk new 1
assert $(metadata | revision) == 3
assert $(metadata | pushed | count) == 2

pk new 2
assert $(metadata | revision) == 4
assert $(metadata | pushed | count) == 3

fail pk new | grep "error: missing required free argument"

# End of file
