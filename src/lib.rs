pub mod rocksdb {
    extern crate libc;

    mod ffi {
        extern crate libc;

        pub enum Db {}
        pub enum Options {}

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

            pub fn rocksdb_open(
                options: *const Options,
                name: *const libc::c_char,
                errptr: *mut *mut libc::c_char)
                -> *mut Db;
            pub fn rocksdb_close(db: *mut Db);
        }
    }
    
    pub struct Options {
        ptr: *mut ffi::Options,
    }

    impl Options {

        pub fn new() -> Options {
            Options {
                ptr: unsafe { ffi::rocksdb_options_create() }
            }
        }

        pub fn increase_parallelism(&mut self, total_threads: int) {
            unsafe { 
                ffi::rocksdb_options_increase_parallelism(
                    self.ptr, total_threads as libc::c_int)
            }
        }

        pub fn optimize_for_point_lookup(&mut self, block_cache_size_mb: u64) {
            unsafe {
                ffi::rocksdb_options_optimize_for_point_lookup(
                    self.ptr, block_cache_size_mb as libc::uint64_t);
            }
        }

        pub fn optimize_level_style_compaction(&mut self, memtable_memory_budget: u64) {
            unsafe {
                ffi::rocksdb_options_optimize_level_style_compaction(
                    self.ptr, memtable_memory_budget as libc::uint64_t);
            }
        }

        pub fn optimize_universal_style_compaction(&mut self, memtable_memory_budget: u64) {
            unsafe {
                ffi::rocksdb_options_optimize_universal_style_compaction(
                    self.ptr, memtable_memory_budget as libc::uint64_t);
            }
        }

        pub fn set_compression_per_level(&mut self, levels: &[int]) {
            unsafe {
                let len = levels.len() as libc::size_t;
                let ptr = levels.as_ptr();

                ffi::rocksdb_options_set_compression_per_level(
                    self.ptr, ptr as *const libc::c_int, len);
            }
        }

        pub fn set_create_if_missing(&mut self, flag: bool) {
            unsafe {
                ffi::rocksdb_options_set_create_if_missing(
                    self.ptr, flag as libc::uint8_t);
            }
        }

        pub fn set_create_missing_column_families(&mut self, flag: bool) {
            unsafe {
                ffi::rocksdb_options_set_create_missing_column_families(
                    self.ptr, flag as libc::uint8_t);
            }
        }

        pub fn set_error_if_exists(&mut self, flag: bool) {
            unsafe {
                ffi::rocksdb_options_set_error_if_exists(
                    self.ptr, flag as libc::uint8_t);
            }
        }

        pub fn set_paranoid_checks(&mut self, flag: bool) {
            unsafe {
                ffi::rocksdb_options_set_paranoid_checks(
                    self.ptr, flag as libc::uint8_t);
            }
        }
    }

    impl Drop for Options {
        fn drop(&mut self) {
            unsafe { ffi::rocksdb_options_destroy(self.ptr); }
        }
    }

    pub struct Db {
        ptr: *mut ffi::Db,
    }

    impl Db {
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
    }

    impl Drop for Db {
        fn drop(&mut self) {
            unsafe { ffi::rocksdb_close(self.ptr); }
        }
    }
}

#[test]
fn test_options() {
    let mut opts = rocksdb::Options::new();
    opts.increase_parallelism(2);
    opts.optimize_for_point_lookup(16);
    opts.optimize_level_style_compaction(16);
    opts.optimize_universal_style_compaction(16);
    opts.set_compression_per_level([1,2,3,4,5]);
    opts.set_create_if_missing(true);
    opts.set_create_missing_column_families(true);
    opts.set_error_if_exists(true);
    opts.set_paranoid_checks(true);
}

#[test]
fn test_db() {
    use std::io;

    let mut opts = rocksdb::Options::new();
    let tmpdir = io::TempDir::new("rocksdb_create_db").unwrap();
    let dbpath = tmpdir.path().join("db");
    let tmppath = dbpath.as_str().unwrap();

    opts.set_create_if_missing(true);
    let mut db = rocksdb::Db::new(&opts, tmppath).unwrap();
}

