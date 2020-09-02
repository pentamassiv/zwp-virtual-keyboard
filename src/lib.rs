// The generated code will import stuff from wayland_commons
extern crate wayland_client;
extern crate wayland_commons;

// Re-export only the actual code, and then only use this re-export
// The `generated` module below is just some boilerplate to properly isolate stuff
// and avoid exposing internal details.
//
// You can use all the types from my_protocol as if they went from `wayland_client::protocol`.
pub use generated::client as virtual_keyboard_unstable_v1;

//let mut file = File::create("foo.txt")?;
//let raw_fd= file.as_raw_fd();

mod generated {
    // The generated code tends to trigger a lot of warnings
    // so we isolate it into a very permissive module
    #![allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables)]
    #![allow(non_upper_case_globals, non_snake_case, unused_imports)]

    pub mod client {
        // These imports are used by the generated code
        pub(crate) use wayland_client::protocol::*;
        pub(crate) use wayland_client::sys;
        pub(crate) use wayland_client::{AnonymousObject, Attached, Main, Proxy, ProxyMap};
        pub(crate) use wayland_commons::map::{Object, ObjectMetadata};
        pub(crate) use wayland_commons::smallvec;
        pub(crate) use wayland_commons::wire::{Argument, ArgumentType, Message, MessageDesc};
        pub(crate) use wayland_commons::{Interface, MessageGroup};
        // If you protocol interacts with objects from other protocols, you'll need to import
        // their modules, like so:
        pub(crate) use wayland_client::protocol::{wl_region, wl_surface};
        include!(concat!(env!("OUT_DIR"), "/virtual_keyboard_api.rs"));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
