<!-- 本文件由 ./make.md 自动生成，请不要直接修改此文件 -->

# 基于 blake3 的 merkle tree

[blake3](https://github.com/BLAKE3-team/BLAKE3) 的底层是一棵 merkle tree ，但是暴露的接口无法导出 merkle tree 。

[bao](https://github.com/oconnor663/bao) 实现了 blake3 流式验证，但无法调整底层块大小 ( [support larger "chunk groups" for reduced space overhead](https://github.com/oconnor663/bao/issues/34) ) 。

也就是说， bao 会消耗 6% 的额外存储空间来记录 merkle tree。对于分布式内容索引来说，这是挺大的开销。

于是，我实现了 [blake3_merkle](https://github.com/rmw-lib/blake3_merkle) ，每 1MB 内容导出 32 字节的哈希，额外存储开销只有 0.3‱  。

`./examples/main.rs` 如下 :

```rust
use blake3_merkle::Merkle;

use std::{env, error::Error, fs::File, io::copy};

fn main() -> Result<(), Box<dyn Error>> {
  let fpath = env::current_dir()?.join("test.pdf");

  let mut blake3 = blake3::Hasher::new();
  copy(&mut File::open(&fpath)?, &mut blake3)?;

  let mut merkle = Merkle::new();
  copy(&mut File::open(&fpath)?, &mut merkle)?;
  merkle.finalize();
  dbg!(&merkle.li);
  dbg!(merkle.blake3());
  dbg!(blake3.finalize());
  Ok(())
}
```

运行 `./example.main.sh`，输出如下

```
[examples/main.rs:14] &merkle.li = [
    HashDepth {
        hash: Hash(
            "eb896f431b7ff8acb4749b54981d461359a01ded0261fa0da856dd28bf29d3b3",
        ),
        depth: 10,
    },
    HashDepth {
        hash: Hash(
            "4a84cc85f03f47a7c32755f8d9d81c5d3f3e04548ee8129fd480cb71c7dbc5b4",
        ),
        depth: 10,
    },
    HashDepth {
        hash: Hash(
            "fbfe78e550b355cb6775e324c4fed7eb987084b115dca599aaf40056bfb031c3",
        ),
        depth: 10,
    },
    HashDepth {
        hash: Hash(
            "392878c3bdc9c315d6cc8a1721d8cd0a39e49ac8716f4cb8cdf6cf83fbb666f5",
        ),
        depth: 6,
    },
]
[examples/main.rs:15] merkle.blake3() = Hash(
    "74a79d0bc37dcac64c493e872252f19e8bdb32dee306481a6827fa037b378c76",
)
[examples/main.rs:16] blake3.finalize() = Hash(
    "74a79d0bc37dcac64c493e872252f19e8bdb32dee306481a6827fa037b378c76",
)
```

# Merkle tree based on blake3

The underlying merkle tree of [blake3](https://github.com/BLAKE3-team/BLAKE3) is a merkle tree, but the exposed interface cannot export the merkle tree.

[bao](https://github.com/oconnor663/bao) implements blake3 streaming verification, but cannot resize the underlying [chunks (support larger "chunk groups" for reduced space overhead](https://github.com/
oconnor663/bao/issues/34) ).

This means that bao consumes 6% extra storage space to record the merkle tree, which is a significant overhead for a distributed content index.

So, I implemented [blake3_merkle](https://github.com/rmw-lib/blake3_merkle) to export 32 bytes of hash per 1MB of content with an additional storage overhead of only 0.3‱.

`./examples/main.rs` As follows :

```rust
use blake3_merkle::Merkle;

use std::{env, error::Error, fs::File, io::copy};

fn main() -> Result<(), Box<dyn Error>> {
  let fpath = env::current_dir()?.join("test.pdf");

  let mut blake3 = blake3::Hasher::new();
  copy(&mut File::open(&fpath)?, &mut blake3)?;

  let mut merkle = Merkle::new();
  copy(&mut File::open(&fpath)?, &mut merkle)?;
  merkle.finalize();
  dbg!(&merkle.li);
  dbg!(merkle.blake3());
  dbg!(blake3.finalize());
  Ok(())
}
```

Run `./example.main.sh`and the output is as follows

```
[examples/main.rs:14] &merkle.li = [
    HashDepth {
        hash: Hash(
            "eb896f431b7ff8acb4749b54981d461359a01ded0261fa0da856dd28bf29d3b3",
        ),
        depth: 10,
    },
    HashDepth {
        hash: Hash(
            "4a84cc85f03f47a7c32755f8d9d81c5d3f3e04548ee8129fd480cb71c7dbc5b4",
        ),
        depth: 10,
    },
    HashDepth {
        hash: Hash(
            "fbfe78e550b355cb6775e324c4fed7eb987084b115dca599aaf40056bfb031c3",
        ),
        depth: 10,
    },
    HashDepth {
        hash: Hash(
            "392878c3bdc9c315d6cc8a1721d8cd0a39e49ac8716f4cb8cdf6cf83fbb666f5",
        ),
        depth: 6,
    },
]
[examples/main.rs:15] merkle.blake3() = Hash(
    "74a79d0bc37dcac64c493e872252f19e8bdb32dee306481a6827fa037b378c76",
)
[examples/main.rs:16] blake3.finalize() = Hash(
    "74a79d0bc37dcac64c493e872252f19e8bdb32dee306481a6827fa037b378c76",
)
```