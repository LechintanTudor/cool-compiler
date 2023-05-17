use crate::mangle_item_path;
use cool_resolve::{tys, ResolveContext, TyId, TyKind};
use inkwell::context::Context;
use inkwell::targets::TargetData;
use inkwell::types::{
    AnyType, AnyTypeEnum, BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType, IntType,
};
use rustc_hash::FxHashMap;
use std::ops;

type TyMap<'a> = FxHashMap<TyId, AnyTypeEnum<'a>>;

pub struct GeneratedTys<'a> {
    tys: TyMap<'a>,
    void_ty: AnyTypeEnum<'a>,
    i8_ty: IntType<'a>,
    isize_ty: IntType<'a>,
}

impl<'a> GeneratedTys<'a> {
    pub fn new(
        context: &'a Context,
        target_data: &TargetData,
        resolve: &'a ResolveContext,
    ) -> Self {
        let mut tys = TyMap::default();
        Self::insert_builtin_tys(&mut tys, context, target_data);
        Self::insert_derived_tys(&mut tys, context, resolve);

        Self {
            tys,
            void_ty: context.void_type().into(),
            i8_ty: context.i8_type(),
            isize_ty: context.ptr_sized_int_type(target_data, Default::default()),
        }
    }

    fn insert_builtin_tys(tys: &mut TyMap<'a>, context: &'a Context, target_data: &TargetData) {
        let ty_mappings = [
            (tys::UNIT, context.void_type().into()),
            // Integers
            (tys::I8, context.i8_type().into()),
            (tys::I16, context.i16_type().into()),
            (tys::I32, context.i32_type().into()),
            (tys::I64, context.i64_type().into()),
            (tys::I128, context.i128_type().into()),
            (
                tys::ISIZE,
                context
                    .ptr_sized_int_type(target_data, Default::default())
                    .into(),
            ),
            // Unsigned integers
            (tys::U8, context.i8_type().into()),
            (tys::U16, context.i16_type().into()),
            (tys::U32, context.i32_type().into()),
            (tys::U64, context.i64_type().into()),
            (tys::U128, context.i128_type().into()),
            (
                tys::USIZE,
                context
                    .ptr_sized_int_type(target_data, Default::default())
                    .into(),
            ),
            // Floats
            (tys::F32, context.f32_type().into()),
            (tys::F64, context.f64_type().into()),
            // Other
            (tys::BOOL, context.i8_type().into()),
            (tys::CHAR, context.i32_type().into()),
            (
                tys::C_STR,
                context.i8_type().ptr_type(Default::default()).into(),
            ),
        ];

        tys.extend(ty_mappings);
    }

    fn insert_derived_tys(tys: &mut TyMap<'a>, context: &'a Context, resolve: &'a ResolveContext) {
        for ty_id in resolve.iter_ty_ids() {
            if resolve[ty_id].kind.is_defined() {
                Self::insert_ty(tys, context, resolve, ty_id);
            }
        }
    }

    fn insert_ty(
        tys: &mut TyMap<'a>,
        context: &'a Context,
        resolve: &'a ResolveContext,
        ty_id: TyId,
    ) -> AnyTypeEnum<'a> {
        if let Some(&ty) = tys.get(&ty_id) {
            return ty;
        }

        let ty: AnyTypeEnum = match &resolve[ty_id].kind {
            TyKind::Fn(fn_ty) => {
                let params = fn_ty
                    .params
                    .iter()
                    .map(|&param| Self::insert_ty(tys, context, resolve, param))
                    .flat_map(BasicMetadataTypeEnum::try_from)
                    .collect::<Vec<_>>();

                let ret = Self::insert_ty(tys, context, resolve, fn_ty.ret);
                fn_type_from_any_type_enum(ret, &params, fn_ty.is_variadic).into()
            }
            TyKind::Array(array_ty) => {
                let elem_ty = Self::insert_ty(tys, context, resolve, array_ty.elem);

                BasicTypeEnum::try_from(elem_ty)
                    .map(|ty| ty.array_type(array_ty.len as u32))
                    .map(|ty| ty.as_any_type_enum())
                    .unwrap_or_else(|_| tys[&tys::UNIT])
            }
            TyKind::Pointer(pointer_ty) => {
                let pointee_ty = Self::insert_ty(tys, context, resolve, pointer_ty.pointee);

                BasicTypeEnum::try_from(pointee_ty)
                    .map(|ty| ty.ptr_type(Default::default()))
                    .map(|ty| ty.as_any_type_enum())
                    .unwrap_or_else(|_| tys[&tys::C_STR])
            }
            TyKind::Struct(struct_ty) => {
                // TODO: Properly handle aggregate types
                let struct_name = mangle_item_path(resolve.get_path_by_item_id(struct_ty.item_id));

                let fields = struct_ty
                    .fields
                    .iter()
                    .map(|(_, ty_id)| Self::insert_ty(tys, context, resolve, *ty_id))
                    .map(|ty| ty.try_into().unwrap())
                    .collect::<Vec<_>>();

                let struct_type = context.opaque_struct_type(&struct_name);
                struct_type.set_body(&fields, false);
                struct_type.as_any_type_enum()
            }
            ty => todo!("Unimplemented ty: {:?}", ty),
        };

        tys.insert(ty_id, ty);
        ty
    }

    #[inline]
    pub fn void_ty(&self) -> AnyTypeEnum<'a> {
        self.void_ty
    }

    #[inline]
    pub fn i8_ty(&self) -> IntType<'a> {
        self.i8_ty
    }

    #[inline]
    pub fn isize_ty(&self) -> IntType<'a> {
        self.isize_ty
    }
}

impl<'a> ops::Index<TyId> for GeneratedTys<'a> {
    type Output = AnyTypeEnum<'a>;

    #[inline]
    fn index(&self, ty_id: TyId) -> &Self::Output {
        &self.tys[&ty_id]
    }
}

fn fn_type_from_any_type_enum<'a>(
    ret: AnyTypeEnum<'a>,
    params: &[BasicMetadataTypeEnum<'a>],
    is_variadic: bool,
) -> FunctionType<'a> {
    match ret {
        AnyTypeEnum::ArrayType(ty) => ty.fn_type(params, is_variadic),
        AnyTypeEnum::FloatType(ty) => ty.fn_type(params, is_variadic),
        AnyTypeEnum::FunctionType(ty) => ty,
        AnyTypeEnum::IntType(ty) => ty.fn_type(params, is_variadic),
        AnyTypeEnum::PointerType(ty) => ty.fn_type(params, is_variadic),
        AnyTypeEnum::StructType(ty) => ty.fn_type(params, is_variadic),
        AnyTypeEnum::VectorType(ty) => ty.fn_type(params, is_variadic),
        AnyTypeEnum::VoidType(ty) => ty.fn_type(params, is_variadic),
    }
}
