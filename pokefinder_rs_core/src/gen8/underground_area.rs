use crate::rng::{RNGList, Xorshift};
use crate::util::translator;
use no_std_io::EndianRead;

#[derive(Copy, Clone, Debug)]
pub struct TypeRate {
    pub rate: u16,
    pub ty: u8,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct TypeSize {
    pub value: u16,
    pub size: u8,
    pub ty: u8,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Pokemon {
    pub rate: u16,
    pub species: u16,
    pub size: u8,
    pub ty: [u8; 2],
}

#[derive(Copy, Clone, Default, EndianRead, Debug)]
pub struct SpecialPokemon {
    pub rate: u16,
    pub species: u16,
}

#[derive(Clone, Default, Debug)]
pub struct UndergroundArea {
    pub pokemon: Vec<Pokemon>,
    pub special_pokemon: Vec<SpecialPokemon>,
    pub type_rates: Vec<TypeRate>,
    pub type_sizes: Vec<TypeSize>,
    pub special_sum: u16,
    pub type_sum: u16,
    pub location: u8,
    pub max: u8,
    pub min: u8,
}

fn rand(prng: u32) -> f32 {
    let t = (prng & 0x7fffff) as f32 / 8388607.0;
    1.0 - t
}

impl UndergroundArea {
    pub fn new(
        location: u8,
        min: u8,
        max: u8,
        pokemon: Vec<Pokemon>,
        special_pokemon: Vec<SpecialPokemon>,
        type_rates: [u8; 18],
        type_sizes: Vec<TypeSize>,
    ) -> Self {
        let mut new = Self {
            pokemon,
            special_pokemon,
            type_sizes,
            location,
            max,
            min,
            ..Default::default()
        };
        for i in 1..new.special_pokemon.len() {
            new.special_pokemon[i].rate += new.special_pokemon[i - 1].rate;
        }
        new.special_sum = new.special_pokemon.last().unwrap().rate;

        for (i, rate) in type_rates.into_iter().enumerate() {
            if new.type_sizes.iter().any(|s| s.ty as usize == i) {
                new.type_sum += rate as u16;
                let type_rate = TypeRate {
                    rate: rate as u16,
                    ty: i as u8,
                };
                new.type_rates.push(type_rate);
            }
        }

        new.type_rates.sort_by(|l, r| r.rate.cmp(&l.rate));

        for i in 1..new.type_rates.len() {
            new.type_rates[i].rate += new.type_rates[i - 1].rate;
        }

        new
    }

    pub fn get_pokemon(&self, rng_list: &mut RNGList<u32, Xorshift, 256>, ty: TypeSize) -> u16 {
        let mut temp_count = 0;
        let mut temp = [TypeSize::default(); 23];

        for type_size in &self.type_sizes {
            if ty.value == type_size.value {
                temp[temp_count] = *type_size;
                temp_count += 1;
            }
        }

        let mut filtered_count = 0;
        let mut sum = 0;
        let mut filtered = [Pokemon::default(); 23];
        for mon in &self.pokemon {
            if temp[..temp_count]
                .iter()
                .any(|ts| ts.size == mon.size && (ts.ty == mon.ty[0] || ts.ty == mon.ty[1]))
            {
                sum += mon.rate;
                filtered[filtered_count] = *mon;
                filtered_count += 1;
            }
        }
        filtered[..filtered_count].sort_by(|l, r| r.rate.cmp(&l.rate));

        let mut rate = rng_list.next_alt(rand) * sum as f32;
        for filter in filtered.iter().take(filtered_count) {
            if rate < filter.rate as f32 {
                return filter.species;
            }
            rate -= filter.rate as f32;
        }
        0
    }

    pub fn get_slots(
        &self,
        rng_list: &mut RNGList<u32, Xorshift, 256>,
        count: u8,
    ) -> [TypeSize; 10] {
        let mut slots = [TypeSize::default(); 10];
        for i in 0..count {
            let mut ty = 0;
            let rate = rng_list.next_alt(rand) * self.type_sum as f32;
            let it = self.type_rates.iter().find(|tr| rate < tr.rate as f32);
            if let Some(it) = it {
                ty = it.ty;
            }

            let mut sizes = [255, 255, 255];
            let mut size_count = 0;
            for t in &self.type_sizes {
                if t.ty == ty && !sizes[..size_count].iter().any(|&s| s == t.size) {
                    sizes[size_count] = t.size;
                    size_count += 1;
                }
            }

            let size = sizes[(rng_list.next() as usize) % size_count];
            let value = 10u16.pow(size as u32) + ty as u16;

            let slot = TypeSize { value, size, ty };
            slots[i as usize] = slot;
        }
        slots
    }

    pub fn get_special_pokemon(&self, rng_list: &mut RNGList<u32, Xorshift, 256>) -> u16 {
        if (rng_list.next() % 100) < 50 {
            let rate = rng_list.next_alt(rand) * self.special_sum as f32;
            let it = self.special_pokemon.iter().find(|sp| rate < sp.rate as f32);
            if let Some(it) = it {
                it.species
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn get_species(&self) -> Vec<u16> {
        let mut nums = self
            .pokemon
            .iter()
            .map(|mon| mon.species)
            .collect::<Vec<_>>();
        nums.extend(self.special_pokemon.iter().map(|mon| mon.species));
        nums.sort();
        nums
    }

    pub fn get_species_names(&self) -> Vec<&'static str> {
        translator::get_species_list(&self.get_species())
    }
}
