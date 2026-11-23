//! The configuration of a system.

/// The configuration of a [`Staquia`](crate::Staquia) system, for [`Staquia::create_with_options`](crate::Staquia::create_with_options).
///
/// Its [`Default`] is the default system, so only the fields that change need to be written.
///
/// # Example
///
/// ```
/// use staquia::{Options, Staquia};
///
/// // The default characters with a longer `N` section
/// let system = Staquia::create_with_options(Options { ln: 6, ..Options::default() })?;
/// assert_eq!(system.start()?.to_string(), "ʰʰʰʰʰʰ!");
/// # Ok::<(), staquia::Error>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Options {
    /// The character of the zero digit.
    pub zero: char,
    /// The character of the first digit (`1`).
    pub first: char,
    /// The character of the last digit (`base - 1`).
    pub last: char,
    /// The fixed length of the `N` section, used to stack items without making positions longer.
    pub ln: usize,
    /// The max length of the `Z` section, used when items are moved between two others.
    pub lz: usize,
    /// A factor from `0` to `1`: the threshold is reached at the length `ceil((1 - threshold) * (ln + lz))`.
    pub threshold: f64,
}

impl Default for Options {
    /// The default system: the zero `!`, the digits from `°` (`U+00B0`) to `ұ` (`U+04B1`), a base `1027`
    /// system, with `ln = 5`, `lz = 15` and a threshold of `0.15`.
    fn default() -> Self {
        Self {
            zero: '!',
            first: '\u{00B0}',
            last: '\u{04B1}',
            ln: 5,
            lz: 15,
            threshold: 0.15,
        }
    }
}
