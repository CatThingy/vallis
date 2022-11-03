use crate::river_layout_v3::RiverLayoutV3;

pub struct LayoutOptions {
    pub main_ratio: f32,
}

struct View {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl View {
    fn split_off_right(&mut self, ratio: f32) -> View {
        assert!(ratio > 0.0 && ratio < 1.0);

        let old_width = self.width;
        self.width = (self.width as f32 * ratio).ceil() as u32;

        View {
            x: self.x + self.width as i32,
            y: self.y,
            width: old_width - self.width,
            height: self.height,
        }
    }

    fn split_vertical(self, count: u32) -> Vec<View> {
        let height = self.height / count;
        let mut out = Vec::with_capacity(count as usize);
        for i in 0..count {
            out.push(View {
                x: self.x,
                y: self.y + (height * i as u32) as i32,
                width: self.width,
                height,
            })
        }
        out
    }

    fn send(self, layout: &RiverLayoutV3, serial: u32) {
        layout.push_view_dimensions(self.x, self.y, self.width, self.height, serial);
    }
}

pub fn handle_layout_demand(
    layout: &RiverLayoutV3,
    view_count: u32,
    usable_width: u32,
    usable_height: u32,
    tags: u32,
    serial: u32,
) {
    let secondary_count = view_count.saturating_sub(1);

    if secondary_count == 0 {
        layout.push_view_dimensions(0, 0, usable_width, usable_height, serial);
        layout.commit("vallis".into(), serial);
        return;
    }

    let mut primary_view = View {
        x: 0,
        y: 0,
        width: usable_width,
        height: usable_height,
    };

    let secondary_views = primary_view
        .split_off_right(0.7)
        .split_vertical(secondary_count);

    primary_view.send(layout, serial);
    for view in secondary_views {
        view.send(layout, serial);
    }

    layout.commit("vallis".into(), serial);
}
