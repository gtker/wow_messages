use std::convert::{TryFrom, TryInto};
use crate::logon::all::{InnerFlag};
use crate::logon::all::{TestFlag};
use crate::ClientMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TestStruct {
    pub f: TestStructTestFlag,
}

impl ClientMessage for TestStruct {
    const OPCODE: u8 = 0xff;
}
impl ReadableAndWritable for TestStruct {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // f: TestFlag
        let f = TestFlag::read(r)?;

        let f_A = if f.is_A() {
            // b_A1: u8
            let b_A1 = crate::util::read_u8_le(r)?;

            Some(TestStructTestFlagA::A {
                b_A1,
            })
        }
        else if f.is_B() {
            // i: InnerFlag
            let i = InnerFlag::read(r)?;

            let i_H = if i.is_H() {
                // b_H1: u8
                let b_H1 = crate::util::read_u8_le(r)?;

                Some(TestStructInnerFlagH {
                    b_H1,
                })
            } else {
                None
            };

            // b_B1: u8
            let b_B1 = crate::util::read_u8_le(r)?;

            let i = TestStructInnerFlag {
                inner: i.as_int(),
                h: i_H,
            };

            Some(TestStructTestFlagA::B {
                i,
                b_B1,
            })
        }
        else {
            None
        };

        let f_C = if f.is_C() {
            // b_C1: u8
            let b_C1 = crate::util::read_u8_le(r)?;

            Some(TestStructTestFlagC {
                b_C1,
            })
        } else {
            None
        };

        let f_E = if f.is_E() {
            // b_E1: u8
            let b_E1 = crate::util::read_u8_le(r)?;

            Some(TestStructTestFlagE {
                b_E1,
            })
        } else {
            None
        };

        let f = TestStructTestFlag {
            inner: f.as_int(),
            a: f_A,
            c: f_C,
            e: f_E,
        };

        Ok(Self {
            f,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // f: TestFlag
        self.f.write(w)?;

        if let Some(s) = &self.f.a {
            s.write(w)?;
        }

        if let Some(s) = &self.f.c {
            s.write(w)?;
        }

        if let Some(s) = &self.f.e {
            s.write(w)?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // f: TestFlag
            let f = TestFlag::tokio_read(r).await?;

            let f_A = if f.is_A() {
                // b_A1: u8
                let b_A1 = crate::util::tokio_read_u8_le(r).await?;

                Some(TestStructTestFlagA::A {
                    b_A1,
                })
            }
            else if f.is_B() {
                // i: InnerFlag
                let i = InnerFlag::tokio_read(r).await?;

                let i_H = if i.is_H() {
                    // b_H1: u8
                    let b_H1 = crate::util::tokio_read_u8_le(r).await?;

                    Some(TestStructInnerFlagH {
                        b_H1,
                    })
                } else {
                    None
                };

                // b_B1: u8
                let b_B1 = crate::util::tokio_read_u8_le(r).await?;

                let i = TestStructInnerFlag {
                    inner: i.as_int(),
                    h: i_H,
                };

                Some(TestStructTestFlagA::B {
                    i,
                    b_B1,
                })
            }
            else {
                None
            };

            let f_C = if f.is_C() {
                // b_C1: u8
                let b_C1 = crate::util::tokio_read_u8_le(r).await?;

                Some(TestStructTestFlagC {
                    b_C1,
                })
            } else {
                None
            };

            let f_E = if f.is_E() {
                // b_E1: u8
                let b_E1 = crate::util::tokio_read_u8_le(r).await?;

                Some(TestStructTestFlagE {
                    b_E1,
                })
            } else {
                None
            };

            let f = TestStructTestFlag {
                inner: f.as_int(),
                a: f_A,
                c: f_C,
                e: f_E,
            };

            Ok(Self {
                f,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // opcode: u8
            w.write_all(&Self::OPCODE.to_le_bytes()).await?;

            // f: TestFlag
            self.f.tokio_write(w).await?;

            if let Some(s) = &self.f.a {
                s.tokio_write(w).await?;
            }

            if let Some(s) = &self.f.c {
                s.tokio_write(w).await?;
            }

            if let Some(s) = &self.f.e {
                s.tokio_write(w).await?;
            }

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // f: TestFlag
            let f = TestFlag::astd_read(r).await?;

            let f_A = if f.is_A() {
                // b_A1: u8
                let b_A1 = crate::util::astd_read_u8_le(r).await?;

                Some(TestStructTestFlagA::A {
                    b_A1,
                })
            }
            else if f.is_B() {
                // i: InnerFlag
                let i = InnerFlag::astd_read(r).await?;

                let i_H = if i.is_H() {
                    // b_H1: u8
                    let b_H1 = crate::util::astd_read_u8_le(r).await?;

                    Some(TestStructInnerFlagH {
                        b_H1,
                    })
                } else {
                    None
                };

                // b_B1: u8
                let b_B1 = crate::util::astd_read_u8_le(r).await?;

                let i = TestStructInnerFlag {
                    inner: i.as_int(),
                    h: i_H,
                };

                Some(TestStructTestFlagA::B {
                    i,
                    b_B1,
                })
            }
            else {
                None
            };

            let f_C = if f.is_C() {
                // b_C1: u8
                let b_C1 = crate::util::astd_read_u8_le(r).await?;

                Some(TestStructTestFlagC {
                    b_C1,
                })
            } else {
                None
            };

            let f_E = if f.is_E() {
                // b_E1: u8
                let b_E1 = crate::util::astd_read_u8_le(r).await?;

                Some(TestStructTestFlagE {
                    b_E1,
                })
            } else {
                None
            };

            let f = TestStructTestFlag {
                inner: f.as_int(),
                a: f_A,
                c: f_C,
                e: f_E,
            };

            Ok(Self {
                f,
            })
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // opcode: u8
            w.write_all(&Self::OPCODE.to_le_bytes()).await?;

            // f: TestFlag
            self.f.astd_write(w).await?;

            if let Some(s) = &self.f.a {
                s.astd_write(w).await?;
            }

            if let Some(s) = &self.f.c {
                s.astd_write(w).await?;
            }

            if let Some(s) = &self.f.e {
                s.astd_write(w).await?;
            }

            Ok(())
        })
    }

}

impl VariableSized for TestStruct {
    fn size(&self) -> usize {
        0
        + self.f.size() // f: TestStructTestFlag
    }
}

impl MaximumPossibleSized for TestStruct {
    fn maximum_possible_size() -> usize {
        0
        + 7 // f: TestStructTestFlag
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct TestStructInnerFlag {
    inner: u8,
    h: Option<TestStructInnerFlagH>,
}

impl From<&TestStructInnerFlag> for InnerFlag {
    fn from(e: &TestStructInnerFlag) -> Self {
        Self::new(e.inner)
    }
}

impl TestStructInnerFlag {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: InnerFlag = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: InnerFlag = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: InnerFlag = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            h: None,
        }
    }

    pub const fn new_H(h: TestStructInnerFlagH) -> Self {
        Self {
            inner: InnerFlag::H,
            h: Some(h),
        }
    }

    pub fn set_H(&mut self, h: TestStructInnerFlagH) -> Self {
        self.inner |= InnerFlag::H;
        self.h = Some(h);
        self.clone()
    }

    pub const fn get_H(&self) -> Option<&TestStructInnerFlagH> {
        self.h.as_ref()
    }

    pub fn clear_H(&mut self) -> Self {
        self.inner &= InnerFlag::H.reverse_bits();
        self.h = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_I() -> Self {
        Self {
            inner: InnerFlag::I,
            h: None,
        }
    }

    pub fn set_I(&mut self) -> Self {
        self.inner |= InnerFlag::I;
        self.clone()
    }

    pub const fn get_I(&self) -> bool {
        (self.inner & InnerFlag::I) != 0
    }

    pub fn clear_I(&mut self) -> Self {
        self.inner &= InnerFlag::I.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_J() -> Self {
        Self {
            inner: InnerFlag::J,
            h: None,
        }
    }

    pub fn set_J(&mut self) -> Self {
        self.inner |= InnerFlag::J;
        self.clone()
    }

    pub const fn get_J(&self) -> bool {
        (self.inner & InnerFlag::J) != 0
    }

    pub fn clear_J(&mut self) -> Self {
        self.inner &= InnerFlag::J.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_K() -> Self {
        Self {
            inner: InnerFlag::K,
            h: None,
        }
    }

    pub fn set_K(&mut self) -> Self {
        self.inner |= InnerFlag::K;
        self.clone()
    }

    pub const fn get_K(&self) -> bool {
        (self.inner & InnerFlag::K) != 0
    }

    pub fn clear_K(&mut self) -> Self {
        self.inner &= InnerFlag::K.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_L() -> Self {
        Self {
            inner: InnerFlag::L,
            h: None,
        }
    }

    pub fn set_L(&mut self) -> Self {
        self.inner |= InnerFlag::L;
        self.clone()
    }

    pub const fn get_L(&self) -> bool {
        (self.inner & InnerFlag::L) != 0
    }

    pub fn clear_L(&mut self) -> Self {
        self.inner &= InnerFlag::L.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

}
impl VariableSized for TestStructInnerFlag {
    fn size(&self) -> usize {
        1 // inner
        + {
            if let Some(s) = &self.h {
                s.size()
            } else {
                0
            }
        }
    }
}

impl MaximumPossibleSized for TestStructInnerFlag {
    fn maximum_possible_size() -> usize {
        1 // inner
        + TestStructInnerFlagH::maximum_possible_size() // H enumerator
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TestStructInnerFlagH {
    pub b_H1: u8,
}

impl VariableSized for TestStructInnerFlagH {
    fn size(&self) -> usize {
        1 // b_H1: u8
    }
}

impl MaximumPossibleSized for TestStructInnerFlagH {
    fn maximum_possible_size() -> usize {
        1 // b_H1: u8
    }
}

impl TestStructInnerFlagH {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // b_H1: u8
        w.write_all(&self.b_H1.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // b_H1: u8
        w.write_all(&self.b_H1.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // b_H1: u8
        w.write_all(&self.b_H1.to_le_bytes()).await?;

        Ok(())
    }

}

#[derive(Debug, PartialEq, Clone)]
pub enum TestStructTestFlagA {
    A {
        b_A1: u8,
    },
    B {
        b_B1: u8,
        i: TestStructInnerFlag,
    },
}

impl TestStructTestFlagA {
    pub(crate) const fn as_flag_value(&self) -> u16 {
        match self {
            Self::A { .. } => 1,
            Self::B { .. } => 2,
        }
    }

}

impl VariableSized for TestStructTestFlagA {
    fn size(&self) -> usize {
        match self {
            Self::A {
                b_A1,
            } => {
                0 // Not an actual enum sent over the wire
                + 1 // b_A1: u8
            }
            Self::B {
                b_B1,
                i,
            } => {
                0 // Not an actual enum sent over the wire
                + 1 // b_B1: u8
                + i.size() // i: TestStructInnerFlag
            }
        }
    }
}

impl MaximumPossibleSized for TestStructTestFlagA {
    fn maximum_possible_size() -> usize {
        0
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct TestStructTestFlag {
    inner: u16,
    a: Option<TestStructTestFlagA>,
    c: Option<TestStructTestFlagC>,
    e: Option<TestStructTestFlagE>,
}

impl From<&TestStructTestFlag> for TestFlag {
    fn from(e: &TestStructTestFlag) -> Self {
        Self::new(e.inner)
    }
}

impl TestStructTestFlag {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: TestFlag = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: TestFlag = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: TestFlag = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            a: None,
            c: None,
            e: None,
        }
    }

    pub const fn new_A(a: TestStructTestFlagA) -> Self {
        Self {
            inner: a.as_flag_value(),
            a: Some(a),
            c: None,
            e: None,
        }
    }

    pub fn set_A(&mut self, a: TestStructTestFlagA) -> Self {
        self.inner |= a.as_flag_value();
        self.a = Some(a);
        self.clone()
    }

    pub const fn get_A(&self) -> Option<&TestStructTestFlagA> {
        self.a.as_ref()
    }

    pub fn clear_A(&mut self) -> Self {
        self.inner &= TestFlag::A.reverse_bits();
        self.a = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_C(c: TestStructTestFlagC) -> Self {
        Self {
            inner: TestFlag::C,
            a: None,
            c: Some(c),
            e: None,
        }
    }

    pub fn set_C(&mut self, c: TestStructTestFlagC) -> Self {
        self.inner |= TestFlag::C;
        self.c = Some(c);
        self.clone()
    }

    pub const fn get_C(&self) -> Option<&TestStructTestFlagC> {
        self.c.as_ref()
    }

    pub fn clear_C(&mut self) -> Self {
        self.inner &= TestFlag::C.reverse_bits();
        self.c = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_D() -> Self {
        Self {
            inner: TestFlag::D,
            a: None,
            c: None,
            e: None,
        }
    }

    pub fn set_D(&mut self) -> Self {
        self.inner |= TestFlag::D;
        self.clone()
    }

    pub const fn get_D(&self) -> bool {
        (self.inner & TestFlag::D) != 0
    }

    pub fn clear_D(&mut self) -> Self {
        self.inner &= TestFlag::D.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_E(e: TestStructTestFlagE) -> Self {
        Self {
            inner: TestFlag::E,
            a: None,
            c: None,
            e: Some(e),
        }
    }

    pub fn set_E(&mut self, e: TestStructTestFlagE) -> Self {
        self.inner |= TestFlag::E;
        self.e = Some(e);
        self.clone()
    }

    pub const fn get_E(&self) -> Option<&TestStructTestFlagE> {
        self.e.as_ref()
    }

    pub fn clear_E(&mut self) -> Self {
        self.inner &= TestFlag::E.reverse_bits();
        self.e = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

}
impl VariableSized for TestStructTestFlag {
    fn size(&self) -> usize {
        2 // inner
        + {
            if let Some(s) = &self.a {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.c {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.e {
                s.size()
            } else {
                0
            }
        }
    }
}

impl MaximumPossibleSized for TestStructTestFlag {
    fn maximum_possible_size() -> usize {
        2 // inner
        + TestStructTestFlagA::maximum_possible_size() // A enumerator
        + TestStructTestFlagC::maximum_possible_size() // C enumerator
        + TestStructTestFlagE::maximum_possible_size() // E enumerator
    }
}

impl TestStructTestFlagA {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        match &self {
            Self::A {
                b_A1,
            } => {
                // b_A1: u8
                w.write_all(&b_A1.to_le_bytes())?;

            }
            Self::B {
                b_B1,
                i,
            } => {
                // i: InnerFlag
                i.write(w)?;

                if let Some(s) = &i.h {
                    s.write(w)?;
                }

                // b_B1: u8
                w.write_all(&b_B1.to_le_bytes())?;

            }
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        match &self {
            Self::A {
                b_A1,
            } => {
                // b_A1: u8
                w.write_all(&b_A1.to_le_bytes()).await?;

            }
            Self::B {
                b_B1,
                i,
            } => {
                // i: InnerFlag
                i.tokio_write(w).await?;

                if let Some(s) = &i.h {
                    s.tokio_write(w).await?;
                }

                // b_B1: u8
                w.write_all(&b_B1.to_le_bytes()).await?;

            }
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        match &self {
            Self::A {
                b_A1,
            } => {
                // b_A1: u8
                w.write_all(&b_A1.to_le_bytes()).await?;

            }
            Self::B {
                b_B1,
                i,
            } => {
                // i: InnerFlag
                i.astd_write(w).await?;

                if let Some(s) = &i.h {
                    s.astd_write(w).await?;
                }

                // b_B1: u8
                w.write_all(&b_B1.to_le_bytes()).await?;

            }
        }

        Ok(())
    }

}

#[derive(Debug, PartialEq, Clone)]
pub struct TestStructTestFlagC {
    pub b_C1: u8,
}

impl VariableSized for TestStructTestFlagC {
    fn size(&self) -> usize {
        1 // b_C1: u8
    }
}

impl MaximumPossibleSized for TestStructTestFlagC {
    fn maximum_possible_size() -> usize {
        1 // b_C1: u8
    }
}

impl TestStructTestFlagC {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // b_C1: u8
        w.write_all(&self.b_C1.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // b_C1: u8
        w.write_all(&self.b_C1.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // b_C1: u8
        w.write_all(&self.b_C1.to_le_bytes()).await?;

        Ok(())
    }

}

#[derive(Debug, PartialEq, Clone)]
pub struct TestStructTestFlagE {
    pub b_E1: u8,
}

impl VariableSized for TestStructTestFlagE {
    fn size(&self) -> usize {
        1 // b_E1: u8
    }
}

impl MaximumPossibleSized for TestStructTestFlagE {
    fn maximum_possible_size() -> usize {
        1 // b_E1: u8
    }
}

impl TestStructTestFlagE {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // b_E1: u8
        w.write_all(&self.b_E1.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // b_E1: u8
        w.write_all(&self.b_E1.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // b_E1: u8
        w.write_all(&self.b_E1.to_le_bytes()).await?;

        Ok(())
    }

}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use super::TestStruct;
    use crate::VariableSized;
    use crate::logon::all::InnerFlag;
    use crate::logon::all::TestFlag;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ClientOpcodeMessage;

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn TestStruct0() {
        let raw: Vec<u8> = vec![ 0xFF, 0x0E, 0x00, 0x02, 0x01, 0x02, ];

        let expected = TestStruct {
            f: TestStructTestFlag::empty()
                .set_A(TestStructTestFlagA::B {
                    b_B1: 0x1,
                    i: TestStructInnerFlag::empty()
                        .set_I()
                        ,
                })
                .set_C(TestStructTestFlagC {
                    b_C1: 0x2,
                })
                .set_D()
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::TestStruct(t) => t,
            opcode => panic!("incorrect opcode. Expected TestStruct, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.f, expected.f);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut std::io::Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_tokio")]
    #[cfg_attr(feature = "async_tokio", tokio::test)]
    async fn tokio_TestStruct0() {
        let raw: Vec<u8> = vec![ 0xFF, 0x0E, 0x00, 0x02, 0x01, 0x02, ];

        let expected = TestStruct {
            f: TestStructTestFlag::empty()
                .set_A(TestStructTestFlagA::B {
                    b_B1: 0x1,
                    i: TestStructInnerFlag::empty()
                        .set_I()
                        ,
                })
                .set_C(TestStructTestFlagC {
                    b_C1: 0x2,
                })
                .set_D()
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::TestStruct(t) => t,
            opcode => panic!("incorrect opcode. Expected TestStruct, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.f, expected.f);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_std")]
    #[cfg_attr(feature = "async_std", async_std::test)]
    async fn astd_TestStruct0() {
        let raw: Vec<u8> = vec![ 0xFF, 0x0E, 0x00, 0x02, 0x01, 0x02, ];

        let expected = TestStruct {
            f: TestStructTestFlag::empty()
                .set_A(TestStructTestFlagA::B {
                    b_B1: 0x1,
                    i: TestStructInnerFlag::empty()
                        .set_I()
                        ,
                })
                .set_C(TestStructTestFlagC {
                    b_C1: 0x2,
                })
                .set_D()
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::TestStruct(t) => t,
            opcode => panic!("incorrect opcode. Expected TestStruct, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.f, expected.f);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn TestStruct1() {
        let raw: Vec<u8> = vec![ 0xFF, 0x0D, 0x00, 0x01, 0x02, ];

        let expected = TestStruct {
            f: TestStructTestFlag::empty()
                .set_A(TestStructTestFlagA::A {
                    b_A1: 0x1,
                })
                .set_C(TestStructTestFlagC {
                    b_C1: 0x2,
                })
                .set_D()
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::TestStruct(t) => t,
            opcode => panic!("incorrect opcode. Expected TestStruct, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.f, expected.f);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut std::io::Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_tokio")]
    #[cfg_attr(feature = "async_tokio", tokio::test)]
    async fn tokio_TestStruct1() {
        let raw: Vec<u8> = vec![ 0xFF, 0x0D, 0x00, 0x01, 0x02, ];

        let expected = TestStruct {
            f: TestStructTestFlag::empty()
                .set_A(TestStructTestFlagA::A {
                    b_A1: 0x1,
                })
                .set_C(TestStructTestFlagC {
                    b_C1: 0x2,
                })
                .set_D()
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::TestStruct(t) => t,
            opcode => panic!("incorrect opcode. Expected TestStruct, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.f, expected.f);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_std")]
    #[cfg_attr(feature = "async_std", async_std::test)]
    async fn astd_TestStruct1() {
        let raw: Vec<u8> = vec![ 0xFF, 0x0D, 0x00, 0x01, 0x02, ];

        let expected = TestStruct {
            f: TestStructTestFlag::empty()
                .set_A(TestStructTestFlagA::A {
                    b_A1: 0x1,
                })
                .set_C(TestStructTestFlagC {
                    b_C1: 0x2,
                })
                .set_D()
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::TestStruct(t) => t,
            opcode => panic!("incorrect opcode. Expected TestStruct, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.f, expected.f);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

}
