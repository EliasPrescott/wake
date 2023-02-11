pub fn get_primary_color(thread_index: usize) -> u8 {
    let thread_index = thread_index + 2;
    if thread_index > 256 {
        0
    } else {
        thread_index as u8
    }
}
