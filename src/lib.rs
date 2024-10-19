//! # Ratatui-widgets
//!
//! Please note: this crate is deprecated so that we can use the name for an internal Ratatui widget
//! crate. Please use [tui-framework-experiment] instead.
//!
//! [tui-framework-experiment]: https://crates.io/crates/tui-framework-experiment
//!
//! [![Crates.io Badge]][Crate] [![License Badge]](#license) [![Docs.rs Badge]][API Docs]<br>
//! [![Deps.rs Badge]][Dependencies] [![Codecov.io Badge]][Coverage] [![Discord Badge]][Ratatui
//! Discord]
//!
//! `ratatui-widgets` is a Rust crate with extra widgets for [Ratatui].
//!
//! ## Installation
//!
//! ```shell
//! cargo add ratatui-widgets
//! ```
//!
//! ## Usage
//!
//! ```rust
//! // TODO: Add usage examples
//! ```
//!
//! ## Example
//!
//! ![Button](https://vhs.charm.sh/vhs-MSE5G9byLklG23xdJwbqR.gif)
//!
//! [Crates.io Badge]: https://img.shields.io/crates/v/ratatui-widgets?logo=rust&style=for-the-badge
//! [License Badge]: https://img.shields.io/crates/l/ratatui-widgets?style=for-the-badge
//! [Docs.rs Badge]: https://img.shields.io/docsrs/ratatui-widgets?logo=rust&style=for-the-badge
//! [Deps.rs Badge]:
//!     https://deps.rs/repo/github/joshka/ratatui-widgets/status.svg?style=for-the-badge
//! [Codecov.io Badge]:
//!     https://img.shields.io/codecov/c/github/joshka/ratatui-widgets?logo=codecov&style=for-the-badge&token=BAQ8SOKEST
//! [Discord Badge]:
//!     https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge
//!
//! [Crate]: https://crates.io/crates/ratatui-widgets
//! [API Docs]: https://docs.rs/crate/ratatui-widgets/
//! [Dependencies]: https://deps.rs/repo/github/joshka/ratatui-widgets
//! [Coverage]: https://app.codecov.io/gh/joshka/ratatui-widgets
//! [Ratatui Discord]: https://discord.gg/pMCEU9hNEj
//!
//! [Ratatui]: https://crates.io/crates/ratatui

#[deprecated(note = "Use tui-framework-experiment instead")]
pub mod button;
#[deprecated(note = "Use tui-framework-experiment instead")]
pub mod events;
#[deprecated(note = "Use tui-framework-experiment instead")]
pub mod stack_container;
#[deprecated(note = "Use tui-framework-experiment instead")]
pub mod toggle_switch;

#[deprecated(note = "Use tui-framework-experiment instead")]
pub use button::{Button, State as ButtonState, Theme as ButtonTheme};
#[deprecated(note = "Use tui-framework-experiment instead")]
pub use stack_container::StackContainer;
