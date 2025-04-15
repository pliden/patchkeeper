# Test resolve all

pk init
pk new INITIAL
pk finalize

pk new A
echo "VERSION A0" > file0
echo "VERSION A1" > file1
echo "VERSION A2" > file2
pk add file0 file1 file2
pk refresh

pk new B
echo "VERSION B0" > file0
echo "VERSION B1" > file1
echo "VERSION B2" > file2
pk refresh

pk pop
echo "VERSION A0 PRIME" > file0
echo "VERSION A1 PRIME" > file1
echo "VERSION A2 PRIME" > file2
pk refresh

pk push | grep "merge conflict(s)"
grep "^VERSION A0 PRIME$" file0
grep "^VERSION B0$" file0
echo "VERSION B0 PRIME" > file0

grep "^VERSION A1 PRIME$" file1
grep "^VERSION B1$" file1
echo "VERSION B1 PRIME" > file1

grep "^VERSION A2 PRIME$" file2
grep "^VERSION B2$" file2
echo "VERSION B2 PRIME" > file2

fail pk refresh | grep "error: unresolved merge conflicts found"
pk resolve -a

pk refresh
pk pop

# End of file
