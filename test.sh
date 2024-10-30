#!/bin/bash
#
# Test runner
#

arguments() {
    VERBOSE=""
    RELEASE=""
    HELP=""
    TESTS=""

    while [ "$1" ]; do
        case $1 in
            -v|--verbose)
                VERBOSE="yes" ;;
            -r|--release)
                RELEASE="yes" ;;
            -h|--help)
                HELP="yes" ;;
            tests/*)
                if [ -f "$1" ]; then
                    TESTS="$TESTS $1"
                else
                    echo "error: $1 is not a test"
                    exit 1
                fi ;;
            *)
                echo "error: invalid option $1"
                exit 1 ;;
        esac
        shift
    done

    if [ "$HELP" ]; then
        echo "usage: test [options] [<test>...]"
        echo "options:"
        echo "  -v, --verbose     Use verbose output"
        echo "  -r, --release     Build and test artifacts in release mode"
        echo "  -h, --help        Print help and exit"
        exit 0
    fi
}

environment() {
    if [ "$VERBOSE" ]; then
        SHELL_VERBOSE="-v -x"
        REDIRECT="/dev/stdout"
    else
        SHELL_VERBOSE=""
        REDIRECT="/dev/null"
    fi

    if [ "$RELEASE" ]; then
        TARGET="release"
        CARGO_BUILD_OPTIONS="-r"
    else
        TARGET="debug"
        CARGO_BUILD_OPTIONS=""
    fi

    BUILD="build.sh"

    if [ ! "$TESTS" ]; then
        TESTS=$(find tests -type f -name '[0-9][0-9][0-9]_*' | sort)
    fi

    TESTDIR="$PWD/scratch"
    PATH="$PWD/target/$TARGET:$PATH"

    echo "Target: $TARGET" > $REDIRECT
    uname -a > $REDIRECT

    if ! git version | grep "^git version" &> $REDIRECT; then
        echo "error: git not found"
        exit -1
    fi

    if ! cargo version | grep "^cargo" &> $REDIRECT; then
        echo "error: cargo not found"
        exit -1
    fi
}

run_tests() {
    source tests/lib

    for FILE in $BUILD $TESTS; do
        if [ "$FILE" != "$BUILD" ]; then
            # Pick up .gitconfig from tests directory
            HOME="$PWD/tests"
        fi

        FILE=$(realpath $FILE)
        NAME=$(basename -s .sh $FILE)

        if [ $VERBOSE ]; then
            echo "---------------------------------------------------"
            echo " $NAME"
            echo "---------------------------------------------------"
        else
            echo "--- $NAME"
        fi

        rm -rf $TESTDIR
        mkdir -p $TESTDIR

        (
            cd $TESTDIR
            set -e $SHELL_VERBOSE
            source $FILE
        ) &> $REDIRECT

        STATUS=$?
        if [ $STATUS != 0 ]; then
            echo "---------------------------------------------------"
            echo "                      FAILED"
            echo "---------------------------------------------------"
            exit 1
        fi

        rm -rf $TESTDIR

        if [ $VERBOSE ]; then
            echo
        fi
    done
}

cd $(dirname $(realpath $0))
arguments "$@"
environment
run_tests

# End of file
