# Test fetch

ORIGIN_MAIN=.git/refs/remotes/origin/main

pk fetch -h | usage fetch

pk clone $URL
cd patchkeeper
ORIG_REF=$(cat $ORIGIN_MAIN)

pk fetch | grep "nothing to fetch"

rm $ORIGIN_MAIN
pk fetch
assert $(cat $ORIGIN_MAIN) = $ORIG_REF

echo $(PREV_HEAD) > $ORIGIN_MAIN
pk fetch
assert $(cat $ORIGIN_MAIN) = $ORIG_REF

# End of file
