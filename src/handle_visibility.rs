use crate::BoxFrame;
use bevy::{
    ecs::prelude::*,
    picking::events::{Move, Out, Over, Pointer},
    prelude::Visibility,
};

pub fn handle_visibility(
    mut over_events: MessageReader<Pointer<Over>>,
    mut move_events: MessageReader<Pointer<Move>>,
    mut out_events: MessageReader<Pointer<Out>>,
    box_frames: Query<&BoxFrame>,
    mut visibility: Query<&mut Visibility>,
) {
    let normalized_over = over_events.read().map(|e| (e.entity, Visibility::Visible));
    let normalized_move = move_events.read().map(|e| (e.entity, Visibility::Visible));
    let normalized_out = out_events.read().map(|e| (e.entity, Visibility::Hidden));

    for (target, set_visibility) in normalized_over.chain(normalized_move).chain(normalized_out) {
        let Ok(frame) = box_frames.get(target) else {
            continue;
        };

        for handle_enitity in frame.handle_entities() {
            let Ok(mut vis) = visibility.get_mut(handle_enitity) else {
                continue;
            };
            *vis = set_visibility;
        }
    }

    // Dragging frames never have visible handles.
    for frame in &box_frames {
        if frame.dragging_face.is_none() {
            continue;
        }

        for handle_enitity in frame.handle_entities() {
            let Ok(mut vis) = visibility.get_mut(handle_enitity) else {
                continue;
            };
            *vis = Visibility::Hidden;
        }
    }
}
