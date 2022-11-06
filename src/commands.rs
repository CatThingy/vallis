use crate::{layout, Output};

pub fn parse(command: String, output: &mut Output) {
    let mut commands = command.split_whitespace();

    let Some(command) = commands.next() else { return };

    let options = if output.current_tag.count_ones() > 1 {
        &mut output.options[32]
    } else {
        &mut output.options[output.current_tag.trailing_zeros() as usize]
    };

    match command {
        "primary_ratio" => {
            let Some(arg) = commands.next() else { return };
            let Ok(num) = arg.parse() else { return };

            if arg.starts_with('-') || arg.starts_with('+') {
                options.primary_ratio += num
            } else {
                options.primary_ratio = num
            }
            options.primary_ratio = options.primary_ratio.clamp(0.1, 0.9);
        }
        "gap" => {
            let Some(arg) = commands.next() else { return };
            let Ok(num) = arg.parse::<i32>() else { return };

            if arg.starts_with('-') || arg.starts_with('+') {
                options.gap = (i32::try_from(options.gap).unwrap() + num)
                    .try_into()
                    .unwrap_or(options.gap)
            } else {
                options.gap = num.try_into().unwrap_or(options.gap)
            }

            options.gap = options.gap.clamp(0, 100);
        }
        "layout" => {
            let Some(arg) = commands.next() else { return };
            match arg {
                "tile" => {
                    options.layout = layout::standard_tile;
                }
                "stack" => {
                    options.layout = layout::stack;
                }
                _ => (),
            }
        }
        _ => (),
    }
}
