use crate::mangle_item_path;
use cool_resolve::{tys, ResolveContext, TyId, TyKind};
use inkwell::context::Context;
use inkwell::targets::TargetData;
use inkwell::types::{
    BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType, IntType, PointerType, VoidType,
};
use rustc_hash::FxHashMap;
use std::ops;

#[derive(Clone, Debug)]
pub struct GeneratedTys<'a> {
    fns: FxHashMap<TyId, FunctionType<'a>>,
    tys: FxHashMap<TyId, Option<BasicTypeEnum<'a>>>,
    void_ty: VoidType<'a>,
    i8_ty: IntType<'a>,
    isize_ty: IntType<'a>,
    i8_ptr_ty: PointerType<'a>,
}

impl<'a> GeneratedTys<'a> {
    pub fn new(
        context: &'a Context,
        target_data: &TargetData,
        resolve: &'a ResolveContext,
    ) -> Self {
        let mut generated_tys = Self {
            fns: Default::default(),
            tys: Default::default(),
            void_ty: context.void_type(),
            i8_ty: context.i8_type(),
            isize_ty: context.ptr_sized_int_type(target_data, Default::default()),
            i8_ptr_ty: context.i8_type().ptr_type(Default::default()),
        };

        generated_tys.insert_builtin_tys(context);
        generated_tys.insert_derived_tys(context, resolve);
        generated_tys
    }

    fn insert_builtin_tys(&mut self, context: &'a Context) {
        self.tys.insert(tys::UNIT, None);

        let ty_mappings = [
            // Integers
            (tys::I8, context.i8_type().as_basic_type_enum()),
            (tys::I16, context.i16_type().as_basic_type_enum()),
            (tys::I32, context.i32_type().as_basic_type_enum()),
            (tys::I64, context.i64_type().as_basic_type_enum()),
            (tys::I128, context.i128_type().as_basic_type_enum()),
            (tys::ISIZE, self.isize_ty.as_basic_type_enum()),
            // Unsigned integers
            (tys::U8, context.i8_type().as_basic_type_enum()),
            (tys::U16, context.i16_type().as_basic_type_enum()),
            (tys::U32, context.i32_type().as_basic_type_enum()),
            (tys::U64, context.i64_type().as_basic_type_enum()),
            (tys::U128, context.i128_type().as_basic_type_enum()),
            (tys::USIZE, self.isize_ty.as_basic_type_enum()),
            // Floats
            (tys::F32, context.f32_type().as_basic_type_enum()),
            (tys::F64, context.f64_type().as_basic_type_enum()),
            // Other
            (tys::BOOL, context.i8_type().as_basic_type_enum()),
            (tys::CHAR, context.i32_type().as_basic_type_enum()),
            (
                tys::C_STR,
                context
                    .i8_type()
                    .ptr_type(Default::default())
                    .as_basic_type_enum(),
            ),
        ];

        self.tys
            .extend(ty_mappings.map(|(ty_id, ty)| (ty_id, Some(ty))));
    }

    fn insert_derived_tys(&mut self, context: &'a Context, resolve: &'a ResolveContext) {
        for ty_id in resolve.iter_ty_ids() {
            if resolve[ty_id].kind.is_defined() {
                self.insert_ty(context, resolve, ty_id);
            }
        }
    }

    fn insert_ty(
        &mut self,
        context: &'a Context,
        resolve: &'a ResolveContext,
        ty_id: TyId,
    ) -> Option<BasicTypeEnum<'a>> {
        if let Some(&ty) = self.tys.get(&ty_id) {
            return ty;
        }

        let ty: Option<BasicTypeEnum> = match &resolve[ty_id].kind {
            TyKind::Fn(fn_ty) => {
                let params = fn_ty
                    .params
                    .iter()
                    .flat_map(|&param| self.insert_ty(context, resolve, param))
                    .map(BasicMetadataTypeEnum::from)
                    .collect::<Vec<_>>();

                let fn_item_ty = self
                    .insert_ty(context, resolve, fn_ty.ret)
                    .map(|ty| ty.fn_type(&params, fn_ty.is_variadic))
                    .unwrap_or_else(|| self.void_ty.fn_type(&params, fn_ty.is_variadic));

                self.fns.insert(ty_id, fn_item_ty);
                Some(fn_item_ty.ptr_type(Default::default()).as_basic_type_enum())
            }
            TyKind::Array(array_ty) => {
                self.insert_ty(context, resolve, array_ty.elem)
                    .filter(|_| array_ty.len != 0)
                    .map(|elem_ty| elem_ty.array_type(array_ty.len as u32))
                    .map(BasicTypeEnum::from)
            }
            TyKind::Pointer(pointer_ty) => {
                Some(
                    self.insert_ty(context, resolve, pointer_ty.pointee)
                        .map(|pointee_ty| pointee_ty.ptr_type(Default::default()))
                        .unwrap_or(self.i8_ptr_ty)
                        .as_basic_type_enum(),
                )
            }
            TyKind::Struct(struct_ty) => {
                // TODO: Properly handle aggregate types
                let struct_name = mangle_item_path(resolve.get_path_by_item_id(struct_ty.item_id));

                let fields = struct_ty
                    .fields
                    .iter()
                    .flat_map(|(_, ty_id)| self.insert_ty(context, resolve, *ty_id))
                    .collect::<Vec<_>>();

                let struct_type = context.opaque_struct_type(&struct_name);
                struct_type.set_body(&fields, false);
                Some(struct_type.as_basic_type_enum())
            }
            ty => todo!("Unimplemented ty: {:?}", ty),
        };

        self.tys.insert(ty_id, ty);
        ty
    }

    #[inline]
    pub fn get_fn_ty(&self, ty_id: TyId) -> FunctionType<'a> {
        *self.fns.get(&ty_id).unwrap()
    }

    #[inline]
    pub fn void_ty(&self) -> VoidType<'a> {
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
    type Output = Option<BasicTypeEnum<'a>>;

    #[inline]
    fn index(&self, ty_id: TyId) -> &Self::Output {
        &self.tys[&ty_id]
    }
}
