// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::Variant;
use crate::helpers::ShortVec;

use alloc::vec::Vec;
use core::ops::Deref;

/// A list of variants (examples: `["macos", "posix"]`, etc.)
///
/// [`Variants`] stores a list of [`Variant`] subtags in a canonical form
/// by sorting and deduplicating them.
///
/// # Examples
///
/// ```
/// use icu::locid::subtags::{Variant, Variants};
///
/// let variant1: Variant =
///     "posix".parse().expect("Failed to parse a variant subtag.");
///
/// let variant2: Variant =
///     "macos".parse().expect("Failed to parse a variant subtag.");
/// let mut v = vec![variant1, variant2];
/// v.sort();
/// v.dedup();
///
/// let variants: Variants = Variants::from_vec_unchecked(v);
/// assert_eq!(variants.to_string(), "macos-posix");
/// ```
#[derive(Default, Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Variants(ShortVec<Variant>);

impl Variants {
    /// Returns a new empty list of variants. Same as [`default()`](Default::default()), but is `const`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Variants;
    ///
    /// assert_eq!(Variants::new(), Variants::default());
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self(ShortVec::new())
    }

    /// Creates a new [`Variants`] set from a single [`Variant`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::{Variant, Variants};
    ///
    /// let variant: Variant = "posix".parse().expect("Parsing failed.");
    /// let variants = Variants::from_variant(variant);
    /// ```
    #[inline]
    pub const fn from_variant(variant: Variant) -> Self {
        Self(ShortVec::new_single(variant))
    }

    /// Creates a new [`Variants`] set from a [`Vec`].
    /// The caller is expected to provide sorted and deduplicated vector as
    /// an input.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::{Variant, Variants};
    ///
    /// let variant1: Variant = "posix".parse().expect("Parsing failed.");
    /// let variant2: Variant = "macos".parse().expect("Parsing failed.");
    /// let mut v = vec![variant1, variant2];
    /// v.sort();
    /// v.dedup();
    ///
    /// let variants = Variants::from_vec_unchecked(v);
    /// ```
    ///
    /// Notice: For performance- and memory-constrained environments, it is recommended
    /// for the caller to use [`binary_search`](slice::binary_search) instead of [`sort`](slice::sort)
    /// and [`dedup`](Vec::dedup()).
    pub fn from_vec_unchecked(input: Vec<Variant>) -> Self {
        Self(ShortVec::from(input))
    }

    /// Empties the [`Variants`] list.
    ///
    /// Returns the old list.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::{Variant, Variants};
    ///
    /// let variant1: Variant = "posix".parse().expect("Parsing failed.");
    /// let variant2: Variant = "macos".parse().expect("Parsing failed.");
    /// let mut v = vec![variant1, variant2];
    /// v.sort();
    /// v.dedup();
    ///
    /// let mut variants: Variants = Variants::from_vec_unchecked(v);
    ///
    /// assert_eq!(variants.to_string(), "macos-posix");
    ///
    /// variants.clear();
    ///
    /// assert_eq!(variants.to_string(), "");
    /// ```
    pub fn clear(&mut self) -> Self {
        core::mem::take(self)
    }

    pub(crate) fn for_each_subtag_str<E, F>(&self, f: &mut F) -> Result<(), E>
    where
        F: FnMut(&str) -> Result<(), E>,
    {
        self.deref().iter().map(|t| t.as_str()).try_for_each(f)
    }
}

impl_writeable_for_subtag_list!(Variants, "macos", "posix");

impl Deref for Variants {
    type Target = [Variant];

    fn deref(&self) -> &[Variant] {
        self.0.as_slice()
    }
}
