/*!
Definitions for index bounds checking.
*/

use crate::arena::{Handle, HandleSet, UniqueArena};
use crate::valid;

/// How should code generated by Naga do bounds checks?
///
/// When a vector, matrix, or array index is out of bounds—either negative, or
/// greater than or equal to the number of elements in the type—WGSL requires
/// that some other index of the implementation's choice that is in bounds is
/// used instead. (There are no types with zero elements.)
///
/// Similarly, when out-of-bounds coordinates, array indices, or sample indices
/// are presented to the WGSL `textureLoad` and `textureStore` operations, the
/// operation is redirected to do something safe.
///
/// Different users of Naga will prefer different defaults:
///
/// -   When used as part of a WebGPU implementation, the WGSL specification
///     requires the `Restrict` behavior for array, vector, and matrix accesses,
///     and either the `Restrict` or `ReadZeroSkipWrite` behaviors for texture
///     accesses.
///
/// -   When used by the `wgpu` crate for native development, `wgpu` selects
///     `ReadZeroSkipWrite` as its default.
///
/// -   Naga's own default is `Unchecked`, so that shader translations
///     are as faithful to the original as possible.
///
/// Sometimes the underlying hardware and drivers can perform bounds checks
/// themselves, in a way that performs better than the checks Naga would inject.
/// If you're using native checks like this, then having Naga inject its own
/// checks as well would be redundant, and the `Unchecked` policy is
/// appropriate.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub enum BoundsCheckPolicy {
    /// Replace out-of-bounds indexes with some arbitrary in-bounds index.
    ///
    /// (This does not necessarily mean clamping. For example, interpreting the
    /// index as unsigned and taking the minimum with the largest valid index
    /// would also be a valid implementation. That would map negative indices to
    /// the last element, not the first.)
    Restrict,

    /// Out-of-bounds reads return zero, and writes have no effect.
    ///
    /// When applied to a chain of accesses, like `a[i][j].b[k]`, all index
    /// expressions are evaluated, regardless of whether prior or later index
    /// expressions were in bounds. But all the accesses per se are skipped
    /// if any index is out of bounds.
    ReadZeroSkipWrite,

    /// Naga adds no checks to indexing operations. Generate the fastest code
    /// possible. This is the default for Naga, as a translator, but consumers
    /// should consider defaulting to a safer behavior.
    Unchecked,
}

/// Policies for injecting bounds checks during code generation.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BoundsCheckPolicies {
    /// How should the generated code handle array, vector, or matrix indices
    /// that are out of range?
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub index: BoundsCheckPolicy,

    /// How should the generated code handle array, vector, or matrix indices
    /// that are out of range, when those values live in a [`GlobalVariable`] in
    /// the [`Storage`] or [`Uniform`] address spaces?
    ///
    /// Some graphics hardware provides "robust buffer access", a feature that
    /// ensures that using a pointer cannot access memory outside the 'buffer'
    /// that it was derived from. In Naga terms, this means that the hardware
    /// ensures that pointers computed by applying [`Access`] and
    /// [`AccessIndex`] expressions to a [`GlobalVariable`] whose [`space`] is
    /// [`Storage`] or [`Uniform`] will never read or write memory outside that
    /// global variable.
    ///
    /// When hardware offers such a feature, it is probably undesirable to have
    /// Naga inject bounds checking code for such accesses, since the hardware
    /// can probably provide the same protection more efficiently. However,
    /// bounds checks are still needed on accesses to indexable values that do
    /// not live in buffers, like local variables.
    ///
    /// So, this option provides a separate policy that applies only to accesses
    /// to storage and uniform globals. When depending on hardware bounds
    /// checking, this policy can be `Unchecked` to avoid unnecessary overhead.
    ///
    /// When special hardware support is not available, this should probably be
    /// the same as `index_bounds_check_policy`.
    ///
    /// [`GlobalVariable`]: crate::GlobalVariable
    /// [`space`]: crate::GlobalVariable::space
    /// [`Restrict`]: crate::back::BoundsCheckPolicy::Restrict
    /// [`ReadZeroSkipWrite`]: crate::back::BoundsCheckPolicy::ReadZeroSkipWrite
    /// [`Access`]: crate::Expression::Access
    /// [`AccessIndex`]: crate::Expression::AccessIndex
    /// [`Storage`]: crate::AddressSpace::Storage
    /// [`Uniform`]: crate::AddressSpace::Uniform
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub buffer: BoundsCheckPolicy,

    /// How should the generated code handle image texel loads that are out
    /// of range?
    ///
    /// This controls the behavior of [`ImageLoad`] expressions when a coordinate,
    /// texture array index, level of detail, or multisampled sample number is out of range.
    ///
    /// There is no corresponding policy for [`ImageStore`] statements. All the
    /// platforms we support already discard out-of-bounds image stores,
    /// effectively implementing the "skip write" part of [`ReadZeroSkipWrite`].
    ///
    /// [`ImageLoad`]: crate::Expression::ImageLoad
    /// [`ImageStore`]: crate::Statement::ImageStore
    /// [`ReadZeroSkipWrite`]: BoundsCheckPolicy::ReadZeroSkipWrite
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub image_load: BoundsCheckPolicy,

    /// How should the generated code handle binding array indexes that are out of bounds.
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub binding_array: BoundsCheckPolicy,
}

/// The default `BoundsCheckPolicy` is `Unchecked`.
impl Default for BoundsCheckPolicy {
    fn default() -> Self {
        BoundsCheckPolicy::Unchecked
    }
}

impl BoundsCheckPolicies {
    /// Determine which policy applies to `base`.
    ///
    /// `base` is the "base" expression (the expression being indexed) of a `Access`
    /// and `AccessIndex` expression. This is either a pointer, a value, being directly
    /// indexed, or a binding array.
    ///
    /// See the documentation for [`BoundsCheckPolicy`] for details about
    /// when each policy applies.
    pub fn choose_policy(
        &self,
        base: Handle<crate::Expression>,
        types: &UniqueArena<crate::Type>,
        info: &valid::FunctionInfo,
    ) -> BoundsCheckPolicy {
        let ty = info[base].ty.inner_with(types);

        if let crate::TypeInner::BindingArray { .. } = *ty {
            return self.binding_array;
        }

        match ty.pointer_space() {
            Some(crate::AddressSpace::Storage { access: _ } | crate::AddressSpace::Uniform) => {
                self.buffer
            }
            // This covers other address spaces, but also accessing vectors and
            // matrices by value, where no pointer is involved.
            _ => self.index,
        }
    }

    /// Return `true` if any of `self`'s policies are `policy`.
    pub fn contains(&self, policy: BoundsCheckPolicy) -> bool {
        self.index == policy || self.buffer == policy || self.image_load == policy
    }
}

/// An index that may be statically known, or may need to be computed at runtime.
///
/// This enum lets us handle both [`Access`] and [`AccessIndex`] expressions
/// with the same code.
///
/// [`Access`]: crate::Expression::Access
/// [`AccessIndex`]: crate::Expression::AccessIndex
#[derive(Clone, Copy, Debug)]
pub enum GuardedIndex {
    Known(u32),
    Expression(Handle<crate::Expression>),
}

/// Build a set of expressions used as indices, to cache in temporary variables when
/// emitted.
///
/// Given the bounds-check policies `policies`, construct a `HandleSet` containing the handle
/// indices of all the expressions in `function` that are ever used as guarded indices
/// under the [`ReadZeroSkipWrite`] policy. The `module` argument must be the module to
/// which `function` belongs, and `info` should be that function's analysis results.
///
/// Such index expressions will be used twice in the generated code: first for the
/// comparison to see if the index is in bounds, and then for the access itself, should
/// the comparison succeed. To avoid computing the expressions twice, the generated code
/// should cache them in temporary variables.
///
/// Why do we need to build such a set in advance, instead of just processing access
/// expressions as we encounter them? Whether an expression needs to be cached depends on
/// whether it appears as something like the [`index`] operand of an [`Access`] expression
/// or the [`level`] operand of an [`ImageLoad`] expression, and on the index bounds check
/// policies that apply to those accesses. But [`Emit`] statements just identify a range
/// of expressions by index; there's no good way to tell what an expression is used
/// for. The only way to do it is to just iterate over all the expressions looking for
/// relevant `Access` expressions --- which is what this function does.
///
/// Simple expressions like variable loads and constants don't make sense to cache: it's
/// no better than just re-evaluating them. But constants are not covered by `Emit`
/// statements, and `Load`s are always cached to ensure they occur at the right time, so
/// we don't bother filtering them out from this set.
///
/// Fortunately, we don't need to deal with [`ImageStore`] statements here. When we emit
/// code for a statement, the writer isn't in the middle of an expression, so we can just
/// emit declarations for temporaries, initialized appropriately.
///
/// None of these concerns apply for SPIR-V output, since it's easy to just reuse an
/// instruction ID in two places; that has the same semantics as a temporary variable, and
/// it's inherent in the design of SPIR-V. This function is more useful for text-based
/// back ends.
///
/// [`ReadZeroSkipWrite`]: BoundsCheckPolicy::ReadZeroSkipWrite
/// [`index`]: crate::Expression::Access::index
/// [`Access`]: crate::Expression::Access
/// [`level`]: crate::Expression::ImageLoad::level
/// [`ImageLoad`]: crate::Expression::ImageLoad
/// [`Emit`]: crate::Statement::Emit
/// [`ImageStore`]: crate::Statement::ImageStore
pub fn find_checked_indexes(
    module: &crate::Module,
    function: &crate::Function,
    info: &valid::FunctionInfo,
    policies: BoundsCheckPolicies,
) -> HandleSet<crate::Expression> {
    use crate::Expression as Ex;

    let mut guarded_indices = HandleSet::for_arena(&function.expressions);

    // Don't bother scanning if we never need `ReadZeroSkipWrite`.
    if policies.contains(BoundsCheckPolicy::ReadZeroSkipWrite) {
        for (_handle, expr) in function.expressions.iter() {
            // There's no need to handle `AccessIndex` expressions, as their
            // indices never need to be cached.
            match *expr {
                Ex::Access { base, index } => {
                    if policies.choose_policy(base, &module.types, info)
                        == BoundsCheckPolicy::ReadZeroSkipWrite
                        && access_needs_check(
                            base,
                            GuardedIndex::Expression(index),
                            module,
                            &function.expressions,
                            info,
                        )
                        .is_some()
                    {
                        guarded_indices.insert(index);
                    }
                }
                Ex::ImageLoad {
                    coordinate,
                    array_index,
                    sample,
                    level,
                    ..
                } => {
                    if policies.image_load == BoundsCheckPolicy::ReadZeroSkipWrite {
                        guarded_indices.insert(coordinate);
                        if let Some(array_index) = array_index {
                            guarded_indices.insert(array_index);
                        }
                        if let Some(sample) = sample {
                            guarded_indices.insert(sample);
                        }
                        if let Some(level) = level {
                            guarded_indices.insert(level);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    guarded_indices
}

/// Determine whether `index` is statically known to be in bounds for `base`.
///
/// If we can't be sure that the index is in bounds, return the limit within
/// which valid indices must fall.
///
/// The return value is one of the following:
///
/// - `Some(Known(n))` indicates that `n` is the largest valid index.
///
/// - `Some(Computed(global))` indicates that the largest valid index is one
///   less than the length of the array that is the last member of the
///   struct held in `global`.
///
/// - `None` indicates that the index need not be checked, either because it
///   is statically known to be in bounds, or because the applicable policy
///   is `Unchecked`.
///
/// This function only handles subscriptable types: arrays, vectors, and
/// matrices. It does not handle struct member indices; those never require
/// run-time checks, so it's best to deal with them further up the call
/// chain.
pub fn access_needs_check(
    base: Handle<crate::Expression>,
    mut index: GuardedIndex,
    module: &crate::Module,
    expressions: &crate::Arena<crate::Expression>,
    info: &valid::FunctionInfo,
) -> Option<IndexableLength> {
    let base_inner = info[base].ty.inner_with(&module.types);
    // Unwrap safety: `Err` here indicates unindexable base types and invalid
    // length constants, but `access_needs_check` is only used by back ends, so
    // validation should have caught those problems.
    let length = base_inner.indexable_length(module).unwrap();
    index.try_resolve_to_constant(expressions, module);
    if let (&GuardedIndex::Known(index), &IndexableLength::Known(length)) = (&index, &length) {
        if index < length {
            // Index is statically known to be in bounds, no check needed.
            return None;
        }
    };

    Some(length)
}

impl GuardedIndex {
    /// Make a `GuardedIndex::Known` from a `GuardedIndex::Expression` if possible.
    ///
    /// Return values that are already `Known` unchanged.
    pub(crate) fn try_resolve_to_constant(
        &mut self,
        expressions: &crate::Arena<crate::Expression>,
        module: &crate::Module,
    ) {
        if let GuardedIndex::Expression(expr) = *self {
            *self = GuardedIndex::from_expression(expr, expressions, module);
        }
    }

    pub(crate) fn from_expression(
        expr: Handle<crate::Expression>,
        expressions: &crate::Arena<crate::Expression>,
        module: &crate::Module,
    ) -> Self {
        match module.to_ctx().eval_expr_to_u32_from(expr, expressions) {
            Ok(value) => Self::Known(value),
            Err(_) => Self::Expression(expr),
        }
    }
}

#[derive(Clone, Copy, Debug, thiserror::Error, PartialEq)]
pub enum IndexableLengthError {
    #[error("Type is not indexable, and has no length (validation error)")]
    TypeNotIndexable,
    #[error("Array length constant {0:?} is invalid")]
    InvalidArrayLength(Handle<crate::Expression>),
}

impl crate::TypeInner {
    /// Return the length of a subscriptable type.
    ///
    /// The `self` parameter should be a handle to a vector, matrix, or array
    /// type, a pointer to one of those, or a value pointer. Arrays may be
    /// fixed-size, dynamically sized, or sized by a specializable constant.
    /// This function does not handle struct member references, as with
    /// `AccessIndex`.
    ///
    /// The value returned is appropriate for bounds checks on subscripting.
    ///
    /// Return an error if `self` does not describe a subscriptable type at all.
    pub fn indexable_length(
        &self,
        module: &crate::Module,
    ) -> Result<IndexableLength, IndexableLengthError> {
        use crate::TypeInner as Ti;
        let known_length = match *self {
            Ti::Vector { size, .. } => size as _,
            Ti::Matrix { columns, .. } => columns as _,
            Ti::Array { size, .. } | Ti::BindingArray { size, .. } => {
                return size.to_indexable_length(module);
            }
            Ti::ValuePointer {
                size: Some(size), ..
            } => size as _,
            Ti::Pointer { base, .. } => {
                // When assigning types to expressions, ResolveContext::Resolve
                // does a separate sub-match here instead of a full recursion,
                // so we'll do the same.
                let base_inner = &module.types[base].inner;
                match *base_inner {
                    Ti::Vector { size, .. } => size as _,
                    Ti::Matrix { columns, .. } => columns as _,
                    Ti::Array { size, .. } | Ti::BindingArray { size, .. } => {
                        return size.to_indexable_length(module)
                    }
                    _ => return Err(IndexableLengthError::TypeNotIndexable),
                }
            }
            _ => return Err(IndexableLengthError::TypeNotIndexable),
        };
        Ok(IndexableLength::Known(known_length))
    }
}

/// The number of elements in an indexable type.
///
/// This summarizes the length of vectors, matrices, and arrays in a way that is
/// convenient for indexing and bounds-checking code.
#[derive(Debug)]
pub enum IndexableLength {
    /// Values of this type always have the given number of elements.
    Known(u32),

    /// The number of elements is determined at runtime.
    Dynamic,
}

impl crate::ArraySize {
    pub const fn to_indexable_length(
        self,
        _module: &crate::Module,
    ) -> Result<IndexableLength, IndexableLengthError> {
        Ok(match self {
            Self::Constant(length) => IndexableLength::Known(length.get()),
            Self::Dynamic => IndexableLength::Dynamic,
        })
    }
}
