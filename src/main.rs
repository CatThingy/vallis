mod commands;
mod layout;
mod river_layout_v3;

const WL_OUTPUT_VERSION: u32 = 4;
const RIVER_LAYOUT_MANAGER_V3_VERSION: u32 = 2;

use river_layout_v3::{Event, RiverLayoutManagerV3, RiverLayoutV3};
use wayland_client::{
    self,
    protocol::{
        wl_output::{self, WlOutput},
        wl_registry::{self, WlRegistry},
    },
    Connection, Dispatch, Proxy,
};

pub struct GlobalManager;

pub struct GlobalInit {
    pub output: Option<WlOutput>,
    pub layout_manager: Option<RiverLayoutManagerV3>,
}

pub struct State {
    pub output: WlOutput,
    pub layout_manager: RiverLayoutManagerV3,
    pub namespace: String,
    pub current_tag: u32,
    pub options: [LayoutOptions; 33],
}

impl Dispatch<WlRegistry, wayland_client::QueueHandle<State>> for GlobalInit {
    fn event(
        globals: &mut Self,
        registry: &WlRegistry,
        event: <WlRegistry as Proxy>::Event,
        data: &wayland_client::QueueHandle<State>,
        _: &Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name, interface, ..
        } = event
        {
            match interface.as_str() {
                "wl_output" => {
                    globals.output =
                        Some(registry.bind::<WlOutput, _, _>(name, WL_OUTPUT_VERSION, data, ()));
                }
                "river_layout_manager_v3" => {
                    globals.layout_manager = Some(registry.bind::<RiverLayoutManagerV3, _, _>(
                        name,
                        RIVER_LAYOUT_MANAGER_V3_VERSION,
                        data,
                        (),
                    ));
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<WlOutput, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &WlOutput,
        event: <WlOutput as Proxy>::Event,
        _: &(),
        _: &Connection,
        qh: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            wl_output::Event::Done => {
                state
                    .layout_manager
                    .get_layout(proxy, "vallis".to_string(), qh, ());
            }
            _ => {}
        }
    }
}

impl Dispatch<RiverLayoutManagerV3, ()> for State {
    fn event(
        _: &mut State,
        _: &RiverLayoutManagerV3,
        _: <RiverLayoutManagerV3 as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &wayland_client::QueueHandle<State>,
    ) {
        unreachable!();
    }
}

impl Dispatch<RiverLayoutV3, ()> for State {
    fn event(
        state: &mut Self,
        layout: &RiverLayoutV3,
        event: <RiverLayoutV3 as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            Event::NamespaceInUse => {
                layout.destroy();
                panic!("Namespace already in use");
            }
            Event::LayoutDemand {
                view_count,
                usable_width,
                usable_height,
                tags,
                serial,
            } => {
                let options = if tags.count_ones() > 1 {
                    &state.options[32]
                } else {
                    &state.options[tags.trailing_zeros() as usize]
                };
                let mut views = Vec::with_capacity(view_count as usize);
                (options.layout)(
                    &mut views,
                    options.primary_ratio,
                    options.gap,
                    view_count,
                    usable_width,
                    usable_height,
                );
                for view in views {
                    view.send(&layout, serial);
                }

                layout.commit("vallis".to_owned(), serial);
            }
            Event::UserCommand { command } => {
                commands::parse(command, state);
            }
            Event::UserCommandTags { tags } => {
                state.current_tag = tags;
            }
        }
    }
}

pub struct LayoutOptions {
    pub primary_ratio: f32,
    pub gap: u32,
    pub layout: fn(&mut Vec<View>, f32, u32, u32, u32, u32) -> (),
}

pub struct View {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl View {
    pub fn send(self, layout: &RiverLayoutV3, serial: u32) {
        layout.push_view_dimensions(self.x, self.y, self.width, self.height, serial);
    }
}

fn help() -> ! {
    println!("Usage: vallis [options]\n");
    println!("  --gap [num]\n\tSets the default gaps between views in pixels (default 6)\n");
    println!(
        "  --primary_ratio [num]\n\tSets the default ratio of the primary view (default 0.6)\n"
    );
    println!("  --layout [tile|stack|mono_stack]\n\tSets the default layout (default 'tile')\n");

    std::process::exit(0)
}

fn main() {
    let mut args = std::env::args().skip(1);

    let mut default_gaps = 6;
    let mut default_ratio = 0.6;
    // compiler can't properly infer the right type here
    let mut default_layout: fn(&mut Vec<View>, f32, u32, u32, u32, u32) = layout::standard_tile;

    loop {
        match args.next() {
            Some(arg) => match arg.as_str() {
                "--gap" => {
                    if let Some(Ok(gap)) = args.next().map(|v| v.parse::<u32>()) {
                        default_gaps = gap.clamp(0, 100);
                    } else {
                        help();
                    }
                }
                "--primary_ratio" => {
                    if let Some(Ok(ratio)) = args.next().map(|v| v.parse::<f32>()) {
                        default_ratio = ratio.clamp(0.1, 0.9);
                    } else {
                        help();
                    }
                }

                "--layout" => {
                    if let Some(arg) = args.next() {
                        default_layout = match arg.as_str() {
                            "tile" => layout::standard_tile,
                            "stack" => layout::stack,
                            "mono_stack" => layout::mono_stack,
                            _ => help(),
                        }
                    } else {
                        help();
                    }
                }
                v => {
                    dbg!(v);
                    help();
                }
            },
            None => break,
        }
    }

    let default_option = move |_: usize| LayoutOptions {
        gap: default_gaps,
        primary_ratio: default_ratio,
        layout: default_layout,
    };

    let connection = Connection::connect_to_env().unwrap();
    let display = connection.display();
    let mut globals_event_queue = connection.new_event_queue();
    let globals_queue_handle = globals_event_queue.handle();

    let mut layout_event_queue = connection.new_event_queue();
    let layout_queue_handle = layout_event_queue.handle();

    let mut globals = GlobalInit {
        output: None,
        layout_manager: None,
    };
    display.get_registry(&globals_queue_handle, layout_queue_handle.clone());

    globals_event_queue.roundtrip(&mut globals).unwrap();

    drop(globals_queue_handle);
    drop(globals_event_queue);

    let mut state = State {
        output: globals.output.unwrap(),
        layout_manager: globals.layout_manager.unwrap(),
        namespace: "vallis".to_owned(),
        current_tag: 0,
        options: std::array::from_fn(default_option),
    };

    loop {
        match layout_event_queue.blocking_dispatch(&mut state) {
            Ok(_) => {}
            Err(e) => {
                panic!("{e:?}")
            }
        }
    }
}
