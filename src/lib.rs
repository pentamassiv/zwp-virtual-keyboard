// Re-exports only the actual code, and then only uses this re-export
// The `generated` module below is just some boilerplate to properly isolate stuff
// and avoid exposing internal details.
pub use generated::client as virtual_keyboard_unstable_v1;

mod generated {
    // The generated code tends to trigger a lot of warnings
    // so we isolate it into a very permissive module
    #![allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables)]
    #![allow(non_upper_case_globals, non_snake_case, unused_imports)]
    pub mod client {
        // These imports are used by the generated code
        use wayland_client::protocol::*;
        use wayland_client::sys;
        use wayland_client::{AnonymousObject, Attached, Main, Proxy, ProxyMap};
        use wayland_commons::map::{Object, ObjectMetadata};
        use wayland_commons::smallvec;
        use wayland_commons::wire::{Argument, ArgumentType, Message, MessageDesc};
        use wayland_commons::{Interface, MessageGroup};
        // Include generated code
        include!(concat!(env!("OUT_DIR"), "/virtual_keyboard_api.rs"));
    }
}
