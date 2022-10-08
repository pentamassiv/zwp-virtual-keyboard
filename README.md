# DEPRECATED: When I created this crate, the protocol was not available in the wayland_protocols_misc crate. Since v0.30.0-beta.10 it is available so there is no reason for this crate to continue to exits. Please use the wayland_protocols_misc crate instead.


[![Crate](https://img.shields.io/crates/v/zwp-virtual-keyboard.svg)](https://crates.io/crates/zwp-virtual-keyboard)
[![dependency status](https://deps.rs/repo/github/grelltrier/zwp-virtual-keyboard/status.svg)](https://deps.rs/repo/github/grelltrier/zwp-virtual-keyboard)
[![docs.rs](https://docs.rs/zwp-virtual-keyboard/badge.svg)](https://docs.rs/zwp-virtual-keyboard)
![Build](https://github.com/grelltrier/zwp-virtual-keyboard/workflows/Build/badge.svg)
![dependabot status](https://img.shields.io/badge/dependabot-enabled-025e8c?logo=Dependabot)

# zwp-virtual-keyboard
Rust code generated with wayland-scanner crate for virtual_keyboard_unstable_v1 protocol. Some parts might not be safe, even though they are not marked ad "unsafe". One example is that you need to send the correct length of the file when sending a keymap. There might be more though.

## Run the example
In order to run the example, your compositor must understand the zwp-virtual-keyboard protocol. Phosh/phoc on the Pinephone or Librem 5 understands it. I usually build the example for my Pinephone and open the editor on it. Then I use scp to copy the executable to the phone and execute it over ssh. You should now see that multiple 'x' characters are entered into the editor.

## Bug with rust-analyser
Rust-analyser complains about unresolved issues, but this is a [bug](https://github.com/rust-analyzer/rust-analyzer/issues/6038). It builds just fine.

## Contributing
PR are always welcome :)
