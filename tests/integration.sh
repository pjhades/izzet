#!/bin/bash

set -o errexit -o nounset -o pipefail

IZZET="$(pwd)/target/${BUILD:-release}/izzet"
SITE=$(mktemp -d /tmp/izzet_temp.XXXXXX)

trap clean_site EXIT

clean_site() {
    rm -rf $SITE
    pkill izzet || :
}

assert_site_files() {
    test -f $SITE/.nojekyll
    test -f $SITE/.izzetconfig
    test -d $SITE/src
    test -d $SITE/theme
}

test_create_new_site() {
    echo -n "${FUNCNAME[0]} ... "

    $IZZET -n $SITE
    assert_site_files

    clean_site

    mkdir -p $SITE
    cd $SITE
    $IZZET -n
    assert_site_files
    cd - >/dev/null

    ! $IZZET -n $SITE &>/dev/null
    $IZZET -n -f $SITE
    assert_site_files

    clean_site
    echo 'ok'
}

test_create_post() {
    echo -n "${FUNCNAME[0]} ... "

    $IZZET -n $SITE
    cd $SITE

    $IZZET -a a
    ! $IZZET -a a &>/dev/null
    $IZZET -a -f a

    $IZZET -p p
    ! $IZZET -p p &>/dev/null
    $IZZET -p -f p

    clean_site
    cd - >/dev/null
    echo 'ok'
}

test_generate_site_and_local_server() {
    echo -n "${FUNCNAME[0]} ... "

    $IZZET -n $SITE
    $IZZET -c $SITE -a a
    test -f a.md
    $IZZET -c $SITE -p p
    test -f p.md
    mv a.md p.md $SITE/src

    $IZZET -c $SITE -g -i $SITE -o $SITE &>/dev/null
    ! $IZZET -c $SITE -g -i $SITE -o $SITE &>/dev/null
    $IZZET -c $SITE -g -f -i $SITE -o $SITE &>/dev/null

    cd $SITE
    local ts=($(find . -mindepth 4 -type f -a -name '*.html' | \
          awk -F'/' '{print $2,$3,$4}'))
    cd - >/dev/null
    local year=${ts[0]}
    local month=${ts[1]}
    local day=${ts[2]}

    $IZZET -c $SITE -s $SITE -l 9999 >/dev/null &
    while ! pgrep izzet &>/dev/null; do
        sleep 0.5
    done
    local server=$!
    curl --silent --fail 0.0.0.0:9999/index.html >/dev/null
    curl --silent --fail 0.0.0.0:9999/archive.html >/dev/null
    curl --silent --fail 0.0.0.0:9999/p.html >/dev/null
    curl --silent --fail 0.0.0.0:9999/$year/$month/$day/a.html >/dev/null
    kill $server
    wait $server &>/dev/null || :

    clean_site
    echo 'ok'
}

main() {
    test_create_new_site
    test_create_post
    test_generate_site_and_local_server
}

main
