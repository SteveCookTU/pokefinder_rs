use crate::enums::{Encounter, Lead, Method};
use crate::gen8::filters::WildStateFilter8;
use crate::gen8::{EncounterArea8, Profile8};
use crate::parents::filters::WildFilter;
use crate::parents::generators::WildGenerator;
use crate::parents::states::WildGeneratorState;
use crate::parents::{EncounterAreaT, PersonalInfo, Profile};
use crate::rng::{RNGList, Xorshift};
use crate::util::encounter_slot;

fn get_item(rand: u8, lead: Lead, info: &PersonalInfo) -> u16 {
    const ITEM_TABLE_RANGE: [[u8; 2]; 2] = [[50, 5], [60, 20]];

    let thresh1 = ITEM_TABLE_RANGE[if lead == Lead::COMPOUND_EYES { 1 } else { 0 }][0];
    let thresh2 = ITEM_TABLE_RANGE[if lead == Lead::COMPOUND_EYES { 1 } else { 0 }][1];

    if rand >= thresh1 {
        if rand >= thresh1 + thresh2 {
            info.get_item(2)
        } else {
            info.get_item(1)
        }
    } else {
        info.get_item(0)
    }
}

fn rand(prng: u32) -> u32 {
    (prng % 0xffffffff).wrapping_add(0x80000000)
}

#[derive(Clone)]
pub struct WildGenerator8<'a, 'b> {
    pub base: WildGenerator<'a, 'b, Profile8, WildStateFilter8>,
}

impl<'a, 'b> WildGenerator8<'a, 'b> {
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        delay: u32,
        encounter: Encounter,
        lead: Lead,
        profile: &'a Profile8,
        filter: &'b WildStateFilter8,
    ) -> Self {
        let mut new = Self {
            base: WildGenerator::new(
                initial_advances,
                max_advances,
                delay,
                Method::None,
                encounter,
                lead,
                profile,
                filter,
            ),
        };
        new.base.base.tsv = (profile.get_tid() & 0xFFF0) ^ profile.get_sid();
        new
    }

    pub fn generate(
        &self,
        seed0: u64,
        seed1: u64,
        encounter_area: &EncounterArea8,
    ) -> Vec<WildGeneratorState> {
        let rng = Xorshift::new_with_initial_advances(
            seed0,
            seed1,
            self.base
                .base
                .initial_advances
                .wrapping_add(self.base.base.delay),
        );
        let mut rng_list = RNGList::<u32, Xorshift, 128>::new(rng, None);

        let encounter_force = matches!(
            self.base.lead,
            Lead::MAGNET_PULL | Lead::STATIC | Lead::HARVEST | Lead::FLASH_FIRE | Lead::STORM_DRAIN
        );
        let modified_slots = encounter_area.get_slots_by_lead(self.base.lead);

        let mut states = vec![];
        let mut cnt = 0;
        while cnt <= self.base.base.max_advances {
            let encounter_slot =
                if encounter_force && (rng_list.next() % 2) == 0 && !modified_slots.is_empty() {
                    modified_slots[rng_list.next() as usize % modified_slots.len()]
                } else {
                    encounter_slot::bdsp_slot((rng_list.next() % 100) as u8, self.base.encounter)
                };

            if !self.base.base.filter.compare_encounter_slot(encounter_slot) {
                cnt += 1;
                rng_list.advance_state();
                continue;
            }

            let slot = encounter_area.get_specific_pokemon(encounter_slot as usize);
            let form = if slot.get_species() == 201 {
                encounter_area.unown_form(rng_list.next())
            } else {
                0
            };

            rng_list.advance(84);

            let level = if self.base.encounter == Encounter::Grass {
                encounter_area.calculate_level_with_list::<false>(
                    encounter_slot,
                    &mut rng_list,
                    self.base.lead == Lead::PRESSURE,
                )
            } else {
                encounter_area.calculate_level_with_list::<true>(
                    encounter_slot,
                    &mut rng_list,
                    self.base.lead == Lead::PRESSURE,
                )
            };

            let ec = rng_list.next_alt(rand);
            let sidtid = rng_list.next_alt(rand);
            let mut pid = rng_list.next_alt(rand);

            let psv = ((pid >> 16) ^ (pid & 0xFFF0)) as u16;
            let fake_xor = (((sidtid >> 16) ^ (sidtid & 0xFFF0)) as u16) ^ psv;
            let shiny = if fake_xor < 16 {
                let shiny = if fake_xor == 0 { 2 } else { 1 };
                let real_xor = psv ^ self.base.base.tsv;
                let real_shiny = if real_xor == 0 {
                    2
                } else if real_xor < 16 {
                    1
                } else {
                    0
                };

                if real_shiny != shiny {
                    let high = ((pid & 0xFFFF) as u16) ^ self.base.base.tsv ^ (2 - shiny);
                    pid = ((high as u32) << 16) | (pid & 0xFFFF);
                }
                shiny as u8
            } else {
                if (psv ^ self.base.base.tsv) < 16 {
                    pid ^= 0x10000000;
                }
                0
            };

            let mut ivs = [0; 6];
            ivs.iter_mut().for_each(|iv| {
                *iv = (rng_list.next_alt(rand) % 32) as u8;
            });

            let ability = (rng_list.next_alt(rand) % 2) as u8;

            let info = slot.get_info();

            let gender = match info.get_gender() {
                255 => 2,
                254 => 1,
                0 => 0,
                i => {
                    if (self.base.lead == Lead::CUTE_CHARM_F
                        || self.base.lead == Lead::CUTE_CHARM_M)
                        && (rng_list.next_alt(rand) % 3) != 0
                    {
                        if self.base.lead == Lead::CUTE_CHARM_F {
                            0
                        } else {
                            1
                        }
                    } else {
                        u8::from((rng_list.next_alt(rand) % 253) as u8 + 1 < i)
                    }
                }
            };

            let nature = if self.base.lead.bits() <= Lead::SYNCHRONIZE_END.bits() {
                self.base.lead.bits()
            } else {
                (rng_list.next_alt(rand) % 25) as u8
            };

            rng_list.advance(4);

            let item = get_item((rng_list.next() % 100) as u8, self.base.lead, info);

            let state = WildGeneratorState::new_with_ec(
                self.base.base.initial_advances.wrapping_add(cnt),
                ec,
                pid,
                ivs,
                ability,
                gender,
                level,
                nature,
                shiny,
                encounter_slot,
                item,
                slot.get_species(),
                form,
                info,
            );
            if self.base.base.filter.compare_generator_state(&state) {
                states.push(state);
            }

            cnt += 1;
            rng_list.advance_state();
        }

        states
    }
}
