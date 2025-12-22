use bytemuck::{Pod, Zeroable};
use shank::ShankType;

/// Nested data structure for MyAccount.
///
/// # Layout
/// - field1: 2 bytes
/// - _padding: 2 bytes (alignment for field2)
/// - field2: 4 bytes
///
/// Total: 8 bytes (8-byte aligned)
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Pod, Zeroable, ShankType)]
pub struct MyData {
    /// Some description for field1.
    pub field1: u16,
    /// Padding for alignment.
    pub _padding: [u8; 2],
    /// Some description for field2.
    pub field2: u32,
}

// Compile-time assertion to ensure struct is 8-byte aligned.
const _: () = assert!(core::mem::size_of::<MyData>() % 8 == 0);
const _: () = assert!(core::mem::size_of::<MyData>() == 8);

impl MyData {
    /// The length of this data structure in bytes.
    pub const LEN: usize = core::mem::size_of::<MyData>();

    /// Create a new MyData instance.
    #[inline]
    pub fn new(field1: u16, field2: u32) -> Self {
        Self {
            field1,
            _padding: [0u8; 2],
            field2,
        }
    }
}
