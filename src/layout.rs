use crate::river_layout_v3::RiverLayoutV3;

pub struct LayoutOptions {
    pub main_ratio: f32,
    pub gap: u32,
    pub layout: fn(&mut Vec<View>, f32, u32, u32, u32, u32) -> (),
}

impl Default for LayoutOptions {
    fn default() -> Self {
        LayoutOptions {
            main_ratio: 0.6,
            gap: 4,
            layout: standard_tile,
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

pub fn standard_tile(
    views: &mut Vec<View>,
    main_ratio: f32,
    gap: u32,
    view_count: u32,
    usable_width: u32,
    usable_height: u32,
) {
    let secondary_count = view_count.saturating_sub(1);

    if secondary_count == 0 {
        views.push(View {
            x: 0,
            y: 0,
            width: usable_width,
            height: usable_height,
        });
        return;
    }

    let mut primary_view = View {
        x: 0,
        y: 0,
        width: usable_width,
        height: usable_height,
    };

    primary_view.width = (primary_view.width as f32 * main_ratio).ceil() as u32;
    primary_view.width = primary_view.width.saturating_sub(gap as u32);

    let secondary_width = usable_width - primary_view.width - gap;
    let secondary_offset = (primary_view.width + gap) as i32;
    let secondary_height = usable_height / secondary_count;

    views.push(primary_view);

    for i in 0..(secondary_count - 1) {
        views.push(View {
            x: secondary_offset,
            y: (secondary_height * i) as i32 + if i != 0 { (gap / 2 * i) as i32 } else { 0 },
            width: secondary_width,
            height: secondary_height - gap / 2,
        });
    }

    let final_y = (secondary_height + gap / 2) * (secondary_count - 1);

    views.push(View {
        x: secondary_offset,
        y: final_y as i32,
        width: secondary_width,
        height: usable_height - ((secondary_height + gap / 2) * (secondary_count - 1)),
    });
}
