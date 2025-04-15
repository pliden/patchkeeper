# Test message

pk m -h | usage m
pk msg -h | usage msg
pk message -h | usage message

fail pk message | grep "error: missing required free argument"

pk init

pk new INITIAL
assert "$(HEAD | summary)" == "INITIAL"

pk message MESSAGE
assert "$(HEAD | summary)" == "MESSAGE"

pk message MESSAGE 0 1 2 3
assert "$(HEAD | summary)" == "MESSAGE 0 1 2 3"

# End of file
