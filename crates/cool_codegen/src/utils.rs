use cool_collections::SmallString;
use cool_lexer::symbols::sym;
use cool_resolve::ItemPath;
use inkwell::types::{AnyTypeEnum, BasicTypeEnum};
use inkwell::values::{AnyValueEnum, BasicMetadataValueEnum, BasicValueEnum};

pub trait AnyTypeEnumExt<'a> {
    fn try_into_basic_type(self) -> Option<BasicTypeEnum<'a>>;

    fn into_basic_type(self) -> BasicTypeEnum<'a>;
}

impl<'a> AnyTypeEnumExt<'a> for AnyTypeEnum<'a> {
    fn try_into_basic_type(self) -> Option<BasicTypeEnum<'a>> {
        let ty = match self {
            Self::ArrayType(ty) => BasicTypeEnum::ArrayType(ty),
            Self::FloatType(ty) => BasicTypeEnum::FloatType(ty),
            Self::IntType(ty) => BasicTypeEnum::IntType(ty),
            Self::PointerType(ty) => BasicTypeEnum::PointerType(ty),
            Self::StructType(ty) => BasicTypeEnum::StructType(ty),
            Self::VectorType(ty) => BasicTypeEnum::VectorType(ty),
            _ => return None,
        };

        Some(ty)
    }

    #[inline]
    fn into_basic_type(self) -> BasicTypeEnum<'a> {
        self.try_into_basic_type().unwrap()
    }
}

pub trait AnyValueEnumExt<'a> {
    fn try_into_basic_value(self) -> Option<BasicValueEnum<'a>>;

    fn try_into_basic_metadata_value(self) -> Option<BasicMetadataValueEnum<'a>>;

    fn into_basic_value(self) -> BasicValueEnum<'a>;

    fn into_basic_metadata_value(self) -> BasicMetadataValueEnum<'a>;
}

impl<'a> AnyValueEnumExt<'a> for AnyValueEnum<'a> {
    fn try_into_basic_value(self) -> Option<BasicValueEnum<'a>> {
        let value = match self {
            Self::ArrayValue(v) => BasicValueEnum::ArrayValue(v),
            Self::IntValue(v) => BasicValueEnum::IntValue(v),
            Self::FloatValue(v) => BasicValueEnum::FloatValue(v),
            Self::PointerValue(v) => BasicValueEnum::PointerValue(v),
            Self::StructValue(v) => BasicValueEnum::StructValue(v),
            Self::VectorValue(v) => BasicValueEnum::VectorValue(v),
            _ => return None,
        };

        Some(value)
    }

    fn try_into_basic_metadata_value(self) -> Option<BasicMetadataValueEnum<'a>> {
        let value = match self {
            Self::ArrayValue(v) => BasicMetadataValueEnum::ArrayValue(v),
            Self::IntValue(v) => BasicMetadataValueEnum::IntValue(v),
            Self::FloatValue(v) => BasicMetadataValueEnum::FloatValue(v),
            Self::PointerValue(v) => BasicMetadataValueEnum::PointerValue(v),
            Self::StructValue(v) => BasicMetadataValueEnum::StructValue(v),
            Self::VectorValue(v) => BasicMetadataValueEnum::VectorValue(v),
            Self::MetadataValue(v) => BasicMetadataValueEnum::MetadataValue(v),
            _ => return None,
        };

        Some(value)
    }

    #[inline]
    fn into_basic_value(self) -> BasicValueEnum<'a> {
        self.try_into_basic_value().unwrap()
    }

    #[inline]
    fn into_basic_metadata_value(self) -> BasicMetadataValueEnum<'a> {
        self.try_into_basic_metadata_value().unwrap()
    }
}

pub fn mangle_item_path<'a, P>(path: P) -> SmallString
where
    P: Into<ItemPath<'a>>,
{
    let path: ItemPath = path.into();

    if path.last() == sym::MAIN {
        return SmallString::from("main");
    }

    let path = path.as_symbol_slice();

    let Some((&first, others)) = path.split_first() else {
        return SmallString::new();
    };

    let mut result = SmallString::from(first.as_str());

    for other in others {
        result.push_str("__");
        result.push_str(other.as_str());
    }

    result
}
