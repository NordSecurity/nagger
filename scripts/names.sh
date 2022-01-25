#! /bin/bash

OLD_UPPER="${OLD_TEMPLATE^^}"
NEW_UPPER="${NEW_TEMPLATE^^}"

camelize() {
    UNDERSCORE=1
    while read -n1 X; do
        if [[ "$X" = '_' ]]; then
            UNDERSCORE=1
        elif [[ -n "$UNDERSCORE" ]]; then
            echo -n "${X^}"
            UNDERSCORE=
        else
            echo -n "$X"
        fi
    done
}

OLD_CAMEL="$(echo -n "$OLD_TEMPLATE" | camelize)"
NEW_CAMEL="$(echo -n "$NEW_TEMPLATE" | camelize)"

OLD_KEBAB="$(echo -n "$OLD_TEMPLATE" | tr '_' '-')"
NEW_KEBAB="$(echo -n "$NEW_TEMPLATE" | tr '_' '-')"