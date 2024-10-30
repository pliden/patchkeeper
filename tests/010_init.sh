# Test init

pk init -h | usage init

pk init
assert $(metadata | top) == "PatchKeeper"
assert $(metadata | revision) == 1
assert $(metadata | hidden | count) == 0
assert $(metadata | popped | count) == 0
assert $(metadata | pushed | count) == 0

fail pk init | grep "error: attempt to reinitialize"

# End of file
