# 基于 blake3 的 merkle tree

[blake3](https://github.com/BLAKE3-team/BLAKE3) 底层是 merkle tree ，但是暴露的接口无法导出 merkle tree 。

[bao](https://github.com/oconnor663/bao) 实现了 blake3 流式验证，但无法调整底层块大小 (参见 [support larger "chunk groups" for reduced space overhead](https://github.com/oconnor663/bao/issues/34) ) 。

也就是说， bao 会消耗 6% 的额外存储空间来记录 merkle tree。对于分布式内容索引来说，这是挺大的开销。

于是，我实现了 [blake3_merkle](https://github.com/rmw-lib/blake3_merkle) ，每 1MB 内容导出 32 字节的哈希，额外存储开销只有 0.3‱  。

`./examples/main.rs` 如下 :

```rust
#include ./examples/main.rs
```

运行 `./example.main.sh`，输出如下

```
#include ./main.out
```

# Merkle tree based on blake3

[blake3](https://github.com/BLAKE3-team/BLAKE3) is based on merkle tree, but the exposed interface cannot export merkle tree.

[bao](https://github.com/oconnor663/bao) implements blake3 streaming verification, but cannot resize the underlying chunks (see [support larger "chunk groups" for reduced space overhead](https://github.com/oconnor663/bao/issues/34) ).

This means that bao consumes 6% extra storage space to record the merkle tree, which is a significant overhead for a distributed content index.

So, I implemented [blake3_merkle](https://github.com/rmw-lib/blake3_merkle) to export 32 bytes of hash per 1MB of content with an additional storage overhead of only 0.3‱.

`./examples/main.rs` As follows :

```rust
#include ./examples/main.rs
```

Run `./example.main.sh`and the output is as follows

```
#include ./main.out
```
