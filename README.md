# mosaic
Modern cross-platform scripting language for creating native GUIs, written in Rust.

## Why Mosaic?

I started this project as a way for me to become more familiar with compilers and low-level GUI rendering.
I wanted to create a UI language with:

- semantics, a type system, and developer tooling inspired by Rust
- effortless memory safety inspired by Swift (i.e. reference counted)
- declarative UI markup inspired by SwiftUI, Jetpack Compose, and Flutter
- components, state, context, and event system inspired by React
- declarative rule-based styling inspired by CSS
- first-class async/await and lazy UI rendering
- easy integration with LLMs

### Features

- strongly and statically typed, with a Rust-inspired subtyping system using structs, enums, traits, and impls
- UI-first design with first-class components, reactive states, context, events, and component modifiers
- explicit optionals and results, similar to Rust (essentially no concept of "null")
- everything is immutable by default
- can compile to HTML/CSS/JS for the web as well as native targets (desktop, mobile)
- versatile everywhere else, so outside of UI you should be able to do anything
    - rich standard library, supporting http, websockets, native io and system functions, etc out of the box

## Example

```mosaic
template GlobalStyle {
    set background = .black
    set align      = .center

    Button {
        set pad     = 4px
        set rounded = .large
    }
    
    set #dec.background = .red
    set #inc.background = .green
}

component App {
    use GlobalStyle
    state count: uint = 0

    Column {
        Text { "Welcome to Mosaic" }
        Row(pad: 4px, gap: 2px) {
            ^dec = Button { "-" }
                on click => count -= 1

            Text { "Count: $count" }

            ^inc = Button { "+" }
                on click => count -= 1
        }
    }
}
```