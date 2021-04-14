# Cash

Yeah, one more shell... I was wondering if I could make a flexible shell with minimalistic syntax, and what problems I would face if I try. So, that's it.

Before Rust, I'd been using C++ for Cash, and the old source code can be found here: [Legacy Cash](https://github.com/lunakoly/CashLegacy)

## Project Structure

There're several crates:

* `helpers` - Common pieces of Rust code I use all the time. For instance, due to the lack of Kotlin-like Elvis operator `?:`, I made my own `elvis!` macro and shortcuts for `Option` and `Result`.
* `degen` - Helps generating source code. Contains some handy `render*()` functions for rendering various Rust (and possibly non-Rust) code structures (these are simple template-processing functions with some sort of indent management). Code generation is necessary for the Visitor pattern and some other things.
* `building` - This contains code that parses some input JSON's of my internal format and generates resulting Rust code. This module uses `degen` for source code generation.
* `parsing` - This defines the `Stream` abstraction - a simple API for working with data that flows sequentially, and contains a bunch of typical implementations. Also contains a TDLtR parser implementation that can handle left-recursion.
* `frontend` - Contains everything related lexical analysis and parsing. It's `build.rs` uses `building`. Parser input data structures (`Rule`'s) are generated from `grammar.json`, and AST nodes are defined via `ast.json`.
* `processing` - Functions for running processes in a platform-independent way.
* `backend` - Contains code relevant to the actual command execution, defines `Value`'s and such things.
* `terminals` - Module that implements a custom user input management (because I wanted to see how I could implement such a thing manually, in a platform-dependent way).

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
