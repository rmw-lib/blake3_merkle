#![feature(new_uninit)]

use blake3::{
  guts::{parent_cv, ChunkState, CHUNK_LEN},
  Hash,
};

use std::{
  io::{Error, Write},
  mem::replace,
};

#[derive(Debug, Clone)]
pub struct HashDepth {
  hash: Hash,
  depth: u8,
}

// (1<<10) * 1024 = 1MB
pub const BLOCK_CHUNK: u8 = 10;

#[derive(Debug)]
pub struct Merkle {
  pub li: Vec<HashDepth>,
  pub pos: usize,
  pub n: u64,
  pub state: ChunkState,
}

impl Merkle {
  pub fn new() -> Self {
    Merkle {
      li: vec![],
      pos: 0,
      n: 0,
      state: ChunkState::new(0),
    }
  }

  pub fn finalize(&mut self) {
    let mut len = self.li.len();
    let end = len == 0;
    if self.pos != 0 {
      self.push(true);
      len = self.li.len();
    } else if end {
      return;
    }

    let li = &mut self.li;
    len -= 1;
    let mut hash = li[len].hash;

    while len > 0 {
      len -= 1;
      let left = &li[len];
      if left.depth == BLOCK_CHUNK {
        len += 1;
        li[len].hash = hash;
        li.truncate(len + 1);
        break;
      }
      hash = parent_cv(&left.hash, &hash, 0 == len);
    }
  }

  pub fn blake3(&self) -> Hash {
    let li = &self.li;
    let len = li.len();
    match len {
      0 => ChunkState::new(0).update(&[]).finalize(true),
      1 => li[0].hash,
      2 => parent_cv(&li[0].hash, &li[1].hash, true),
      len => {
        /*
        let mut len = len;
        let mut pre_li;

        'outer: while len > 2 {
        let mut hash_li = Vec::with_capacity(len);
        let mut n = 0;
        let mut n_1 = n + 1;
        while n_1 < len {
        let li_n = &li[n];
        let li_n_1 = &li[n_1];
        let depth = li_n.depth;
        if depth == li_n_1.depth {
        hash_li.push(HashDepth {
        depth: depth + 1,
        hash: parent_cv(&li_n.hash, &li_n_1.hash, false),
        })
        } else if n == 0 {
        break 'outer;
        }
        n = n_1 + 1;
        n_1 = n + 1;
        }
        if n == len {
        hash_li.copy_from_slice(li[n - 1].clone());
        }
        len = hash_li.len();
        pre_li = hash_li;
        li = &pre_li;
        }
        */

        let mut len = len;
        let hash_len = len / 2;
        let end = len % 2;
        let mut box_len = hash_len + end;
        let mut hash_li = unsafe { Box::<[Hash]>::new_uninit_slice(box_len).assume_init() };

        if end != 0 {
          hash_li[0] = li[0].hash;
        }

        while len >= 2 {
          let t = len - 1;
          len = t - 1;
          hash_li[t / 2] = parent_cv(&li[len].hash, &li[t].hash, false);
        }

        len = hash_len;
        let mut li = hash_li;

        while box_len > 2 {
          let hash_len = len / 2;
          let end = len % 2;
          let mut box_len = hash_len + end;
          let mut hash_li = unsafe { Box::<[Hash]>::new_uninit_slice(box_len).assume_init() };

          if end != 0 {
            hash_li[0] = li[0];
          }

          while len >= 2 {
            let t = len - 1;
            len = t - 1;
            hash_li[t / 2] = parent_cv(&li[len], &li[t], false);
          }
          len = hash_len;
          li = hash_li;
        }

        parent_cv(&li[0], &li[1], true)
      }
    }
  }

  fn push(&mut self, finalize: bool) {
    let li = &mut self.li;
    let mut len = li.len();
    let mut hash = self.state.finalize(finalize && len == 0);

    let mut depth = 0;
    while len > 0 {
      len -= 1;
      let left = &li[len];
      if left.depth == depth {
        depth += 1;
        hash = parent_cv(&left.hash, &hash, finalize && len == 0);
        li.pop();
        if depth == BLOCK_CHUNK {
          break;
        }
      }
    }
    li.push(HashDepth { depth, hash });
  }
}

impl Write for Merkle {
  fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
    let len = buf.len();
    let mut pos = self.pos;
    let mut n = self.n;
    let mut remain = CHUNK_LEN - pos;
    let mut begin = 0;

    while begin < len {
      if remain == 0 {
        self.push(false);
        n += 1;
        self.state = ChunkState::new(n);
        pos = 0;
        remain = CHUNK_LEN;
      }
      let diff = len - begin;
      if diff < remain {
        pos += diff;
        self.state.update(&buf[begin..]);
        break;
      } else {
        let end = begin + remain;
        self.state.update(&buf[begin..end]);
        begin = end;
        remain = 0;
        pos = CHUNK_LEN;
      }
    }
    self.pos = pos;
    self.n = n;
    Ok(len)
  }

  fn flush(&mut self) -> Result<(), Error> {
    Ok(())
  }
}
