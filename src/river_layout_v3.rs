// re-exports for easier use
pub use wayland::client::{
    river_layout_manager_v3::RiverLayoutManagerV3,
    river_layout_v3::{Event, RiverLayoutV3},
};

mod wayland {
    // The generated code tends to trigger a lot of warnings
    // so we isolate it into a very permissive module
    #![allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables)]
    #![allow(non_upper_case_globals, non_snake_case, unused_imports, unknown_lints)]
    #![allow(clippy::all)]

    pub mod client {
        use wayland_client;
        use wayland_client::protocol::*;
        pub mod interfaces {
            use wayland_backend;
            use wayland_client::protocol::__interfaces::*;
            wayland_scanner::generate_interfaces!("protocol/river-layout-v3.xml");
        }

        use interfaces::RIVER_LAYOUT_MANAGER_V3_INTERFACE;
        use interfaces::RIVER_LAYOUT_V3_INTERFACE;
        wayland_scanner::generate_client_code!("protocol/river-layout-v3.xml");
    }
}
