#! /bin/sh

# Automatically generate bindings for rctl functions.  Doing it here and
# commiting the results to version control is faster than using bindgen at
# build time, and works as long as the bound functions are stable across OS
# versions.

CRATEDIR=`dirname $0`/..

bindgen \
	--allowlist-function 'rctl_get_limits' \
	--allowlist-function 'rctl_get_racct' \
	--allowlist-function 'rctl_add_rule' \
	--allowlist-function 'rctl_get_rules' \
	--allowlist-function 'rctl_remove_rule' \
	/usr/include/sys/rctl.h > ${CRATEDIR}/src/ffi.rs
