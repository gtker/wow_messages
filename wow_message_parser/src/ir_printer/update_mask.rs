use crate::ir_printer::definer::{definer_to_ir, IrDefiner};
use crate::parser::types::objects::Objects;
use crate::parser::types::version::MajorWorldVersion;
use crate::rust_printer::{ByteType, UpdateMaskDataType, UpdateMaskMember, UpdateMaskObjectType};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub(crate) struct IrUpdateMaskMember {
    object_type: UpdateMaskObjectType,
    name: &'static str,
    offset: i32,
    size: i32,
    data_type: IrUpdateMaskType,
}

impl IrUpdateMaskMember {
    pub(crate) fn new_array(
        fields: &[UpdateMaskMember],
        o: &Objects,
        version: MajorWorldVersion,
    ) -> Vec<Self> {
        let mut members = Vec::with_capacity(fields.len());

        for field in fields {
            let data_type = match field.ty() {
                UpdateMaskDataType::Guid => IrUpdateMaskType::Guid,
                UpdateMaskDataType::Int => IrUpdateMaskType::Int,
                UpdateMaskDataType::Float => IrUpdateMaskType::Float,
                UpdateMaskDataType::TwoShort => IrUpdateMaskType::TwoShort,
                UpdateMaskDataType::Bytes(first, second, third, fourth) => {
                    IrUpdateMaskType::Bytes {
                        first,
                        second,
                        third,
                        fourth,
                    }
                }
                UpdateMaskDataType::GuidArrayUsingEnum {
                    name,
                    variable_name,
                    ..
                } => IrUpdateMaskType::GuidArrayUsingEnum {
                    definer: definer_to_ir(o.get_world_enum(name, version)),
                    variable_name,
                },
                UpdateMaskDataType::ArrayOfStruct { .. } => continue,
            };

            members.push(IrUpdateMaskMember {
                object_type: field.object_ty(),
                name: field.name(),
                offset: field.offset(),
                size: field.size(),
                data_type,
            });
        }

        members
    }
}

#[derive(Serialize, Debug)]
#[serde(tag = "update_mask_type_tag", content = "content")]
pub(crate) enum IrUpdateMaskType {
    Guid,
    Int,
    Float,
    TwoShort,
    Bytes {
        first: ByteType,
        second: ByteType,
        third: ByteType,
        fourth: ByteType,
    },
    GuidArrayUsingEnum {
        definer: IrDefiner,
        variable_name: &'static str,
    },
}
