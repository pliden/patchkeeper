# Test move paths

pk init
pk new INITIAL
pk finalize

fail pk move | grep "error: missing argument '<src>'"
fail pk move INVALID | grep "error: missing argument '<dest>'"
fail pk move INVALID INVALID | grep "error: file not found: INVALID"

pk new A
echo A0 > file0
echo A1 > file1
echo A2 > file2

pk add -a
assert $(INDEX | count) == 3
INDEX | grep "^file0$"
INDEX | grep "^file1$"
INDEX | grep "^file2$"

fail pk move file0 file1 file2 | grep "error: destination must be a directory"

pk move file0 file10
assert $(INDEX | count) == 3
INDEX | grep "^file10$"
INDEX | grep "^file1$"
INDEX | grep "^file2$"
fail test -f file0

mkdir -p dir0/dir1
pk move file1 file2 dir0/dir1
assert $(INDEX | count) == 3
INDEX | grep "^file10$"
INDEX | grep "^dir0/dir1/file1$"
INDEX | grep "^dir0/dir1/file2$"
fail test -f file1
fail test -f file2

pk move dir0/dir1/file1 dir0/dir1/file2 .
assert $(INDEX | count) == 3
INDEX | grep "^file10$"
INDEX | grep "^file1$"
INDEX | grep "^file2$"
fail test -d dir0

# End of file
