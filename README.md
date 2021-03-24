# Some Random Rust Code

Well, in fact, not _that_ random, it's a reincarnation of [Cash](https://github.com/lunakoly/Cash)...

## Project Structure

There're several crates:

* `degen` - Helps generating source code. Contains some handy `render*()` functions for rendering various Rust (and possibly non-Rust) code structures (these are simple template-processing functions with some sort of indent management). Code generation is necessary for the Visitor pattern and some other things.
* `building` - This contains code that parses some input JSON's of my internal format and generates resulting Rust code. This module uses `degen` for source code generation.
* `parsing` - This defines the `Stream` abstraction - a simple API for working with data that flows sequentially. Also contains a bunch of typical implementations.
* `cash` - Contains everything related to Cash. It's `build.rs` uses `building`.
* `cherry` - Oh, think of it as an example of a parser with no tokenizer. Just some obsolete code, I doubt it even works with the existing infrastructure now. Uses `building`.

## Cash Parser

Initially I was going to use a parser without a tokenizer (hello, `cherry`), but later decided that parsing `{ a, b -> command }` without a tokenizer would be a bit hard, so I implemented a top-down left-to-right parser that can handle left recursion.

For example, here's a piece of grammar:

```json
"plus": {
    "@plus + @times": "handle_binary",
    "@plus - @times": "handle_binary",
    "@times": "handle_pass"
},
```

Instead of _just_ trying to apply a single rule from `plus` to the token stream, we are _trying to apply the rule as many times as we can_. For simple rules (non-left-recurrent) it makes no difference - after applying the first time, we "have a value of type `@plus` on top of our hypothetical token stack", and being a non-left-recurrent means, it's first item is not `@plus`. But we can then in a loop attempt to apply left-recurrent rules as much times as we'd like :)

