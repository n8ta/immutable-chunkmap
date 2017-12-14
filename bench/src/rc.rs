extern crate immutable;
use self::immutable::rc::map::Map;
use std::time::{Duration, Instant};
use std::vec::{Vec};
use utils;

fn bench_add(len: usize) -> (Map<i64, i64>, Vec<i64>, Duration) {
  let mut m = Map::new();
  let data = utils::randvec::<i64>(len);
  let elapsed = {
    let mut pairs : Vec<(&i64, &i64)> = Vec::new();
    let mut i = 0;
    let csize = data.len() / 10;
    let begin = Instant::now();
    for k in &data {
      pairs.push((&k, &k));
      if i % csize == 0 || i == data.len() - 1 {
        pairs.sort_unstable_by(|&(k0, _), &(k1, _)| k0.cmp(k1));
        m = m.add_sorted(&pairs);
        pairs.clear();
      }
      i = i + 1;
    }
    begin.elapsed()
  };
  (m, data, elapsed)
}

fn bench_find(m: &Map<i64, i64>, d: &Vec<i64>) -> Duration {
  let begin = Instant::now();
  for kv in d { m.find(&kv).unwrap(); }
  begin.elapsed()
}

fn bench_remove(m: Map<i64, i64>, d: &Vec<i64>) -> Duration {
  let mut m = m;
  let begin = Instant::now();
  for kv in d { m = m.remove(&kv) }
  begin.elapsed()
}

pub(crate) fn run(size: usize) -> () {
  let (m, d, add) = bench_add(size);
  let find = bench_find(&m, &d);
  let rm = bench_remove(m, &d);
  println!("add: {}, find: {}, remove: {}", 
    utils::to_ms(add), utils::to_ms(find), utils::to_ms(rm));
}