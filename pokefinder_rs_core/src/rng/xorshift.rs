use crate::rng::Rng;

const JUMP_TABLE: [[u64; 2]; 25] = [
    [0x10046d8b3, 0xf985d65ffd3c8001],
    [0x956c89fbfa6b67e9, 0xa42ca9aeb1e10da6],
    [0xff7aa97c47ec17c7, 0x1a0988e988f8a56e],
    [0x9dff33679bd01948, 0xfb6668ff443b16f0],
    [0xbd36a1d3e3b212da, 0x46a4759b1dc83ce2],
    [0x6d2f354b8b0e3c0b, 0x9640bc4ca0cbaa6c],
    [0xecf6383dca4f108f, 0x947096c72b4d52fb],
    [0xe1054e817177890a, 0xdaf32f04ddca12e],
    [0x2ae1912115107c6, 0xb9fa05aab78641a5],
    [0x59981d3df81649be, 0x382fa5aa95f950e3],
    [0x6644b35f0f8cee00, 0xdba31d29fc044fdb],
    [0xecff213c169fd455, 0x3ca16b953c338c19],
    [0xa9dfd9fb0a094939, 0x3ffdcb096a60ecbe],
    [0x79d7462b16c479f, 0xfd6aef50f8c0b5fa],
    [0x3896736d707b6b6, 0x9148889b8269b55d],
    [0xdea22e8899dbbeaa, 0x4c6ac659b91ef36a],
    [0xc1150ddd5ae7d320, 0x67ccf586cddb0649],
    [0x5f0be91ac7e9c381, 0x33c8177d6b2cc0f0],
    [0xcd15d2ba212e573, 0x4a5f78fc104e47b9],
    [0xab586674147dec3e, 0xd69063e6e8a0b936],
    [0x4bfd9d67ed372866, 0x7071114af22d34f5],
    [0xdaf387cab4ef5c18, 0x686287302b5cd38c],
    [0xffaf82745790af3e, 0xbb7d371f547cca1e],
    [0x7b932849fe573afa, 0xeb96acd6c88829f9],
    [0x8cedf8dfe2d6e821, 0xb4fd2c6573bf7047],
];

#[derive(Copy, Clone)]
pub struct Xorshift {
    state: [u32; 4],
}

impl Xorshift {
    pub fn new(seed0: u64, seed1: u64) -> Self {
        let mut state = [0; 4];

        let flip_and_set = |state: &mut [u32; 4], index: usize, seed: u64| {
            state[index + 1] = (seed & 0xFFFFFFFF) as u32;
            state[index] = (seed >> 32) as u32;
        };

        flip_and_set(&mut state, 0, seed0);
        flip_and_set(&mut state, 2, seed1);

        Self { state }
    }

    pub fn new_with_initial_advances(seed0: u64, seed1: u64, advances: u32) -> Self {
        let mut new = Xorshift::new(seed0, seed1);
        new.jump(advances);
        new
    }
}

impl Rng for Xorshift {
    type Output = u32;

    fn next(&mut self) -> Self::Output {
        let ptr = &mut self.state;
        let mut t = ptr[0];
        let s = ptr[3];

        t ^= t << 11;
        t ^= t >> 8;
        t ^= s ^ (s >> 19);

        ptr[0] = ptr[1];
        ptr[1] = ptr[2];
        ptr[2] = ptr[3];
        ptr[3] = t;

        t
    }

    fn advance(&mut self, advances: u32) {
        for _ in 0..advances {
            self.next();
        }
    }

    fn jump(&mut self, mut advances: u32) {
        self.advance(advances & 0x7F);
        advances >>= 7;
        let mut i = 0;
        while advances > 0 {
            if (advances & 1) != 0 {
                let mut jump = [0u32; 4];
                for j in (0..=1).rev() {
                    let mut val = JUMP_TABLE[i][j];
                    for _ in 0..64 {
                        if val & 1 != 0 {
                            jump = xor(jump, self.state);
                        }
                        self.next();
                        val >>= 1;
                    }
                }
                self.state = jump;
            }
            advances >>= 1;
            i += 1;
        }
    }
}

fn xor(mut x: [u32; 4], y: [u32; 4]) -> [u32; 4] {
    x[0] ^= y[0];
    x[1] ^= y[1];
    x[2] ^= y[2];
    x[3] ^= y[3];
    x
}
