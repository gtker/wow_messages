use crate::parser::types::IntegerType;
use crate::rust_printer::rust_view::rust_enumerator::RustEnumerator;
use crate::rust_printer::rust_view::rust_member::RustMember;
use crate::rust_printer::rust_view::rust_type::RustType;
use crate::rust_printer::DefinerType;

#[derive(Debug, Copy, Clone)]
pub(crate) struct RustDefiner<'a> {
    inner: &'a RustMember,
    definer_type: DefinerType,
    enumerators: &'a [RustEnumerator],
    ty_name: &'a str,
    int_ty: IntegerType,
    is_simple: bool,
    is_elseif: bool,
    original_ty_name: &'a str,
    has_separate_if_statements: bool,
    is_single_rust_definer: bool,
}

impl<'a> RustDefiner<'a> {
    pub(crate) fn all_members(&self) -> Vec<&RustMember> {
        let mut v = Vec::new();

        v.push(self.inner);

        for enumerator in self.enumerators() {
            v.append(&mut enumerator.all_members());
        }

        v
    }
    pub(crate) fn variable_name(&self) -> &str {
        self.inner().name()
    }
    pub(crate) fn has_separate_if_statements(&self) -> bool {
        self.has_separate_if_statements
    }
    pub(crate) fn inner(&self) -> &RustMember {
        self.inner
    }
    pub(crate) fn enumerators(&self) -> &'a [RustEnumerator] {
        self.enumerators
    }
    pub(crate) fn size_is_const_fn(&self) -> bool {
        self.enumerators()
            .iter()
            .flat_map(|a| a.members_in_struct())
            .all(|a| a.ty().size_is_const_fn())
    }

    pub(crate) fn complex_flag_enumerators(&self) -> Vec<&RustEnumerator> {
        self.enumerators
            .iter()
            .filter(|a| !a.should_not_be_in_flag_types())
            .collect()
    }
    pub(crate) fn ty_name(&self) -> &str {
        self.ty_name
    }
    pub(crate) fn int_ty(&self) -> IntegerType {
        self.int_ty
    }
    pub(crate) fn is_simple(&self) -> bool {
        self.is_simple && !self.is_elseif
    }
    pub(crate) fn is_single_rust_definer(&self) -> bool {
        self.is_single_rust_definer
    }

    pub(crate) fn is_elseif(&self) -> bool {
        self.is_elseif
    }
    pub(crate) fn original_ty_name(&self) -> &str {
        self.original_ty_name
    }
    pub(crate) fn definer_type(&self) -> DefinerType {
        self.definer_type
    }
    pub(crate) fn get_enumerator(&self, enumerator_name: &str) -> &RustEnumerator {
        for enumerator in self.enumerators() {
            if enumerator.contains_elseif() {
                match enumerator.members()[0].ty() {
                    RustType::Enum { enumerators, .. } => {
                        for enumerator in enumerators {
                            if enumerator.name() == enumerator_name {
                                return enumerator;
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }

            if enumerator.name() == enumerator_name {
                return enumerator;
            }
        }

        unreachable!()
    }

    pub(crate) fn contains_enumerator(&self, enumerator_name: &str) -> bool {
        for enumerator in self.enumerators() {
            if enumerator.contains_elseif() {
                match enumerator.members()[0].ty() {
                    RustType::Enum { enumerators, .. } => {
                        for enumerator in enumerators {
                            if enumerator.name() == enumerator_name {
                                return true;
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }

            if enumerator.name() == enumerator_name {
                return true;
            }
        }

        false
    }

    pub fn new(
        inner: &'a RustMember,
        definer_type: DefinerType,
        enumerators: &'a [RustEnumerator],
        ty_name: &'a str,
        int_ty: IntegerType,
        is_simple: bool,
        is_elseif: bool,
        original_ty_name: &'a str,
        has_separate_if_statements: bool,
        is_single_rust_definer: bool,
    ) -> Self {
        Self {
            inner,
            definer_type,
            enumerators,
            ty_name,
            int_ty,
            is_simple,
            is_elseif,
            original_ty_name,
            has_separate_if_statements,
            is_single_rust_definer,
        }
    }
}
