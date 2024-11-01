# Test add paths

pk init
pk new INITIAL
pk finalize

fail pk add | grep "error: missing option"
fail pk add INVALID | grep "error: file not found: INVALID"

pk new A
echo A0 > file0
echo A1 > file1
echo A2 > file2
assert $(INDEX | count) == 0

pk add file0
assert $(INDEX | count) == 1
INDEX | grep file0

pk add file1 file2
assert $(INDEX | count) == 3
INDEX | grep file0
INDEX | grep file1
INDEX | grep file2

pk refresh

pk pop
fail test -f file0
fail test -f file1
fail test -f file2

pk push
grep A0 file0
grep A1 file1
grep A2 file2

# End of file
