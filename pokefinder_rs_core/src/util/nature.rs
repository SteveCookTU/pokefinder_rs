const MODIFIERS: [[f32; 5]; 25] = [
    [1.0, 1.0, 1.0, 1.0, 1.0], // Hardy
    [1.1, 0.9, 1.0, 1.0, 1.0], // Lonely
    [1.1, 1.0, 1.0, 1.0, 0.9], // Brave
    [1.1, 1.0, 0.9, 1.0, 1.0], // Adamant
    [1.1, 1.0, 1.0, 0.9, 1.0], // Naughty
    [0.9, 1.1, 1.0, 1.0, 1.0], // Bold
    [1.0, 1.0, 1.0, 1.0, 1.0], // Docile
    [1.0, 1.1, 1.0, 1.0, 0.9], // Relaxed
    [1.0, 1.1, 0.9, 1.0, 1.0], // Impish
    [1.0, 1.1, 1.0, 0.9, 1.0], // Lax
    [0.9, 1.0, 1.0, 1.0, 1.1], // Timid
    [1.0, 0.9, 1.0, 1.0, 1.1], // Hasty
    [1.0, 1.0, 1.0, 1.0, 1.0], // Serious
    [1.0, 1.0, 0.9, 1.0, 1.1], // Jolly
    [1.0, 1.0, 1.0, 0.9, 1.1], // Naive
    [0.9, 1.0, 1.1, 1.0, 1.0], // Modest
    [1.0, 0.9, 1.1, 1.0, 1.0], // Mild
    [1.0, 1.0, 1.1, 1.0, 0.9], // Quiet
    [1.0, 1.0, 1.0, 1.0, 1.0], // Bashful
    [1.0, 1.0, 1.1, 0.9, 1.0], // Rash
    [0.9, 1.0, 1.0, 1.1, 1.0], // Calm
    [1.0, 0.9, 1.0, 1.1, 1.0], // Gentle
    [1.0, 1.0, 1.0, 1.1, 0.9], // Sassy
    [1.0, 1.0, 0.9, 1.1, 1.0], // Careful
    [1.0, 1.0, 1.0, 1.0, 1.0], // Quirky
];

/// Computes a modified stat based on a `nature` and stat `index`.
///
/// Natures do not modify the HP stat but the function expects an index with HP in mind.
/// For example, Attack would still be an index of 1.
///
/// # Panics
///
/// This function will panic if the nature does not exist or the stat index is either 0 or above 5.
/// ```should_panic
/// # use pokefinder_rs_core::util::nature::compute_stat;
/// let nature = 27; // Does not exist
/// let index = 1; // Attack
/// let stat = 76;
/// compute_stat(stat, nature, index);
/// ```
///
/// # Example
/// ```
/// # use pokefinder_rs_core::util::nature::compute_stat;
/// let nature = 3; // Adamant
/// let index = 1; // Attack
/// let stat = 76;
/// assert_eq!(compute_stat(stat, nature, index), 83);
/// ```
pub fn compute_stat(stat: u16, nature: u8, index: usize) -> u16 {
    ((stat as f32) * MODIFIERS[nature as usize][index - 1]) as u16
}
