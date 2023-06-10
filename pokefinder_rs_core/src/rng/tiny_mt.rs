#[derive(Copy, Clone)]
pub struct TinyMT {
    pub state: [u32; 4],
}

const JUMP_TABLE: [[u64; 2]; 25] = [
    [0xb0a48045db1bfe95, 0x1b98a18f31f57486],
    [0xe29d1503ee564039, 0x342d0c6dc777e228],
    [0xfd7a37b1acaa7823, 0x9951a06456708b7e],
    [0x5ab81fcd13ccd9fa, 0xce6673b3d158340e],
    [0xe7d0c5907aee0eea, 0x90d98e45a895878],
    [0x2e1bd6473d093826, 0x61def4964ec4ab34],
    [0x33ae14e5d2005a71, 0x334a0fe77ab182de],
    [0xd654b5930b12fe3e, 0x3794cc23a5de8a5e],
    [0x586e1d6b2670a75, 0x86bf0979d37c9a1e],
    [0x8d859b2a345b1a3f, 0xe2d08ec75db83196],
    [0x9d2132eac57edc3a, 0xd8731c41bcf9f318],
    [0xa5c8c0d51e112335, 0x2ebb41367c1e3386],
    [0x7c5c99ea483c815a, 0x9f1173b680f6752e],
    [0x658cd2f421d18c04, 0x41fbd20233bcb628],
    [0x694898799783db46, 0xc8fc1f0f485cc220],
    [0x4cf6c5ecc4826e0b, 0x8e695f0109724eb6],
    [0xf20cef18f4cd9a96, 0x7478b18cfd3ccb36],
    [0x9f0de9fe452bc110, 0x7feb70c475efda16],
    [0xabf913e20fcbe635, 0x1ad541a07a6c610a],
    [0x20999170716ca869, 0x203777ca7d356342],
    [0x5dcb2d78b3e9ca0f, 0x7222f0529a9dd99c],
    [0x197365ac9569a8b4, 0x6dd7a644730f081a],
    [0xf2156d44b37e61be, 0x80bfd2b6153ed5cc],
    [0xac7a0ab2f43b15a9, 0x227df3de640734f4],
    [0x40afea91e9ad4b2c, 0x58440d15ded1d336],
];

impl TinyMT {
    pub fn new(seed: u32) -> Self {
        let mut new = Self::new_from_full(seed, 0x8f7011ee, 0xfc78ff1f, 0x3793fdff);

        for i in 1..8 {
            new.state[i & 3] ^= (i as u32).wrapping_add(
                0x6c078965u32.wrapping_mul(new.state[(i - 1) & 3] ^ (new.state[(i - 1) & 3] >> 30)),
            );
        }

        if (new.state[0] & 0x7FFFFFFF) == 0
            && new.state[1] == 0
            && new.state[2] == 0
            && new.state[3] == 0
        {
            new.state[0] = b'T' as u32;
            new.state[1] = b'I' as u32;
            new.state[2] = b'N' as u32;
            new.state[3] = b'Y' as u32;
        }

        new.advance(8);

        new
    }

    pub fn new_from_full(seed0: u32, seed1: u32, seed2: u32, seed3: u32) -> Self {
        Self {
            state: [seed0, seed1, seed2, seed3],
        }
    }

    pub fn advance(&mut self, advances: u32) {
        for _ in 0..advances {
            self.next_state();
        }
    }

    pub fn jump(&mut self, mut advances: u32) {
        self.advance(advances & 0x7f);
        advances >>= 7;

        let mut i = 0;
        while advances > 0 {
            if (advances & 1) != 0 {
                let mut jump = [0, 0, 0, 0];
                for j in (0..=1).rev() {
                    let mut val = JUMP_TABLE[i][j];
                    for _ in 0..64 {
                        if (val & 1) != 0 {
                            jump = xor(jump, self.state);
                        }
                        self.next_state();
                        val >>= 1;
                    }
                }
                self.state = jump;
            }
            advances >>= 1;
            i += 1;
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> u32 {
        self.next_state();
        self.temper()
    }

    pub fn next_u16(&mut self) -> u16 {
        (self.next() >> 16) as u16
    }

    pub fn next_state(&mut self) {
        let mut y = self.state[3];
        let mut x = (self.state[0] & 0x7FFFFFFF) ^ self.state[1] ^ self.state[2];

        x ^= x << 1;
        y ^= (y >> 1) ^ x;

        self.state[0] = self.state[1];
        self.state[1] = self.state[2] ^ ((y & 1).wrapping_mul(0x8f7011ee));
        self.state[2] = x ^ (y << 10) ^ ((y & 1).wrapping_mul(0xfc78ff1f));
        self.state[3] = y;
    }

    pub fn temper(&self) -> u32 {
        let mut t0 = self.state[3];
        let t1 = self.state[0].wrapping_add(self.state[2] >> 8);

        t0 ^= t1;
        if (t1 & 1) != 0 {
            t0 ^= 0x3793fdff;
        }

        t0
    }
}

#[inline]
fn xor(mut x: [u32; 4], y: [u32; 4]) -> [u32; 4] {
    x[0] ^= y[0];
    x[1] ^= y[1];
    x[2] ^= y[2];
    x[3] ^= y[3];
    x
}
