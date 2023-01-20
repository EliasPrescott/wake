pub const DEV_COLOR_LIST: [(u8, u8); 8] = [
    (2, 1),
    (3, 1),
    (4, 1),
    (5, 1),
    (6, 1),
    (43, 1),
    (21, 1),
    (15, 1),
];

pub fn get_primary_color(thread_index: usize) -> u8 {
    let thread_index = thread_index + 2;
    if thread_index > 256 {
        0
    } else {
        thread_index as u8
    }
}
