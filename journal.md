I did a first attempt at polyominos that I will archive. I was overwhelmed by the many balls (new to me) I had
to juggle with. My web skills were superficial and
15-20 years old. [Front end Masters](https://frontendmasters.com/) and [fireship](https://fireship.io/) are
helping though.

The result was underwhelming even if I learned a lot. 

Overwhelmed person ==> underwhelming results :)

I am again at it juggling with over more balls starting from a chat with
[ChatGPT 4o](https://chatgpt.com/share/66e6d670-3170-8001-a5df-127b7e2e4500).
I hope they will make work this shareable link which is currently broken.


 app using [ChatGPT 4o](https://openai.com/index/hello-gpt-4o/)
and [shadcn-svelte](https://www.shadcn-svelte.com/).



Note : so far, within and outside vscode I am using
phind.com. I have been unable to use my openai credentials
in chatGPT extensions.

I am trying to follow the instruction of 4o.

<details>
<summary>wasm-pack build --target web</summary>
<code>
wasm-pack build --target web
[INFO]: 🎯  Checking for the Wasm target...
[INFO]: 🌀  Compiling to Wasm...
   Compiling proc-macro2 v1.0.86
   Compiling unicode-ident v1.0.13
   Compiling wasm-bindgen-shared v0.2.93
   Compiling once_cell v1.20.0
   Compiling log v0.4.22
   Compiling bumpalo v3.16.0
   Compiling wasm-bindgen v0.2.93
   Compiling cfg-if v1.0.0
   Compiling quote v1.0.37
   Compiling syn v2.0.77
   Compiling wasm-bindgen-backend v0.2.93
   Compiling wasm-bindgen-macro-support v0.2.93
   Compiling wasm-bindgen-macro v0.2.93
   Compiling polyomino-solver v0.1.0 (/Users/cog/junk/polyomino-app/polyomino-solver)
error[E0277]: the trait bound `(usize, usize): JsObject` is not satisfied
  --> src/lib.rs:53:1
   |
53 | #[wasm_bindgen]
   | ^^^^^^^^^^^^^^^ the trait `JsObject` is not implemented for `(usize, usize)`, which is required by `Vec<(usize, usize)>: FromWasmAbi`
   |
   = help: the trait `FromWasmAbi` is implemented for `Vec<T>`
   = note: required for `(usize, usize)` to implement `VectorFromWasmAbi`
   = note: required for `Vec<(usize, usize)>` to implement `FromWasmAbi`
   = note: this error originates in the attribute macro `wasm_bindgen::prelude::__wasm_bindgen_class_marker` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `(usize, usize): JsObject` is not satisfied
   --> src/lib.rs:53:1
    |
53  | #[wasm_bindgen]
    | ^^^^^^^^^^^^^^^ the trait `JsObject` is not implemented for `(usize, usize)`, which is required by `Vec<(usize, usize)>: wasm_bindgen::describe::WasmDescribe`
    |
    = help: the trait `wasm_bindgen::describe::WasmDescribe` is implemented for `Vec<T>`
    = note: required for `(usize, usize)` to implement `WasmDescribeVector`
    = note: required for `Box<[(usize, usize)]>` to implement `wasm_bindgen::describe::WasmDescribe`
    = note: 1 redundant requirement hidden
    = note: required for `Vec<(usize, usize)>` to implement `wasm_bindgen::describe::WasmDescribe`
note: required by a bound in `ReturnWasmAbi`
   --> /Users/cog/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasm-bindgen-0.2.93/src/convert/traits.rs:237:26
    |
237 | pub trait ReturnWasmAbi: WasmDescribe {
    |                          ^^^^^^^^^^^^ required by this bound in `ReturnWasmAbi`
    = note: this error originates in the attribute macro `wasm_bindgen::prelude::__wasm_bindgen_class_marker` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `(usize, usize): JsObject` is not satisfied
  --> src/lib.rs:53:1
   |
53 | #[wasm_bindgen]
   | ^^^^^^^^^^^^^^^ the trait `JsObject` is not implemented for `(usize, usize)`, which is required by `(usize, usize): VectorFromWasmAbi`
   |
   = help: the following other types implement trait `VectorFromWasmAbi`:
             Board
             JsValue
             Polyomino
             String
             f32
             f64
             i16
             i32
           and 8 others
   = note: required for `(usize, usize)` to implement `VectorFromWasmAbi`
   = note: this error originates in the attribute macro `wasm_bindgen::prelude::__wasm_bindgen_class_marker` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `(usize, usize): JsObject` is not satisfied
  --> src/lib.rs:56:24
   |
56 |     pub fn new(blocks: Vec<(usize, usize)>) -> Polyomino {
   |                        ^^^^^^^^^^^^^^^^^^^ the trait `JsObject` is not implemented for `(usize, usize)`, which is required by `Vec<(usize, usize)>: wasm_bindgen::describe::WasmDescribe`
   |
   = help: the trait `wasm_bindgen::describe::WasmDescribe` is implemented for `Vec<T>`
   = note: required for `(usize, usize)` to implement `WasmDescribeVector`
   = note: required for `Box<[(usize, usize)]>` to implement `wasm_bindgen::describe::WasmDescribe`
   = note: 1 redundant requirement hidden
   = note: required for `Vec<(usize, usize)>` to implement `wasm_bindgen::describe::WasmDescribe`

error[E0600]: cannot apply unary operator `-` to type `usize`
  --> src/lib.rs:61:57
   |
61 |         self.blocks = self.blocks.iter().map(|&(x, y)| (-y as usize, x as usize)).collect();
   |                                                         ^^ cannot apply unary operator `-`
   |
   = note: unsigned values cannot be negated

error[E0277]: the trait bound `(usize, usize): JsObject` is not satisfied
  --> src/lib.rs:53:1
   |
53 | #[wasm_bindgen]
   | ^^^^^^^^^^^^^^^ the trait `JsObject` is not implemented for `(usize, usize)`, which is required by `(usize, usize): VectorIntoWasmAbi`
   |
   = help: the following other types implement trait `VectorIntoWasmAbi`:
             Board
             JsValue
             Polyomino
             String
             f32
             f64
             i16
             i32
           and 8 others
   = note: required for `(usize, usize)` to implement `VectorIntoWasmAbi`
   = note: this error originates in the attribute macro `wasm_bindgen::prelude::__wasm_bindgen_class_marker` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `(usize, usize): JsObject` is not satisfied
  --> src/lib.rs:64:33
   |
64 |     pub fn get_blocks(&self) -> Vec<(usize, usize)> {
   |                                 ^^^^^^^^^^^^^^^^^^^ the trait `JsObject` is not implemented for `(usize, usize)`, which is required by `Vec<(usize, usize)>: wasm_bindgen::describe::WasmDescribe`
   |
   = help: the trait `wasm_bindgen::describe::WasmDescribe` is implemented for `Vec<T>`
   = note: required for `(usize, usize)` to implement `WasmDescribeVector`
   = note: required for `Box<[(usize, usize)]>` to implement `wasm_bindgen::describe::WasmDescribe`
   = note: 1 redundant requirement hidden
   = note: required for `Vec<(usize, usize)>` to implement `wasm_bindgen::describe::WasmDescribe`

Some errors have detailed explanations: E0277, E0600.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `polyomino-solver` (lib) due to 7 previous errors
Error: Compiling your crate to WebAssembly failed
Caused by: Compiling your crate to WebAssembly failed
Caused by: failed to execute `cargo build`: exited with exit status: 101
  full command: cd "/Users/cog/junk/polyomino-app/polyomino-solver" && "cargo" "build" "--lib" "--release" "--target" "wasm32-unknown-unknown"
➜  polyomino-solver git:(master) ✗ 
</code>
</details>