# Test resolve list

pk init
pk new INITIAL
pk finalize

pk new A
echo "VERSION A0" > file0
echo "VERSION A1" > file1
echo "VERSION A2" > file2
git add file0 file1 file2
pk refresh

pk resolve -l | grep "no merge conflicts"

pk new B
echo "VERSION B0" > file0
echo "VERSION B1" > file1
echo "VERSION B2" > file2
pk refresh

pk resolve -l | grep "no merge conflicts"

pk pop
echo "VERSION A0 PRIME" > file0
echo "VERSION A1 PRIME" > file1
echo "VERSION A2 PRIME" > file2
pk refresh

pk push | grep "merge conflict(s)"
pk resolve -l > list
assert $(cat list | count) == 3
grep file0 list
grep file1 list
grep file2 list
pk resolve -a

pk resolve -l | grep "no merge conflicts"

# End of file
