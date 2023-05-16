use no_std_io::{Cursor, StreamContainer, StreamReader};
use std::collections::HashMap;

pub fn read_map_names(data: &[u8]) -> Vec<String> {
    let mut reader = StreamContainer::new(data);
    let characters =
        serde_json::from_str::<HashMap<String, String>>(include_str!("characters.json")).unwrap();

    let string_count = reader.read_stream_le::<u16>().unwrap();
    let initial_key = reader.read_stream_le::<u16>().unwrap();

    let mut key1 = initial_key.wrapping_mul(0x2FD);
    let mut key2;
    let mut real_key;
    let mut special_char_on = false;

    let mut current_offset = vec![];
    let mut current_size = vec![];

    for i in 0..string_count {
        key2 = key1.wrapping_mul(i + 1);
        real_key = (key2 as u32) | ((key2 as u32) << 16);
        current_offset.push(reader.read_stream_le::<u32>().unwrap() ^ real_key);
        current_size.push(reader.read_stream_le::<u32>().unwrap() ^ real_key);
    }

    let mut map_names = vec![];

    for i in 0..string_count {
        key1 = (0x91BD3u32.wrapping_mul((i + 1) as u32) & 0xFFFF) as u16;
        reader.set_index(current_offset[i as usize] as usize);
        let mut text = String::new();
        for _ in 0..current_size[i as usize] {
            let char = reader.read_stream_le::<u16>().unwrap() ^ key1;

            match char {
                0xe000 => {
                    text += "\n";
                }
                0x25bc => {
                    text += "\r";
                }
                0x25bd => {
                    text += "\x0C";
                }
                0xFFFE => {
                    text += "\x0B";
                    special_char_on = true;
                }
                0xFFFF => {}
                _ => {
                    if special_char_on {
                        text += &format!("{:>4X}", char);
                        special_char_on = false;
                    } else {
                        text += characters.get(&char.to_string()).unwrap();
                    }
                }
            }
            key1 = key1.wrapping_add(0x493D);
        }
        map_names.push(text);
    }

    map_names
}
