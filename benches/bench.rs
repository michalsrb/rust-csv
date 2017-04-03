#![feature(test)]

extern crate csv;
extern crate test;

use std::io;

use test::Bencher;

use csv::{ByteRecord, Reader, ReaderBuilder, StringRecord};

static NFL: &'static str = include_str!("../examples/data/nfl.csv");
static GAME: &'static str = include_str!("../examples/data/game.csv");
static POP: &'static str = include_str!("../examples/data/worldcitiespop.csv");

macro_rules! bench {
    ($name:ident, $data:ident, $counter:ident, $result:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let data = $data.as_bytes();
            b.bytes = data.len() as u64;
            b.iter(|| {
                let mut rdr = ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(data);
                assert_eq!($counter(&mut rdr), $result);
            })
        }
    };
}

bench!(count_nfl_iter_bytes, NFL, count_iter_record_bytes, 10000);
bench!(count_nfl_read_bytes, NFL, count_read_record_bytes, 10000);
bench!(count_nfl_read_str, NFL, count_read_record_str, 10000);
bench!(count_game_iter_bytes, GAME, count_iter_record_bytes, 100000);
bench!(count_game_read_bytes, GAME, count_read_record_bytes, 100000);
bench!(count_game_read_str, GAME, count_read_record_str, 100000);
bench!(count_pop_iter_bytes, POP, count_iter_record_bytes, 20001);
bench!(count_pop_read_bytes, POP, count_read_record_bytes, 20001);
bench!(count_pop_read_str, POP, count_read_record_str, 20001);

fn count_iter_record_bytes<R: io::Read>(rdr: &mut Reader<R>) -> u64 {
    let mut count = 0;
    for rec in rdr.byte_records() {
        let _ = rec.unwrap();
        count += 1;
    }
    count
}

fn count_read_record_bytes<R: io::Read>(rdr: &mut Reader<R>) -> u64 {
    let mut count = 0;
    let mut rec = ByteRecord::new();
    while !rdr.read_record_bytes(&mut rec).unwrap() {
        count += 1;
    }
    count
}

fn count_read_record_str<R: io::Read>(rdr: &mut Reader<R>) -> u64 {
    let mut count = 0;
    let mut rec = StringRecord::new();
    while !rdr.read_record(&mut rec).unwrap() {
        count += 1;
    }
    count
}
