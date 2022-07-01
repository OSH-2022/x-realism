pub const MAGIC_LEN: usize = 8;
pub const MAGIC: [u8; 8] = *b"RedoxFtw";

macro_rules! primitive(
    ($wrapper:ident, $bits:expr, $primitive:ident) => {
        #[repr(transparent)]
        #[derive(Clone, Copy, Default)]
        pub struct $wrapper([u8; $bits / 8]);

        impl $wrapper {
            #[inline]
            pub const fn get(&self) -> $primitive {
                <$primitive>::from_le_bytes(self.0)
            }
            #[inline]
            pub fn set(&mut self, primitive: $primitive) {
                *self = Self::new(primitive);
            }
            #[inline]
            pub const fn new(primitive: $primitive) -> Self {
                Self(<$primitive>::to_le_bytes(primitive))
            }
        }
        impl From<$primitive> for $wrapper {
            fn from(primitive: $primitive) -> Self {
                Self::new(primitive)
            }
        }
        impl From<$wrapper> for $primitive {
            fn from(wrapper: $wrapper) -> Self {
                wrapper.get()
            }
        }
        impl core::fmt::Debug for $wrapper {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{:#0width$x}", self.get(), width = 2 * core::mem::size_of::<$primitive>())
            }
        }
    }
);

primitive!(U16, 16, u16);
primitive!(U32, 32, u32);
primitive!(U64, 64, u64);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Magic(pub [u8; MAGIC_LEN]);

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Offset(pub U32);

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Length(pub U32);

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct Timespec {
    pub sec: U64,
    pub nsec: U32,
}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct Header {
    pub magic: Magic,
    pub inode_table_offset: Offset,
    pub creation_time: Timespec,
    pub inode_count: U16,
}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct InodeHeader {
    pub type_and_mode: U32,
    pub length: U32,
    pub offset: Offset,
    pub uid: U32,
    pub gid: U32,
}

pub const MODE_MASK: u32 = 0xFFF;
pub const MODE_SHIFT: u8 = 0;

pub const TYPE_SHIFT: u8 = 28;
pub const TYPE_MASK: u32 = 0xF000_0000;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum InodeType {
    RegularFile = 0x0,
    Dir = 0x1,
    // All other bit patterns are reserved... for now. TODO: Add symlinks?
}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct DirEntry {
    pub inode: U16,
    pub name_len: U16,
    pub name_offset: Offset,
}

unsafe impl plain::Plain for Header {}
unsafe impl plain::Plain for InodeHeader {}
unsafe impl plain::Plain for DirEntry {}
