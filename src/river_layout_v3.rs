pub mod client {
    #![allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables)]
    #![allow(non_upper_case_globals, non_snake_case, unused_imports)]
    // These imports are used by the generated code
    pub(crate) use wayland_client::protocol::*;
    pub(crate) use wayland_client::protocol::{wl_region, wl_surface};
    pub(crate) use wayland_client::sys;
    pub(crate) use wayland_client::{AnonymousObject, Attached, Main, Proxy, ProxyMap};
    pub(crate) use wayland_commons::map::{Object, ObjectMetadata};
    pub(crate) use wayland_commons::smallvec;
    pub(crate) use wayland_commons::wire::{Argument, ArgumentType, Message, MessageDesc};
    pub(crate) use wayland_commons::{Interface, MessageGroup};
    include!("../generated/river_layout_v3.rs");
}
