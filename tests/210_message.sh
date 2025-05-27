# Test message

pk message -h | usage message
pk msg -h | usage message
pk m -h | usage message

fail pk message | grep "error: missing argument '<message...>'"

pk init

pk new INITIAL
assert "$(HEAD | summary)" == "INITIAL"

pk message MESSAGE
assert "$(HEAD | summary)" == "MESSAGE"

pk message MESSAGE 0 1 2 3
assert "$(HEAD | summary)" == "MESSAGE 0 1 2 3"

# End of file
