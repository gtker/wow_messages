#[doc(hidden)]
pub(crate) mod private {
    pub trait Sealed {}

    pub(crate) struct Internal;

    impl Sealed for Internal {}
}

#[doc(hidden)]
pub trait Message: Sized + private::Sealed {
    const OPCODE: u32;

    fn size_without_header(&self) -> u32;

    fn write_into_vec(&self, w: impl std::io::Write) -> Result<(), std::io::Error>;

    fn read_body<S: private::Sealed>(
        r: &mut &[u8],
        body_size: u32,
    ) -> Result<Self, crate::errors::ParseError>;
}
