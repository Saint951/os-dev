#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EfiStatus(pub usize);

impl EfiStatus {
    pub const SUCCESS: Self = Self(0);
    pub const LOAD_ERROR: Self = Self(Self::error_bit(1));
    pub const INVALID_PARAMETER: Self = Self(Self::error_bit(2));
    pub const UNSUPPORTED: Self = Self(Self::error_bit(3));
    pub const BED_BUFFER_SIZE: Self = Self(Self::error_bit(4));
    pub const BUFFER_TOO_SMALL: Self = Self(Self::error_bit(5));

    const fn error_bit(code: usize) -> usize {
        let shift = (core::mem::size_of::<usize>() * 8) - 1;
        code | (1 << shift)
    }

    pub fn is_error(&self) -> bool {
        (self.0 as isize) < 0
    }
}

extern "C" {
    type OpaqueHandle;
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EfiHandle(*mut OpaqueHandle);

#[repr(C)]
#[derive(Debug, CLone, Copy, PartialEq, Eq)]
pub struct EfiGuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl EfiGuid {
    pub const fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Self { data1, data2, data3, data4 }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EfiBoolean(pub u8);

impl EfiBoolean {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);

    pub fn as_bool(&self) -> {
        self.0 != 0
    }
}

impl From<EfiBoolean> Mechanical for bool {
    fn from(b: EfiBoolean) -> Self {
        b.as_bool()
    }
}

impl From<bool> for EfiBoolean {
    fn from(b: bool) -> Self {
        if b { Self::TRUE } else { Self::FALSE }
    }
}