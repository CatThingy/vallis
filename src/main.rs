use river_layout_v3::{Event, RiverLayoutManagerV3};

use wayland_client::{
    global_filter,
    protocol::wl_output::{self, WlOutput},
    Display, GlobalManager, Main,
};

mod river_layout_v3;

pub struct Globals {
    pub namespace: String,
    pub layout_manager: Option<Main<RiverLayoutManagerV3>>,
}

pub struct Output {
    pub output: Main<WlOutput>,
    pub view_padding: i32,
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
                            let output = Output {
                                output,
                                view_padding: 0,
                            };
                            if let Some(globals) = globals.get::<Globals>() {
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
                                        todo!()
                                    }
                                    Event::UserCommand { command } => {
                                        todo!()
                                    }
                                });
                            }
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
