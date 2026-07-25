# Test clone

pk clone -h | usage clone

fail pk clone | grep "error: missing argument <url>"

pk clone $URL
test -d patchkeeper/.git

pk clone $URL alt_dir
test -d alt_dir/.git

# End of file
