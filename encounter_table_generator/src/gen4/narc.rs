use no_std_io::{Cursor, StreamContainer, StreamReader};

#[derive(Clone, Default)]
pub struct Narc {
    pub fat_b_offset: usize,
    pub fnt_b_offset: usize,
    pub fimg_offset: usize,
    pub elements: Vec<Vec<u8>>,
}

impl Narc {
    pub fn new(mut reader: StreamContainer<&[u8]>) -> Self {
        let fat_b_offset = 0x10;

        reader.set_index(0x18);
        let fnt_b_offset = reader.read_stream_le::<u32>().unwrap() * 8 + fat_b_offset + 12;

        reader.set_index(fnt_b_offset as usize + 4);
        let fimg_offset = reader.read_stream_le::<u32>().unwrap() + fnt_b_offset;

        reader.set_index(0x18);
        let number_of_elements = reader.read_stream_le::<u32>().unwrap();

        let mut elements = vec![];
        let mut start_offsets = vec![];
        let mut end_offsets = vec![];

        reader.set_index(fat_b_offset as usize + 0xC);

        for _ in 0..number_of_elements {
            start_offsets.push(reader.read_stream_le::<u32>().unwrap());
            end_offsets.push(reader.read_stream_le::<u32>().unwrap());
        }

        for i in 0..number_of_elements {
            reader.set_index(fimg_offset as usize + start_offsets[i as usize] as usize + 8);
            elements.push(
                reader
                    .read_byte_stream(
                        (end_offsets[i as usize] - start_offsets[i as usize]) as usize,
                    )
                    .unwrap(),
            );
        }

        Self {
            fat_b_offset: fat_b_offset as usize,
            fnt_b_offset: fnt_b_offset as usize,
            fimg_offset: fimg_offset as usize,
            elements,
        }
    }
}
