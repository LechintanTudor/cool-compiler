#!/bin/sh

cargo clippy -- \
    -A clippy::collapsible_else_if \
    -A clippy::module_inception \
    -A clippy::missing_safety_doc \
    -A clippy::new_ret_no_self \
    -A clippy::obfuscated_if_else
