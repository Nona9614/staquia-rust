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
    /// The characters of the system when its digits are not a range: the zero followed by every digit, sorted by
    /// their code (`"-0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz"` for [`Options::url_safe`]).
    /// When it is set, `zero`, `first` and `last` are ignored. `None` uses the range from `first` to `last`.
    ///
    /// It is a reference to a static string (`Some(&"...")`, or `Some(&CONSTANT)`): positions keep that one pointer
    /// instead of a copy of the characters, so they stay small `Copy` values that never allocate.
    pub characters: Option<&'static &'static str>,
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
            characters: None,
            ln: 5,
            lz: 15,
            threshold: 0.15,
        }
    }
}

/// The characters of [`Options::ascii`]: `!` as the zero and every printable ASCII character from `#` to `~` except
/// `'`, `\` and `` ` `` (base `90`), so positions never need escaping in JSON, SQL literals, strings or logs.
const ASCII: &str =
    "!#$%&()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[]^_abcdefghijklmnopqrstuvwxyz{|}~";
/// The characters of [`Options::url_safe`]: the URL safe base64 characters sorted by their code, `-` as the zero.
const URL_SAFE: &str = "-0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz";
/// The characters of [`Options::base36`]: `0` as the zero, the digits and the lowercase letters.
const BASE36: &str = "0123456789abcdefghijklmnopqrstuvwxyz";

impl Options {
    /// The options of a preset: its characters with the default lengths (`ln = 5`, `lz = 15`, threshold `0.15`).
    const fn preset(
        zero: char,
        first: char,
        last: char,
        characters: Option<&'static &'static str>,
    ) -> Self {
        Self {
            zero,
            first,
            last,
            characters,
            ln: 5,
            lz: 15,
            threshold: 0.15,
        }
    }

    /// The alphabet: `!` as the zero and the lowercase letters from `a` to `z` (base `27`). Readable positions,
    /// safe in URLs, the system of every example of the documentation.
    ///
    /// ```
    /// use staquia::{Options, Staquia};
    ///
    /// let system = Staquia::create_with_options(Options::alphabet())?;
    /// assert_eq!(system.start()?.to_string(), "mmmmm!");
    ///
    /// // A longer `N` section, the same characters
    /// let longer = Staquia::create_with_options(Options { ln: 6, ..Options::alphabet() })?;
    /// assert_eq!(longer.start()?.to_string(), "mmmmmm!");
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub const fn alphabet() -> Self {
        Self::preset('!', 'a', 'z', None)
    }

    /// Plain ASCII: `!` as the zero and every printable character from `#` to `~` except `'`, `\` and `` ` ``
    /// (base `90`). The most compact choice per byte, and nothing to escape in JSON, SQL literals or logs.
    ///
    /// ```
    /// use staquia::{Options, Staquia};
    ///
    /// let system = Staquia::create_with_options(Options::ascii())?;
    /// assert_eq!(system.base(), 90);
    /// assert_eq!(system.start()?.to_string(), "PPPPP!");
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub const fn ascii() -> Self {
        Self::preset('!', '#', '~', Some(&ASCII))
    }

    /// URL safe: the characters of URL safe base64 (`-`, `0`-`9`, `A`-`Z`, `_`, `a`-`z`) sorted by their code, `-`
    /// as the zero (base `64`). Positions go in URLs, file names and HTML ids without any encoding.
    ///
    /// The characters are not in the base64 order: positions are sort keys, not encoded data, so they keep the
    /// order a database compares them in.
    ///
    /// ```
    /// use staquia::{Options, Staquia};
    ///
    /// let system = Staquia::create_with_options(Options::url_safe())?;
    /// assert_eq!(system.base(), 64);
    /// assert_eq!(system.start()?.to_string(), "VVVVV-");
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub const fn url_safe() -> Self {
        Self::preset('-', '0', 'z', Some(&URL_SAFE))
    }

    /// Base 36: `0` as the zero, the digits `1`-`9` and the lowercase letters (base `36`). Safe with case
    /// insensitive collations (the MySQL default) and where keys must be alphanumeric.
    ///
    /// ```
    /// use staquia::{Options, Staquia};
    ///
    /// let system = Staquia::create_with_options(Options::base36())?;
    /// assert_eq!(system.base(), 36);
    /// assert_eq!(system.start()?.to_string(), "iiiii0");
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub const fn base36() -> Self {
        Self::preset('0', '1', 'z', Some(&BASE36))
    }
}
