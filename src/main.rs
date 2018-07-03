extern crate rocksdb;

fn main() {
    let mut opts = rocksdb::Options::default();
    opts.create_if_missing(true);

    let db = rocksdb::DB::open(&opts, "/tmp/test-db").unwrap();
    db.create_cf("")
}
