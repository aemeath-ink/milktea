# milktea

A small typed lambda calculus interpreter, in Rust. Mostly a teaching project for myself — I wanted to read someone else's STLC implementation and ended up writing my own, because that's how I read.

```
$ milktea
λ> let id = \x:Int. x in id 42
42 : Int

λ> let twice = \f:Int->Int. \x:Int. f (f x) in twice (\n:Int. n + 1) 7
9 : Int

λ> \x:Bool. if x then 1 else "two"
type error: branches of `if` have different types: Int vs String
   at <repl>:1:25
```

## What's in it

- Simply typed lambda calculus with `Int`, `Bool`, `String`, and arrow types
- `let`, `if`-`then`-`else`, integer arithmetic, equality
- Hindley–Milner-style type *inference* is **not** here yet — annotations are required
- A pretty REPL that does syntax highlighting and shows inferred types
- ~700 lines of Rust, no parser-generator dependency (handwritten Pratt-ish parser)

## What's not in it (and probably won't be soon)

- Type inference. Doing it right takes a lot more code than the rest of the language combined, and I want to keep `milktea` readable.
- Polymorphism. See above.
- Effects, mutability, IO. This is a calculus, not a programming language.
- Pattern matching. Maybe later, on a branch.

## Why "milktea"

I drink a lot of cold milk tea while editing. Most of it is not finished by the time I move to the next thing. The interpreter is named for the cup beside the keyboard.

## Building

```bash
cargo build --release
./target/release/milktea
```

Or run the test suite:

```bash
cargo test
```

## Reading

If you're learning this material yourself I'd recommend, in order:

1. Pierce, *Types and Programming Languages*, ch. 9 (the canonical reference)
2. Harper, *Practical Foundations for Programming Languages*, ch. 4–8 (more compact, more austere)
3. The "Write You a Haskell" tutorial by Stephen Diehl (free online)
4. Then maybe read this code

This implementation is closest in spirit to Pierce — including some variable-name choices.

## License

MIT.
