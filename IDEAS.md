# Grammar Ideas

So, here're various approaches for the grammar of an abstract shell language.

## Emoji Meaning

Emojis are used to highlight the good and bad sides of a decision.

* ⚡ Simplicity/Conceptuality

* 📃 Code Conventions
* ⛔️ Limitations
* ⚠️ Possible Problem
* 🔱 Unclear
* 🔰 // saving for later

## _Just a Command_ Syntax

Let's define a _command_ in a way suitable for most of the shells, so that it can be applied to all the following:

```
echo Hi there!
g++ --std=c++17 test.c -o out
build/runMyApp +feature1 -feature2
./someUtil -i input.mp4 c:a=4,b=1 -v something print=yes output.mp4
/bin/something doSomething
```

 If the first _item_ contains `/` - it's a path. Command arguments _may_ contain `+`/`-`/`=`/`:` or some other symbols (it's just how it is, and we must deal with it).

## Minimal

* ⚡ There's only one possible grammar structure - a _command_.
* ⚡ Closures `{}` are primitive language items.

```
if true {
	echo This is true
} else {
	echo This is false
}
```

This is just a single command `if`. The first argument is the string `true`, the second is a closure `{ ... }`. If the third argument is present and is the string `else`, then we expect one more closure for the else branch or one more `if`.

* Closure as the first argument is executed immediately.

```
echo This is some test
{
	echo Testing...
}
```

* ⚡ Quotes `""` only preserve spaces, there's no difference between `"test"` and `test` (both are _just_ strings).

### Results in

* ⚡ `\n` __always__ means the end of the command.
* 📃 Closure-argument `{}` __must__ be written on the same line, otherwise it's no longer an argument for the command.
* 📃 Braces are __required__ for all code blocks.

* ⛔️ Can't extend the grammar with a simple syntax for complex constructs.

What if we'd like to introduce functions with named parameters and their default values? A possible syntax is:

```
function give(name, object = "pen") {
	echo Hi, $name, here is your $object!
}

give Nick object=pencil
```

First of all, this syntax violates the rule of a single _command-like_ grammar.

Second, passing named parameters is ambiguous for the parser. What if the user made a mistake and typed `objec=pencil`? Is the whole thing a single string? Should we always cut away the `\w+=` part? How to pass precisely `objec=pencil` to the function without naming the parameter?

Third, can we specify the default values as `object = pen` without quotes? Or maybe if we type `object = pen`, then `object` must be assigned the value of a command `pen`?

### Allowing Complex Grammar (Partially)

* ~~⚡ There's only one possible grammar structure - a _command_.~~
* If the first token is a keyword (= built-in command), the following grammar may be _anything_. Otherwise, it's _just a command_.

### Results in

* 🔱 There must be a way to embed complex syntax into _commands_ (e. g. substitute variables).
* ⚠️ Complex syntax may require complex tokenization, and not performing it for _commands_ means making the tokenizer more difficult (it must properly switch the context). Not using a tokenizer does allow contextual dependency, but implementing grammar like Kotlin closures `{ param1, param2 -> ... }` becomes problematic (we can't know if we're going to parse the first identifier of the `ident[, ident]*` sequence or an expression).

### Allowing Complex Grammar (Fully)

- ~~If the first token is a keyword (= built-in command), the following grammar may be _anything_. Otherwise, it's _just a command_.~~

* There is a tokenizer, and tokenization is context-independent.

### Results in

* 🔱 Either _commands_ __must__ apply special rules to things like `+feature`  ("sequence `OPERATOR,STRING` is transformed into another `STRING`") or the tokenizer can't split entities like `+feature` into separate tokens.

The first approach assumes we introduce a `WHITESPACE` token to distinguish `+ feature` from `+feature`, and the second one implies `+feature` being a `STRING`, but a stand-alone `+` being an `OPERATOR`.

How do we detect math operations? We __must__ support arguments like `+feature`, and a hypothetical command might accept `+5` (`OPERATOR,NUMBER`) (although, it seems unlikely, but treating this case differently from `OPERATOR,STRING` may result in "visual ambiguity").

Let's go with the first approach because it allows us to type `5+6` instead of `5 + 6` if we _just want to do some calculations_. The second way is better in terms of 📃 code conventions, but I find it ok for a shell to not punish the user for such a small thing as a space between numbers.

By the way, we could introduce an option like `--strict` that would force the second-approach style (or `--lax` for going the other way around).

### Tokenization with Transformations

* The input is split into tokens like `OPERATOR`, `NUMBER`, `STRING`, `WHITESPACE` ...
* Transformation: `[NOT STRING],STRING` => `STRING`.
* Transformation: `STRING,[NOT STRING]` => `STRING`.
* Instead of parsing a line as a _command_, it's parsed as an _expression_, where `OPERATOR`'s denote some operations.
* _Command_ arguments have _higher precision_ than binary operations.
* Parentheses `()` substitute another command.

```
echo test +fest
=> ['echo', 'test', '+fest']

echo 4 +3
=> +(['echo', 4], 3)

echo (4 +3)
=> ['echo', +(4, 3)]

assignAge Nick (20 + 1) tomorrow
=> ['assignAge', 'Nick', +(20, 1), 'tomorrow']

assignAge Nick 20 + 1 tomorrow
=> +(['assignAge', 'Nick', 20], [1, 'tomorrow'])
```

### Results in

* 📃 Complex arguments require parentheses `()`.

## Adding Variables and Functions

* ⚡ Everything is an object with some properties.
* ⚡ Variables are the properties of the current scope object (or its parents).
* ⚡ Accessing undefined variables throws an error.
* ⚡ Accessing object properties via the `.` operator.
* ⚡ Property is a command. Always. Even for simple variables.

```
echo Hello, (name)!
```

* ⚡ Checking if a property exists is done via `?`.

```
if (name?) {
	echo Hello, (name)!
} else {
	echo Hi!
}
```

* Closures may be called with implicit arguments or explicit ones, and the latter ones are declared as follows:

```
greet = { name ->
	echo Hello, (name)!
}
```

* If no value supplied to the explicit parameter on the caller site, the variable is undefined.
* Closures may return their value either via `return`, or if the value happens to be the last one received during the closure execution.
* 📃 `echo`, `read` and `warn` __always__ work with the global script _stdin_, _stdout_ and _stderr_.
* 📃 `echoPipe`, `readPipe` and `warnPipe` may work with local _stdin_, _stdout_ and _stderr_ (if a closure is a step of a pipeline).

### Results in

* The same behavior can be achieved in 3 distinct ways:

```
count = 100
count = {
	return count
}
count = { count }
```

* The only purpose of `return` is to escape the closure earlier
* ⛔️ Hard to pass lambdas with many arguments

```
apply = { name, act ->
	act (name)
}
greet = { name ->
	echo Hello, (name)!
}
apply Nick { name -> greet (name) }
```

We need a way to get a reference to the value that a property holds, which is the underlying closure:

```
apply Nick (get greet)
```

If `(get variable)` returns the value of the variable for non-closure variables, it's always safer to do `(get variable)` instead of `(variable)`. Bad for code conventions.

If it instead returns a closure that promises to return a value (a _reference_ to the value?), then passing variables through should be done via `(get variable)` and accessing their values at the end of the chain should be done via `(variable)`. Not the most elegant solution... or is it?

The need to use `(variable)` syntax to get the value means we can easily pass a closure as a parameter, even if a simple value was expected:

```
apply { getTarget } (get greet)
```

This way we don't need to refactor the code that accepts our parameters in case we suddenly realize, we'd like to have a one more level of indirection here.

* ⛔️ Hard to assign strings to variables

```
name = text Nick

# name = Nick
# => Command `Nick` not found
```

- ⚠️ Lifetime of a variable captured by a closure

```
doSomething = {
	a = 10
	getA = { b -> add a b }
	$getA
}

(doSomething) 20
```

Should `a` be deleted as soon as the `doSomething`'s scope was left? Should it now contain some `None` value or this should be handled by a garbage collector (which may be unaffordable for a shell).

- ⚠️ Comparison operators vs redirects

It's unclear, what `<`/`>`/... should mean.

```
if (getAgeOf Nick > 20) {
	echo Oh...
}
```

Should `> 20` mean a redirection to the 20th descriptor or we should wait until the left command returns it's value, and analyze it then? Or maybe something else?

### Replacing `?` with the more general _big money_ operator `$`

* ~~⚡ Checking if a property exists is done via `?`.~~
* ⚡ Checking if a property exists is done via `$`.

```
if ($name) {
	echo Hello, (name)!
} else {
	echo Hi!
}
```

```
apply = { name, act ->
	act $name
}
greet = { name ->
	echo Hello, (name)!
}
apply Nick $greet
```

Attempt to do:

`````
echo My name is $name
`````

 Should result in something like:

```
My name is Getter{name}
```

This would ensure everyone uses the same notation and every piece of code supports 'getter substitution'.

### Results in

* 📃 Using getters is the default code convention
* 📃 Most of the times we'll see either `myFun $param1 $param2` or `echo Path is /my/(result)/path`
* ⛔️ Hard to assign strings to variables

```
name = text Nick

# name = Nick
# => Command `Nick` not found
```

- ⚠️ Lifetime of a variable captured by a closure

```
doSomething = {
	a = 10
	getA = { b -> add a b }
	$getA
}

(doSomething) 20
```

Should `a` be deleted as soon as the `doSomething`'s scope was left? Should it now contain some `None` value or this should be handled by a garbage collector (which may be unaffordable for a shell).

- ⚠️ Comparison operators vs redirects

It's unclear, what `<`/`>`/... should mean.

```
if (getAgeOf Nick > 20) {
	echo Oh...
}
```

Should `> 20` mean a redirection to the 20th descriptor or we should wait until the left command returns it's value, and analyze it then? Or maybe something else?