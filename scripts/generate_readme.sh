#!/bin/bash

set -eu

cd $(dirname "${BASH_SOURCE[0]}")/..

REPO_BASE="https://raw.githubusercontent.com/nine-sols-modding/StateDump/main"
OUT="ALL.md"

function picture() {
    printf '<picture>\n'
    test -v 3 && printf "<source media=\"(prefers-color-scheme: dark)\" srcset=\"$REPO_BASE/$3\">\n"
    printf "<img alt=\"$1\" src=\"$REPO_BASE/$2\">\n"
    printf '</picture>\n\n'
}

printf "# All\n\n" > "$OUT"
for path in Attacks/*/*/*.svg; do
    name=$(dirname "$path")
    printf "## $name\n\n" >> "$OUT"
    # picture "$name" Attacks/Boss/{light,dark}/"$file" >> README.md
    picture "$name" "$path" >> "$OUT"
done
