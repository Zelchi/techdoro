pub(in crate::ui) fn insertion_index(
    top: f32,
    heights: impl Iterator<Item = f32>,
    gap: f32,
) -> usize {
    let mut nearest = 0;
    let mut distance = top.abs();
    let mut slot_top = 0.;
    for (index, height) in heights.enumerate() {
        slot_top += height + gap;
        let candidate_distance = (top - slot_top).abs();
        if candidate_distance < distance {
            nearest = index + 1;
            distance = candidate_distance;
        }
    }
    nearest
}

#[cfg(test)]
mod tests {
    use super::insertion_index;

    #[test]
    fn release_uses_the_nearest_slot_including_between_rows() {
        for (top, expected) in [
            (-20., 0),
            (0., 0),
            (24., 0),
            (26., 1),
            (50., 1),
            (74., 1),
            (76., 2),
            (100., 2),
            (200., 2),
        ] {
            assert_eq!(insertion_index(top, [42., 42.].into_iter(), 8.), expected);
        }
    }

    #[test]
    fn wrapped_tasks_use_their_actual_heights() {
        // Insertion slots are at 0, 50, and 140 pixels.
        assert_eq!(insertion_index(94., [42., 82.].into_iter(), 8.), 1);
        assert_eq!(insertion_index(96., [42., 82.].into_iter(), 8.), 2);
        assert_eq!(insertion_index(140., [42., 82.].into_iter(), 8.), 2);
    }

    #[test]
    fn a_single_task_stays_in_its_only_slot() {
        assert_eq!(insertion_index(120., std::iter::empty(), 8.), 0);
    }
}
