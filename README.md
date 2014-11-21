# magnetite

Magnetite is a [Rust](http://rust-lang.org) wrapper around the
[RocksDB](http://rocksdb.org) embedded key-value store.

> WARNING: This library is really unstable. If you want to use it,
> great! Please report all the *many* issues you find on the
> [issues page](http://github.com/zerebubuth/magnetite/issues).

## Building and Running

At the moment, I find that it needs to be pointed at RocksDB
directly. I've used RocksDB 3.6.1, but it might work with other
versions. For example, if I've built RocksDB in `$ROCKSDB_DIR` then,
to run the tests:

    LD_LIBRARY_PATH=$ROCKSDB_DIR RUST_PATH=$ROCKSDB_DIR cargo test

