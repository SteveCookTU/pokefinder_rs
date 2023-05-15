use crate::gen3::ShadowTemplate;
use crate::rng::{XDRNG, XDRNGR};

#[inline]
fn is_shiny(pid: u32, tsv: u16) -> bool {
    ((pid >> 16) ^ (pid & 0xFFF) ^ tsv as u32) < 8
}

#[inline]
fn get_pid_backward(rng: &mut XDRNGR) -> u32 {
    let mut pid = rng.next_u16() as u32;
    pid |= rng.next() & 0xFFFF0000;
    pid
}

#[inline]
fn get_pid_forward(rng: &mut XDRNG) -> u32 {
    let mut pid = rng.next() & 0xFFFF0000;
    pid |= rng.next_u16() as u32;
    pid
}

pub fn colo_shadow(seed: u32, shadow_template: &ShadowTemplate) -> bool {
    let mut backward = XDRNGR::new(seed);
    backward.advance(1);

    let pid_original = get_pid_backward(&mut backward);
    let mut index = shadow_template.get_count() - 1;
    if !shadow_template.get_lock(index).compare(pid_original) {
        return false;
    }

    let mut pid = 0;
    index -= 1;
    while index >= 0 {
        let lock = &shadow_template.get_lock(index);
        backward.advance(5);
        if !lock.get_ignore() {
            while {
                pid = get_pid_backward(&mut backward);
                !lock.compare(pid)
            } {}
        }
        index -= 1;
    }

    let mut forward = XDRNG::new(backward.seed);
    forward.advance(1);

    let mut index = 1;
    let count = shadow_template.get_count();
    while index < count {
        let lock = shadow_template.get_lock(index);
        forward.advance(5);
        if !lock.get_ignore() {
            while {
                pid = get_pid_forward(&mut forward);
                !lock.compare(pid)
            } {}
        }
        index += 1;
    }

    pid_original == pid
}

pub fn ereader(seed: u32, reader_pid: u32, shadow_template: &ShadowTemplate) -> bool {
    let mut index = shadow_template.get_count() - 1;
    if !shadow_template.get_lock(index).compare(reader_pid) {
        return false;
    }

    let mut backward = XDRNGR::new(seed);
    backward.advance(1);

    let mut pid = 0;
    index -= 1;
    while index >= 0 {
        let lock = &shadow_template.get_lock(index);
        if index != shadow_template.get_count() - 2 {
            backward.advance(5);
        }

        while {
            pid = get_pid_backward(&mut backward);
            !lock.compare(pid)
        } {}

        index -= 1;
    }

    let mut forward = XDRNG::new(backward.seed);
    forward.advance(1);

    let mut index = 1;
    let count = shadow_template.get_count();
    while index < count {
        let lock = shadow_template.get_lock(index);
        forward.advance(5);
        if !lock.get_ignore() {
            while {
                pid = get_pid_forward(&mut forward);
                !lock.compare(pid)
            } {}
        }
        index += 1;
    }

    reader_pid == pid
}

pub fn first_shadow_normal(seed: u32, tsv: u16, shadow_template: &ShadowTemplate) -> bool {
    let mut backward = XDRNGR::new(seed);
    backward.advance(1);

    let pid_original = get_pid_backward(&mut backward);
    let mut index = shadow_template.get_count() - 1;
    if !shadow_template.get_lock(index).compare(pid_original) {
        return false;
    }

    let mut pid = 0;
    index -= 1;
    while index >= 0 {
        let lock = &shadow_template.get_lock(index);
        backward.advance(5);
        if !lock.get_ignore() {
            while {
                pid = get_pid_backward(&mut backward);
                !lock.compare(pid)
            } {}
        }
        index -= 1;
    }

    let mut forward = XDRNG::new(backward.seed);
    forward.advance(1);

    let mut index = 1;
    let count = shadow_template.get_count();
    while index < count {
        let lock = shadow_template.get_lock(index);
        forward.advance(5);
        if !lock.get_ignore() {
            while {
                pid = get_pid_forward(&mut forward);
                !lock.compare(pid) || is_shiny(pid, tsv)
            } {}
        }
        index += 1;
    }

    pid_original == pid
}

pub fn first_shadow_set(seed: u32, tsv: u16, shadow_template: &ShadowTemplate) -> bool {
    let mut backward = XDRNGR::new(seed);
    backward.advance(6);

    let pid_original = get_pid_backward(&mut backward);
    let mut index = shadow_template.get_count() - 2;
    if !shadow_template.get_lock(index).compare(pid_original) {
        return false;
    }

    let mut pid = 0;
    index -= 1;
    while index >= 0 {
        let lock = &shadow_template.get_lock(index);
        backward.advance(5);
        if !lock.get_ignore() {
            while {
                pid = get_pid_backward(&mut backward);
                !lock.compare(pid)
            } {}
        }
        index -= 1;
    }

    let mut forward = XDRNG::new(backward.seed);
    forward.advance(1);

    let mut index = 1;
    let count = shadow_template.get_count();
    while index < count {
        let lock = shadow_template.get_lock(index);
        forward.advance(5);
        if !lock.get_ignore() {
            while {
                pid = get_pid_forward(&mut forward);
                !lock.compare(pid) || is_shiny(pid, tsv)
            } {}
        }
        index += 1;
    }

    pid_original == pid
}

pub fn first_shadow_unset(seed: u32, tsv: u16, shadow_template: &ShadowTemplate) -> bool {
    let mut backward = XDRNGR::new(seed);
    backward.advance(3);

    let mut test = backward;
    let mut shadow_psv = test.next_u16() ^ test.next_u16();
    while (shadow_psv ^ tsv) < 8 {
        backward.seed = test.seed;
        shadow_psv = test.next_u16() ^ test.next_u16();
    }

    backward.advance(5);

    let pid_original = get_pid_backward(&mut backward);
    let mut index = shadow_template.get_count() - 2;
    if !shadow_template.get_lock(index).compare(pid_original) {
        return false;
    }

    let mut pid = 0;
    index -= 1;
    while index >= 0 {
        let lock = &shadow_template.get_lock(index);
        backward.advance(5);
        if !lock.get_ignore() {
            while {
                pid = get_pid_backward(&mut backward);
                !lock.compare(pid)
            } {}
        }
        index -= 1;
    }

    let mut forward = XDRNG::new(backward.seed);
    forward.advance(1);

    let mut index = 1;
    let count = shadow_template.get_count();
    while index < count {
        let lock = shadow_template.get_lock(index);
        forward.advance(5);
        if !lock.get_ignore() {
            while {
                pid = get_pid_forward(&mut forward);
                !lock.compare(pid) || is_shiny(pid, tsv)
            } {}
        }
        index += 1;
    }

    pid_original == pid
}

pub fn salamence_set(seed: u32, tsv: u16, shadow_template: &ShadowTemplate) -> bool {
    let mut backward = XDRNGR::new(seed);
    backward.advance(6);

    let pid = get_pid_backward(&mut backward);
    shadow_template.get_lock(0).compare(pid) && !is_shiny(pid, tsv)
}

pub fn salamence_unset(seed: u32, tsv: u16, shadow_template: &ShadowTemplate) -> bool {
    let mut backward = XDRNGR::new(seed);
    backward.advance(3);

    let mut test = backward;
    let mut shadow_psv = test.next_u16() ^ test.next_u16();
    while (shadow_psv ^ tsv) < 8 {
        backward.seed = test.seed;
        shadow_psv = test.next_u16() ^ test.next_u16();
    }

    backward.advance(5);

    let pid = get_pid_backward(&mut backward);

    shadow_template.get_lock(0).compare(pid) && !is_shiny(pid, tsv)
}

pub fn single_nl(seed: u32, tsv: u16, shadow_template: &ShadowTemplate) -> bool {
    let mut backward = XDRNGR::new(seed);
    backward.advance(1);

    let pid = get_pid_backward(&mut backward);
    shadow_template.get_lock(0).compare(pid) && !is_shiny(pid, tsv)
}
