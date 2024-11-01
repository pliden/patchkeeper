# Test add all

pk init
pk new INITIAL
pk finalize

pk new A
echo A0 > file0
echo A1 > file1
echo A2 > file2
assert $(INDEX | count) == 0

pk add -a
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
