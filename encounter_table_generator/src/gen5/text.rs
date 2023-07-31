use no_std_io::{Cursor, StreamContainer, StreamReader};

pub fn read_map_names(data: &[u8]) -> Vec<String> {
    let mut reader = StreamContainer::new(data);

    let mut main_key = 31881;
    let _ = reader.read_stream_le::<u16>().unwrap();
    let name_count = reader.read_stream_le::<u16>().unwrap();

    let mut _section_size = reader.read_stream_le::<u32>().unwrap();
    let _padding = reader.read_stream_le::<[u8; 4]>().unwrap();

    let section_offset = reader.read_stream_le::<u32>().unwrap();

    let mut map_names = vec![];

    for i in 0..name_count {
        reader.set_index(section_offset as usize);

        _section_size = reader.read_stream_le::<u32>().unwrap();
        reader.set_index(reader.get_index() + (i as usize) * 8);
        let string_offset = reader.read_stream_le::<u32>().unwrap();
        let string_size = reader.read_stream_le::<u16>().unwrap();
        let _ = reader.read_stream_le::<u16>().unwrap();

        reader.set_index((section_offset as usize) + string_offset as usize);
        let mut text = String::new();
        let mut key = main_key;

        for _ in 0..string_size {
            let char = reader.read_stream_le::<u16>().unwrap() ^ key;
            match char {
                0xFFFF => {}
                0xF100 => {
                    text += "\u{f100}";
                }
                0xFFFE => {
                    text += "\n";
                }
                i if i > 20 && i <= 0xFFF0 && i != 0xF000 => {
                    text += &char::from_u32(char as u32).unwrap().to_string();
                }
                _ => {
                    text += &char::from_u32(char as u32).unwrap().to_string();
                }
            }

            key = (key << 3) | (key >> 13);
        }

        main_key = main_key.wrapping_add(0x2983);

        map_names.push(text);
    }

    map_names
}
