use cool_resolve::ty::{tys, TyId, TyKind, TyTable};
use inkwell::context::Context;
use inkwell::targets::TargetData;
use inkwell::types::{AnyTypeEnum, BasicMetadataTypeEnum, FunctionType};
use rustc_hash::FxHashMap;

type TyMap<'ctx> = FxHashMap<TyId, AnyTypeEnum<'ctx>>;

pub struct GeneratedTys<'ctx> {
    tys: TyMap<'ctx>,
}

impl<'ctx> GeneratedTys<'ctx> {
    pub fn new(context: &'ctx Context, target_data: &TargetData, ty_table: &TyTable) -> Self {
        let mut tys = TyMap::default();
        Self::insert_builtin_tys(&mut tys, context, target_data);
        Self::insert_derived_tys(&mut tys, context, ty_table);
        Self { tys }
    }

    #[inline]
    pub fn get(&self, ty_id: TyId) -> AnyTypeEnum<'ctx> {
        *self.tys.get(&ty_id).unwrap()
    }

    fn insert_builtin_tys(tys: &mut TyMap<'ctx>, context: &'ctx Context, target_data: &TargetData) {
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
        ];

        tys.extend(ty_mappings);
    }

    fn insert_derived_tys(tys: &mut TyMap<'ctx>, context: &'ctx Context, ty_table: &TyTable) {
        for ty_id in ty_table.iter_ids() {
            Self::insert_ty(tys, context, ty_table, ty_id);
        }
    }

    fn insert_ty(
        tys: &mut TyMap<'ctx>,
        context: &'ctx Context,
        ty_table: &TyTable,
        ty_id: TyId,
    ) -> AnyTypeEnum<'ctx> {
        if let Some(&ty) = tys.get(&ty_id) {
            return ty;
        }

        let ty_kind = ty_table.get_kind_by_id(ty_id).unwrap();

        let ty = match ty_kind {
            TyKind::Tuple(tuple_ty) => {
                let elems = tuple_ty
                    .elems
                    .iter()
                    .map(|&ty| Self::insert_ty(tys, context, ty_table, ty))
                    .flat_map(|ty| ty.try_into())
                    .collect::<Vec<_>>();

                context.struct_type(&elems, false).into()
            }
            TyKind::Fn(fn_ty) => {
                let args = fn_ty
                    .args
                    .iter()
                    .map(|&ty| Self::insert_ty(tys, context, ty_table, ty))
                    .flat_map(|ty| ty.try_into())
                    .collect::<Vec<_>>();
                let ret = Self::insert_ty(tys, context, ty_table, fn_ty.ret);

                fn_type_from_any_type_enum(ret, &args, false).into()
            }
            _ => unreachable!(),
        };

        tys.insert(ty_id, ty);
        ty
    }
}

pub fn fn_type_from_any_type_enum<'ctx>(
    ret: AnyTypeEnum<'ctx>,
    params: &[BasicMetadataTypeEnum<'ctx>],
    is_var_args: bool,
) -> FunctionType<'ctx> {
    match ret {
        AnyTypeEnum::ArrayType(ty) => ty.fn_type(params, is_var_args),
        AnyTypeEnum::FloatType(ty) => ty.fn_type(params, is_var_args),
        AnyTypeEnum::FunctionType(_ty) => todo!(),
        AnyTypeEnum::IntType(ty) => ty.fn_type(params, is_var_args),
        AnyTypeEnum::PointerType(ty) => ty.fn_type(params, is_var_args),
        AnyTypeEnum::StructType(ty) => ty.fn_type(params, is_var_args),
        AnyTypeEnum::VectorType(ty) => ty.fn_type(params, is_var_args),
        AnyTypeEnum::VoidType(ty) => ty.fn_type(params, is_var_args),
    }
}
