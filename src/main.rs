mod commands;
mod layout;
mod river_layout_v3;

use wayland_client::{
    global_filter,
    protocol::wl_output::{self, WlOutput},
    Display, GlobalManager, Main,
};

use river_layout_v3::{Event, RiverLayoutManagerV3, RiverLayoutV3};

pub struct Globals {
    pub namespace: String,
    pub layout_manager: Option<Main<RiverLayoutManagerV3>>,
}

pub struct Output {
    pub output: Main<WlOutput>,
    pub current_tag: u32,
    pub options: [LayoutOptions; 33],
}

pub struct LayoutOptions {
    pub primary_ratio: f32,
    pub gap: u32,
    pub layout: fn(&mut Vec<View>, f32, u32, u32, u32, u32) -> (),
}

impl Default for LayoutOptions {
    fn default() -> Self {
        LayoutOptions {
            primary_ratio: 0.6,
            gap: 4,
            layout: layout::standard_tile,
        }
    }
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

fn main() {
    let display = Display::connect_to_env().unwrap();
    let mut event_queue = display.create_event_queue();

    let attached_display = (*display).clone().attach(event_queue.token());

    let mut globals = Globals {
        namespace: "vallis".to_owned(),
        layout_manager: None,
    };

    GlobalManager::new_with_cb(
        &attached_display,
        global_filter!(
            [
                RiverLayoutManagerV3,
                1,
                |layout_manager: Main<RiverLayoutManagerV3>, mut globals: DispatchData| {
                    globals.get::<Globals>().unwrap().layout_manager = Some(layout_manager);
                }
            ],
            [
                WlOutput,
                3,
                |output: Main<WlOutput>, _globals: DispatchData| {
                    output.quick_assign(move |output, event, mut globals| match event {
                        wl_output::Event::Done => {
                            let mut output = Output {
                                output,
                                current_tag: 0,
                                options: std::array::from_fn(|_| LayoutOptions::default()),
                            };
                            let globals = globals.get::<Globals>().unwrap();
                            let layout = globals
                                .layout_manager
                                .as_ref()
                                .expect("Compositor doesn't implement river_layout_v3")
                                .get_layout(&output.output, globals.namespace.clone());
                            layout.quick_assign(move |layout, event, _| match event {
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
                                    output.current_tag = tags;
                                    let options = if tags.count_ones() > 1 {
                                        &output.options[32]
                                    } else {
                                        &output.options[tags.trailing_zeros() as usize]
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
                                    commands::parse(command, &mut output);
                                }
                            });
                        }
                        _ => {}
                    });
                }
            ]
        ),
    );

    loop {
        event_queue
            .dispatch(&mut globals, |event, object, _| {
                panic!(
                    "orphan event: {}@{}: {}",
                    event.interface,
                    object.as_ref().id(),
                    event.name
                );
            })
            .unwrap();
    }
}
