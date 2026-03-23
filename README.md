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
      for item in items { Item(item: item) }
    } else {
      Text { "Loading..." }
    }
  }
}
```