# blake3_merkle

[→ 中文说明](#cn)

[blake3](https://github.com/BLAKE3-team/BLAKE3) is based on merkle tree, but the exposed interface cannot export merkle tree.

[bao](https://github.com/oconnor663/bao) implements blake3 streaming verification, but cannot resize the underlying [chunks](https://github.com/oconnor663/bao/issues/34) (see [support larger "chunk groups" for reduced space overhead](https://github.com/oconnor663/bao/issues/34) ).

That is, bao consumes 6% extra storage space to record the merkle tree, which is a significant overhead for a distributed content index.

So, I implemented [blake3_merkle](https://github.com/rmw-lib/blake3_merkle) to export 32 bytes of hash per 1MB of content with an additional storage overhead of only 0.3‱.

The merkle tree can generate hash values consistent with blake3.

When the content is less than or equal to 1MB, the merkle tree has only one node, and the hash of this node is equal to the hash of blake3.

`./examples/main.rs` As follows :

```rust
#include ./examples/main.rs
```

Run `./example.main.sh` and the output is as follows

```
#include ./main.out
```

<b id=cn></b>

## 基于 blake3 的 merkle tree

[blake3](https://github.com/BLAKE3-team/BLAKE3) 底层是 merkle tree ，但是暴露的接口无法导出 merkle tree 。

[bao](https://github.com/oconnor663/bao) 实现了 blake3 流式验证，但无法调整底层块大小 (参见 [support larger "chunk groups" for reduced space overhead](https://github.com/oconnor663/bao/issues/34) ) 。

也就是说，bao 会消耗 6% 的额外存储空间来记录 merkle tree。对于分布式内容索引来说，这是挺大的开销。

于是，我实现了 [blake3_merkle](https://github.com/rmw-lib/blake3_merkle) ，每 1MB 内容导出 32 字节的哈希，额外存储开销只有 0.3‱  。

通过 merkle tree 可以生成和 blake3 一致的哈希值。

当内容小于等于 1MB 时，merkle tree 只有一个节点，并且这个节点的哈希值等于 blake3 的哈希值。

`./examples/main.rs` 如下 :

```rust
#include ./examples/main.rs
```

运行 `./example.main.sh`，输出如下

```
#include ./main.out
```
