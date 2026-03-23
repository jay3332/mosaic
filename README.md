# Mosaic
Modern cross-platform scripting language for creating native GUIs, written in Rust.

## Why Mosaic?

I started this project as a way for me to become more familiar with compilers and low-level GUI rendering.
The goal was to create a UI language with:

- Semantics, a type system, and developer tooling inspired by Rust
- Effortless memory safety inspired by Swift (i.e. reference counted)
- Declarative UI markup inspired by SwiftUI, Jetpack Compose, and Flutter
- Components, state, context, and event system inspired by React
- Declarative rule-based styling inspired by CSS
- First-class async/await and lazy UI rendering
- Easy integration with LLMs

### Features

- Strongly and statically typed, with a Rust-inspired subtyping system using structs, enums, traits, and impls
- UI-first design with first-class components, reactive states, context, events, and component modifiers
- Explicit optionals and results, similar to Rust (essentially no concept of "null")
- Everything is immutable by default
- Rich packaging and module system
- Hot reloading and devtools (inspect element, console)
- Either AoT and JIT compiled through a common IR/debuginfo
- Choose your own allocator and event loop implementations (or use the default one)
    - Support for multithreading and GPU execution for native targets
- Can compile to HTML/CSS/JS for the web as well as native targets (desktop, mobile)
- Versatile everywhere else, so outside of UI you should be able to do anything
    - Rich standard library, supporting HTTP, WebSockets, native IO and system functions, etc. out of the box

## Examples

### Simple Counter w/ Styling

```mosaic
template GlobalStyle {
  set background = .black
  set align = .center

  Button {
    set pad = 4px
    set rounded = .large
  }
    
  set ^dec.background = .red
  set ^inc.background = .green
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
        on click => count += 1
    }
  }
}
```

### HTTP Requests & Lazy Loading

```mosaic
import std.http: get_json

const BASE_URL = "https://myapi.com/api/v1"

@derive(Deserializable)
struct Item(parent_id: uint, name: string, description: string)

async func fetch_items(id: uint) -> List<Item> {
  return await get_json<List<Item>>(BASE_URL + "/items/$id")
}

component ItemEntry {
  property item: Item
  state checked = false

  Row(pad: 4px, gap: 4px) {
    Checkbox()
      bind checked

    Column {
      Text(font: .(size: 1.2em)) { item.name }
      Text(font: .(size: 0.7em)) { item.description }
    }
  }
}

@entry
component ItemsList {
  async computed items: List<Item> = {
    await fetch_items()
  }

  Column(gap: 2px) {
    if let items = items {
      for item in items { ItemEntry(item: item) }
    } else {
      Text { "Loading..." }
    }
  }
}
```