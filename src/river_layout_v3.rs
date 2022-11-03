// re-exports for easier use
pub use wayland::client::{
    river_layout_manager_v3::RiverLayoutManagerV3,
    river_layout_v3::{Event, RiverLayoutV3},
};

pub mod wayland {
    // The generated code tends to trigger a lot of warnings
    // so we isolate it into a very permissive module
    #![allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables)]
    #![allow(non_upper_case_globals, non_snake_case, unused_imports, unknown_lints)]
    #![allow(clippy::all)]

    pub mod client {
        // These imports are used by the generated code
        pub(crate) use wayland_client::protocol::wl_output;
        pub(crate) use wayland_client::sys;
        pub(crate) use wayland_client::{AnonymousObject, Main, Proxy, ProxyMap};
        pub(crate) use wayland_commons::map::{Object, ObjectMetadata};
        pub(crate) use wayland_commons::smallvec;
        pub(crate) use wayland_commons::wire::{Argument, ArgumentType, Message, MessageDesc};
        pub(crate) use wayland_commons::{Interface, MessageGroup};

        include!("../generated/river_layout_v3.rs");
    }
}
