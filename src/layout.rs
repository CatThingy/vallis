use crate::View;

pub fn standard_tile(
    views: &mut Vec<View>,
    primary_ratio: f32,
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

    primary_view.width = (primary_view.width as f32 * primary_ratio).ceil() as u32;
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

pub fn stack(
    views: &mut Vec<View>,
    primary_ratio: f32,
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

    primary_view.width = (primary_view.width as f32 * primary_ratio).ceil() as u32;
    primary_view.width = primary_view.width.saturating_sub(gap as u32);

    let secondary_width = usable_width - primary_view.width - gap;
    let secondary_offset = (primary_view.width + gap) as i32;

    let secondary_height = usable_height - 2 * gap * (secondary_count - 1);

    views.push(primary_view);

    for i in (0..secondary_count).rev() {
        views.push(View {
            x: secondary_offset,
            y: (2 * gap * i) as i32,
            width: secondary_width,
            height: secondary_height,
        });
    }
}

pub fn mono_stack(
    views: &mut Vec<View>,
    _primary_ratio: f32,
    gap: u32,
    view_count: u32,
    usable_width: u32,
    usable_height: u32,
) {
    for i in (0..view_count).rev() {
        views.push(View {
            x: 0,
            y: (2 * gap * i) as i32,
            width: usable_width,
            height: usable_height - 2 * gap * (view_count - 1),
        });
    }
}
