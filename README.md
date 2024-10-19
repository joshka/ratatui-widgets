<!-- cargo-rdme start -->

# Ratatui-widgets

[![Crates.io Badge]][Crate] [![License Badge]](#license) [![Docs.rs Badge]][API Docs]<br>
[![Deps.rs Badge]][Dependencies] [![Codecov.io Badge]][Coverage] [![Discord Badge]][Ratatui
Discord]

`ratatui-widgets` is a Rust crate with extra widgets for [Ratatui].

## Installation

```shell
cargo add ratatui-widgets
```

## Usage

```rust
// TODO: Add usage examples
```

## Example

![Button](https://vhs.charm.sh/vhs-MSE5G9byLklG23xdJwbqR.gif)

[Crates.io Badge]: https://img.shields.io/crates/v/ratatui-widgets?logo=rust&style=for-the-badge
[License Badge]: https://img.shields.io/crates/l/ratatui-widgets?style=for-the-badge
[Docs.rs Badge]: https://img.shields.io/docsrs/ratatui-widgets?logo=rust&style=for-the-badge
[Deps.rs Badge]:
    https://deps.rs/repo/github/joshka/ratatui-widgets/status.svg?style=for-the-badge
[Codecov.io Badge]:
    https://img.shields.io/codecov/c/github/joshka/ratatui-widgets?logo=codecov&style=for-the-badge&token=BAQ8SOKEST
[Discord Badge]:
    https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge

[Crate]: https://crates.io/crates/ratatui-widgets
[API Docs]: https://docs.rs/crate/ratatui-widgets/
[Dependencies]: https://deps.rs/repo/github/joshka/ratatui-widgets
[Coverage]: https://app.codecov.io/gh/joshka/ratatui-widgets
[Ratatui Discord]: https://discord.gg/pMCEU9hNEj

[Ratatui]: https://crates.io/crates/ratatui

<!-- cargo-rdme end -->

## Status

This README sets up an initial goal for a library of widgets to supplement Ratatui. This library
**will release breaking changes regularly** - move fast and break things. A focus on fast delivery
rather than gating on perfection is key here. Expect to see releases anytime a feature is changed or
added. Release-plz will keep the version updated with respect to semver and will update the
[CHANGELOG](./CHANGELOG.md) with information from every PR.

This is not (yet?) an official Ratatui-org project, but may be at some point in the future.

## Features

- Basic event handling abstraction and keyboard / mouse handling
- Basic buttons

  ```rust
  let button = Button::new("Click me");
  ```

- Stack container that handles widgets and layouts

  ```rust
  let stack = StackContainer::horizontal().with_widgets(vec![
      (Box::new(Line::raw("Left")), Constraint::Fill(1)),
      (Box::new(Text::raw("Center")), Constraint::Fill(1)),
      (Box::new(Span::raw("Right")), Constraint::Fill(1)),
  ]);
  ```

## TODO

Most of this list of widgets from
<https://en.wikipedia.org/wiki/Graphical_widget#List_of_common_generic_widgets>

- [ ] Create an abstraction over the backend events to an event type for mouse and keyboard events
  - [x] Crossterm
  - [ ] Termion
  - [ ] Termwiz
  - [ ] Consider how to handle non mouse / keyboard events (resize, paste, focus, etc.)
- [ ] Support keyboard shortcuts / accellerators for buttons / menus etc.
- [ ] Consider supporting keyboard customization
- [ ] Support themes that apply to all the widgets (colors and modifiers)
- [ ] Decide on how to handle state (StatefulWidget vs other options)
- [ ] Decide on how to handle events that occur on click / change etc.
- [ ] Decide how to handle internal state (focus, hover)
- [ ] Decide how to handle focus order / selection
- [ ] Decide how containers work generally
- Buttons
  - [ ] Button `[Submit]`
  - [ ] Radio Button `(*) item ( ) item`
  - [ ] Check Box `[x] item`
  - [X] Toggle Switch `<_ON_ / OFF >`
  - [ ] Toggle Button `[ON]`
  - [ ] Split Button `[Submit][↓]`
  - [ ] Cycle Button `[Red]` => `[Green]` => `[Blue]`
- [ ] Slider `[------------|---------------]`
- [ ] List Box (low priority as there is already one in core)
- [ ] Spinner `[123][+][-]`
- [ ] Drop-down List `[Selected Item][↓]`
- [ ] Menu
  - [ ] Context Menu
  - [ ] Pie Menu (probably not worth it in the console, but perhaps there's uses for something similar)
- [ ] Menu Bar (<https://crates.io/crates/tui-menu> might be worth bringing in)
- [ ] Tool Bar (good for mouse UIs, bad for keyboard)
  - [ ] Ribbon (same)
- [ ] Combo Box `[____________][↓]`
- [ ] Icon (not sure what use we'd see out of this one)
- [ ] Tree View (<https://crates.io/crates/tui-tree-widget>)
- [ ] Tree map (not listed on wiki page, but <https://crates.io/crates/tui-treemap>)
- [ ] Grid View / DataGrid (similar to the built-in table, but we can do much more with this)
- [ ] Link `<go here>` (integrate with OSC 8)
- [ ] Tab (compared to the built-in Tabs, this would be a container widget)
- [ ] ScrollBar (built-in)
- [ ] Text Box (<https://crates.io/crates/tui-textarea> <https://crates.io/crates/tui-input>)
- [ ] Label (mostly the same as just a Paragraph, but links to another field)
- [ ] Tooltip (displayed underneath when field is focused - dim text etc.)
- [ ] Balloon help (similar to tooltip - perhaps this is a larger popup for help that has to be dismissed)
- [ ] Status Bar (Mostly useful for putting a background, and adding multiple elements that are auto
      spaced)
- [ ] Progress Bar (Gauge / Linegauge, but optimised for time / progress)
- [ ] Infobar - like a popup or flash element, but generally non modal, dismissable, top of screen
- Containers
  - [ ] Window (not sure we need this concept, but could be useful as a top-level idea)
  - [ ] Collapsible Panel - VSCode like panels
    - [ ] Drawers - related to panels - unsure where the distinction of this lies
  - [ ] Accordion - Vertically stacked list of items that can be expanded by selection
  - [ ] Modal Window - Popup that contains other elements
  - [ ] Dialog Box - Display info in a popup and wait for a response
        (<https://crates.io/crates/tui-confirm-dialog> <https://crates.io/crates/tui-file-dialog>)
  - [ ] Popup (not in wikipedia, <https://crates.io/crates/tui-popup>)
  - [ ] Palette Window / Utility Window - floating window with commands / buttons
  - [ ] Inspector Window - shows info about the selected item elsewhere (perhaps useful as a
        debugging tool)
  - [ ] Frame - Grouping mechanism - perhaps this is a border with a title and acts as a container
  - [ ] Canvas (built-in perhaps)
  - [ ] Cover flow - large selection usually with images / snapshots horizontal scroll
  - [ ] Bubble flow - discussion thread (example <https://github.com/dustinblackman/oatmeal>)
  - [ ] Carousel - Display Visual Cards - different from cover flow in that it displays multiple
        cards at once
  - [ ] Scrollview - not in the wikipedia article, but <https://crates.io/crates/tui-scrollview>
- [ ] Suggestions for other items welcome!

## License

Copyright (c) 2024 Josh McKinney

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
