use crate::{with_session_globals, Span};
use rsolc_data_structures::{index::BaseIndex32, map::FxIndexSet, sync::Lock};
use rsolc_macros::symbols;
use std::{cmp, fmt, hash, str};

// The proc macro code for this is in `crates/macros/src/symbols/mod.rs`.
symbols! {
    Keywords {
        Abstract:    "abstract",
        Address:     "address",
        Anonymous:   "anonymous",
        As:          "as",
        Assembly:    "assembly",
        Bool:        "bool",
        Break:       "break",
        Byte:        "byte",
        Bytes:       "bytes",
        Bytes1:      "bytes1",
        Bytes2:      "bytes2",
        Bytes3:      "bytes3",
        Bytes4:      "bytes4",
        Bytes5:      "bytes5",
        Bytes6:      "bytes6",
        Bytes7:      "bytes7",
        Bytes8:      "bytes8",
        Bytes9:      "bytes9",
        Bytes10:     "bytes10",
        Bytes11:     "bytes11",
        Bytes12:     "bytes12",
        Bytes13:     "bytes13",
        Bytes14:     "bytes14",
        Bytes15:     "bytes15",
        Bytes16:     "bytes16",
        Bytes17:     "bytes17",
        Bytes18:     "bytes18",
        Bytes19:     "bytes19",
        Bytes20:     "bytes20",
        Bytes21:     "bytes21",
        Bytes22:     "bytes22",
        Bytes23:     "bytes23",
        Bytes24:     "bytes24",
        Bytes25:     "bytes25",
        Bytes26:     "bytes26",
        Bytes27:     "bytes27",
        Bytes28:     "bytes28",
        Bytes29:     "bytes29",
        Bytes30:     "bytes30",
        Bytes31:     "bytes31",
        Bytes32:     "bytes32",
        Calldata:    "calldata",
        Catch:       "catch",
        Constant:    "constant",
        Constructor: "constructor",
        Continue:    "continue",
        Contract:    "contract",
        Delete:      "delete",
        Do:          "do",
        Else:        "else",
        Emit:        "emit",
        Enum:        "enum",
        Event:       "event",
        External:    "external",
        Fallback:    "fallback",
        False:       "false",
        For:         "for",
        Function:    "function",
        If:          "if",
        Immutable:   "immutable",
        Import:      "import",
        Indexed:     "indexed",
        Int:         "int",
        Int8:        "int8",
        Int16:       "int16",
        Int24:       "int24",
        Int32:       "int32",
        Int40:       "int40",
        Int48:       "int48",
        Int56:       "int56",
        Int64:       "int64",
        Int72:       "int72",
        Int80:       "int80",
        Int88:       "int88",
        Int96:       "int96",
        Int104:      "int104",
        Int112:      "int112",
        Int120:      "int120",
        Int128:      "int128",
        Int136:      "int136",
        Int144:      "int144",
        Int152:      "int152",
        Int160:      "int160",
        Int168:      "int168",
        Int176:      "int176",
        Int184:      "int184",
        Int192:      "int192",
        Int200:      "int200",
        Int208:      "int208",
        Int216:      "int216",
        Int224:      "int224",
        Int232:      "int232",
        Int240:      "int240",
        Int248:      "int248",
        Int256:      "int256",
        Interface:   "interface",
        Internal:    "internal",
        Is:          "is",
        Let:         "let",
        Library:     "library",
        Mapping:     "mapping",
        Memory:      "memory",
        Modifier:    "modifier",
        New:         "new",
        Override:    "override",
        Payable:     "payable",
        Pragma:      "pragma",
        Private:     "private",
        Public:      "public",
        Pure:        "pure",
        Receive:     "receive",
        Return:      "return",
        Returns:     "returns",
        Storage:     "storage",
        String:      "string",
        Struct:      "struct",
        This:        "this",
        Throw:       "throw",
        True:        "true",
        Try:         "try",
        Type:        "type",
        Uint:        "uint",
        Uint8:       "uint8",
        Uint16:      "uint16",
        Uint24:      "uint24",
        Uint32:      "uint32",
        Uint40:      "uint40",
        Uint48:      "uint48",
        Uint56:      "uint56",
        Uint64:      "uint64",
        Uint72:      "uint72",
        Uint80:      "uint80",
        Uint88:      "uint88",
        Uint96:      "uint96",
        Uint104:     "uint104",
        Uint112:     "uint112",
        Uint120:     "uint120",
        Uint128:     "uint128",
        Uint136:     "uint136",
        Uint144:     "uint144",
        Uint152:     "uint152",
        Uint160:     "uint160",
        Uint168:     "uint168",
        Uint176:     "uint176",
        Uint184:     "uint184",
        Uint192:     "uint192",
        Uint200:     "uint200",
        Uint208:     "uint208",
        Uint216:     "uint216",
        Uint224:     "uint224",
        Uint232:     "uint232",
        Uint240:     "uint240",
        Uint248:     "uint248",
        Uint256:     "uint256",
        Unchecked:   "unchecked",
        Using:       "using",
        View:        "view",
        Virtual:     "virtual",
        While:       "while",

        // Weak keywords, can be used as variable names
        Case:        "case",
        Default:     "default",
        Leave:       "leave",
        Revert:      "revert",
        Switch:      "switch",
    }

    // Pre-interned symbols that can be referred to with `sym::*`.
    //
    // The symbol is the stringified identifier unless otherwise specified, in
    // which case the name should mention the non-identifier punctuation.
    // E.g. `sym::proc_dash_macro` represents "proc-macro", and it shouldn't be
    // called `sym::proc_macro` because then it's easy to mistakenly think it
    // represents "proc_macro".
    //
    // As well as the symbols listed, there are symbols for the strings
    // "0", "1", ..., "9", which are accessible via `sym::integer`.
    //
    // The proc macro will abort if symbols are not in alphabetical order (as
    // defined by `impl Ord for str`) or if any symbols are duplicated. Vim
    // users can sort the list by selecting it and executing the command
    // `:'<,'>!LC_ALL=C sort`.
    //
    // There is currently no checking that all symbols are used; that would be
    // nice to have.
    Symbols {
        underscore: "_",
    }
}

// This module has a very short name because it's used a lot.
/// This module contains all the defined keyword `Symbol`s.
///
/// Given that `kw` is imported, use them like `kw::keyword_name`.
/// For example `kw::For` or `kw::Break`.
pub mod kw {
    #[doc(inline)]
    pub use super::kw_generated::*;
}

// This module has a very short name because it's used a lot.
/// This module contains all the defined non-keyword `Symbol`s.
///
/// Given that `sym` is imported, use them like `sym::symbol_name`.
/// For example `sym::rustfmt` or `sym::u8`.
pub mod sym {
    use super::Symbol;

    #[doc(inline)]
    pub use super::sym_generated::*;

    /// Get the symbol for an integer.
    ///
    /// The first few non-negative integers each have a static symbol and therefore are fast.
    pub fn integer<N: TryInto<usize> + Copy + ToString>(n: N) -> Symbol {
        if let Ok(idx) = n.try_into() {
            if idx < 10 {
                return Symbol::new(super::SYMBOL_DIGITS_BASE + idx as u32);
            }
        }
        Symbol::intern(&n.to_string())
    }
}

/// An identifier.
#[derive(Clone, Copy)]
pub struct Ident {
    /// The identifier's name.
    pub name: Symbol,
    /// The identifier's span.
    pub span: Span,
}

impl Ident {
    /// Constructs a new identifier from a symbol and a span.
    #[inline]
    pub const fn new(name: Symbol, span: Span) -> Self {
        Self { name, span }
    }

    /// Constructs a new identifier with a dummy span.
    #[inline]
    pub const fn with_dummy_span(name: Symbol) -> Self {
        Self::new(name, Span::DUMMY)
    }

    // #[inline]
    // pub fn empty() -> Ident {
    //     Ident::with_dummy_span(kw::Empty)
    // }

    /// Maps a string to an identifier with a dummy span.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(string: &str) -> Self {
        Self::with_dummy_span(Symbol::intern(string))
    }

    /// Maps a string and a span to an identifier.
    pub fn from_str_and_span(string: &str, span: Span) -> Self {
        Self::new(Symbol::intern(string), span)
    }

    // /// Replaces `lo` and `hi` with those from `span`, but keep hygiene context.
    // pub fn with_span_pos(self, span: Span) -> Ident {
    //     Ident::new(self.name, span.with_ctxt(self.span.ctxt()))
    // }

    pub fn without_first_quote(self) -> Self {
        Self::new(Symbol::intern(self.as_str().trim_start_matches('\'')), self.span)
    }

    /// Access the underlying string. This is a slowish operation because it
    /// requires locking the symbol interner.
    ///
    /// Note that the lifetime of the return value is a lie. See
    /// [`Symbol::as_str()`] for details.
    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
}

impl PartialEq for Ident {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.name == rhs.name
        // && self.span.eq_ctxt(rhs.span)
    }
}

impl Eq for Ident {}

impl PartialOrd for Ident {
    #[inline]
    fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Ident {
    #[inline]
    fn cmp(&self, rhs: &Self) -> cmp::Ordering {
        self.name.cmp(&rhs.name)
    }
}

impl hash::Hash for Ident {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        // self.span.ctxt().hash(state);
    }
}

impl fmt::Debug for Ident {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
        // fmt::Debug::fmt(&self.span.ctxt(), f)
    }
}

impl fmt::Display for Ident {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)
    }
}

/// An interned string.
///
/// Internally, a `Symbol` is implemented as an index, and all operations
/// (including hashing, equality, and ordering) operate on that index.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(BaseIndex32);

impl Symbol {
    const fn new(n: u32) -> Self {
        Self(BaseIndex32::new(n))
    }

    /// Maps a string to its interned representation.
    pub fn intern(string: &str) -> Self {
        with_session_globals(|session_globals| session_globals.symbol_interner.intern(string))
    }

    /// Access the underlying string. This is a slowish operation because it
    /// requires locking the symbol interner.
    ///
    /// Note that the lifetime of the return value is a lie. It's not the same
    /// as `&self`, but actually tied to the lifetime of the underlying
    /// interner. Interners are long-lived, and there are very few of them, and
    /// this function is typically used for short-lived things, so in practice
    /// it works out ok.
    pub fn as_str(&self) -> &str {
        with_session_globals(|session_globals| unsafe {
            std::mem::transmute::<&str, &str>(session_globals.symbol_interner.get(*self))
        })
    }

    /// "Specialization" of [`ToString`] using [`as_str`](Self::as_str).
    #[inline]
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }

    /// Returns the internal representation of the symbol.
    #[inline]
    pub const fn as_u32(self) -> u32 {
        self.0.get()
    }

    /// Returns `true` if the symbol is a weak keyword and can be used in variable names.
    #[inline]
    pub const fn is_weak_keyword(self) -> bool {
        matches!(self, kw::Case | kw::Default | kw::Leave | kw::Revert | kw::Switch)
    }

    /// Returns `true` if the symbol is `true` or `false`.
    #[inline]
    pub const fn is_bool_lit(self) -> bool {
        matches!(self, kw::True | kw::False)
    }

    /// Returns `true` if the symbol was interned in the compiler's `symbols!` macro.
    #[inline]
    pub const fn is_preinterned(self) -> bool {
        self.as_u32() < PREINTERNED_SYMBOLS_COUNT
    }
}

impl fmt::Debug for Symbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl fmt::Display for Symbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

// no spec
// impl ToString for Symbol {
//     fn to_string(&self) -> String {
//         self.as_str().to_string()
//     }
// }

pub(crate) struct Interner(Lock<InternerInner>);

// The `&'static str`s in this type actually point into the arena.
//
// This type is private to prevent accidentally constructing more than one
// `Interner` on the same thread, which makes it easy to mix up `Symbol`s
// between `Interner`s.
struct InternerInner {
    arena: bumpalo::Bump,
    strings: FxIndexSet<&'static str>,
}

impl Interner {
    fn prefill(init: &[&'static str]) -> Self {
        Self(Lock::new(InternerInner {
            arena: Default::default(),
            strings: init.iter().copied().collect(),
        }))
    }

    #[inline]
    fn intern(&self, string: &str) -> Symbol {
        let mut inner = self.0.lock();
        if let Some(idx) = inner.strings.get_index_of(string) {
            return Symbol::new(idx as u32);
        }

        let string: &str = inner.arena.alloc_str(string);

        // SAFETY: we can extend the arena allocation to `'static` because we
        // only access these while the arena is still alive.
        let string: &'static str = unsafe { &*(string as *const str) };

        // This second hash table lookup can be avoided by using `RawEntryMut`,
        // but this code path isn't hot enough for it to be worth it. See
        // #91445 for details.
        let (idx, is_new) = inner.strings.insert_full(string);
        debug_assert!(is_new); // due to the get_index_of check above

        Symbol::new(idx as u32)
    }

    /// Get the symbol as a string.
    ///
    /// [`Symbol::as_str()`] should be used in preference to this function.
    fn get(&self, symbol: Symbol) -> &str {
        self.0.lock().strings.get_index(symbol.as_u32() as usize).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interner_tests() {
        let i = Interner::prefill(&[]);
        // first one is zero:
        assert_eq!(i.intern("dog"), Symbol::new(0));
        // re-use gets the same entry:
        assert_eq!(i.intern("dog"), Symbol::new(0));
        // different string gets a different #:
        assert_eq!(i.intern("cat"), Symbol::new(1));
        assert_eq!(i.intern("cat"), Symbol::new(1));
        // dog is still at zero
        assert_eq!(i.intern("dog"), Symbol::new(0));
    }
}
