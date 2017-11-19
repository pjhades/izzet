#!/bin/bash

set -o errexit -o nounset -o pipefail

IZZET="$(pwd)/target/${BUILD:-release}/izzet"
SITE="/tmp/izzet_temp"

trap clean_site EXIT

clean_site() {
    rm -rf $SITE
}

assert_site_files() {
    test -f $SITE/.nojekyll
    test -f $SITE/.izzetconfig
    test -d $SITE/src
    test -d $SITE/theme
    test -d $SITE/files
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
    cd ..

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
    cd ..
    echo 'ok'
}

test_generate_site() {
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

    clean_site
    echo 'ok'
}

main() {
    test_create_new_site
    test_create_post
    test_generate_site
}

main
