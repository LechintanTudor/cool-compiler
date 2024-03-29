use crate::{mangle_item_path, TyFieldMap};
use cool_lexer::{sym, Symbol};
use cool_resolve::{Field, ItemId, ResolveContext, TaggedUnionKind, TyId, ValueTy};
use inkwell::context::Context;
use inkwell::targets::TargetData;
use inkwell::types::{
    BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType, IntType, PointerType, VoidType,
};
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use std::ops;

#[derive(Clone, Debug)]
pub struct GeneratedTys<'a> {
    fns: FxHashMap<TyId, FunctionType<'a>>,
    tys: FxHashMap<TyId, Option<BasicTypeEnum<'a>>>,
    field_maps: FxHashMap<TyId, TyFieldMap>,
    void_ty: VoidType<'a>,
    i8_ty: IntType<'a>,
    i8_ptr_ty: PointerType<'a>,
    isize_ty: IntType<'a>,
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
            field_maps: Default::default(),
            void_ty: context.void_type(),
            i8_ty: context.i8_type(),
            i8_ptr_ty: context.i8_type().ptr_type(Default::default()),
            isize_ty: context.ptr_sized_int_type(target_data, Default::default()),
        };

        generated_tys.insert_builtin_tys(context, resolve);
        generated_tys.insert_derived_tys(context, resolve);
        generated_tys
    }

    fn insert_builtin_tys(&mut self, context: &'a Context, resolve: &'a ResolveContext) {
        let tys = resolve.ty_consts();

        self.tys.insert(tys.unit, None);

        let ty_mappings = [
            // Integers
            (tys.i8, context.i8_type().as_basic_type_enum()),
            (tys.i16, context.i16_type().as_basic_type_enum()),
            (tys.i32, context.i32_type().as_basic_type_enum()),
            (tys.i64, context.i64_type().as_basic_type_enum()),
            (tys.i128, context.i128_type().as_basic_type_enum()),
            (tys.isize, self.isize_ty.as_basic_type_enum()),
            // Unsigned integers
            (tys.u8, context.i8_type().as_basic_type_enum()),
            (tys.u16, context.i16_type().as_basic_type_enum()),
            (tys.u32, context.i32_type().as_basic_type_enum()),
            (tys.u64, context.i64_type().as_basic_type_enum()),
            (tys.u128, context.i128_type().as_basic_type_enum()),
            (tys.usize, self.isize_ty.as_basic_type_enum()),
            // Floats
            (tys.f32, context.f32_type().as_basic_type_enum()),
            (tys.f64, context.f64_type().as_basic_type_enum()),
            // Other
            (tys.bool, context.i8_type().as_basic_type_enum()),
            (tys.char, context.i32_type().as_basic_type_enum()),
            (
                tys.c_str,
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
        let mut declared_structs = Vec::<TyId>::new();

        for ty_id in resolve.iter_value_ty_ids() {
            if let Some(struct_ty) = ty_id.as_struct() {
                if !resolve.get_ty_def(ty_id).unwrap().is_zero_sized() {
                    self.declare_struct_ty(context, ty_id, struct_ty.item_id);
                    declared_structs.push(ty_id);
                }
            }
        }

        for ty_id in resolve.iter_value_ty_ids() {
            self.insert_ty(context, resolve, ty_id);
        }

        for &ty_id in declared_structs.iter() {
            let fields = resolve
                .get_ty_def(ty_id)
                .unwrap()
                .get_aggregate_fields()
                .unwrap();

            self.define_struct_ty(context, resolve, ty_id, fields);
        }
    }

    fn declare_struct_ty(&mut self, context: &'a Context, ty_id: TyId, item_id: ItemId) {
        let struct_name = mangle_item_path(&*item_id);
        let struct_ty = context.opaque_struct_type(&struct_name);
        self.tys.insert(ty_id, Some(struct_ty.as_basic_type_enum()));
    }

    fn define_struct_ty(
        &mut self,
        context: &'a Context,
        resolve: &'a ResolveContext,
        ty_id: TyId,
        fields: &[Field],
    ) {
        let struct_decl = self.tys[&ty_id].unwrap().into_struct_type();
        let mut field_tys = Vec::<BasicTypeEnum>::new();
        let mut field_map = FxHashMap::<Symbol, u32>::default();

        fields
            .iter()
            .flat_map(|field| {
                self.insert_ty(context, resolve, field.ty_id)
                    .map(|ty| (field.symbol, ty))
            })
            .enumerate()
            .for_each(|(i, (symbol, ty))| {
                field_tys.push(ty);
                field_map.insert(symbol, i as u32);
            });

        struct_decl.set_body(&field_tys, false);
        self.field_maps.insert(ty_id, field_map.into());
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

        let ty: Option<BasicTypeEnum> = match ty_id.get_value() {
            ValueTy::Fn(fn_ty) => {
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
            ValueTy::Array(array_ty) => {
                self.insert_ty(context, resolve, array_ty.elem)
                    .filter(|_| array_ty.len != 0)
                    .map(|elem_ty| elem_ty.array_type(array_ty.len as u32))
                    .map(BasicTypeEnum::from)
            }
            ValueTy::Tuple(_) | ValueTy::Struct(_) | ValueTy::Slice(_) => {
                let fields = resolve
                    .get_ty_def(ty_id)
                    .unwrap()
                    .get_aggregate_fields()
                    .unwrap();

                self.insert_aggregate_ty(context, resolve, ty_id, fields)
            }
            ValueTy::Ptr(ptr_ty) => {
                let ty = self
                    .insert_ty(context, resolve, ptr_ty.pointee)
                    .map(|pointee| pointee.ptr_type(Default::default()).as_basic_type_enum())
                    .unwrap_or(self.isize_ty.as_basic_type_enum());

                Some(ty)
            }
            ValueTy::ManyPtr(many_ptr_ty) => {
                let ty = self
                    .insert_ty(context, resolve, many_ptr_ty.pointee)
                    .map(|pointee| pointee.ptr_type(Default::default()).as_basic_type_enum())
                    .unwrap_or(self.isize_ty.as_basic_type_enum());

                Some(ty)
            }
            ValueTy::Variant(_) => Some(self.insert_variant_ty(context, resolve, ty_id)),
            ValueTy::EmptyStruct(_) => None,
            ty => unimplemented!("{}", ty),
        };

        self.tys.insert(ty_id, ty);
        ty
    }

    fn insert_aggregate_ty(
        &mut self,
        context: &'a Context,
        resolve: &'a ResolveContext,
        ty_id: TyId,
        fields: &[Field],
    ) -> Option<BasicTypeEnum<'a>> {
        let non_zst_fields = fields
            .iter()
            .flat_map(|field| {
                self.insert_ty(context, resolve, field.ty_id)
                    .map(|ty| (field, ty))
            })
            .collect::<Vec<_>>();

        let field_map: TyFieldMap = non_zst_fields
            .iter()
            .enumerate()
            .map(|(i, (field, _))| (field.symbol, i as u32))
            .collect::<FxHashMap<_, _>>()
            .into();

        self.field_maps.insert(ty_id, field_map);
        let field_tys = non_zst_fields.iter().map(|(_, ty)| *ty).collect::<Vec<_>>();
        (!field_tys.is_empty()).then(|| context.struct_type(&field_tys, false).as_basic_type_enum())
    }

    fn insert_variant_ty(
        &mut self,
        context: &'a Context,
        resolve: &'a ResolveContext,
        ty_id: TyId,
    ) -> BasicTypeEnum<'a> {
        let tagged_union_ty = resolve
            .get_ty_def(ty_id)
            .unwrap()
            .kind
            .as_tagged_union()
            .unwrap();

        let dominant_ty = self
            .insert_ty(context, resolve, tagged_union_ty.dominant_ty_id)
            .map(|ty| (ty, sym::VARIANT_ELEM));

        let (padding_ty, index_ty) = match tagged_union_ty.kind {
            TaggedUnionKind::Basic {
                padding_before_index,
            } => {
                let padding_ty = (padding_before_index != 0)
                    .then(|| self.i8_ty.array_type(padding_before_index as u32))
                    .map(|ty| ty.as_basic_type_enum());

                (
                    padding_ty.map(|ty| (ty, sym::VARIANT_PADDING)),
                    Some((self.i8_ty.as_basic_type_enum(), sym::VARIANT_INDEX)),
                )
            }
            TaggedUnionKind::NullablePtr => (None, None),
        };

        let field_map: TyFieldMap = [dominant_ty, padding_ty, index_ty]
            .into_iter()
            .flatten()
            .enumerate()
            .map(|(i, (_, symbol))| (symbol, i as u32))
            .collect::<FxHashMap<_, _>>()
            .into();

        self.field_maps.insert(ty_id, field_map);

        let fields = [dominant_ty, padding_ty, index_ty]
            .into_iter()
            .flatten()
            .map(|(ty, _)| ty)
            .collect::<SmallVec<[_; 3]>>();

        context.struct_type(&fields, false).as_basic_type_enum()
    }

    #[inline]
    pub fn get_fn_ty(&self, ty_id: TyId) -> FunctionType<'a> {
        self.fns[&ty_id]
    }

    #[inline]
    pub fn get_field_map(&self, ty_id: TyId) -> &TyFieldMap {
        &self.field_maps[&ty_id]
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
    pub fn i8_ptr_ty(&self) -> PointerType<'a> {
        self.i8_ptr_ty
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
