# Test brename

pk br -h | usage br
pk brename -h | usage brename

fail pk brename | grep "error: missing required free argument"
fail pk brename a b c | grep "error: too many arguments"

pk init
pk new INITIAL
pk finalize

pk bnew branch0
pk new A
assert $(HEAD | summary) == A
assert $(metadata | pushed | top | summary) == A
assert $(pk blist | count) == 2
pk blist | grep "main"
pk blist | grep "branch0"

pk bnew branch1
pk new B
assert $(HEAD | summary) == B
assert $(metadata | pushed | top | summary) == B
assert $(pk blist | count) == 3
pk blist | grep "main"
pk blist | grep "branch0"
pk blist | grep "branch1"

pk brename branch1_prime
assert $(HEAD | summary) == B
assert $(metadata | pushed | top | summary) == B
assert $(pk blist | count) == 3
pk blist | grep "main"
pk blist | grep "branch0"
pk blist | grep "branch1_prime"

pk brename branch0 branch0_prime
assert $(HEAD | summary) == B
assert $(metadata | pushed | top | summary) == B
assert $(pk blist | count) == 3
pk blist | grep "main"
pk blist | grep "branch0_prime"
pk blist | grep "branch1_prime"

pk bset branch0_prime
assert $(HEAD | summary) == A
assert $(metadata | pushed | top | summary) == A
assert $(pk blist | count) == 3
pk blist | grep "branch0_prime"
pk blist | grep "branch1_prime"

# End of file
