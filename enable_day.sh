#!/bin/bash

set -e

if [[ ${#} -eq 0 ]]; then
    echo "Usage:"
    echo "    ${0} N"
    echo "where N is the day number."
    exit 1
fi

day=`printf "%02d\n" ${@}`

if ! git diff --exit-code --quiet; then
    echo "Commit or stash modifications. Exiting."
    exit 2
fi

cmd="git checkout -b day${day}"
echo "--> ${cmd}"
eval ${cmd}

cmd="cp -r dayXX day${day}"
echo "--> ${cmd}"
eval ${cmd}

cmd="sed -i.bak 's|   \# \"day${day}\",|   \"day${day}\",|g' Cargo.toml"
echo "--> ${cmd}"
eval ${cmd}

cmd="sed -i.bak 's|name = \"dayXX\"|name = \"day${day}\"|g' day${day}/Cargo.toml"
echo "--> ${cmd}"
eval ${cmd}

cmd="sed -i.bak 's|dayXX|day${day}|g' day${day}/src/lib.rs"
echo "--> ${cmd}"
eval ${cmd}

cmd="sed -i.bak 's|Day XX|Day ${day}|g' day${day}/src/lib.rs"
echo "--> ${cmd}"
eval ${cmd}

cmd="find . -name '*.bak' -exec rm -f {} \;"
echo "--> ${cmd}"
eval ${cmd}

cmd="cargo build"
echo "--> ${cmd}"
eval ${cmd}

cmd="git add day${day} Cargo.toml Cargo.lock"
echo "--> ${cmd}"
eval ${cmd}

cmd="git commit -m 'Add initial files from template for day ${day}'"
echo "--> ${cmd}"
eval ${cmd}
