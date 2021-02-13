[![Crate](https://img.shields.io/crates/v/zwp-virtual-keyboard.svg)](https://crates.io/crates/zwp-virtual-keyboard)
[![dependency status](https://deps.rs/repo/github/grelltrier/zwp-virtual-keyboard/status.svg)](https://deps.rs/repo/github/grelltrier/zwp-virtual-keyboard)

# zwp-virtual-keyboard
Rust code generated with wayland-scanner crate for virtual_keyboard_unstable_v1 protocol. Some parts might not be safe, even though they are not marked ad "unsafe". One example is that you need to send the correct length of the file when sending a keymap. There might be more though.

## Run the example
In order to run the example, your compositor must understand the zwp-virtual-keyboard protocol. Phosh/phoc on the Pinephone or Librem 5 understands it. I usually build the example for my Pinephone and open the editor on it. Then I use scp to copy the executable to the phone and execute it over ssh. You should now see that multiple 'x' characters are entered into the editor.

## Bug with rust-analyser
Rust-analyser complains about unresolved issues, but this is a [bug](https://github.com/rust-analyzer/rust-analyzer/issues/6038). It builds just fine.

## Contributing
PR are always welcome :)