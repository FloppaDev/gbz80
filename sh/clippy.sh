#!/bin/sh

lints="\
    -Dunreachable_patterns\
    -Dnon_snake_case\
    \
    -Wclippy::pedantic\
    -Wclippy::nursery\
    \
    -Aclippy::wildcard-imports\
    -Aclippy::cast-sign-loss\
    -Aclippy::cast-possible-truncation\
    -Aclippy::cast-precision-loss\
    -Aclippy::enum-glob-use\
    -Aclippy::too_many_lines\
    -Aclippy::too_many_arguments\
    -Aclippy::cognitive_complexity\
    -Aclippy::unnecessary_wraps\
    -Aclippy::if_not_else\
    -Aclippy::non_ascii_literal\
    -Aclippy::match_wildcard_for_single_variants\
    -Aclippy::module_name_repetitions\
    -Aclippy::needless_range_loop\
    -Aclippy::cast_possible_wrap\
    -Aclippy::type_complexity\
"

cargo clippy --tests 2>&1 -- $lints
cargo clippy -- $lints
cd gen/lex
cargo clippy -- $lints
cd ../..
cd gen/image
cargo clippy -- $lints
cd ../..

