# Test refresh resolve

pk init
pk new INITIAL
pk finalize

pk new A
echo "VERSION A" > file0
git add file0
pk refresh

pk new B
echo "VERSION B" > file0
pk refresh

pk pop
echo "VERSION A PRIME" > file0
pk refresh

pk push | grep "merge conflict(s)"
grep "^VERSION A PRIME$" file0
grep "^VERSION B$" file0
echo "VERSION B PRIME" > file0
fail pk refresh | grep "error: unresolved merge conflicts found"
pk refresh -r

pk pop
grep "^VERSION A PRIME$" file0

pk push
grep "^VERSION B PRIME$" file0

# End of file
