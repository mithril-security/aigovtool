#!/bin/bash

set -e

TEST_CASE_DIR=$(dirname $0)
FAILURES=""
FAILED=()

if [ "$#" -gt 0 ]
then
    TEST_CASES="$@"
else
    TEST_CASES="$TEST_CASE_DIR/*"
fi

: ${TRACT_RUN:=cargo run -p tract $CARGO_OPTS --}

for tc in $TEST_CASES
do
    if [ ! -e "$tc/vars.sh" ]
    then
        continue
    fi
    . $tc/vars.sh
    for pass in txt bin bin-opti
    do
        [[ "$pass" = "txt" ]] && suffix=.txt || suffix=""
        [[ "$pass" = "bin-opti" ]] && opti="-O" || opti=""
        printf "$tc ($pass) "
        cmd="$TRACT_RUN \
            -f kaldi $tc/model.raw$suffix \
            --output-node output \
            --input-facts-from-bundle $tc/io.npz \
            --kaldi-downsample $subsampling \
            --kaldi-left-context $left_context \
            --kaldi-right-context $right_context \
            --kaldi-adjust-final-offset $adjust_final_offset \
            $opti \
            run \
            --input-from-bundle $tc/io.npz \
            --assert-output-bundle $tc/io.npz"

        # echo $cmd
        # if $($cmd)
        if $($cmd 2> /dev/null > /dev/null)
        then
            printf "\e[92mOK\e[39m\n"
        else
            printf "\e[91mFAIL\e[39m\n"
            FAILED+=("$cmd")
            FAILURES="$FAILURES $tc"
        fi
    done
done

if [ -n "$FAILURES" ]
then
    echo
    printf "    \e[91m$(echo $FAILURES | wc -w) FAILURES\e[39m\n"
    echo
fi

for cmd in "${FAILED[@]}"
do
    echo $cmd
done

[ -z "$FAILURES" ]
