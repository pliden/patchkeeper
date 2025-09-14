# Test include

pk i -h | usage i
pk include -h | usage include

fail pk include | grep "error: missing argument '<path>...'"

pk init
pk new INITIAL
pk finalize

pk new A
echo A > file0
echo A > file1
echo A > file2
echo A > file3
echo A > file4
pk add -a
pk refresh

pk new B
echo B > file1
echo C > file2
echo C > file3
echo D > file4
pk include file1

pk new C
pk include file2 file3

pk new D
pk refresh

pk pop -a

pk push
cat file0 | grep "A"
cat file1 | grep "A"
cat file2 | grep "A"
cat file3 | grep "A"
cat file4 | grep "A"

pk push
cat file0 | grep "A"
cat file1 | grep "B"
cat file2 | grep "A"
cat file3 | grep "A"
cat file4 | grep "A"

pk push
cat file0 | grep "A"
cat file1 | grep "B"
cat file2 | grep "C"
cat file3 | grep "C"
cat file4 | grep "A"

pk push
cat file0 | grep "A"
cat file1 | grep "B"
cat file2 | grep "C"
cat file3 | grep "C"
cat file4 | grep "D"

# End of file
