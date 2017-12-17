#!/bin/sh

if ! type sqlite3 >/dev/null 2>&1 ; then
    echo 'Please install sqlite3'
    exit
fi

cat << EOF | sqlite3 data.db
create table Memo (
       id integer primary key autoincrement,
       content text

);
EOF

