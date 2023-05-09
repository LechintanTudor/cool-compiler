use cool_resolve::{tys, ResolveContext, TyId, TyKind};
use inkwell::context::Context;
use inkwell::targets::TargetData;
use inkwell::types::{AnyTypeEnum, BasicMetadataTypeEnum, FunctionType, IntType, PointerType};
use inkwell::AddressSpace;
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
        ];

        tys.extend(ty_mappings);
    }

    fn insert_derived_tys(tys: &mut TyMap<'a>, context: &'a Context, resolve: &'a ResolveContext) {
        for ty_id in resolve.iter_ty_ids() {
            if !ty_id.is_inferred() && ty_id != tys::MODULE {
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
            TyKind::Pointer(pointer_ty) => {
                let pointee_ty = Self::insert_ty(tys, context, resolve, pointer_ty.pointee);

                ptr_type_from_any_type_enum(pointee_ty)
                    .unwrap_or_else(|| context.i8_type().ptr_type(Default::default()))
                    .into()
            }
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

fn ptr_type_from_any_type_enum(ty: AnyTypeEnum) -> Option<PointerType> {
    let address_space = AddressSpace::default();

    let pointer_ty = match ty {
        AnyTypeEnum::ArrayType(ty) => ty.ptr_type(address_space),
        AnyTypeEnum::FloatType(ty) => ty.ptr_type(address_space),
        AnyTypeEnum::FunctionType(ty) => ty.ptr_type(address_space),
        AnyTypeEnum::IntType(ty) => ty.ptr_type(address_space),
        AnyTypeEnum::PointerType(ty) => ty.ptr_type(address_space),
        AnyTypeEnum::StructType(ty) => ty.ptr_type(address_space),
        AnyTypeEnum::VectorType(ty) => ty.ptr_type(address_space),
        AnyTypeEnum::VoidType(_) => return None,
    };

    Some(pointer_ty)
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
