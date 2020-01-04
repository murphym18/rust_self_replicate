# What is this?

This program outputs its source code. If you were to run the following:

```bash
cargo run > main_copy.rs
sha256sum src/main.rs main_copy.rs
```

You'd see that `src/main.rs` and `main_copy.rs` both hash to the same string. On my system, `sha256sum` outputs:

```
eb103e614b2169fc6544498da083f7c3a46e4b7c58bb6b9b3442aa4de2d855fe  src/main.rs
eb103e614b2169fc6544498da083f7c3a46e4b7c58bb6b9b3442aa4de2d855fe  main_copy.rs
```

# Why
I came across Ken Thompson's [Reflections on Trusting Trust](https://www.archive.ece.cmu.edu/~ganger/712.fall02/papers/p761-thompson.pdf). In the document, he recommends:

>[You] write a source program that, when compiled and executed, will produce as output an exact copy of its source.

Anyways, the program was amusing to write, and [Reflections on Trusting Trust](https://www.archive.ece.cmu.edu/~ganger/712.fall02/papers/p761-thompson.pdf) is a spooky read that more people should understand.
  