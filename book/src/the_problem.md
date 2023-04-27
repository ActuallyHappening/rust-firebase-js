# The problem
Imagine you want to build a website (and/or integrate a custom backend), using the `firebase` provided by Google.
You have many options:
- Plain JavaScript with the `firebase` npm package / SDK
- Flutter / Dart with `flutterfire` package
<!-- TODO: LInk to flutterfire -->
- JavaScript frameworks, integration the `firebase` JS SDK

However, all of the above solutions use languages that are sub-optimal.

In `dart`, for example, you cannot establish type-safe JavaScript interop. Period.
The interop does exist, but for your own custom purposes it requires you authoring (and probably publishing) a Flutter plugin.
Then, the types are validated at runtime only, since `dart` typing is like that, and compiling with optimizations has the potential for the wrong types to 'leak' through your code base.

'Well,' you might respond, 'what about the other JS solutions? They seem pretty standard, being the only solution for many decades.'
I would resopnd with the argument, 'Can't we do better?'
If you have dealt with `npm` and large interrelated JS projects before, you have often come up against an error like `undefined is not a function`. Tracking this errors is relatively time consuming compared with tracking `Rust` errors, for a few major reasons:
- **Rust is compiled *and* knows everything about your project** (not necessarily guarenteed with other compiled languages)
- **JavaScript errors primarily happen at runtime** (I often encounter SyntaxErrors at runtime, when dynamic programming like `eval` or appending `script` tags to the DOM are involved)
- **Meta programming** is hard to trace errors through in edge cases, which Rust solves quite practically with its implementations of macros.

'Well, how can you use `firebase` in a stable and debuggable manner?'
My answer is: `Rust`
