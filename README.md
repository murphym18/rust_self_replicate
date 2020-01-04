# What is this?

This program outputs its source code. If you were to run the following:

```bash
cargo run > main_copy.rs
sha256sum src/main.rs main_copy.rs
```

You'd see that `src/main.rs` and `main_copy.rs` both hash to the same string. On my Linux system, the commands above give the following output:

```
dab8a69abc1bf1a12758ac2c03eba92c3ee6680c9d24a08ea82b874e4f7c36f4  src/main.rs
dab8a69abc1bf1a12758ac2c03eba92c3ee6680c9d24a08ea82b874e4f7c36f4  main_copy.rs
```

# Why
I came across Ken Thompson's [Reflections on Trusting Trust](https://www.archive.ece.cmu.edu/~ganger/712.fall02/papers/p761-thompson.pdf). In the document, he recommends:

>[You] write a source program that, when compiled and executed, will produce as output an exact copy of its source.

Anyways, the program was amusing to write, and [Reflections on Trusting Trust](https://www.archive.ece.cmu.edu/~ganger/712.fall02/papers/p761-thompson.pdf) is a spooky read that more people should understand.
  