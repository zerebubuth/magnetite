pub mod magnetite {
    extern crate libc;

    mod ffi {
        extern crate libc;

        pub enum Db {}
        pub enum Options {}
        pub enum WriteOptions {}
        pub enum ReadOptions {}
        pub enum Snapshot {}

        #[link(name = "rocksdb")]
        extern "C" {
            pub fn rocksdb_options_create() -> *mut Options;
            pub fn rocksdb_options_destroy(ptr: *mut Options);

            pub fn rocksdb_options_increase_parallelism(
                opt: *mut Options, total_threads: libc::c_int);
            pub fn rocksdb_options_optimize_for_point_lookup(
                opt: *mut Options, block_cache_size_mb: libc::uint64_t);
            pub fn rocksdb_options_optimize_level_style_compaction(
                opt: *mut Options, memtable_memory_budget: libc::uint64_t);
            pub fn rocksdb_options_optimize_universal_style_compaction(
                opt: *mut Options, memtable_memory_budget: libc::uint64_t);
            // pub fn rocksdb_options_set_compaction_filter(
            //     opt: *mut Options,
            //     rocksdb_compactionfilter_t*);
            // pub fn rocksdb_options_set_compaction_filter_factory(
            //     opt: *mut Options, rocksdb_compactionfilterfactory_t*);
            // pub fn rocksdb_options_set_compaction_filter_factory_v2(
            //     opt: *mut Options,
            //     rocksdb_compactionfilterfactoryv2_t*);
            // pub fn rocksdb_options_set_comparator(
            //     opt: *mut Options,
            //     rocksdb_comparator_t*);
            // pub fn rocksdb_options_set_merge_operator(
            //     opt: *mut Options,
            //     rocksdb_mergeoperator_t*);
            pub fn rocksdb_options_set_compression_per_level(
                opt: *mut Options,
                level_values: *const libc::c_int,
                num_levels: libc::size_t);
            pub fn rocksdb_options_set_create_if_missing(
                opt: *mut Options, flag: libc::uint8_t);
            pub fn rocksdb_options_set_create_missing_column_families(
                opt: *mut Options, flag: libc::uint8_t);
            pub fn rocksdb_options_set_error_if_exists(
                opt: *mut Options, flag: libc::uint8_t);
            pub fn rocksdb_options_set_paranoid_checks(
                opt: *mut Options, flag: libc::uint8_t);

            pub fn rocksdb_writeoptions_create() -> *mut WriteOptions;
            pub fn rocksdb_writeoptions_destroy(
                opts: *mut WriteOptions);
            pub fn rocksdb_writeoptions_set_sync(
                opts: *mut WriteOptions, flag: libc::uint8_t);
            pub fn rocksdb_writeoptions_disable_WAL(
                opts: *mut WriteOptions, flag: libc::c_int);

            pub fn rocksdb_readoptions_create() -> *mut ReadOptions;
            pub fn rocksdb_readoptions_destroy(
                opts: *mut ReadOptions);
            pub fn rocksdb_readoptions_set_verify_checksums(
                opts: *mut ReadOptions, flag: libc::uint8_t);
            pub fn rocksdb_readoptions_set_fill_cache(
                opts: *mut ReadOptions, flag: libc::uint8_t);
            pub fn rocksdb_readoptions_set_snapshot(
                opts: *mut ReadOptions, snapshot: *const Snapshot);
            // pub fn rocksdb_readoptions_set_iterate_upper_bound(
            //     opts: *mut ReadOptions, key: *const libc::c_char,
            //     key_len: libc::size_t);
            pub fn rocksdb_readoptions_set_read_tier(
                opts: *mut ReadOptions, tier: libc::c_int);
            pub fn rocksdb_readoptions_set_tailing(
                opts: *mut ReadOptions, flag: libc::uint8_t);

            pub fn rocksdb_open(
                options: *const Options,
                name: *const libc::c_char,
                errptr: *mut *mut libc::c_char)
                -> *mut Db;
            pub fn rocksdb_close(db: *mut Db);
            pub fn rocksdb_create_snapshot(db: *mut Db) -> *const Snapshot;
            pub fn rocksdb_get(
                db: *mut Db, read_opts: *const ReadOptions,
                key: *const libc::c_char, keylen: libc::size_t,
                vallen: *mut libc::size_t,
                errptr: *mut *mut libc::c_char)
                -> *mut libc::c_char;
            pub fn rocksdb_put(
                db: *mut Db, write_opts: *const WriteOptions,
                key: *const libc::c_char, keylen: libc::size_t,
                val: *const libc::c_char, vallen: libc::size_t,
                errptr: *mut *mut libc::c_char);
                
            pub fn rocksdb_release_snapshot(
                db: *mut Db, snapshot: *const Snapshot);
        }
    }

    #[experimental]
    pub struct Options {
        ptr: *mut ffi::Options,
    }

    #[experimental]
    pub struct ReadOptions {
        ptr: *mut ffi::ReadOptions,
    }

    #[experimental]
    pub struct WriteOptions {
        ptr: *mut ffi::WriteOptions,
    }

    #[experimental]
    pub struct Db {
        ptr: *mut ffi::Db,
    }

    #[experimental]
    pub struct Snapshot {
        db: *mut ffi::Db,
        ptr: *const ffi::Snapshot,
    }

    impl Options {

        #[experimental]
        pub fn new() -> Options {
            Options {
                ptr: unsafe { ffi::rocksdb_options_create() }
            }
        }

        #[experimental]
        pub fn increase_parallelism(&mut self, total_threads: int) {
            unsafe { 
                ffi::rocksdb_options_increase_parallelism(
                    self.ptr, total_threads as libc::c_int)
            }
        }

        #[experimental]
        pub fn optimize_for_point_lookup(&mut self, block_cache_size_mb: u64) {
            unsafe {
                ffi::rocksdb_options_optimize_for_point_lookup(
                    self.ptr, block_cache_size_mb as libc::uint64_t);
            }
        }

        #[experimental]
        pub fn optimize_level_style_compaction(&mut self, memtable_memory_budget: u64) {
            unsafe {
                ffi::rocksdb_options_optimize_level_style_compaction(
                    self.ptr, memtable_memory_budget as libc::uint64_t);
            }
        }

        #[experimental]
        pub fn optimize_universal_style_compaction(&mut self, memtable_memory_budget: u64) {
            unsafe {
                ffi::rocksdb_options_optimize_universal_style_compaction(
                    self.ptr, memtable_memory_budget as libc::uint64_t);
            }
        }

        #[experimental]
        pub fn set_compression_per_level(&mut self, levels: &[int]) {
            unsafe {
                let len = levels.len() as libc::size_t;
                let ptr = levels.as_ptr();

                ffi::rocksdb_options_set_compression_per_level(
                    self.ptr, ptr as *const libc::c_int, len);
            }
        }

        #[experimental]
        pub fn set_create_if_missing(&mut self, flag: bool) {
            unsafe {
                ffi::rocksdb_options_set_create_if_missing(
                    self.ptr, flag as libc::uint8_t);
            }
        }

        #[experimental]
        pub fn set_create_missing_column_families(&mut self, flag: bool) {
            unsafe {
                ffi::rocksdb_options_set_create_missing_column_families(
                    self.ptr, flag as libc::uint8_t);
            }
        }

        #[experimental]
        pub fn set_error_if_exists(&mut self, flag: bool) {
            unsafe {
                ffi::rocksdb_options_set_error_if_exists(
                    self.ptr, flag as libc::uint8_t);
            }
        }

        #[experimental]
        pub fn set_paranoid_checks(&mut self, flag: bool) {
            unsafe {
                ffi::rocksdb_options_set_paranoid_checks(
                    self.ptr, flag as libc::uint8_t);
            }
        }
    }

    impl Drop for Options {
        #[experimental]
        fn drop(&mut self) {
            unsafe { ffi::rocksdb_options_destroy(self.ptr); }
        }
    }

    impl WriteOptions {
        #[experimental]
        pub fn new() -> WriteOptions {
            WriteOptions {
                ptr: unsafe { ffi::rocksdb_writeoptions_create() }
            }
        }

        #[experimental]
        pub fn set_sync(&mut self, flag: bool) {
            unsafe { ffi::rocksdb_writeoptions_set_sync(self.ptr, flag as libc::uint8_t); }
        }

        #[experimental]
        pub fn disable_wal(&mut self, flag: bool) {
            unsafe { ffi::rocksdb_writeoptions_disable_WAL(self.ptr, flag as libc::c_int); }
        }
    }

    impl Drop for WriteOptions {
        #[experimental]
        fn drop(&mut self) {
            unsafe { ffi::rocksdb_writeoptions_destroy(self.ptr); }
        }
    }

    impl ReadOptions {
        #[experimental]
        pub fn new() -> ReadOptions {
            ReadOptions {
                ptr: unsafe { ffi::rocksdb_readoptions_create() }
            }
        }

        #[experimental]
        pub fn set_verify_checksums(&mut self, flag: bool) {
            unsafe { ffi::rocksdb_readoptions_set_verify_checksums(self.ptr, flag as libc::uint8_t); }
        }

        #[experimental]
        pub fn set_fill_cache(&mut self, flag: bool) {
            unsafe { ffi::rocksdb_readoptions_set_fill_cache(self.ptr, flag as libc::uint8_t); }
        }

        #[experimental]
        pub fn set_snapshot(&mut self, snapshot: &Snapshot) {
            unsafe { ffi::rocksdb_readoptions_set_snapshot(self.ptr, snapshot.ptr); }
        }

        // NOTE: this is totally broken - it assigns a pointer from the stack, and
        // the lifetime / ownership is not specified.
        //
        // pub fn set_iterate_upper_bound(&mut self, key: &str) {
        //     unsafe { ffi::rocksdb_readoptions_(self.ptr, flag as libc::uint8_t); }
        // }

        #[experimental]
        pub fn set_read_tier(&mut self, tier: int) {
            unsafe { ffi::rocksdb_readoptions_set_read_tier(self.ptr, tier as libc::c_int); }
        }

        #[experimental]
        pub fn set_tailing(&mut self, flag: bool) {
            unsafe { ffi::rocksdb_readoptions_set_tailing(self.ptr, flag as libc::uint8_t); }
        }
    }

    impl Drop for ReadOptions {
        #[experimental]
        fn drop(&mut self) {
            unsafe { ffi::rocksdb_readoptions_destroy(self.ptr); }
        }
    }

    impl Db {
        #[experimental]
        pub fn new(opts: &Options, name: &str) -> Result<Db, String> {
            use std::ptr;
            use std::c_str::CString;

            unsafe {
                let mut errstr : *mut libc::c_char = ptr::null_mut::<libc::c_char>();
                let c_str = name.as_ptr();
                let ptr = ffi::rocksdb_open(opts.ptr as *const ffi::Options,
                                            c_str as *const libc::c_char,
                                            &mut errstr);

                if ptr.is_null() {
                    let cstring = CString::new(errstr as *const libc::c_char, true);
                    Err(String::from_str(
                        cstring
                            .as_str()
                            .expect("Bad UTF-8 in rocksdb_open error message")))
                } else {
                    if !errstr.is_null() { libc::free(errstr as *mut libc::c_void) }
                    Ok(Db { ptr: ptr })
                }
            }
        }

        #[experimental]
        pub fn snapshot(&mut self) -> Snapshot {
            Snapshot {
                db: self.ptr,
                ptr: unsafe { ffi::rocksdb_create_snapshot(self.ptr) }
            }
        }

        #[experimental]
        pub fn get(&mut self, read_opts: &ReadOptions, key: &[u8]) -> Result<Option<Vec<u8>>, String> {
            use std::ptr;
            use std::c_str::CString;

            unsafe {
                let mut errstr : *mut libc::c_char = ptr::null_mut::<libc::c_char>();
                let key_ptr = key.as_ptr();
                let mut vallen : libc::size_t = 0;
                let ptr = ffi::rocksdb_get(self.ptr, 
                                           read_opts.ptr as *const ffi::ReadOptions,
                                           key_ptr as *const libc::c_char,
                                           key.len() as libc::size_t,
                                           &mut vallen,
                                           &mut errstr);

                if errstr.is_null() {
                    Ok(if ptr.is_null() {
                        None

                    } else {
                        let arr = Vec::from_raw_buf(ptr as *const u8, vallen as uint);
                        libc::free(ptr as *mut libc::c_void);
                        Some(arr)
                    })

                } else {
                    let cstring = CString::new(errstr as *const libc::c_char, true);
                    Err(String::from_str(
                        cstring
                            .as_str()
                            .expect("Bad UTF-8 in rocksdb_get error message")))
                }
            }
        }

        #[experimental]
        pub fn put(&mut self, write_opts: &WriteOptions, key: &[u8], val: &[u8])
                   -> Result<(), String> {
            use std::ptr;
            use std::c_str::CString;

            unsafe {
                let mut errstr : *mut libc::c_char = ptr::null_mut::<libc::c_char>();
                let key_ptr = key.as_ptr();
                let val_ptr = val.as_ptr();
                ffi::rocksdb_put(
                    self.ptr, write_opts.ptr as *const ffi::WriteOptions,
                    key_ptr as *const libc::c_char, key.len() as libc::size_t,
                    val_ptr as *const libc::c_char, val.len() as libc::size_t,
                    &mut errstr);

                if errstr.is_null() {
                    Ok(())

                } else {
                    let cstring = CString::new(errstr as *const libc::c_char, true);
                    Err(String::from_str(
                        cstring
                            .as_str()
                            .expect("Bad UTF-8 in rocksdb_put error message")))
                }
            }
        }
    }

    impl Drop for Db {
        #[experimental]
        fn drop(&mut self) {
            unsafe { ffi::rocksdb_close(self.ptr); }
        }
    }

    impl Drop for Snapshot {
        #[experimental]
        fn drop(&mut self) {
            unsafe { ffi::rocksdb_release_snapshot(self.db, self.ptr); }
        }
    }
}

#[test]
fn test_options() {
    let mut opts = magnetite::Options::new();
    opts.increase_parallelism(2);
    opts.optimize_for_point_lookup(16);
    opts.optimize_level_style_compaction(16);
    opts.optimize_universal_style_compaction(16);
    opts.set_compression_per_level([1,2,3,4,5].as_slice());
    opts.set_create_if_missing(true);
    opts.set_create_missing_column_families(true);
    opts.set_error_if_exists(true);
    opts.set_paranoid_checks(true);
}

#[test]
fn test_db() {
    use std::io;

    let mut opts = magnetite::Options::new();
    let tmpdir = io::TempDir::new("magnetite_create_db").unwrap();
    let dbpath = tmpdir.path().join("db");
    let tmppath = dbpath.as_str().unwrap();

    let write_opts = magnetite::WriteOptions::new();
    let read_opts = magnetite::ReadOptions::new();

    opts.set_create_if_missing(true);
    let mut db = magnetite::Db::new(&opts, tmppath).unwrap();

    let key = [104u8, 101u8, 108u8, 108u8, 111u8];
    let val = [119u8, 111u8, 114u8, 108u8, 100u8];
    let first_value = db.get(&read_opts, key.as_slice()).unwrap();
    assert!(first_value.is_none());
    db.put(&write_opts, key.as_slice(), val.as_slice()).unwrap();
    let second_value = db.get(&read_opts, key.as_slice()).unwrap();
    assert!(second_value.is_some());
    assert!(second_value.unwrap().as_slice() == val.as_slice());
}

#[test]
fn test_snapshot() {
    use std::io;

    let mut opts = magnetite::Options::new();
    let tmpdir = io::TempDir::new("magnetite_create_db").unwrap();
    let dbpath = tmpdir.path().join("db");
    let tmppath = dbpath.as_str().unwrap();

    let write_opts = magnetite::WriteOptions::new();
    let read_opts = magnetite::ReadOptions::new();

    opts.set_create_if_missing(true);
    let mut db = magnetite::Db::new(&opts, tmppath).unwrap();

    let key = [104u8, 101u8, 108u8, 108u8, 111u8];
    let val = [119u8, 111u8, 114u8, 108u8, 100u8];
    let first_value = db.get(&read_opts, key.as_slice()).unwrap();
    assert!(first_value.is_none());
    db.put(&write_opts, key.as_slice(), val.as_slice()).unwrap();
    let second_value = db.get(&read_opts, key.as_slice()).unwrap();
    assert!(second_value.is_some());
    assert!(second_value.unwrap().as_slice() == val.as_slice());

    // take snapshot
    let snapshot = db.snapshot();

    // then alter the key we're about to read
    let val2 = [119u8, 111u8, 114u8, 108u8, 100u8, 50u8];
    db.put(&write_opts, key.as_slice(), val2.as_slice()).unwrap();

    // read snapshot should return old value
    let mut read_opts_snap = magnetite::ReadOptions::new();
    read_opts_snap.set_snapshot(&snapshot);
    let third_value = db.get(&read_opts_snap, key.as_slice()).unwrap();
    assert!(third_value.is_some());
    assert!(third_value.unwrap().as_slice() == val.as_slice());

    // but a read outside the snapshot returns the new value
    let fourth_value = db.get(&read_opts, key.as_slice()).unwrap();
    assert!(fourth_value.is_some());
    assert!(fourth_value.unwrap().as_slice() == val2.as_slice());
}
