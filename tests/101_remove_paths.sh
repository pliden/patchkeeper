# Test remove paths

pk init
pk new INITIAL
pk finalize

fail pk remove | grep "error: missing required free argument"
fail pk remove INVALID | grep "error: file not found: INVALID"

pk new A
echo A0 > file0
echo A1 > file1
echo A2 > file2

pk add -a
assert $(INDEX | count) == 3
INDEX | grep "^file0$"
INDEX | grep "^file1$"
INDEX | grep "^file2$"

pk remove file0
assert $(INDEX | count) == 2
INDEX | grep "^file1$"
INDEX | grep "^file2$"
fail test -f "^file0$"

pk remove file1 file2
assert $(INDEX | count) == 0
fail test -f file1
fail test -f file2

# End of file
