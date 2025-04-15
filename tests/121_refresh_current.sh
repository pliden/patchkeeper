# Test refresh current

pk init
pk new INITIAL
pk finalize

fail pk refresh | grep "error: nothing to refresh"

pk new A
echo "VERSION A" > file0
pk add file0
pk refresh

pk new B
echo "VERSION B" > file0

fail pk pop | grep "error: unrefreshed changes found"
pk refresh

pk pop
grep "^VERSION A$" file0

pk pop
fail test -f file0

pk push
grep "^VERSION A$" file0

pk push
grep "^VERSION B$" file0

# End of file
