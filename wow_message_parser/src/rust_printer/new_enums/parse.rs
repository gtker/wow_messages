use crate::container::{Container, Equation, IfStatement, StructMember};
use crate::parser::types::objects::Objects;
use crate::rust_printer::new_enums::{
    IfStatementType, NewEnumStructMember, NewEnumerator, NewIfStatement,
};

pub fn add_to_statement(
    statement: &mut IfStatement,
    original_ty_name: &str,
    new_ty_name: &str,
    c: &Container,
    o: &Objects,
) {
    let variable_name = statement.conditional.variable_name().to_string();
    let enum_or_flag = match statement.conditional.equations()[0] {
        Equation::Equals { .. } => IfStatementType::Enum,
        Equation::BitwiseAnd { .. } => IfStatementType::Flag,
        Equation::NotEquals { .. } => IfStatementType::Enum,
    };

    let enumerators = get_enumerators(statement, c, o);

    let new_enum = NewIfStatement::new(
        variable_name.as_str(),
        enum_or_flag,
        original_ty_name,
        new_ty_name,
        enumerators,
    );

    statement.set_new_enum(new_enum);
}

pub fn get_enumerators(
    statement: &mut IfStatement,
    c: &Container,
    o: &Objects,
) -> Vec<NewEnumerator> {
    fn inner(m: &mut StructMember, v: &mut Vec<NewEnumStructMember>, o: &Objects, c: &Container) {
        match m {
            StructMember::Definition(d) => {
                v.push(NewEnumStructMember::Definition(d.clone()));
            }
            StructMember::IfStatement(statement) => {
                let original_ty_name = c.get_type_of_variable(statement.name());
                let new_ty_name = format!("{}{}", c.name(), original_ty_name.rust_str());
                add_to_statement(
                    statement,
                    original_ty_name.rust_str().as_str(),
                    new_ty_name.as_str(),
                    c,
                    o,
                );
                v.push(NewEnumStructMember::IfStatement(
                    statement.new_enum().clone(),
                ));
            }
            StructMember::OptionalStatement(_) => {
                // TODO OPTIONAL
            }
        }
    }

    let d = o.get_definer(
        c.get_type_of_variable(statement.name()).rust_str().as_str(),
        c.tags(),
    );
    let mut enums = Vec::new();

    for f in d.fields() {
        enums.push(NewEnumerator::new(f.name()));
    }

    let mut enumerators_that_arent_else = Vec::new();

    if let Equation::NotEquals { value } = &statement.conditional.equations()[0] {
        enumerators_that_arent_else.push(value.clone());
        let mut v = Vec::new();
        for m in statement.members_mut() {
            inner(m, &mut v, o, c);
        }
        for eq in d.fields() {
            let new_enum = enums.iter_mut().find(|a| a.name == eq.name()).unwrap();
            if !enumerators_that_arent_else
                .iter()
                .any(|a| a.as_str() == eq.name())
            {
                new_enum.add_field(v.clone());
            }
        }

        let mut v = Vec::new();
        for m in statement.else_members_mut() {
            inner(m, &mut v, o, c);
        }
        for eq in statement.member_enumerators() {
            let new_enum = enums
                .iter_mut()
                .find(|a| a.name == eq)
                .unwrap_or_else(|| panic!("unable to find enumerator {}", eq));
            new_enum.add_field(v.clone());
        }

        return enums;
    }

    let mut v = Vec::new();
    for m in statement.members_mut() {
        inner(m, &mut v, o, c);
    }
    for eq in statement.member_enumerators() {
        let new_enum = enums
            .iter_mut()
            .find(|a| a.name == eq)
            .unwrap_or_else(|| panic!("unable to find enumerator {}", eq));
        enumerators_that_arent_else.push(eq.to_string());
        new_enum.add_field(v.clone());
    }

    for is in statement.else_ifs_mut() {
        let mut v = Vec::new();
        for m in is.members_mut() {
            inner(m, &mut v, o, c);
        }
        for eq in is.member_enumerators() {
            let new_enum = enums.iter_mut().find(|a| a.name == eq).expect(eq);
            enumerators_that_arent_else.push(eq.to_string());
            new_enum.add_field(v.clone());
        }
    }

    let mut v = Vec::new();
    for m in statement.else_members_mut() {
        inner(m, &mut v, o, c);
    }
    for eq in d.fields() {
        let new_enum = enums.iter_mut().find(|a| a.name == eq.name()).unwrap();
        if !enumerators_that_arent_else
            .iter()
            .any(|a| a.as_str() == eq.name())
        {
            new_enum.add_field(v.clone());
        }
    }

    enums
}
