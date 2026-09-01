use zhc_utils::BiMap;

use super::{Lut1, Lut2, Lut4, Lut8, LutId, RawLut};

/// A deduplicated registry that maps lookup tables to compact [`LutId`] identifiers.
///
/// `LutRegistry` maintains a bidirectional mapping between [`RawLut`] contents and [`LutId`]
/// values. When a table is registered, it is compared by content (not by name) against
/// previously registered tables. If an identical table already exists, the registration is
/// silently ignored; otherwise a fresh [`LutId`] is assigned sequentially starting from zero.
///
/// This is useful during circuit compilation: register every LUT that appears in the program,
/// then refer to each one by its compact [`LutId`] for the rest of the pipeline.
///
/// The registry provides typed accessors ([`get_l1`](Self::get_l1),
/// [`get_l2`](Self::get_l2), etc.) that return the appropriately typed wrapper. These rely
/// on the caller matching the accessor width to the width that was originally registered —
/// the registry itself does not track table width.
///
/// # Examples
///
/// ```rust,no_run
/// # use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::{Lut1, LutRegistry}};
/// let spec = CiphertextBlockSpec(2, 4);
/// let double = Lut1::from_fn("double", spec, |b| {
///     spec.from_message((b.raw_message_bits() * 2) & spec.message_mask())
/// });
///
/// let mut registry = LutRegistry::empty();
/// registry.register_l1(&double);
///
/// // Retrieve the identifier assigned during registration
/// let lid = registry.get_l1_lid(&double);
///
/// // Round-trip back to the typed LUT
/// let recovered = registry.get_l1(&lid);
/// assert_eq!(recovered, &double);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LutRegistry(BiMap<RawLut, LutId>);

impl LutRegistry {
    /// Creates an empty registry with no registered lookup tables.
    ///
    /// The returned registry contains no mappings. Register tables with
    /// [`register_l1`](Self::register_l1) and its siblings before attempting any lookups.
    pub fn empty() -> Self {
        LutRegistry(BiMap::new())
    }

    /// Registers a single-output lookup table, assigning it a fresh [`LutId`].
    ///
    /// If `lut` is already present in the registry (matched by table contents, not by
    /// name), this method does nothing. Otherwise a new [`LutId`] is assigned sequentially
    /// and the table is stored for later retrieval via [`get_l1`](Self::get_l1) or
    /// [`get_l1_lid`](Self::get_l1_lid).
    pub fn register_l1(&mut self, lut: &Lut1) {
        if !self.0.has_dom(&lut.0) {
            let _ = self.0.insert(lut.0.clone(), LutId(self.0.len()));
        }
    }

    /// Registers a two-output lookup table, assigning it a fresh [`LutId`].
    ///
    /// If `lut` is already present in the registry (matched by table contents, not by
    /// name), this method does nothing. Otherwise a new [`LutId`] is assigned sequentially
    /// and the table is stored for later retrieval via [`get_l2`](Self::get_l2) or
    /// [`get_l2_lid`](Self::get_l2_lid).
    pub fn register_l2(&mut self, lut: &Lut2) {
        if !self.0.has_dom(&lut.0) {
            let _ = self.0.insert(lut.0.clone(), LutId(self.0.len()));
        }
    }

    /// Registers a four-output lookup table, assigning it a fresh [`LutId`].
    ///
    /// If `lut` is already present in the registry (matched by table contents, not by
    /// name), this method does nothing. Otherwise a new [`LutId`] is assigned sequentially
    /// and the table is stored for later retrieval via [`get_l4`](Self::get_l4) or
    /// [`get_l4_lid`](Self::get_l4_lid).
    pub fn register_l4(&mut self, lut: &Lut4) {
        if !self.0.has_dom(&lut.0) {
            let _ = self.0.insert(lut.0.clone(), LutId(self.0.len()));
        }
    }

    /// Registers an eight-output lookup table, assigning it a fresh [`LutId`].
    ///
    /// If `lut` is already present in the registry (matched by table contents, not by
    /// name), this method does nothing. Otherwise a new [`LutId`] is assigned sequentially
    /// and the table is stored for later retrieval via [`get_l8`](Self::get_l8) or
    /// [`get_l8_lid`](Self::get_l8_lid).
    pub fn register_l8(&mut self, lut: &Lut8) {
        if !self.0.has_dom(&lut.0) {
            let _ = self.0.insert(lut.0.clone(), LutId(self.0.len()));
        }
    }

    /// Returns the [`LutId`] previously assigned to a single-output lookup table.
    ///
    /// Looks up `lut` by its table contents and returns the identifier that was assigned
    /// when the table was first registered.
    ///
    /// # Panics
    ///
    /// Panics if `lut` has not been registered.
    pub fn get_l1_lid(&self, lut: &Lut1) -> LutId {
        *self.0.get_dom(&lut.0).expect("Failed to get lut {lut:?}")
    }

    /// Returns the [`LutId`] previously assigned to a two-output lookup table.
    ///
    /// Looks up `lut` by its table contents and returns the identifier that was assigned
    /// when the table was first registered.
    ///
    /// # Panics
    ///
    /// Panics if `lut` has not been registered.
    pub fn get_l2_lid(&self, lut: &Lut2) -> LutId {
        *self.0.get_dom(&lut.0).expect("Failed to get lut {lut:?}")
    }

    /// Returns the [`LutId`] previously assigned to a four-output lookup table.
    ///
    /// Looks up `lut` by its table contents and returns the identifier that was assigned
    /// when the table was first registered.
    ///
    /// # Panics
    ///
    /// Panics if `lut` has not been registered.
    pub fn get_l4_lid(&self, lut: &Lut4) -> LutId {
        *self.0.get_dom(&lut.0).expect("Failed to get lut {lut:?}")
    }

    /// Returns the [`LutId`] previously assigned to an eight-output lookup table.
    ///
    /// Looks up `lut` by its table contents and returns the identifier that was assigned
    /// when the table was first registered.
    ///
    /// # Panics
    ///
    /// Panics if `lut` has not been registered.
    pub fn get_l8_lid(&self, lut: &Lut8) -> LutId {
        *self.0.get_dom(&lut.0).expect("Failed to get lut {lut:?}")
    }

    /// Returns the raw lookup table associated with the given identifier.
    ///
    /// Retrieves the underlying [`RawLut`] stored under `lid`, without any typed wrapper.
    /// This is useful when the table width is not known at the call site.
    ///
    /// # Panics
    ///
    /// Panics if `lid` is not present in the registry.
    pub fn get_raw_lut(&self, lid: &LutId) -> &RawLut {
        self.0.get_codom(lid).expect("Failed to get lid {lid}")
    }

    /// Returns the single-output lookup table associated with the given identifier.
    ///
    /// Retrieves the [`RawLut`] stored under `lid` and reinterprets it as a [`Lut1`].
    /// The caller is responsible for ensuring that `lid` was originally assigned to a
    /// single-output table; using an identifier from a different table width produces an
    /// incorrectly typed result.
    ///
    /// # Panics
    ///
    /// Panics if `lid` is not present in the registry.
    pub fn get_l1(&self, lid: &LutId) -> &Lut1 {
        let raw = self.0.get_codom(lid).expect("Failed to get lid {lid}");
        unsafe { std::mem::transmute::<&RawLut, &Lut1>(raw) }
    }

    /// Returns the two-output lookup table associated with the given identifier.
    ///
    /// Retrieves the [`RawLut`] stored under `lid` and reinterprets it as a [`Lut2`].
    /// The caller is responsible for ensuring that `lid` was originally assigned to a
    /// two-output table; using an identifier from a different table width produces an
    /// incorrectly typed result.
    ///
    /// # Panics
    ///
    /// Panics if `lid` is not present in the registry.
    pub fn get_l2(&self, lid: &LutId) -> &Lut2 {
        let raw = self.0.get_codom(lid).expect("Failed to get lid {lid}");
        unsafe { std::mem::transmute::<&RawLut, &Lut2>(raw) }
    }

    /// Returns the four-output lookup table associated with the given identifier.
    ///
    /// Retrieves the [`RawLut`] stored under `lid` and reinterprets it as a [`Lut4`].
    /// The caller is responsible for ensuring that `lid` was originally assigned to a
    /// four-output table; using an identifier from a different table width produces an
    /// incorrectly typed result.
    ///
    /// # Panics
    ///
    /// Panics if `lid` is not present in the registry.
    pub fn get_l4(&self, lid: &LutId) -> &Lut4 {
        let raw = self.0.get_codom(lid).expect("Failed to get lid {lid}");
        unsafe { std::mem::transmute::<&RawLut, &Lut4>(raw) }
    }

    /// Returns the eight-output lookup table associated with the given identifier.
    ///
    /// Retrieves the [`RawLut`] stored under `lid` and reinterprets it as a [`Lut8`].
    /// The caller is responsible for ensuring that `lid` was originally assigned to an
    /// eight-output table; using an identifier from a different table width produces an
    /// incorrectly typed result.
    ///
    /// # Panics
    ///
    /// Panics if `lid` is not present in the registry.
    pub fn get_l8(&self, lid: &LutId) -> &Lut8 {
        let raw = self.0.get_codom(lid).expect("Failed to get lid {lid}");
        unsafe { std::mem::transmute::<&RawLut, &Lut8>(raw) }
    }

    /// Returns an iterator over all registered lookup tables and their identifiers.
    ///
    /// Each item yields a `(&`[`LutId`]`, &`[`RawLut`]`)` pair. Every table that was
    /// successfully registered appears exactly once.
    pub fn iter_luts(&self) -> impl Iterator<Item = (&LutId, &RawLut)> {
        self.0.iter().map(|(d, c)| (c, d))
    }
}
