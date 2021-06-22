#!/bin/sh

if ! ldconfig -p | grep -q 'libssl.so\s'; then
	echo 'Error: libssl dev library is not installed (on Debian, this is libssl-dev)' >&2;
	exit 1;
fi

if ! [ -x "$(command -v sqlite3)" ]; then
	echo 'Error: sqlite3 is not installed' >&2;
	exit 1;
fi

if ! ldconfig -p | grep -q 'libsqlite3'; then
	echo 'Error: sqlite3 lib is not installed (on Debian, this is libsqlite3-dev)' >&2;
	exit 1;
fi

if ! [ -x "$(command -v diesel)" ]; then
	echo 'Error: diesel is not installed' >&2;
	exit 1;
fi

if ! [ -f '.env' ]; then
	echo 'DATABASE_URL=../test.db' > .env;
fi

diesel setup
