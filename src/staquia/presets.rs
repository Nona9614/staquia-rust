//! The ready to use systems, and systems from a set of characters.

use super::{Options, Staquia};
use crate::error::CreateError;

impl Staquia {
    /// Creates a system of a preset, which is always valid.
    fn preset(options: Options) -> Self {
        match Self::create_with_options(options) {
            Ok(system) => system,
            Err(_) => unreachable!("the presets are valid"),
        }
    }

    /// The alphabet system, see [`Options::alphabet`]: readable positions like `"mmmmm!"` (base `27`).
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::alphabet();
    /// assert_eq!(system.start()?.to_string(), "mmmmm!");
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn alphabet() -> Self {
        Self::preset(Options::alphabet())
    }

    /// The plain ASCII system, see [`Options::ascii`]: positions like `"PPPPP!"` (base `90`), nothing to escape.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::ascii();
    /// let start = system.start()?;
    /// assert_eq!(start.to_string(), "PPPPP!");
    /// assert_eq!(system.next(&start)?.to_string(), "PPPPQ!");
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn ascii() -> Self {
        Self::preset(Options::ascii())
    }

    /// The URL safe system, see [`Options::url_safe`]: positions like `"VVVVV-"` (base `64`) that need no
    /// encoding in URLs, file names or HTML ids.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::url_safe();
    /// let start = system.start()?;
    /// let next = system.next(&start)?;
    /// assert_eq!(next.to_string(), "VVVVW-");
    /// // Between "Z" (digit 36) and "_" (37) there is no digit, the position gets one more
    /// let z = system.forge("VVVVZ")?;
    /// let low = system.forge("VVVV_")?;
    /// assert_eq!(system.core(&z, &low)?.to_string(), "VVVVZV");
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn url_safe() -> Self {
        Self::preset(Options::url_safe())
    }

    /// The base 36 system, see [`Options::base36`]: positions like `"iiiii0"`, only digits and lowercase letters.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::base36();
    /// assert_eq!(system.start()?.to_string(), "iiiii0");
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn base36() -> Self {
        Self::preset(Options::base36())
    }

    /// Creates a system from a set of characters: the zero followed by every digit, sorted by their code, so its
    /// base is the number of characters. Use it when the digits are not a range, for example to leave some
    /// characters out; a set whose digits are consecutive is the same system as [`Staquia::create`] with that
    /// range, and their positions are compatible.
    ///
    /// The set is a reference to a static string, written `&"..."` (or `&CONSTANT`): positions keep only that pointer,
    /// so they stay small `Copy` values that never allocate.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// // The digits without the vowels: "0" is the zero
    /// let system = Staquia::from_characters(&"0bcdfghjklmnpqrstvwxyz", 5, 15, 0.15)?;
    /// assert_eq!(system.base(), 22);
    /// assert_eq!(system.start()?.to_string(), "nnnnn0");
    ///
    /// // Consecutive digits: the same system as the range
    /// let range = Staquia::from_characters(&"!abcdefghijklmnopqrstuvwxyz", 5, 15, 0.15)?;
    /// assert_eq!(range, Staquia::alphabet());
    /// # Ok::<(), staquia::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// [`CreateError::FewCharacters`] with less than two characters, [`CreateError::Unsorted`] if they are not
    /// sorted by their code or one repeats, [`CreateError::Character`] above `U+FFFF`, and the length and threshold
    /// errors of [`Staquia::create`].
    pub fn from_characters(
        characters: &'static &'static str,
        ln: usize,
        lz: usize,
        threshold: f64,
    ) -> Result<Self, CreateError> {
        Self::create_with_options(Options {
            characters: Some(characters),
            ln,
            lz,
            threshold,
            ..Options::default()
        })
    }
}
