#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[cfg(feature = "flutter")]
mod frb_generated {
    #![allow(
        non_camel_case_types,
        unused,
        non_snake_case,
        clippy::needless_return,
        clippy::redundant_closure_call,
        clippy::redundant_closure,
        clippy::useless_conversion,
        clippy::unit_arg,
        clippy::unused_unit,
        clippy::double_parens,
        clippy::let_and_return,
        clippy::too_many_arguments,
        clippy::match_single_binding,
        clippy::clone_on_copy,
        clippy::let_unit_value,
        clippy::deref_addrof,
        clippy::explicit_auto_deref,
        clippy::borrow_deref_ref,
        clippy::needless_borrow
    )]
    use crate::api::api::*;
    use crate::api::error::IntoRemoteErr;
    use crate::api::error::*;
    use crate::api::mdns::*;
    use crate::api::response::*;
    use crate::api::serialize::ToUuid;
    use crate::api::types::GetType;
    use crate::api::types::*;
    use crate::*;
    use flutter_rust_bridge::for_generated::byteorder::{
        NativeEndian, ReadBytesExt, WriteBytesExt,
    };
    use flutter_rust_bridge::for_generated::{
        transform_result_dco, Lifetimeable, Lockable,
    };
    use flutter_rust_bridge::{Handler, IntoIntoDart};
    #[doc(hidden)]
    pub(crate) struct FrbWrapper<T>(T);
    impl<T: Clone> Clone for FrbWrapper<T> {
        fn clone(&self) -> Self {
            FrbWrapper(self.0.clone())
        }
    }
    impl<T: PartialEq> PartialEq for FrbWrapper<T> {
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }
    impl<T: Eq> Eq for FrbWrapper<T> {}
    impl<T: std::hash::Hash> std::hash::Hash for FrbWrapper<T> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state)
        }
    }
    impl<T> From<T> for FrbWrapper<T> {
        fn from(t: T) -> Self {
            FrbWrapper(t)
        }
    }
    use std::collections::HashMap;
    use std::marker::PhantomData;
    use std::sync::Arc;
    pub struct MoiArc<T: ?Sized + MoiArcValue> {
        object_id: Option<ObjectId>,
        value: Option<Arc<T>>,
        _phantom: PhantomData<T>,
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug + ?Sized + MoiArcValue> ::core::fmt::Debug for MoiArc<T> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "MoiArc",
                "object_id",
                &self.object_id,
                "value",
                &self.value,
                "_phantom",
                &&self._phantom,
            )
        }
    }
    impl<T: ?Sized + MoiArcValue> Drop for MoiArc<T> {
        fn drop(&mut self) {
            if let Some(object_id) = self.object_id {
                Self::decrement_strong_count(object_id);
            }
        }
    }
    impl<T: ?Sized + MoiArcValue> AsRef<T> for MoiArc<T> {
        fn as_ref(&self) -> &T {
            self.value.as_ref().unwrap().as_ref()
        }
    }
    impl<T: ?Sized + MoiArcValue> ::flutter_rust_bridge::for_generated::BaseArc<T>
    for MoiArc<T> {
        fn new(value: T) -> Self
        where
            T: Sized,
        {
            let mut pool = T::get_pool().write().unwrap();
            let object_id = pool.id_generator.next_id();
            let value = Arc::new(value);
            let old_value = pool
                .map
                .insert(
                    object_id,
                    MoiArcPoolValue {
                        ref_count: 1,
                        value: value.clone(),
                    },
                );
            if !old_value.is_none() {
                ::core::panicking::panic("assertion failed: old_value.is_none()")
            }
            Self {
                object_id: Some(object_id),
                value: Some(value),
                _phantom: PhantomData,
            }
        }
        fn try_unwrap(mut self) -> Result<T, Self>
        where
            T: Sized,
        {
            let pool = &mut T::get_pool().write().unwrap();
            if pool.map.get(&self.object_id.unwrap()).unwrap().ref_count == 1 {
                Self::decrement_strong_count_raw(self.object_id.unwrap(), pool);
                self.object_id.take().unwrap();
                Ok(Arc::into_inner(self.value.take().unwrap()).unwrap())
            } else {
                Err(self)
            }
        }
        fn into_inner(self) -> Option<T>
        where
            T: Sized,
        {
            self.try_unwrap().ok()
        }
        fn into_raw(mut self) -> usize {
            self.object_id.take().unwrap()
        }
    }
    impl<T: ?Sized + MoiArcValue> Clone for MoiArc<T> {
        fn clone(&self) -> Self {
            Self::increment_strong_count(self.object_id.unwrap());
            Self {
                object_id: self.object_id,
                value: self.value.clone(),
                _phantom: PhantomData,
            }
        }
    }
    impl<T: ?Sized + MoiArcValue> MoiArc<T> {
        pub(crate) fn from_raw(raw: usize) -> Self
        where
            T: Sized,
        {
            let map = &T::get_pool().read().unwrap().map;
            Self {
                object_id: Some(raw),
                value: Some(map.get(&raw).unwrap().value.clone()),
                _phantom: PhantomData,
            }
        }
        pub fn increment_strong_count(raw: usize) {
            let map = &mut T::get_pool().write().unwrap().map;
            map.get_mut(&raw).unwrap().ref_count += 1;
        }
        pub fn decrement_strong_count(raw: usize) {
            let mut pool = T::get_pool().write().unwrap();
            let object = Self::decrement_strong_count_raw(raw, &mut pool);
            drop(pool);
            drop(object);
        }
        fn decrement_strong_count_raw(
            raw: usize,
            pool: &mut MoiArcPoolInner<T>,
        ) -> Option<MoiArcPoolValue<T>> {
            let value = pool.map.get_mut(&raw).unwrap();
            value.ref_count -= 1;
            if value.ref_count == 0 { pool.map.remove(&raw) } else { None }
        }
    }
    pub trait MoiArcValue: 'static {
        fn get_pool() -> &'static MoiArcPool<Self>;
    }
    type ObjectId = usize;
    pub type MoiArcPool<T> = std::sync::RwLock<MoiArcPoolInner<T>>;
    pub struct MoiArcPoolInner<T: ?Sized> {
        map: HashMap<ObjectId, MoiArcPoolValue<T>>,
        id_generator: IdGenerator,
    }
    impl<T: ?Sized> Default for MoiArcPoolInner<T> {
        fn default() -> Self {
            Self {
                map: HashMap::new(),
                id_generator: Default::default(),
            }
        }
    }
    struct IdGenerator {
        next_id: ObjectId,
    }
    impl Default for IdGenerator {
        fn default() -> Self {
            Self { next_id: Self::MIN_ID }
        }
    }
    impl IdGenerator {
        const MIN_ID: ObjectId = 1;
        const MAX_ID: ObjectId = 2147483600;
        fn next_id(&mut self) -> ObjectId {
            let ans = self.next_id;
            self.next_id = if self.next_id >= Self::MAX_ID {
                Self::MIN_ID
            } else {
                self.next_id + 1
            };
            ans
        }
    }
    impl<T: ?Sized> MoiArcPoolInner<T> {}
    struct MoiArcPoolValue<T: ?Sized> {
        ref_count: i32,
        value: Arc<T>,
    }
    use ::flutter_rust_bridge::for_generated::decode_rust_opaque_nom;
    fn decode_rust_opaque_moi<T: MoiArcValue + Send + Sync>(
        ptr: usize,
    ) -> RustOpaqueMoi<T> {
        RustOpaqueMoi::from_arc(MoiArc::<T>::from_raw(ptr))
    }
    use ::flutter_rust_bridge::for_generated::StdArc;
    use ::flutter_rust_bridge::RustOpaqueNom;
    /// Please refer to `RustOpaque` for doc.
    pub type RustOpaqueMoi<T> = ::flutter_rust_bridge::for_generated::RustOpaqueBase<
        T,
        MoiArc<T>,
    >;
    /// A wrapper to support [arbitrary Rust types](https://cjycode.com/flutter_rust_bridge/guides/types/arbitrary).
    pub type RustOpaque<T> = RustOpaqueMoi<T>;
    use ::flutter_rust_bridge::RustAutoOpaqueNom;
    /// Please refer to `RustAutoOpaque` for doc.
    pub type RustAutoOpaqueMoi<T> = ::flutter_rust_bridge::for_generated::RustAutoOpaqueBase<
        T,
        MoiArc<::flutter_rust_bridge::for_generated::RustAutoOpaqueInner<T>>,
    >;
    /// Usually this is unneeded, and just write down arbitrary types.
    /// However, when you need arbitrary types at places that are not supported yet,
    /// use `RustOpaqueOpaque<YourArbitraryType>`.
    pub type RustAutoOpaque<T> = RustAutoOpaqueMoi<T>;
    pub trait CstDecode<T> {
        fn cst_decode(self) -> T;
    }
    impl<T, S> CstDecode<Option<T>> for *mut S
    where
        *mut S: CstDecode<T>,
    {
        fn cst_decode(self) -> Option<T> {
            (!self.is_null()).then(|| self.cst_decode())
        }
    }
    pub trait SseDecode {
        fn sse_decode(
            deserializer: &mut ::flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self;
    }
    pub trait SseEncode {
        fn sse_encode(
            self,
            serializer: &mut ::flutter_rust_bridge::for_generated::SseSerializer,
        );
    }
    fn transform_result_sse<T, E>(
        raw: Result<T, E>,
    ) -> Result<
        ::flutter_rust_bridge::for_generated::Rust2DartMessageSse,
        ::flutter_rust_bridge::for_generated::Rust2DartMessageSse,
    >
    where
        T: SseEncode,
        E: SseEncode,
    {
        use ::flutter_rust_bridge::for_generated::{Rust2DartAction, SseCodec};
        match raw {
            Ok(raw) => {
                Ok(
                    SseCodec::encode(
                        Rust2DartAction::Success,
                        |serializer| { raw.sse_encode(serializer) },
                    ),
                )
            }
            Err(raw) => {
                Err(
                    SseCodec::encode(
                        Rust2DartAction::Error,
                        |serializer| { raw.sse_encode(serializer) },
                    ),
                )
            }
        }
    }
    pub struct StreamSink<
        T,
        Rust2DartCodec: ::flutter_rust_bridge::for_generated::BaseCodec
            = ::flutter_rust_bridge::for_generated::SseCodec,
    > {
        base: ::flutter_rust_bridge::for_generated::StreamSinkBase<T, Rust2DartCodec>,
    }
    #[automatically_derived]
    impl<
        T: ::core::clone::Clone,
        Rust2DartCodec: ::core::clone::Clone
            + ::flutter_rust_bridge::for_generated::BaseCodec,
    > ::core::clone::Clone for StreamSink<T, Rust2DartCodec> {
        #[inline]
        fn clone(&self) -> StreamSink<T, Rust2DartCodec> {
            StreamSink {
                base: ::core::clone::Clone::clone(&self.base),
            }
        }
    }
    impl<
        T,
        Rust2DartCodec: ::flutter_rust_bridge::for_generated::BaseCodec,
    > StreamSink<T, Rust2DartCodec> {
        pub fn deserialize(raw: String) -> Self {
            Self {
                base: ::flutter_rust_bridge::for_generated::StreamSinkBase::deserialize(
                    raw,
                ),
            }
        }
    }
    impl<T> StreamSink<T, ::flutter_rust_bridge::for_generated::DcoCodec> {
        pub fn add<T2>(
            &self,
            value: T,
        ) -> Result<(), ::flutter_rust_bridge::Rust2DartSendError>
        where
            T: ::flutter_rust_bridge::IntoIntoDart<T2>,
            T2: ::flutter_rust_bridge::IntoDart,
        {
            self.add_raw(
                ::flutter_rust_bridge::for_generated::Rust2DartAction::Success,
                value,
            )
        }
        pub fn add_error<TR, T2>(
            &self,
            value: TR,
        ) -> Result<(), ::flutter_rust_bridge::Rust2DartSendError>
        where
            TR: ::flutter_rust_bridge::IntoIntoDart<T2>,
            T2: ::flutter_rust_bridge::IntoDart,
        {
            self.add_raw(
                ::flutter_rust_bridge::for_generated::Rust2DartAction::Error,
                value,
            )
        }
        fn add_raw<TR, T2>(
            &self,
            action: ::flutter_rust_bridge::for_generated::Rust2DartAction,
            value: TR,
        ) -> Result<(), ::flutter_rust_bridge::Rust2DartSendError>
        where
            TR: ::flutter_rust_bridge::IntoIntoDart<T2>,
            T2: ::flutter_rust_bridge::IntoDart,
        {
            self.base
                .add_raw(
                    ::flutter_rust_bridge::for_generated::DcoCodec::encode(
                        action,
                        value.into_into_dart(),
                    ),
                )
        }
    }
    impl<T> StreamSink<T, ::flutter_rust_bridge::for_generated::SseCodec>
    where
        T: SseEncode,
    {
        pub fn add(
            &self,
            value: T,
        ) -> Result<(), ::flutter_rust_bridge::Rust2DartSendError> {
            self.add_raw(
                ::flutter_rust_bridge::for_generated::Rust2DartAction::Success,
                value,
            )
        }
        pub fn add_error<TR: SseEncode>(
            &self,
            value: TR,
        ) -> Result<(), ::flutter_rust_bridge::Rust2DartSendError> {
            self.add_raw(
                ::flutter_rust_bridge::for_generated::Rust2DartAction::Error,
                value,
            )
        }
        pub fn add_raw<TR: SseEncode>(
            &self,
            action: ::flutter_rust_bridge::for_generated::Rust2DartAction,
            value: TR,
        ) -> Result<(), ::flutter_rust_bridge::Rust2DartSendError> {
            self.base
                .add_raw(
                    ::flutter_rust_bridge::for_generated::SseCodec::encode(
                        action,
                        |serializer| value.sse_encode(serializer),
                    ),
                )
        }
    }
    impl<
        T,
        Rust2DartCodec: ::flutter_rust_bridge::for_generated::BaseCodec,
    > ::flutter_rust_bridge::IntoIntoDart<StreamSink<T, Rust2DartCodec>>
    for StreamSink<T, Rust2DartCodec> {
        fn into_into_dart(self) -> StreamSink<T, Rust2DartCodec> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
    }
    impl<
        T,
        Rust2DartCodec: ::flutter_rust_bridge::for_generated::BaseCodec,
    > ::flutter_rust_bridge::IntoDart for StreamSink<T, Rust2DartCodec> {
        fn into_dart(self) -> ::flutter_rust_bridge::for_generated::DartAbi {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
    }
    pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_VERSION: &str = "2.4.0";
    pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_CONTENT_HASH: i32 = -1173764766;
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    pub struct FLUTTER_RUST_BRIDGE_HANDLER {
        __private_field: (),
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    pub static FLUTTER_RUST_BRIDGE_HANDLER: FLUTTER_RUST_BRIDGE_HANDLER = FLUTTER_RUST_BRIDGE_HANDLER {
        __private_field: (),
    };
    impl ::lazy_static::__Deref for FLUTTER_RUST_BRIDGE_HANDLER {
        type Target = ::flutter_rust_bridge::DefaultHandler<
            ::flutter_rust_bridge::for_generated::SimpleThreadPool,
        >;
        fn deref(
            &self,
        ) -> &::flutter_rust_bridge::DefaultHandler<
            ::flutter_rust_bridge::for_generated::SimpleThreadPool,
        > {
            #[inline(always)]
            fn __static_ref_initialize() -> ::flutter_rust_bridge::DefaultHandler<
                ::flutter_rust_bridge::for_generated::SimpleThreadPool,
            > {
                {
                    match (
                        &FLUTTER_RUST_BRIDGE_CODEGEN_VERSION,
                        &flutter_rust_bridge::for_generated::FLUTTER_RUST_BRIDGE_RUNTIME_VERSION,
                    ) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::Some(
                                        format_args!(
                                            "Please ensure flutter_rust_bridge\'s codegen ({0}) and runtime ({1}) versions are the same",
                                            FLUTTER_RUST_BRIDGE_CODEGEN_VERSION,
                                            flutter_rust_bridge::for_generated::FLUTTER_RUST_BRIDGE_RUNTIME_VERSION,
                                        ),
                                    ),
                                );
                            }
                        }
                    };
                    ::flutter_rust_bridge::DefaultHandler::new_simple(Default::default())
                }
            }
            #[inline(always)]
            fn __stability() -> &'static ::flutter_rust_bridge::DefaultHandler<
                ::flutter_rust_bridge::for_generated::SimpleThreadPool,
            > {
                static LAZY: ::lazy_static::lazy::Lazy<
                    ::flutter_rust_bridge::DefaultHandler<
                        ::flutter_rust_bridge::for_generated::SimpleThreadPool,
                    >,
                > = ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for FLUTTER_RUST_BRIDGE_HANDLER {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
    fn wire__crate__Ipv4Addr_is_loopback_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "Ipv4Addr_is_loopback",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <RustOpaqueMoi<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv4Addr>,
                    >>::sse_decode(&mut deserializer);
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let mut api_that_guard = None;
                                let decode_indices_ = flutter_rust_bridge::for_generated::lockable_compute_decode_order(
                                    <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                                &api_that,
                                                0,
                                                false,
                                            ),
                                        ]),
                                    ),
                                );
                                for i in decode_indices_ {
                                    match i {
                                        0 => {
                                            api_that_guard = Some(api_that.lockable_decode_sync_ref());
                                        }
                                        _ => {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        }
                                    }
                                }
                                let api_that_guard = api_that_guard.unwrap();
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::Ipv4Addr::is_loopback(&*api_that_guard))?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__Ipv4Addr_is_multicast_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "Ipv4Addr_is_multicast",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <RustOpaqueMoi<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv4Addr>,
                    >>::sse_decode(&mut deserializer);
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let mut api_that_guard = None;
                                let decode_indices_ = flutter_rust_bridge::for_generated::lockable_compute_decode_order(
                                    <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                                &api_that,
                                                0,
                                                false,
                                            ),
                                        ]),
                                    ),
                                );
                                for i in decode_indices_ {
                                    match i {
                                        0 => {
                                            api_that_guard = Some(api_that.lockable_decode_sync_ref());
                                        }
                                        _ => {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        }
                                    }
                                }
                                let api_that_guard = api_that_guard.unwrap();
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::Ipv4Addr::is_multicast(&*api_that_guard))?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__Ipv4Addr_is_unspecified_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "Ipv4Addr_is_unspecified",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <RustOpaqueMoi<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv4Addr>,
                    >>::sse_decode(&mut deserializer);
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let mut api_that_guard = None;
                                let decode_indices_ = flutter_rust_bridge::for_generated::lockable_compute_decode_order(
                                    <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                                &api_that,
                                                0,
                                                false,
                                            ),
                                        ]),
                                    ),
                                );
                                for i in decode_indices_ {
                                    match i {
                                        0 => {
                                            api_that_guard = Some(api_that.lockable_decode_sync_ref());
                                        }
                                        _ => {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        }
                                    }
                                }
                                let api_that_guard = api_that_guard.unwrap();
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::Ipv4Addr::is_unspecified(&*api_that_guard))?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__Ipv4Addr_to_string_impl(
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) -> flutter_rust_bridge::for_generated::WireSyncRust2DartSse {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_sync::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "Ipv4Addr_to_string",
                    port: None,
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Sync,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <RustOpaqueMoi<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv4Addr>,
                    >>::sse_decode(&mut deserializer);
                    deserializer.end();
                    transform_result_sse::<
                        _,
                        (),
                    >(
                        (move || {
                            let mut api_that_guard = None;
                            let decode_indices_ = flutter_rust_bridge::for_generated::lockable_compute_decode_order(
                                <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                            &api_that,
                                            0,
                                            false,
                                        ),
                                    ]),
                                ),
                            );
                            for i in decode_indices_ {
                                match i {
                                    0 => {
                                        api_that_guard = Some(api_that.lockable_decode_sync_ref());
                                    }
                                    _ => {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                }
                            }
                            let api_that_guard = api_that_guard.unwrap();
                            let output_ok = Result::<
                                _,
                                (),
                            >::Ok(crate::Ipv4Addr::to_string(&*api_that_guard))?;
                            Ok(output_ok)
                        })(),
                    )
                },
            )
    }
    fn wire__crate__Ipv6Addr_is_loopback_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "Ipv6Addr_is_loopback",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <RustOpaqueMoi<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv6Addr>,
                    >>::sse_decode(&mut deserializer);
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let mut api_that_guard = None;
                                let decode_indices_ = flutter_rust_bridge::for_generated::lockable_compute_decode_order(
                                    <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                                &api_that,
                                                0,
                                                false,
                                            ),
                                        ]),
                                    ),
                                );
                                for i in decode_indices_ {
                                    match i {
                                        0 => {
                                            api_that_guard = Some(api_that.lockable_decode_sync_ref());
                                        }
                                        _ => {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        }
                                    }
                                }
                                let api_that_guard = api_that_guard.unwrap();
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::Ipv6Addr::is_loopback(&*api_that_guard))?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__Ipv6Addr_is_multicast_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "Ipv6Addr_is_multicast",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <RustOpaqueMoi<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv6Addr>,
                    >>::sse_decode(&mut deserializer);
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let mut api_that_guard = None;
                                let decode_indices_ = flutter_rust_bridge::for_generated::lockable_compute_decode_order(
                                    <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                                &api_that,
                                                0,
                                                false,
                                            ),
                                        ]),
                                    ),
                                );
                                for i in decode_indices_ {
                                    match i {
                                        0 => {
                                            api_that_guard = Some(api_that.lockable_decode_sync_ref());
                                        }
                                        _ => {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        }
                                    }
                                }
                                let api_that_guard = api_that_guard.unwrap();
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::Ipv6Addr::is_multicast(&*api_that_guard))?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__Ipv6Addr_is_unspecified_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "Ipv6Addr_is_unspecified",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <RustOpaqueMoi<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv6Addr>,
                    >>::sse_decode(&mut deserializer);
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let mut api_that_guard = None;
                                let decode_indices_ = flutter_rust_bridge::for_generated::lockable_compute_decode_order(
                                    <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                                &api_that,
                                                0,
                                                false,
                                            ),
                                        ]),
                                    ),
                                );
                                for i in decode_indices_ {
                                    match i {
                                        0 => {
                                            api_that_guard = Some(api_that.lockable_decode_sync_ref());
                                        }
                                        _ => {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        }
                                    }
                                }
                                let api_that_guard = api_that_guard.unwrap();
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::Ipv6Addr::is_unspecified(&*api_that_guard))?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__Ipv6Addr_to_string_impl(
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) -> flutter_rust_bridge::for_generated::WireSyncRust2DartSse {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_sync::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "Ipv6Addr_to_string",
                    port: None,
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Sync,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <RustOpaqueMoi<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv6Addr>,
                    >>::sse_decode(&mut deserializer);
                    deserializer.end();
                    transform_result_sse::<
                        _,
                        (),
                    >(
                        (move || {
                            let mut api_that_guard = None;
                            let decode_indices_ = flutter_rust_bridge::for_generated::lockable_compute_decode_order(
                                <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                            &api_that,
                                            0,
                                            false,
                                        ),
                                    ]),
                                ),
                            );
                            for i in decode_indices_ {
                                match i {
                                    0 => {
                                        api_that_guard = Some(api_that.lockable_decode_sync_ref());
                                    }
                                    _ => {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                }
                            }
                            let api_that_guard = api_that_guard.unwrap();
                            let output_ok = Result::<
                                _,
                                (),
                            >::Ok(crate::Ipv6Addr::to_string(&*api_that_guard))?;
                            Ok(output_ok)
                        })(),
                    )
                },
            )
    }
    fn wire__crate__api__mdns__ServiceScanner_discover_devices_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_async::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "ServiceScanner_discover_devices",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <RustOpaqueMoi<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            ServiceScanner,
                        >,
                    >>::sse_decode(&mut deserializer);
                    let api_cb = decode_DartFn_Inputs_list_host_record_Output_unit_AnyhowException(
                        <flutter_rust_bridge::DartOpaque>::sse_decode(&mut deserializer),
                    );
                    deserializer.end();
                    move |context| async move {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || async move {
                                let mut api_that_guard = None;
                                let decode_indices_ = flutter_rust_bridge::for_generated::lockable_compute_decode_order(
                                    <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                                &api_that,
                                                0,
                                                true,
                                            ),
                                        ]),
                                    ),
                                );
                                for i in decode_indices_ {
                                    match i {
                                        0 => {
                                            api_that_guard = Some(
                                                api_that.lockable_decode_async_ref_mut().await,
                                            );
                                        }
                                        _ => {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        }
                                    }
                                }
                                let mut api_that_guard = api_that_guard.unwrap();
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(
                                    crate::api::mdns::ServiceScanner::discover_devices(
                                            &mut *api_that_guard,
                                            api_cb,
                                        )
                                        .await,
                                )?;
                                Ok(output_ok)
                            })()
                                .await,
                        )
                    }
                },
            )
    }
    fn wire__crate__api__mdns__ServiceScanner_new_impl(
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) -> flutter_rust_bridge::for_generated::WireSyncRust2DartSse {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_sync::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "ServiceScanner_new",
                    port: None,
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Sync,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    deserializer.end();
                    transform_result_sse::<
                        _,
                        (),
                    >(
                        (move || {
                            let output_ok = Result::<
                                _,
                                (),
                            >::Ok(crate::api::mdns::ServiceScanner::new())?;
                            Ok(output_ok)
                        })(),
                    )
                },
            )
    }
    fn wire__crate__api__mdns__ServiceScanner_stop_scan_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_async::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "ServiceScanner_stop_scan",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <RustOpaqueMoi<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            ServiceScanner,
                        >,
                    >>::sse_decode(&mut deserializer);
                    deserializer.end();
                    move |context| async move {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || async move {
                                let mut api_that_guard = None;
                                let decode_indices_ = flutter_rust_bridge::for_generated::lockable_compute_decode_order(
                                    <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                                &api_that,
                                                0,
                                                true,
                                            ),
                                        ]),
                                    ),
                                );
                                for i in decode_indices_ {
                                    match i {
                                        0 => {
                                            api_that_guard = Some(
                                                api_that.lockable_decode_async_ref_mut().await,
                                            );
                                        }
                                        _ => {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        }
                                    }
                                }
                                let mut api_that_guard = api_that_guard.unwrap();
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok({
                                    crate::api::mdns::ServiceScanner::stop_scan(
                                            &mut *api_that_guard,
                                        )
                                        .await;
                                })?;
                                Ok(output_ok)
                            })()
                                .await,
                        )
                    }
                },
            )
    }
    fn wire__crate__api__mdns__host_record_connect_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_async::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "host_record_connect",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::api::mdns::HostRecord>::sse_decode(
                        &mut deserializer,
                    );
                    let api_state = <crate::api::types::B64SessionState>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| async move {
                        transform_result_sse::<
                            _,
                            flutter_rust_bridge::for_generated::anyhow::Error,
                        >(
                            (move || async move {
                                let output_ok = crate::api::mdns::HostRecord::connect(
                                        api_that,
                                        api_state,
                                    )
                                    .await?;
                                Ok(output_ok)
                            })()
                                .await,
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__ack_get_type_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "ack_get_type",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::Ack::get_type())?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__ack_get_type_message_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "ack_get_type_message",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::Ack>::sse_decode(&mut deserializer);
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::Ack::get_type_message(&api_that))?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__crypto_message_get_type_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "crypto_message_get_type",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::CryptoMessage::get_type())?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__crypto_message_get_type_message_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "crypto_message_get_type_message",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::CryptoMessage>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(
                                    crate::proto::CryptoMessage::get_type_message(&api_that),
                                )?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__get_events_get_type_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "get_events_get_type",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::GetEvents::get_type())?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__get_events_get_type_message_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "get_events_get_type_message",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::GetEvents>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(
                                    crate::proto::GetEvents::get_type_message(&api_that),
                                )?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__get_identity_command_get_type_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "get_identity_command_get_type",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::GetIdentityCommand::get_type())?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__get_identity_command_get_type_message_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "get_identity_command_get_type_message",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::GetIdentityCommand>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(
                                    crate::proto::GetIdentityCommand::get_type_message(
                                        &api_that,
                                    ),
                                )?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__get_messages_cmd_get_type_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "get_messages_cmd_get_type",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::GetMessagesCmd::get_type())?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__get_messages_cmd_get_type_message_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "get_messages_cmd_get_type_message",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::GetMessagesCmd>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(
                                    crate::proto::GetMessagesCmd::get_type_message(&api_that),
                                )?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__identity_response_get_type_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "identity_response_get_type",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::IdentityResponse::get_type())?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__identity_response_get_type_message_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "identity_response_get_type_message",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::IdentityResponse>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(
                                    crate::proto::IdentityResponse::get_type_message(&api_that),
                                )?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__import_identity_command_get_type_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "import_identity_command_get_type",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::ImportIdentityCommand::get_type())?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__import_identity_command_get_type_message_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "import_identity_command_get_type_message",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::ImportIdentityCommand>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(
                                    crate::proto::ImportIdentityCommand::get_type_message(
                                        &api_that,
                                    ),
                                )?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__import_identity_response_get_type_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "import_identity_response_get_type",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::ImportIdentityResponse::get_type())?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__import_identity_response_get_type_message_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "import_identity_response_get_type_message",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::ImportIdentityResponse>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(
                                    crate::proto::ImportIdentityResponse::get_type_message(
                                        &api_that,
                                    ),
                                )?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__message_response_get_type_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "message_response_get_type",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::MessageResponse::get_type())?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__message_response_get_type_message_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "message_response_get_type_message",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::MessageResponse>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(
                                    crate::proto::MessageResponse::get_type_message(&api_that),
                                )?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__message_type_get_type_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "message_type_get_type",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::MessageType::get_type())?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__message_type_get_type_message_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "message_type_get_type_message",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::MessageType>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(
                                    crate::proto::MessageType::get_type_message(&api_that),
                                )?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__pairing_ack_get_type_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "pairing_ack_get_type",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::PairingAck::get_type())?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__pairing_ack_get_type_message_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "pairing_ack_get_type_message",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::PairingAck>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(
                                    crate::proto::PairingAck::get_type_message(&api_that),
                                )?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__pairing_initiate_get_type_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "pairing_initiate_get_type",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::PairingInitiate::get_type())?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__pairing_initiate_get_type_message_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "pairing_initiate_get_type_message",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::PairingInitiate>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(
                                    crate::proto::PairingInitiate::get_type_message(&api_that),
                                )?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__pairing_request_get_type_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "pairing_request_get_type",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::PairingRequest::get_type())?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__pairing_request_get_type_message_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "pairing_request_get_type_message",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::PairingRequest>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(
                                    crate::proto::PairingRequest::get_type_message(&api_that),
                                )?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__proto_uuid_as_proto_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "proto_uuid_as_proto",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::ProtoUuid>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::ProtoUuid::as_proto(&api_that))?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__proto_uuid_as_uuid_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "proto_uuid_as_uuid",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::ProtoUuid>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::ProtoUuid::as_uuid(&api_that))?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__sb_events_get_type_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "sb_events_get_type",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::SbEvents::get_type())?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__sb_events_get_type_message_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "sb_events_get_type_message",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::SbEvents>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::SbEvents::get_type_message(&api_that))?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__send_message_cmd_get_type_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "send_message_cmd_get_type",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::SendMessageCmd::get_type())?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__send_message_cmd_get_type_message_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "send_message_cmd_get_type_message",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::SendMessageCmd>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(
                                    crate::proto::SendMessageCmd::get_type_message(&api_that),
                                )?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__unit_response_get_type_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "unit_response_get_type",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(crate::proto::UnitResponse::get_type())?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__unit_response_get_type_message_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "unit_response_get_type_message",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::UnitResponse>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(
                                    crate::proto::UnitResponse::get_type_message(&api_that),
                                )?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    fn wire__crate__proto__unit_response_into_remote_err_impl(
        port_: flutter_rust_bridge::for_generated::MessagePort,
        ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len_: i32,
        data_len_: i32,
    ) {
        FLUTTER_RUST_BRIDGE_HANDLER
            .wrap_normal::<
                flutter_rust_bridge::for_generated::SseCodec,
                _,
                _,
            >(
                flutter_rust_bridge::for_generated::TaskInfo {
                    debug_name: "unit_response_into_remote_err",
                    port: Some(port_),
                    mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
                },
                move || {
                    let message = unsafe {
                        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                            ptr_,
                            rust_vec_len_,
                            data_len_,
                        )
                    };
                    let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                        message,
                    );
                    let api_that = <crate::proto::UnitResponse>::sse_decode(
                        &mut deserializer,
                    );
                    deserializer.end();
                    move |context| {
                        transform_result_sse::<
                            _,
                            (),
                        >(
                            (move || {
                                let output_ok = Result::<
                                    _,
                                    (),
                                >::Ok(
                                    crate::proto::UnitResponse::into_remote_err(api_that),
                                )?;
                                Ok(output_ok)
                            })(),
                        )
                    }
                },
            )
    }
    #[allow(clippy::unnecessary_literal_unwrap)]
    const _: fn() = || match None::<crate::api::mirror::IpAddr>.unwrap() {
        crate::api::mirror::IpAddr::V4(field0) => {
            let _: Ipv4Addr = field0;
        }
        crate::api::mirror::IpAddr::V6(field0) => {
            let _: Ipv6Addr = field0;
        }
    };
    fn decode_DartFn_Inputs_list_host_record_Output_unit_AnyhowException(
        dart_opaque: flutter_rust_bridge::DartOpaque,
    ) -> impl Fn(
        Vec<crate::api::mdns::HostRecord>,
    ) -> flutter_rust_bridge::DartFnFuture<()> {
        use flutter_rust_bridge::IntoDart;
        async fn body(
            dart_opaque: flutter_rust_bridge::DartOpaque,
            arg0: Vec<crate::api::mdns::HostRecord>,
        ) -> () {
            let args = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([arg0.into_into_dart().into_dart()]),
            );
            let message = FLUTTER_RUST_BRIDGE_HANDLER
                .dart_fn_invoke(dart_opaque, args)
                .await;
            let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(
                message,
            );
            let action = deserializer.cursor.read_u8().unwrap();
            let ans = match action {
                0 => std::result::Result::Ok(<()>::sse_decode(&mut deserializer)),
                1 => {
                    std::result::Result::Err(
                        <flutter_rust_bridge::for_generated::anyhow::Error>::sse_decode(
                            &mut deserializer,
                        ),
                    )
                }
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            };
            deserializer.end();
            let ans = ans
                .expect("Dart throws exception but Rust side assume it is not failable");
            ans
        }
        move |arg0: Vec<crate::api::mdns::HostRecord>| {
            flutter_rust_bridge::for_generated::convert_into_dart_fn_future(
                body(dart_opaque.clone(), arg0),
            )
        }
    }
    impl MoiArcValue
    for flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv4Addr> {
        fn get_pool() -> &'static MoiArcPool<Self> {
            #[allow(missing_copy_implementations)]
            #[allow(non_camel_case_types)]
            #[allow(dead_code)]
            struct POOL {
                __private_field: (),
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            static POOL: POOL = POOL { __private_field: () };
            impl ::lazy_static::__Deref for POOL {
                type Target = MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv4Addr>,
                >;
                fn deref(
                    &self,
                ) -> &MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv4Addr>,
                > {
                    #[inline(always)]
                    fn __static_ref_initialize() -> MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv4Addr>,
                    > {
                        MoiArcPool::new(Default::default())
                    }
                    #[inline(always)]
                    fn __stability() -> &'static MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv4Addr>,
                    > {
                        static LAZY: ::lazy_static::lazy::Lazy<
                            MoiArcPool<
                                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                                    Ipv4Addr,
                                >,
                            >,
                        > = ::lazy_static::lazy::Lazy::INIT;
                        LAZY.get(__static_ref_initialize)
                    }
                    __stability()
                }
            }
            impl ::lazy_static::LazyStatic for POOL {
                fn initialize(lazy: &Self) {
                    let _ = &**lazy;
                }
            }
            &POOL
        }
    }
    impl MoiArcValue
    for flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv6Addr> {
        fn get_pool() -> &'static MoiArcPool<Self> {
            #[allow(missing_copy_implementations)]
            #[allow(non_camel_case_types)]
            #[allow(dead_code)]
            struct POOL {
                __private_field: (),
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            static POOL: POOL = POOL { __private_field: () };
            impl ::lazy_static::__Deref for POOL {
                type Target = MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv6Addr>,
                >;
                fn deref(
                    &self,
                ) -> &MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv6Addr>,
                > {
                    #[inline(always)]
                    fn __static_ref_initialize() -> MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv6Addr>,
                    > {
                        MoiArcPool::new(Default::default())
                    }
                    #[inline(always)]
                    fn __stability() -> &'static MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv6Addr>,
                    > {
                        static LAZY: ::lazy_static::lazy::Lazy<
                            MoiArcPool<
                                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                                    Ipv6Addr,
                                >,
                            >,
                        > = ::lazy_static::lazy::Lazy::INIT;
                        LAZY.get(__static_ref_initialize)
                    }
                    __stability()
                }
            }
            impl ::lazy_static::LazyStatic for POOL {
                fn initialize(lazy: &Self) {
                    let _ = &**lazy;
                }
            }
            &POOL
        }
    }
    impl MoiArcValue
    for flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
        ::Pin<
            Box<dyn Future<Output = SbResult<ImportIdentityState>> + Send + 'async_trait>,
        >,
    > {
        fn get_pool() -> &'static MoiArcPool<Self> {
            #[allow(missing_copy_implementations)]
            #[allow(non_camel_case_types)]
            #[allow(dead_code)]
            struct POOL {
                __private_field: (),
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            static POOL: POOL = POOL { __private_field: () };
            impl ::lazy_static::__Deref for POOL {
                type Target = MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                        ::Pin<
                            Box<
                                dyn Future<
                                    Output = SbResult<ImportIdentityState>,
                                > + Send + 'async_trait,
                            >,
                        >,
                    >,
                >;
                fn deref(
                    &self,
                ) -> &MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                        ::Pin<
                            Box<
                                dyn Future<
                                    Output = SbResult<ImportIdentityState>,
                                > + Send + 'async_trait,
                            >,
                        >,
                    >,
                > {
                    #[inline(always)]
                    fn __static_ref_initialize() -> MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            ::Pin<
                                Box<
                                    dyn Future<
                                        Output = SbResult<ImportIdentityState>,
                                    > + Send + 'async_trait,
                                >,
                            >,
                        >,
                    > {
                        MoiArcPool::new(Default::default())
                    }
                    #[inline(always)]
                    fn __stability() -> &'static MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            ::Pin<
                                Box<
                                    dyn Future<
                                        Output = SbResult<ImportIdentityState>,
                                    > + Send + 'async_trait,
                                >,
                            >,
                        >,
                    > {
                        static LAZY: ::lazy_static::lazy::Lazy<
                            MoiArcPool<
                                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                                    ::Pin<
                                        Box<
                                            dyn Future<
                                                Output = SbResult<ImportIdentityState>,
                                            > + Send + 'async_trait,
                                        >,
                                    >,
                                >,
                            >,
                        > = ::lazy_static::lazy::Lazy::INIT;
                        LAZY.get(__static_ref_initialize)
                    }
                    __stability()
                }
            }
            impl ::lazy_static::LazyStatic for POOL {
                fn initialize(lazy: &Self) {
                    let _ = &**lazy;
                }
            }
            &POOL
        }
    }
    impl MoiArcValue
    for flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
        ::Pin<Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>>,
    > {
        fn get_pool() -> &'static MoiArcPool<Self> {
            #[allow(missing_copy_implementations)]
            #[allow(non_camel_case_types)]
            #[allow(dead_code)]
            struct POOL {
                __private_field: (),
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            static POOL: POOL = POOL { __private_field: () };
            impl ::lazy_static::__Deref for POOL {
                type Target = MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                        ::Pin<
                            Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>,
                        >,
                    >,
                >;
                fn deref(
                    &self,
                ) -> &MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                        ::Pin<
                            Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>,
                        >,
                    >,
                > {
                    #[inline(always)]
                    fn __static_ref_initialize() -> MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            ::Pin<
                                Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>,
                            >,
                        >,
                    > {
                        MoiArcPool::new(Default::default())
                    }
                    #[inline(always)]
                    fn __stability() -> &'static MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            ::Pin<
                                Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>,
                            >,
                        >,
                    > {
                        static LAZY: ::lazy_static::lazy::Lazy<
                            MoiArcPool<
                                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                                    ::Pin<
                                        Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>,
                                    >,
                                >,
                            >,
                        > = ::lazy_static::lazy::Lazy::INIT;
                        LAZY.get(__static_ref_initialize)
                    }
                    __stability()
                }
            }
            impl ::lazy_static::LazyStatic for POOL {
                fn initialize(lazy: &Self) {
                    let _ = &**lazy;
                }
            }
            &POOL
        }
    }
    impl MoiArcValue
    for flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
        ::Pin<Box<dyn Future<Output = SbResult<Vec<Identity>>> + Send + 'async_trait>>,
    > {
        fn get_pool() -> &'static MoiArcPool<Self> {
            #[allow(missing_copy_implementations)]
            #[allow(non_camel_case_types)]
            #[allow(dead_code)]
            struct POOL {
                __private_field: (),
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            static POOL: POOL = POOL { __private_field: () };
            impl ::lazy_static::__Deref for POOL {
                type Target = MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                        ::Pin<
                            Box<
                                dyn Future<
                                    Output = SbResult<Vec<Identity>>,
                                > + Send + 'async_trait,
                            >,
                        >,
                    >,
                >;
                fn deref(
                    &self,
                ) -> &MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                        ::Pin<
                            Box<
                                dyn Future<
                                    Output = SbResult<Vec<Identity>>,
                                > + Send + 'async_trait,
                            >,
                        >,
                    >,
                > {
                    #[inline(always)]
                    fn __static_ref_initialize() -> MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            ::Pin<
                                Box<
                                    dyn Future<
                                        Output = SbResult<Vec<Identity>>,
                                    > + Send + 'async_trait,
                                >,
                            >,
                        >,
                    > {
                        MoiArcPool::new(Default::default())
                    }
                    #[inline(always)]
                    fn __stability() -> &'static MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            ::Pin<
                                Box<
                                    dyn Future<
                                        Output = SbResult<Vec<Identity>>,
                                    > + Send + 'async_trait,
                                >,
                            >,
                        >,
                    > {
                        static LAZY: ::lazy_static::lazy::Lazy<
                            MoiArcPool<
                                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                                    ::Pin<
                                        Box<
                                            dyn Future<
                                                Output = SbResult<Vec<Identity>>,
                                            > + Send + 'async_trait,
                                        >,
                                    >,
                                >,
                            >,
                        > = ::lazy_static::lazy::Lazy::INIT;
                        LAZY.get(__static_ref_initialize)
                    }
                    __stability()
                }
            }
            impl ::lazy_static::LazyStatic for POOL {
                fn initialize(lazy: &Self) {
                    let _ = &**lazy;
                }
            }
            &POOL
        }
    }
    impl MoiArcValue
    for flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
        ::Pin<Box<dyn Future<Output = SbResult<Vec<Message>>> + Send + 'async_trait>>,
    > {
        fn get_pool() -> &'static MoiArcPool<Self> {
            #[allow(missing_copy_implementations)]
            #[allow(non_camel_case_types)]
            #[allow(dead_code)]
            struct POOL {
                __private_field: (),
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            static POOL: POOL = POOL { __private_field: () };
            impl ::lazy_static::__Deref for POOL {
                type Target = MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                        ::Pin<
                            Box<
                                dyn Future<
                                    Output = SbResult<Vec<Message>>,
                                > + Send + 'async_trait,
                            >,
                        >,
                    >,
                >;
                fn deref(
                    &self,
                ) -> &MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                        ::Pin<
                            Box<
                                dyn Future<
                                    Output = SbResult<Vec<Message>>,
                                > + Send + 'async_trait,
                            >,
                        >,
                    >,
                > {
                    #[inline(always)]
                    fn __static_ref_initialize() -> MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            ::Pin<
                                Box<
                                    dyn Future<
                                        Output = SbResult<Vec<Message>>,
                                    > + Send + 'async_trait,
                                >,
                            >,
                        >,
                    > {
                        MoiArcPool::new(Default::default())
                    }
                    #[inline(always)]
                    fn __stability() -> &'static MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            ::Pin<
                                Box<
                                    dyn Future<
                                        Output = SbResult<Vec<Message>>,
                                    > + Send + 'async_trait,
                                >,
                            >,
                        >,
                    > {
                        static LAZY: ::lazy_static::lazy::Lazy<
                            MoiArcPool<
                                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                                    ::Pin<
                                        Box<
                                            dyn Future<
                                                Output = SbResult<Vec<Message>>,
                                            > + Send + 'async_trait,
                                        >,
                                    >,
                                >,
                            >,
                        > = ::lazy_static::lazy::Lazy::INIT;
                        LAZY.get(__static_ref_initialize)
                    }
                    __stability()
                }
            }
            impl ::lazy_static::LazyStatic for POOL {
                fn initialize(lazy: &Self) {
                    let _ = &**lazy;
                }
            }
            &POOL
        }
    }
    impl MoiArcValue
    for flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
        ::Pin<Box<dyn Future<Output = SbResult<Vec<SbEvent>>> + Send + 'async_trait>>,
    > {
        fn get_pool() -> &'static MoiArcPool<Self> {
            #[allow(missing_copy_implementations)]
            #[allow(non_camel_case_types)]
            #[allow(dead_code)]
            struct POOL {
                __private_field: (),
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            static POOL: POOL = POOL { __private_field: () };
            impl ::lazy_static::__Deref for POOL {
                type Target = MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                        ::Pin<
                            Box<
                                dyn Future<
                                    Output = SbResult<Vec<SbEvent>>,
                                > + Send + 'async_trait,
                            >,
                        >,
                    >,
                >;
                fn deref(
                    &self,
                ) -> &MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                        ::Pin<
                            Box<
                                dyn Future<
                                    Output = SbResult<Vec<SbEvent>>,
                                > + Send + 'async_trait,
                            >,
                        >,
                    >,
                > {
                    #[inline(always)]
                    fn __static_ref_initialize() -> MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            ::Pin<
                                Box<
                                    dyn Future<
                                        Output = SbResult<Vec<SbEvent>>,
                                    > + Send + 'async_trait,
                                >,
                            >,
                        >,
                    > {
                        MoiArcPool::new(Default::default())
                    }
                    #[inline(always)]
                    fn __stability() -> &'static MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            ::Pin<
                                Box<
                                    dyn Future<
                                        Output = SbResult<Vec<SbEvent>>,
                                    > + Send + 'async_trait,
                                >,
                            >,
                        >,
                    > {
                        static LAZY: ::lazy_static::lazy::Lazy<
                            MoiArcPool<
                                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                                    ::Pin<
                                        Box<
                                            dyn Future<
                                                Output = SbResult<Vec<SbEvent>>,
                                            > + Send + 'async_trait,
                                        >,
                                    >,
                                >,
                            >,
                        > = ::lazy_static::lazy::Lazy::INIT;
                        LAZY.get(__static_ref_initialize)
                    }
                    __stability()
                }
            }
            impl ::lazy_static::LazyStatic for POOL {
                fn initialize(lazy: &Self) {
                    let _ = &**lazy;
                }
            }
            &POOL
        }
    }
    impl MoiArcValue
    for flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbResult<()>> {
        fn get_pool() -> &'static MoiArcPool<Self> {
            #[allow(missing_copy_implementations)]
            #[allow(non_camel_case_types)]
            #[allow(dead_code)]
            struct POOL {
                __private_field: (),
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            static POOL: POOL = POOL { __private_field: () };
            impl ::lazy_static::__Deref for POOL {
                type Target = MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbResult<()>>,
                >;
                fn deref(
                    &self,
                ) -> &MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbResult<()>>,
                > {
                    #[inline(always)]
                    fn __static_ref_initialize() -> MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            SbResult<()>,
                        >,
                    > {
                        MoiArcPool::new(Default::default())
                    }
                    #[inline(always)]
                    fn __stability() -> &'static MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            SbResult<()>,
                        >,
                    > {
                        static LAZY: ::lazy_static::lazy::Lazy<
                            MoiArcPool<
                                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                                    SbResult<()>,
                                >,
                            >,
                        > = ::lazy_static::lazy::Lazy::INIT;
                        LAZY.get(__static_ref_initialize)
                    }
                    __stability()
                }
            }
            impl ::lazy_static::LazyStatic for POOL {
                fn initialize(lazy: &Self) {
                    let _ = &**lazy;
                }
            }
            &POOL
        }
    }
    impl MoiArcValue
    for flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbResult<String>> {
        fn get_pool() -> &'static MoiArcPool<Self> {
            #[allow(missing_copy_implementations)]
            #[allow(non_camel_case_types)]
            #[allow(dead_code)]
            struct POOL {
                __private_field: (),
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            static POOL: POOL = POOL { __private_field: () };
            impl ::lazy_static::__Deref for POOL {
                type Target = MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                        SbResult<String>,
                    >,
                >;
                fn deref(
                    &self,
                ) -> &MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                        SbResult<String>,
                    >,
                > {
                    #[inline(always)]
                    fn __static_ref_initialize() -> MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            SbResult<String>,
                        >,
                    > {
                        MoiArcPool::new(Default::default())
                    }
                    #[inline(always)]
                    fn __stability() -> &'static MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            SbResult<String>,
                        >,
                    > {
                        static LAZY: ::lazy_static::lazy::Lazy<
                            MoiArcPool<
                                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                                    SbResult<String>,
                                >,
                            >,
                        > = ::lazy_static::lazy::Lazy::INIT;
                        LAZY.get(__static_ref_initialize)
                    }
                    __stability()
                }
            }
            impl ::lazy_static::LazyStatic for POOL {
                fn initialize(lazy: &Self) {
                    let _ = &**lazy;
                }
            }
            &POOL
        }
    }
    impl MoiArcValue
    for flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbSession> {
        fn get_pool() -> &'static MoiArcPool<Self> {
            #[allow(missing_copy_implementations)]
            #[allow(non_camel_case_types)]
            #[allow(dead_code)]
            struct POOL {
                __private_field: (),
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            static POOL: POOL = POOL { __private_field: () };
            impl ::lazy_static::__Deref for POOL {
                type Target = MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbSession>,
                >;
                fn deref(
                    &self,
                ) -> &MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbSession>,
                > {
                    #[inline(always)]
                    fn __static_ref_initialize() -> MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            SbSession,
                        >,
                    > {
                        MoiArcPool::new(Default::default())
                    }
                    #[inline(always)]
                    fn __stability() -> &'static MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            SbSession,
                        >,
                    > {
                        static LAZY: ::lazy_static::lazy::Lazy<
                            MoiArcPool<
                                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                                    SbSession,
                                >,
                            >,
                        > = ::lazy_static::lazy::Lazy::INIT;
                        LAZY.get(__static_ref_initialize)
                    }
                    __stability()
                }
            }
            impl ::lazy_static::LazyStatic for POOL {
                fn initialize(lazy: &Self) {
                    let _ = &**lazy;
                }
            }
            &POOL
        }
    }
    impl MoiArcValue
    for flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ServiceScanner> {
        fn get_pool() -> &'static MoiArcPool<Self> {
            #[allow(missing_copy_implementations)]
            #[allow(non_camel_case_types)]
            #[allow(dead_code)]
            struct POOL {
                __private_field: (),
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            static POOL: POOL = POOL { __private_field: () };
            impl ::lazy_static::__Deref for POOL {
                type Target = MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                        ServiceScanner,
                    >,
                >;
                fn deref(
                    &self,
                ) -> &MoiArcPool<
                    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                        ServiceScanner,
                    >,
                > {
                    #[inline(always)]
                    fn __static_ref_initialize() -> MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            ServiceScanner,
                        >,
                    > {
                        MoiArcPool::new(Default::default())
                    }
                    #[inline(always)]
                    fn __stability() -> &'static MoiArcPool<
                        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                            ServiceScanner,
                        >,
                    > {
                        static LAZY: ::lazy_static::lazy::Lazy<
                            MoiArcPool<
                                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                                    ServiceScanner,
                                >,
                            >,
                        > = ::lazy_static::lazy::Lazy::INIT;
                        LAZY.get(__static_ref_initialize)
                    }
                    __stability()
                }
            }
            impl ::lazy_static::LazyStatic for POOL {
                fn initialize(lazy: &Self) {
                    let _ = &**lazy;
                }
            }
            &POOL
        }
    }
    impl SseDecode for flutter_rust_bridge::for_generated::anyhow::Error {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <String>::sse_decode(deserializer);
            return ::anyhow::Error::msg(
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(format_args!("{0}", inner));
                    res
                }),
            );
        }
    }
    impl SseDecode for Ipv4Addr {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv4Addr>,
            >>::sse_decode(deserializer);
            return flutter_rust_bridge::for_generated::rust_auto_opaque_decode_owned(
                inner,
            );
        }
    }
    impl SseDecode for Ipv6Addr {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv6Addr>,
            >>::sse_decode(deserializer);
            return flutter_rust_bridge::for_generated::rust_auto_opaque_decode_owned(
                inner,
            );
        }
    }
    impl SseDecode
    for ::Pin<
        Box<dyn Future<Output = SbResult<ImportIdentityState>> + Send + 'async_trait>,
    > {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<
                        Box<
                            dyn Future<
                                Output = SbResult<ImportIdentityState>,
                            > + Send + 'async_trait,
                        >,
                    >,
                >,
            >>::sse_decode(deserializer);
            return flutter_rust_bridge::for_generated::rust_auto_opaque_decode_owned(
                inner,
            );
        }
    }
    impl SseDecode
    for ::Pin<Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>>,
                >,
            >>::sse_decode(deserializer);
            return flutter_rust_bridge::for_generated::rust_auto_opaque_decode_owned(
                inner,
            );
        }
    }
    impl SseDecode
    for ::Pin<Box<dyn Future<Output = SbResult<Vec<Identity>>> + Send + 'async_trait>> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<
                        Box<
                            dyn Future<
                                Output = SbResult<Vec<Identity>>,
                            > + Send + 'async_trait,
                        >,
                    >,
                >,
            >>::sse_decode(deserializer);
            return flutter_rust_bridge::for_generated::rust_auto_opaque_decode_owned(
                inner,
            );
        }
    }
    impl SseDecode
    for ::Pin<Box<dyn Future<Output = SbResult<Vec<Message>>> + Send + 'async_trait>> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<
                        Box<
                            dyn Future<
                                Output = SbResult<Vec<Message>>,
                            > + Send + 'async_trait,
                        >,
                    >,
                >,
            >>::sse_decode(deserializer);
            return flutter_rust_bridge::for_generated::rust_auto_opaque_decode_owned(
                inner,
            );
        }
    }
    impl SseDecode
    for ::Pin<Box<dyn Future<Output = SbResult<Vec<SbEvent>>> + Send + 'async_trait>> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<
                        Box<
                            dyn Future<
                                Output = SbResult<Vec<SbEvent>>,
                            > + Send + 'async_trait,
                        >,
                    >,
                >,
            >>::sse_decode(deserializer);
            return flutter_rust_bridge::for_generated::rust_auto_opaque_decode_owned(
                inner,
            );
        }
    }
    impl SseDecode for SbResult<()> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbResult<()>>,
            >>::sse_decode(deserializer);
            return flutter_rust_bridge::for_generated::rust_auto_opaque_decode_owned(
                inner,
            );
        }
    }
    impl SseDecode for SbResult<String> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbResult<String>>,
            >>::sse_decode(deserializer);
            return flutter_rust_bridge::for_generated::rust_auto_opaque_decode_owned(
                inner,
            );
        }
    }
    impl SseDecode for SbSession {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbSession>,
            >>::sse_decode(deserializer);
            return flutter_rust_bridge::for_generated::rust_auto_opaque_decode_owned(
                inner,
            );
        }
    }
    impl SseDecode for ServiceScanner {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ServiceScanner>,
            >>::sse_decode(deserializer);
            return flutter_rust_bridge::for_generated::rust_auto_opaque_decode_owned(
                inner,
            );
        }
    }
    impl SseDecode for chrono::NaiveDateTime {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <i64>::sse_decode(deserializer);
            return chrono::DateTime::from_timestamp_micros(inner)
                .expect("invalid or out-of-range datetime")
                .naive_utc();
        }
    }
    impl SseDecode for flutter_rust_bridge::DartOpaque {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <usize>::sse_decode(deserializer);
            return unsafe {
                flutter_rust_bridge::for_generated::sse_decode_dart_opaque(inner)
            };
        }
    }
    impl SseDecode for std::collections::HashMap<String, Vec<u8>> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <Vec<(String, Vec<u8>)>>::sse_decode(deserializer);
            return inner.into_iter().collect();
        }
    }
    impl SseDecode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv4Addr>,
    > {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <usize>::sse_decode(deserializer);
            return decode_rust_opaque_moi(inner);
        }
    }
    impl SseDecode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv6Addr>,
    > {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <usize>::sse_decode(deserializer);
            return decode_rust_opaque_moi(inner);
        }
    }
    impl SseDecode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
            ::Pin<
                Box<
                    dyn Future<
                        Output = SbResult<ImportIdentityState>,
                    > + Send + 'async_trait,
                >,
            >,
        >,
    > {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <usize>::sse_decode(deserializer);
            return decode_rust_opaque_moi(inner);
        }
    }
    impl SseDecode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
            ::Pin<Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>>,
        >,
    > {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <usize>::sse_decode(deserializer);
            return decode_rust_opaque_moi(inner);
        }
    }
    impl SseDecode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
            ::Pin<
                Box<dyn Future<Output = SbResult<Vec<Identity>>> + Send + 'async_trait>,
            >,
        >,
    > {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <usize>::sse_decode(deserializer);
            return decode_rust_opaque_moi(inner);
        }
    }
    impl SseDecode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
            ::Pin<Box<dyn Future<Output = SbResult<Vec<Message>>> + Send + 'async_trait>>,
        >,
    > {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <usize>::sse_decode(deserializer);
            return decode_rust_opaque_moi(inner);
        }
    }
    impl SseDecode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
            ::Pin<Box<dyn Future<Output = SbResult<Vec<SbEvent>>> + Send + 'async_trait>>,
        >,
    > {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <usize>::sse_decode(deserializer);
            return decode_rust_opaque_moi(inner);
        }
    }
    impl SseDecode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbResult<()>>,
    > {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <usize>::sse_decode(deserializer);
            return decode_rust_opaque_moi(inner);
        }
    }
    impl SseDecode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbResult<String>>,
    > {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <usize>::sse_decode(deserializer);
            return decode_rust_opaque_moi(inner);
        }
    }
    impl SseDecode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbSession>,
    > {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <usize>::sse_decode(deserializer);
            return decode_rust_opaque_moi(inner);
        }
    }
    impl SseDecode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ServiceScanner>,
    > {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <usize>::sse_decode(deserializer);
            return decode_rust_opaque_moi(inner);
        }
    }
    impl SseDecode for String {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <Vec<u8>>::sse_decode(deserializer);
            return String::from_utf8(inner).unwrap();
        }
    }
    impl SseDecode for uuid::Uuid {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <Vec<u8>>::sse_decode(deserializer);
            return uuid::Uuid::from_slice(&inner).expect("fail to decode uuid");
        }
    }
    impl SseDecode for crate::proto::Ack {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_success = <bool>::sse_decode(deserializer);
            let mut var_status = <i32>::sse_decode(deserializer);
            let mut var_ackMaybeMessage = <Option<
                crate::proto::ack::AckMaybeMessage,
            >>::sse_decode(deserializer);
            return crate::proto::Ack {
                success: var_success,
                status: var_status,
                ack_maybe_message: var_ackMaybeMessage,
            };
        }
    }
    impl SseDecode for crate::proto::ack::AckMaybeMessage {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut tag_ = <i32>::sse_decode(deserializer);
            match tag_ {
                0 => {
                    let mut var_field0 = <String>::sse_decode(deserializer);
                    return crate::proto::ack::AckMaybeMessage::Text(var_field0);
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseDecode for crate::proto::ApiHeader {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_session = <Option<
                crate::proto::ProtoUuid,
            >>::sse_decode(deserializer);
            let mut var_stream = <Option<
                crate::proto::api_header::Stream,
            >>::sse_decode(deserializer);
            return crate::proto::ApiHeader {
                session: var_session,
                stream: var_stream,
            };
        }
    }
    impl SseDecode for crate::proto::ApiIdentity {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_fingerprint = <Option<
                crate::proto::ProtoUuid,
            >>::sse_decode(deserializer);
            let mut var_name = <String>::sse_decode(deserializer);
            let mut var_publicKey = <Vec<u8>>::sse_decode(deserializer);
            let mut var_isOwned = <bool>::sse_decode(deserializer);
            let mut var_extra = <std::collections::HashMap<
                String,
                Vec<u8>,
            >>::sse_decode(deserializer);
            let mut var_sig = <Vec<u8>>::sse_decode(deserializer);
            return crate::proto::ApiIdentity {
                fingerprint: var_fingerprint,
                name: var_name,
                public_key: var_publicKey,
                is_owned: var_isOwned,
                extra: var_extra,
                sig: var_sig,
            };
        }
    }
    impl SseDecode for crate::proto::ApiMessage {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_fromFingerprint = <Option<
                crate::proto::ProtoUuid,
            >>::sse_decode(deserializer);
            let mut var_toFingerprint = <Option<
                crate::proto::ProtoUuid,
            >>::sse_decode(deserializer);
            let mut var_application = <String>::sse_decode(deserializer);
            let mut var_extension_ = <String>::sse_decode(deserializer);
            let mut var_mime = <String>::sse_decode(deserializer);
            let mut var_sendDate = <i64>::sse_decode(deserializer);
            let mut var_receiveDate = <i64>::sse_decode(deserializer);
            let mut var_isFile = <bool>::sse_decode(deserializer);
            let mut var_id = <Option<crate::proto::ProtoUuid>>::sse_decode(deserializer);
            let mut var_body = <Vec<u8>>::sse_decode(deserializer);
            let mut var_fileName = <String>::sse_decode(deserializer);
            return crate::proto::ApiMessage {
                from_fingerprint: var_fromFingerprint,
                to_fingerprint: var_toFingerprint,
                application: var_application,
                extension: var_extension_,
                mime: var_mime,
                send_date: var_sendDate,
                receive_date: var_receiveDate,
                is_file: var_isFile,
                id: var_id,
                body: var_body,
                file_name: var_fileName,
            };
        }
    }
    impl SseDecode for crate::api::types::B64SessionState {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_secretkey = <String>::sse_decode(deserializer);
            let mut var_pubkey = <String>::sse_decode(deserializer);
            let mut var_remotekey = <Option<String>>::sse_decode(deserializer);
            return crate::api::types::B64SessionState {
                secretkey: var_secretkey,
                pubkey: var_pubkey,
                remotekey: var_remotekey,
            };
        }
    }
    impl SseDecode for bool {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            deserializer.cursor.read_u8().unwrap() != 0
        }
    }
    impl SseDecode for crate::proto::CryptoMessage {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_nonce = <Vec<u8>>::sse_decode(deserializer);
            let mut var_encrypted = <Vec<u8>>::sse_decode(deserializer);
            return crate::proto::CryptoMessage {
                nonce: var_nonce,
                encrypted: var_encrypted,
            };
        }
    }
    impl SseDecode for crate::proto::import_identity_response::FinalResponse {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_handle = <Option<
                crate::proto::ProtoUuid,
            >>::sse_decode(deserializer);
            let mut var_identity = <Option<
                crate::proto::ProtoUuid,
            >>::sse_decode(deserializer);
            return crate::proto::import_identity_response::FinalResponse {
                handle: var_handle,
                identity: var_identity,
            };
        }
    }
    impl SseDecode for crate::proto::GetEvents {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_header = <Option<
                crate::proto::ApiHeader,
            >>::sse_decode(deserializer);
            let mut var_block = <bool>::sse_decode(deserializer);
            let mut var_maybeCount = <Option<
                crate::proto::get_events::MaybeCount,
            >>::sse_decode(deserializer);
            return crate::proto::GetEvents {
                header: var_header,
                block: var_block,
                maybe_count: var_maybeCount,
            };
        }
    }
    impl SseDecode for crate::proto::GetIdentityCommand {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_header = <Option<
                crate::proto::ApiHeader,
            >>::sse_decode(deserializer);
            let mut var_owned = <bool>::sse_decode(deserializer);
            let mut var_id = <Option<
                crate::proto::get_identity_command::Id,
            >>::sse_decode(deserializer);
            return crate::proto::GetIdentityCommand {
                header: var_header,
                owned: var_owned,
                id: var_id,
            };
        }
    }
    impl SseDecode for crate::proto::GetMessagesCmd {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_header = <Option<
                crate::proto::ApiHeader,
            >>::sse_decode(deserializer);
            let mut var_limit = <i32>::sse_decode(deserializer);
            let mut var_timeSlice = <Option<
                crate::proto::get_messages_cmd::TimeSlice,
            >>::sse_decode(deserializer);
            let mut var_maybeApplication = <Option<
                crate::proto::get_messages_cmd::MaybeApplication,
            >>::sse_decode(deserializer);
            return crate::proto::GetMessagesCmd {
                header: var_header,
                limit: var_limit,
                time_slice: var_timeSlice,
                maybe_application: var_maybeApplication,
            };
        }
    }
    impl SseDecode for crate::api::mdns::HostRecord {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_name = <String>::sse_decode(deserializer);
            let mut var_addr = <Vec<
                crate::api::mirror::IpAddr,
            >>::sse_decode(deserializer);
            let mut var_port = <u16>::sse_decode(deserializer);
            return crate::api::mdns::HostRecord {
                name: var_name,
                addr: var_addr,
                port: var_port,
            };
        }
    }
    impl SseDecode for i32 {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            deserializer.cursor.read_i32::<NativeEndian>().unwrap()
        }
    }
    impl SseDecode for i64 {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            deserializer.cursor.read_i64::<NativeEndian>().unwrap()
        }
    }
    impl SseDecode for crate::proto::get_identity_command::Id {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut tag_ = <i32>::sse_decode(deserializer);
            match tag_ {
                0 => {
                    let mut var_field0 = <crate::proto::ProtoUuid>::sse_decode(
                        deserializer,
                    );
                    return crate::proto::get_identity_command::Id::Identity(var_field0);
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseDecode for crate::proto::IdentityResponse {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_header = <Option<
                crate::proto::ApiHeader,
            >>::sse_decode(deserializer);
            let mut var_identity = <Vec<
                crate::proto::ApiIdentity,
            >>::sse_decode(deserializer);
            let mut var_code = <i32>::sse_decode(deserializer);
            return crate::proto::IdentityResponse {
                header: var_header,
                identity: var_identity,
                code: var_code,
            };
        }
    }
    impl SseDecode for crate::proto::ImportIdentityCommand {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_header = <Option<
                crate::proto::ApiHeader,
            >>::sse_decode(deserializer);
            let mut var_maybeHandle = <Option<
                crate::proto::import_identity_command::MaybeHandle,
            >>::sse_decode(deserializer);
            return crate::proto::ImportIdentityCommand {
                header: var_header,
                maybe_handle: var_maybeHandle,
            };
        }
    }
    impl SseDecode for crate::proto::ImportIdentityResponse {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_header = <Option<
                crate::proto::ApiHeader,
            >>::sse_decode(deserializer);
            let mut var_code = <i32>::sse_decode(deserializer);
            let mut var_state = <Option<
                crate::proto::import_identity_response::State,
            >>::sse_decode(deserializer);
            return crate::proto::ImportIdentityResponse {
                header: var_header,
                code: var_code,
                state: var_state,
            };
        }
    }
    impl SseDecode for crate::api::mirror::IpAddr {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut tag_ = <i32>::sse_decode(deserializer);
            match tag_ {
                0 => {
                    let mut var_field0 = <Ipv4Addr>::sse_decode(deserializer);
                    return crate::api::mirror::IpAddr::V4(var_field0);
                }
                1 => {
                    let mut var_field0 = <Ipv6Addr>::sse_decode(deserializer);
                    return crate::api::mirror::IpAddr::V6(var_field0);
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseDecode for isize {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            deserializer.cursor.read_i64::<NativeEndian>().unwrap() as _
        }
    }
    impl SseDecode for Vec<crate::proto::ApiIdentity> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut len_ = <i32>::sse_decode(deserializer);
            let mut ans_ = ::alloc::vec::Vec::new();
            for idx_ in 0..len_ {
                ans_.push(<crate::proto::ApiIdentity>::sse_decode(deserializer));
            }
            return ans_;
        }
    }
    impl SseDecode for Vec<crate::proto::ApiMessage> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut len_ = <i32>::sse_decode(deserializer);
            let mut ans_ = ::alloc::vec::Vec::new();
            for idx_ in 0..len_ {
                ans_.push(<crate::proto::ApiMessage>::sse_decode(deserializer));
            }
            return ans_;
        }
    }
    impl SseDecode for Vec<crate::api::mdns::HostRecord> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut len_ = <i32>::sse_decode(deserializer);
            let mut ans_ = ::alloc::vec::Vec::new();
            for idx_ in 0..len_ {
                ans_.push(<crate::api::mdns::HostRecord>::sse_decode(deserializer));
            }
            return ans_;
        }
    }
    impl SseDecode for Vec<crate::api::mirror::IpAddr> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut len_ = <i32>::sse_decode(deserializer);
            let mut ans_ = ::alloc::vec::Vec::new();
            for idx_ in 0..len_ {
                ans_.push(<crate::api::mirror::IpAddr>::sse_decode(deserializer));
            }
            return ans_;
        }
    }
    impl SseDecode for Vec<crate::api::response::Message> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut len_ = <i32>::sse_decode(deserializer);
            let mut ans_ = ::alloc::vec::Vec::new();
            for idx_ in 0..len_ {
                ans_.push(<crate::api::response::Message>::sse_decode(deserializer));
            }
            return ans_;
        }
    }
    impl SseDecode for Vec<crate::proto::sb_event::NoBodyMessage> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut len_ = <i32>::sse_decode(deserializer);
            let mut ans_ = ::alloc::vec::Vec::new();
            for idx_ in 0..len_ {
                ans_.push(
                    <crate::proto::sb_event::NoBodyMessage>::sse_decode(deserializer),
                );
            }
            return ans_;
        }
    }
    impl SseDecode for Vec<u8> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut len_ = <i32>::sse_decode(deserializer);
            let mut ans_ = ::alloc::vec::Vec::new();
            for idx_ in 0..len_ {
                ans_.push(<u8>::sse_decode(deserializer));
            }
            return ans_;
        }
    }
    impl SseDecode for Vec<(String, Vec<u8>)> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut len_ = <i32>::sse_decode(deserializer);
            let mut ans_ = ::alloc::vec::Vec::new();
            for idx_ in 0..len_ {
                ans_.push(<(String, Vec<u8>)>::sse_decode(deserializer));
            }
            return ans_;
        }
    }
    impl SseDecode for Vec<crate::proto::SbEvent> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut len_ = <i32>::sse_decode(deserializer);
            let mut ans_ = ::alloc::vec::Vec::new();
            for idx_ in 0..len_ {
                ans_.push(<crate::proto::SbEvent>::sse_decode(deserializer));
            }
            return ans_;
        }
    }
    impl SseDecode for crate::proto::get_messages_cmd::MaybeApplication {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut tag_ = <i32>::sse_decode(deserializer);
            match tag_ {
                0 => {
                    let mut var_field0 = <String>::sse_decode(deserializer);
                    return crate::proto::get_messages_cmd::MaybeApplication::Application(
                        var_field0,
                    );
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseDecode for crate::proto::get_events::MaybeCount {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut tag_ = <i32>::sse_decode(deserializer);
            match tag_ {
                0 => {
                    let mut var_field0 = <u32>::sse_decode(deserializer);
                    return crate::proto::get_events::MaybeCount::Count(var_field0);
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseDecode for crate::proto::sb_event::MaybeEvent {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut tag_ = <i32>::sse_decode(deserializer);
            match tag_ {
                0 => {
                    let mut var_field0 = <crate::proto::sb_event::NewMessage>::sse_decode(
                        deserializer,
                    );
                    return crate::proto::sb_event::MaybeEvent::NewMessage(var_field0);
                }
                1 => {
                    let mut var_field0 = <crate::proto::sb_event::NewIdentity>::sse_decode(
                        deserializer,
                    );
                    return crate::proto::sb_event::MaybeEvent::NewIdentities(var_field0);
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseDecode for crate::proto::import_identity_command::MaybeHandle {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut tag_ = <i32>::sse_decode(deserializer);
            match tag_ {
                0 => {
                    let mut var_field0 = <crate::proto::ProtoUuid>::sse_decode(
                        deserializer,
                    );
                    return crate::proto::import_identity_command::MaybeHandle::Handle(
                        var_field0,
                    );
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseDecode for crate::api::response::Message {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_fromFingerprint = <Option<uuid::Uuid>>::sse_decode(deserializer);
            let mut var_toFingerprint = <Option<uuid::Uuid>>::sse_decode(deserializer);
            let mut var_application = <String>::sse_decode(deserializer);
            let mut var_extension_ = <String>::sse_decode(deserializer);
            let mut var_mime = <String>::sse_decode(deserializer);
            let mut var_sendDate = <i64>::sse_decode(deserializer);
            let mut var_receiveDate = <i64>::sse_decode(deserializer);
            let mut var_isFile = <bool>::sse_decode(deserializer);
            let mut var_id = <Option<uuid::Uuid>>::sse_decode(deserializer);
            let mut var_body = <Vec<u8>>::sse_decode(deserializer);
            let mut var_fileName = <String>::sse_decode(deserializer);
            return crate::api::response::Message {
                from_fingerprint: var_fromFingerprint,
                to_fingerprint: var_toFingerprint,
                application: var_application,
                extension: var_extension_,
                mime: var_mime,
                send_date: var_sendDate,
                receive_date: var_receiveDate,
                is_file: var_isFile,
                id: var_id,
                body: var_body,
                file_name: var_fileName,
            };
        }
    }
    impl SseDecode for crate::proto::MessageResponse {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_header = <Option<
                crate::proto::ApiHeader,
            >>::sse_decode(deserializer);
            let mut var_messsage = <Vec<
                crate::proto::ApiMessage,
            >>::sse_decode(deserializer);
            let mut var_code = <i32>::sse_decode(deserializer);
            return crate::proto::MessageResponse {
                header: var_header,
                messsage: var_messsage,
                code: var_code,
            };
        }
    }
    impl SseDecode for crate::proto::MessageType {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut inner = <i32>::sse_decode(deserializer);
            return match inner {
                0 => crate::proto::MessageType::Advertise,
                1 => crate::proto::MessageType::DeclareHashes,
                2 => crate::proto::MessageType::BlockHeader,
                3 => crate::proto::MessageType::BlockSequence,
                4 => crate::proto::MessageType::ElectLeader,
                5 => crate::proto::MessageType::Upgrade,
                6 => crate::proto::MessageType::RoutingMetadata,
                7 => crate::proto::MessageType::IpAnnounce,
                8 => crate::proto::MessageType::Identity,
                9 => crate::proto::MessageType::Luid,
                10 => crate::proto::MessageType::JustUkes,
                11 => crate::proto::MessageType::Ack,
                12 => crate::proto::MessageType::Invalid,
                13 => crate::proto::MessageType::GetMessage,
                14 => crate::proto::MessageType::GetIdentity,
                15 => crate::proto::MessageType::SendMessage,
                16 => crate::proto::MessageType::Message,
                17 => crate::proto::MessageType::UnitResponse,
                18 => crate::proto::MessageType::CryptoMessage,
                19 => crate::proto::MessageType::PairingRequest,
                20 => crate::proto::MessageType::PairingInitiate,
                21 => crate::proto::MessageType::PairingCompleted,
                22 => crate::proto::MessageType::PairingAck,
                23 => crate::proto::MessageType::IdentityResponse,
                24 => crate::proto::MessageType::ApiIdentity,
                25 => crate::proto::MessageType::MessageResponse,
                26 => crate::proto::MessageType::ApiHeader,
                27 => crate::proto::MessageType::GenerateIdentity,
                28 => crate::proto::MessageType::ImportIdentity,
                29 => crate::proto::MessageType::ImportIdentityResponse,
                30 => crate::proto::MessageType::GenerateIdentityResponse,
                31 => crate::proto::MessageType::GetEvents,
                32 => crate::proto::MessageType::DesktopEvents,
                33 => crate::proto::MessageType::DesktopEvent,
                34 => crate::proto::MessageType::NoBodyMessage,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("Invalid variant for MessageType: {0}", inner),
                        ),
                    );
                }
            };
        }
    }
    impl SseDecode for crate::proto::sb_event::NewIdentity {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_identities = <Vec<
                crate::proto::ApiIdentity,
            >>::sse_decode(deserializer);
            return crate::proto::sb_event::NewIdentity {
                identities: var_identities,
            };
        }
    }
    impl SseDecode for crate::proto::sb_event::NewMessage {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_messages = <Vec<
                crate::proto::sb_event::NoBodyMessage,
            >>::sse_decode(deserializer);
            return crate::proto::sb_event::NewMessage {
                messages: var_messages,
            };
        }
    }
    impl SseDecode for crate::proto::sb_event::NoBodyMessage {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_fromFingerprint = <Option<
                crate::proto::ProtoUuid,
            >>::sse_decode(deserializer);
            let mut var_toFingerprint = <Option<
                crate::proto::ProtoUuid,
            >>::sse_decode(deserializer);
            let mut var_application = <String>::sse_decode(deserializer);
            let mut var_extension_ = <String>::sse_decode(deserializer);
            let mut var_mime = <String>::sse_decode(deserializer);
            let mut var_sendDate = <i64>::sse_decode(deserializer);
            let mut var_receiveDate = <i64>::sse_decode(deserializer);
            let mut var_isFile = <bool>::sse_decode(deserializer);
            let mut var_id = <Option<crate::proto::ProtoUuid>>::sse_decode(deserializer);
            let mut var_fileName = <String>::sse_decode(deserializer);
            return crate::proto::sb_event::NoBodyMessage {
                from_fingerprint: var_fromFingerprint,
                to_fingerprint: var_toFingerprint,
                application: var_application,
                extension: var_extension_,
                mime: var_mime,
                send_date: var_sendDate,
                receive_date: var_receiveDate,
                is_file: var_isFile,
                id: var_id,
                file_name: var_fileName,
            };
        }
    }
    impl SseDecode for Option<String> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            if (<bool>::sse_decode(deserializer)) {
                return Some(<String>::sse_decode(deserializer));
            } else {
                return None;
            }
        }
    }
    impl SseDecode for Option<uuid::Uuid> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            if (<bool>::sse_decode(deserializer)) {
                return Some(<uuid::Uuid>::sse_decode(deserializer));
            } else {
                return None;
            }
        }
    }
    impl SseDecode for Option<SbSession> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            if (<bool>::sse_decode(deserializer)) {
                return Some(<SbSession>::sse_decode(deserializer));
            } else {
                return None;
            }
        }
    }
    impl SseDecode for Option<crate::proto::ack::AckMaybeMessage> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            if (<bool>::sse_decode(deserializer)) {
                return Some(
                    <crate::proto::ack::AckMaybeMessage>::sse_decode(deserializer),
                );
            } else {
                return None;
            }
        }
    }
    impl SseDecode for Option<crate::proto::ApiHeader> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            if (<bool>::sse_decode(deserializer)) {
                return Some(<crate::proto::ApiHeader>::sse_decode(deserializer));
            } else {
                return None;
            }
        }
    }
    impl SseDecode for Option<i32> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            if (<bool>::sse_decode(deserializer)) {
                return Some(<i32>::sse_decode(deserializer));
            } else {
                return None;
            }
        }
    }
    impl SseDecode for Option<crate::proto::get_identity_command::Id> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            if (<bool>::sse_decode(deserializer)) {
                return Some(
                    <crate::proto::get_identity_command::Id>::sse_decode(deserializer),
                );
            } else {
                return None;
            }
        }
    }
    impl SseDecode for Option<crate::proto::get_messages_cmd::MaybeApplication> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            if (<bool>::sse_decode(deserializer)) {
                return Some(
                    <crate::proto::get_messages_cmd::MaybeApplication>::sse_decode(
                        deserializer,
                    ),
                );
            } else {
                return None;
            }
        }
    }
    impl SseDecode for Option<crate::proto::get_events::MaybeCount> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            if (<bool>::sse_decode(deserializer)) {
                return Some(
                    <crate::proto::get_events::MaybeCount>::sse_decode(deserializer),
                );
            } else {
                return None;
            }
        }
    }
    impl SseDecode for Option<crate::proto::sb_event::MaybeEvent> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            if (<bool>::sse_decode(deserializer)) {
                return Some(
                    <crate::proto::sb_event::MaybeEvent>::sse_decode(deserializer),
                );
            } else {
                return None;
            }
        }
    }
    impl SseDecode for Option<crate::proto::import_identity_command::MaybeHandle> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            if (<bool>::sse_decode(deserializer)) {
                return Some(
                    <crate::proto::import_identity_command::MaybeHandle>::sse_decode(
                        deserializer,
                    ),
                );
            } else {
                return None;
            }
        }
    }
    impl SseDecode for Option<crate::proto::ProtoUuid> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            if (<bool>::sse_decode(deserializer)) {
                return Some(<crate::proto::ProtoUuid>::sse_decode(deserializer));
            } else {
                return None;
            }
        }
    }
    impl SseDecode for Option<crate::proto::send_message_cmd::SignIdentity> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            if (<bool>::sse_decode(deserializer)) {
                return Some(
                    <crate::proto::send_message_cmd::SignIdentity>::sse_decode(
                        deserializer,
                    ),
                );
            } else {
                return None;
            }
        }
    }
    impl SseDecode for Option<crate::proto::import_identity_response::State> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            if (<bool>::sse_decode(deserializer)) {
                return Some(
                    <crate::proto::import_identity_response::State>::sse_decode(
                        deserializer,
                    ),
                );
            } else {
                return None;
            }
        }
    }
    impl SseDecode for Option<crate::proto::api_header::Stream> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            if (<bool>::sse_decode(deserializer)) {
                return Some(
                    <crate::proto::api_header::Stream>::sse_decode(deserializer),
                );
            } else {
                return None;
            }
        }
    }
    impl SseDecode for Option<crate::proto::get_messages_cmd::TimeSlice> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            if (<bool>::sse_decode(deserializer)) {
                return Some(
                    <crate::proto::get_messages_cmd::TimeSlice>::sse_decode(deserializer),
                );
            } else {
                return None;
            }
        }
    }
    impl SseDecode for Option<u32> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            if (<bool>::sse_decode(deserializer)) {
                return Some(<u32>::sse_decode(deserializer));
            } else {
                return None;
            }
        }
    }
    impl SseDecode for Option<crate::proto::unit_response::UnitresponseMaybeMessage> {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            if (<bool>::sse_decode(deserializer)) {
                return Some(
                    <crate::proto::unit_response::UnitresponseMaybeMessage>::sse_decode(
                        deserializer,
                    ),
                );
            } else {
                return None;
            }
        }
    }
    impl SseDecode for crate::proto::PairingAck {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_session = <Option<
                crate::proto::ApiHeader,
            >>::sse_decode(deserializer);
            let mut var_pubkey = <Vec<u8>>::sse_decode(deserializer);
            return crate::proto::PairingAck {
                session: var_session,
                pubkey: var_pubkey,
            };
        }
    }
    impl SseDecode for crate::proto::PairingInitiate {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_pubkey = <Vec<u8>>::sse_decode(deserializer);
            return crate::proto::PairingInitiate {
                pubkey: var_pubkey,
            };
        }
    }
    impl SseDecode for crate::proto::PairingRequest {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_session = <Option<
                crate::proto::ApiHeader,
            >>::sse_decode(deserializer);
            let mut var_name = <String>::sse_decode(deserializer);
            return crate::proto::PairingRequest {
                session: var_session,
                name: var_name,
            };
        }
    }
    impl SseDecode for crate::proto::ProtoUuid {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_lower = <u64>::sse_decode(deserializer);
            let mut var_upper = <u64>::sse_decode(deserializer);
            return crate::proto::ProtoUuid {
                lower: var_lower,
                upper: var_upper,
            };
        }
    }
    impl SseDecode for (String, Vec<u8>) {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_field0 = <String>::sse_decode(deserializer);
            let mut var_field1 = <Vec<u8>>::sse_decode(deserializer);
            return (var_field0, var_field1);
        }
    }
    impl SseDecode for crate::proto::SbEvent {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_maybeEvent = <Option<
                crate::proto::sb_event::MaybeEvent,
            >>::sse_decode(deserializer);
            return crate::proto::SbEvent {
                maybe_event: var_maybeEvent,
            };
        }
    }
    impl SseDecode for crate::proto::SbEvents {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_header = <Option<
                crate::proto::ApiHeader,
            >>::sse_decode(deserializer);
            let mut var_events = <Vec<crate::proto::SbEvent>>::sse_decode(deserializer);
            return crate::proto::SbEvents {
                header: var_header,
                events: var_events,
            };
        }
    }
    impl SseDecode for crate::proto::SendMessageCmd {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_header = <Option<
                crate::proto::ApiHeader,
            >>::sse_decode(deserializer);
            let mut var_messages = <Vec<
                crate::proto::ApiMessage,
            >>::sse_decode(deserializer);
            let mut var_signIdentity = <Option<
                crate::proto::send_message_cmd::SignIdentity,
            >>::sse_decode(deserializer);
            return crate::proto::SendMessageCmd {
                header: var_header,
                messages: var_messages,
                sign_identity: var_signIdentity,
            };
        }
    }
    impl SseDecode for crate::proto::send_message_cmd::SignIdentity {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut tag_ = <i32>::sse_decode(deserializer);
            match tag_ {
                0 => {
                    let mut var_field0 = <crate::proto::ProtoUuid>::sse_decode(
                        deserializer,
                    );
                    return crate::proto::send_message_cmd::SignIdentity::Identity(
                        var_field0,
                    );
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseDecode for crate::proto::import_identity_response::State {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut tag_ = <i32>::sse_decode(deserializer);
            match tag_ {
                0 => {
                    let mut var_field0 = <crate::proto::import_identity_response::FinalResponse>::sse_decode(
                        deserializer,
                    );
                    return crate::proto::import_identity_response::State::Final(
                        var_field0,
                    );
                }
                1 => {
                    let mut var_field0 = <crate::proto::ProtoUuid>::sse_decode(
                        deserializer,
                    );
                    return crate::proto::import_identity_response::State::Handle(
                        var_field0,
                    );
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseDecode for crate::proto::api_header::Stream {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut tag_ = <i32>::sse_decode(deserializer);
            match tag_ {
                0 => {
                    let mut var_field0 = <i32>::sse_decode(deserializer);
                    return crate::proto::api_header::Stream::StreamId(var_field0);
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseDecode for crate::proto::get_messages_cmd::TimeRange {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_start = <i64>::sse_decode(deserializer);
            let mut var_end = <i64>::sse_decode(deserializer);
            return crate::proto::get_messages_cmd::TimeRange {
                start: var_start,
                end: var_end,
            };
        }
    }
    impl SseDecode for crate::proto::get_messages_cmd::TimeSlice {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut tag_ = <i32>::sse_decode(deserializer);
            match tag_ {
                0 => {
                    let mut var_field0 = <crate::proto::get_messages_cmd::TimeRange>::sse_decode(
                        deserializer,
                    );
                    return crate::proto::get_messages_cmd::TimeSlice::SendDate(
                        var_field0,
                    );
                }
                1 => {
                    let mut var_field0 = <crate::proto::get_messages_cmd::TimeRange>::sse_decode(
                        deserializer,
                    );
                    return crate::proto::get_messages_cmd::TimeSlice::ReceiveDate(
                        var_field0,
                    );
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseDecode for crate::proto::TypePrefix {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_type = <i32>::sse_decode(deserializer);
            return crate::proto::TypePrefix {
                r#type: var_type,
            };
        }
    }
    impl SseDecode for u16 {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            deserializer.cursor.read_u16::<NativeEndian>().unwrap()
        }
    }
    impl SseDecode for u32 {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            deserializer.cursor.read_u32::<NativeEndian>().unwrap()
        }
    }
    impl SseDecode for u64 {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            deserializer.cursor.read_u64::<NativeEndian>().unwrap()
        }
    }
    impl SseDecode for u8 {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            deserializer.cursor.read_u8().unwrap()
        }
    }
    impl SseDecode for () {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {}
    }
    impl SseDecode for crate::proto::UnitResponse {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut var_header = <Option<
                crate::proto::ApiHeader,
            >>::sse_decode(deserializer);
            let mut var_code = <i32>::sse_decode(deserializer);
            let mut var_unitresponseMaybeMessage = <Option<
                crate::proto::unit_response::UnitresponseMaybeMessage,
            >>::sse_decode(deserializer);
            return crate::proto::UnitResponse {
                header: var_header,
                code: var_code,
                unitresponse_maybe_message: var_unitresponseMaybeMessage,
            };
        }
    }
    impl SseDecode for crate::proto::unit_response::UnitresponseMaybeMessage {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            let mut tag_ = <i32>::sse_decode(deserializer);
            match tag_ {
                0 => {
                    let mut var_field0 = <String>::sse_decode(deserializer);
                    return crate::proto::unit_response::UnitresponseMaybeMessage::MessageCode(
                        var_field0,
                    );
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseDecode for usize {
        fn sse_decode(
            deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
        ) -> Self {
            deserializer.cursor.read_u64::<NativeEndian>().unwrap() as _
        }
    }
    fn pde_ffi_dispatcher_primary_impl(
        func_id: i32,
        port: flutter_rust_bridge::for_generated::MessagePort,
        ptr: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len: i32,
        data_len: i32,
    ) {
        match func_id {
            1 => {
                wire__crate__Ipv4Addr_is_loopback_impl(port, ptr, rust_vec_len, data_len)
            }
            2 => {
                wire__crate__Ipv4Addr_is_multicast_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            3 => {
                wire__crate__Ipv4Addr_is_unspecified_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            5 => {
                wire__crate__Ipv6Addr_is_loopback_impl(port, ptr, rust_vec_len, data_len)
            }
            6 => {
                wire__crate__Ipv6Addr_is_multicast_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            7 => {
                wire__crate__Ipv6Addr_is_unspecified_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            10 => {
                wire__crate__api__mdns__ServiceScanner_discover_devices_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            12 => {
                wire__crate__api__mdns__ServiceScanner_stop_scan_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            13 => {
                wire__crate__api__mdns__host_record_connect_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            26 => {
                wire__crate__proto__ack_get_type_impl(port, ptr, rust_vec_len, data_len)
            }
            27 => {
                wire__crate__proto__ack_get_type_message_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            28 => {
                wire__crate__proto__crypto_message_get_type_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            29 => {
                wire__crate__proto__crypto_message_get_type_message_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            30 => {
                wire__crate__proto__get_events_get_type_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            31 => {
                wire__crate__proto__get_events_get_type_message_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            32 => {
                wire__crate__proto__get_identity_command_get_type_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            33 => {
                wire__crate__proto__get_identity_command_get_type_message_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            34 => {
                wire__crate__proto__get_messages_cmd_get_type_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            35 => {
                wire__crate__proto__get_messages_cmd_get_type_message_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            36 => {
                wire__crate__proto__identity_response_get_type_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            37 => {
                wire__crate__proto__identity_response_get_type_message_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            38 => {
                wire__crate__proto__import_identity_command_get_type_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            39 => {
                wire__crate__proto__import_identity_command_get_type_message_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            40 => {
                wire__crate__proto__import_identity_response_get_type_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            41 => {
                wire__crate__proto__import_identity_response_get_type_message_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            42 => {
                wire__crate__proto__message_response_get_type_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            43 => {
                wire__crate__proto__message_response_get_type_message_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            44 => {
                wire__crate__proto__message_type_get_type_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            45 => {
                wire__crate__proto__message_type_get_type_message_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            46 => {
                wire__crate__proto__pairing_ack_get_type_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            47 => {
                wire__crate__proto__pairing_ack_get_type_message_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            48 => {
                wire__crate__proto__pairing_initiate_get_type_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            49 => {
                wire__crate__proto__pairing_initiate_get_type_message_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            50 => {
                wire__crate__proto__pairing_request_get_type_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            51 => {
                wire__crate__proto__pairing_request_get_type_message_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            52 => {
                wire__crate__proto__proto_uuid_as_proto_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            53 => {
                wire__crate__proto__proto_uuid_as_uuid_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            54 => {
                wire__crate__proto__sb_events_get_type_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            55 => {
                wire__crate__proto__sb_events_get_type_message_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            56 => {
                wire__crate__proto__send_message_cmd_get_type_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            57 => {
                wire__crate__proto__send_message_cmd_get_type_message_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            58 => {
                wire__crate__proto__unit_response_get_type_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            59 => {
                wire__crate__proto__unit_response_get_type_message_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            60 => {
                wire__crate__proto__unit_response_into_remote_err_impl(
                    port,
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    fn pde_ffi_dispatcher_sync_impl(
        func_id: i32,
        ptr: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
        rust_vec_len: i32,
        data_len: i32,
    ) -> flutter_rust_bridge::for_generated::WireSyncRust2DartSse {
        match func_id {
            4 => wire__crate__Ipv4Addr_to_string_impl(ptr, rust_vec_len, data_len),
            8 => wire__crate__Ipv6Addr_to_string_impl(ptr, rust_vec_len, data_len),
            11 => {
                wire__crate__api__mdns__ServiceScanner_new_impl(
                    ptr,
                    rust_vec_len,
                    data_len,
                )
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    impl flutter_rust_bridge::IntoDart for FrbWrapper<Ipv4Addr> {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                _,
                MoiArc<_>,
            >(self.0)
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<Ipv4Addr> {}
    impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<Ipv4Addr>> for Ipv4Addr {
        fn into_into_dart(self) -> FrbWrapper<Ipv4Addr> {
            self.into()
        }
    }
    impl flutter_rust_bridge::IntoDart for FrbWrapper<Ipv6Addr> {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                _,
                MoiArc<_>,
            >(self.0)
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<Ipv6Addr> {}
    impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<Ipv6Addr>> for Ipv6Addr {
        fn into_into_dart(self) -> FrbWrapper<Ipv6Addr> {
            self.into()
        }
    }
    impl flutter_rust_bridge::IntoDart
    for FrbWrapper<
        ::Pin<
            Box<dyn Future<Output = SbResult<ImportIdentityState>> + Send + 'async_trait>,
        >,
    > {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                _,
                MoiArc<_>,
            >(self.0)
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<
        ::Pin<
            Box<dyn Future<Output = SbResult<ImportIdentityState>> + Send + 'async_trait>,
        >,
    > {}
    impl flutter_rust_bridge::IntoIntoDart<
        FrbWrapper<
            ::Pin<
                Box<
                    dyn Future<
                        Output = SbResult<ImportIdentityState>,
                    > + Send + 'async_trait,
                >,
            >,
        >,
    >
    for ::Pin<
        Box<dyn Future<Output = SbResult<ImportIdentityState>> + Send + 'async_trait>,
    > {
        fn into_into_dart(
            self,
        ) -> FrbWrapper<
            ::Pin<
                Box<
                    dyn Future<
                        Output = SbResult<ImportIdentityState>,
                    > + Send + 'async_trait,
                >,
            >,
        > {
            self.into()
        }
    }
    impl flutter_rust_bridge::IntoDart
    for FrbWrapper<::Pin<Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>>> {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                _,
                MoiArc<_>,
            >(self.0)
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<::Pin<Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>>> {}
    impl flutter_rust_bridge::IntoIntoDart<
        FrbWrapper<::Pin<Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>>>,
    > for ::Pin<Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>> {
        fn into_into_dart(
            self,
        ) -> FrbWrapper<
            ::Pin<Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>>,
        > {
            self.into()
        }
    }
    impl flutter_rust_bridge::IntoDart
    for FrbWrapper<
        ::Pin<Box<dyn Future<Output = SbResult<Vec<Identity>>> + Send + 'async_trait>>,
    > {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                _,
                MoiArc<_>,
            >(self.0)
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<
        ::Pin<Box<dyn Future<Output = SbResult<Vec<Identity>>> + Send + 'async_trait>>,
    > {}
    impl flutter_rust_bridge::IntoIntoDart<
        FrbWrapper<
            ::Pin<
                Box<dyn Future<Output = SbResult<Vec<Identity>>> + Send + 'async_trait>,
            >,
        >,
    >
    for ::Pin<Box<dyn Future<Output = SbResult<Vec<Identity>>> + Send + 'async_trait>> {
        fn into_into_dart(
            self,
        ) -> FrbWrapper<
            ::Pin<
                Box<dyn Future<Output = SbResult<Vec<Identity>>> + Send + 'async_trait>,
            >,
        > {
            self.into()
        }
    }
    impl flutter_rust_bridge::IntoDart
    for FrbWrapper<
        ::Pin<Box<dyn Future<Output = SbResult<Vec<Message>>> + Send + 'async_trait>>,
    > {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                _,
                MoiArc<_>,
            >(self.0)
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<
        ::Pin<Box<dyn Future<Output = SbResult<Vec<Message>>> + Send + 'async_trait>>,
    > {}
    impl flutter_rust_bridge::IntoIntoDart<
        FrbWrapper<
            ::Pin<Box<dyn Future<Output = SbResult<Vec<Message>>> + Send + 'async_trait>>,
        >,
    > for ::Pin<Box<dyn Future<Output = SbResult<Vec<Message>>> + Send + 'async_trait>> {
        fn into_into_dart(
            self,
        ) -> FrbWrapper<
            ::Pin<Box<dyn Future<Output = SbResult<Vec<Message>>> + Send + 'async_trait>>,
        > {
            self.into()
        }
    }
    impl flutter_rust_bridge::IntoDart
    for FrbWrapper<
        ::Pin<Box<dyn Future<Output = SbResult<Vec<SbEvent>>> + Send + 'async_trait>>,
    > {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                _,
                MoiArc<_>,
            >(self.0)
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<
        ::Pin<Box<dyn Future<Output = SbResult<Vec<SbEvent>>> + Send + 'async_trait>>,
    > {}
    impl flutter_rust_bridge::IntoIntoDart<
        FrbWrapper<
            ::Pin<Box<dyn Future<Output = SbResult<Vec<SbEvent>>> + Send + 'async_trait>>,
        >,
    > for ::Pin<Box<dyn Future<Output = SbResult<Vec<SbEvent>>> + Send + 'async_trait>> {
        fn into_into_dart(
            self,
        ) -> FrbWrapper<
            ::Pin<Box<dyn Future<Output = SbResult<Vec<SbEvent>>> + Send + 'async_trait>>,
        > {
            self.into()
        }
    }
    impl flutter_rust_bridge::IntoDart for FrbWrapper<SbResult<()>> {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                _,
                MoiArc<_>,
            >(self.0)
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<SbResult<()>> {}
    impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<SbResult<()>>> for SbResult<()> {
        fn into_into_dart(self) -> FrbWrapper<SbResult<()>> {
            self.into()
        }
    }
    impl flutter_rust_bridge::IntoDart for FrbWrapper<SbResult<String>> {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                _,
                MoiArc<_>,
            >(self.0)
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<SbResult<String>> {}
    impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<SbResult<String>>>
    for SbResult<String> {
        fn into_into_dart(self) -> FrbWrapper<SbResult<String>> {
            self.into()
        }
    }
    impl flutter_rust_bridge::IntoDart for FrbWrapper<SbSession> {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                _,
                MoiArc<_>,
            >(self.0)
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<SbSession> {}
    impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<SbSession>> for SbSession {
        fn into_into_dart(self) -> FrbWrapper<SbSession> {
            self.into()
        }
    }
    impl flutter_rust_bridge::IntoDart for FrbWrapper<ServiceScanner> {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                _,
                MoiArc<_>,
            >(self.0)
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<ServiceScanner> {}
    impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<ServiceScanner>>
    for ServiceScanner {
        fn into_into_dart(self) -> FrbWrapper<ServiceScanner> {
            self.into()
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::Ack {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.success.into_into_dart().into_dart(),
                self.status.into_into_dart().into_dart(),
                self.ack_maybe_message.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::Ack {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::Ack> for crate::proto::Ack {
        fn into_into_dart(self) -> crate::proto::Ack {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::ack::AckMaybeMessage {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            match self {
                crate::proto::ack::AckMaybeMessage::Text(field0) => {
                    [0.into_dart(), field0.into_into_dart().into_dart()].into_dart()
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::ack::AckMaybeMessage {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::ack::AckMaybeMessage>
    for crate::proto::ack::AckMaybeMessage {
        fn into_into_dart(self) -> crate::proto::ack::AckMaybeMessage {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::ApiHeader {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.session.into_into_dart().into_dart(),
                self.stream.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::ApiHeader {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::ApiHeader>
    for crate::proto::ApiHeader {
        fn into_into_dart(self) -> crate::proto::ApiHeader {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::ApiIdentity {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.fingerprint.into_into_dart().into_dart(),
                self.name.into_into_dart().into_dart(),
                self.public_key.into_into_dart().into_dart(),
                self.is_owned.into_into_dart().into_dart(),
                self.extra.into_into_dart().into_dart(),
                self.sig.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::ApiIdentity {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::ApiIdentity>
    for crate::proto::ApiIdentity {
        fn into_into_dart(self) -> crate::proto::ApiIdentity {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::ApiMessage {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.from_fingerprint.into_into_dart().into_dart(),
                self.to_fingerprint.into_into_dart().into_dart(),
                self.application.into_into_dart().into_dart(),
                self.extension.into_into_dart().into_dart(),
                self.mime.into_into_dart().into_dart(),
                self.send_date.into_into_dart().into_dart(),
                self.receive_date.into_into_dart().into_dart(),
                self.is_file.into_into_dart().into_dart(),
                self.id.into_into_dart().into_dart(),
                self.body.into_into_dart().into_dart(),
                self.file_name.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::ApiMessage {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::ApiMessage>
    for crate::proto::ApiMessage {
        fn into_into_dart(self) -> crate::proto::ApiMessage {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::api::types::B64SessionState {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.secretkey.into_into_dart().into_dart(),
                self.pubkey.into_into_dart().into_dart(),
                self.remotekey.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::api::types::B64SessionState {}
    impl flutter_rust_bridge::IntoIntoDart<crate::api::types::B64SessionState>
    for crate::api::types::B64SessionState {
        fn into_into_dart(self) -> crate::api::types::B64SessionState {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::CryptoMessage {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.nonce.into_into_dart().into_dart(),
                self.encrypted.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::CryptoMessage {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::CryptoMessage>
    for crate::proto::CryptoMessage {
        fn into_into_dart(self) -> crate::proto::CryptoMessage {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart
    for crate::proto::import_identity_response::FinalResponse {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.handle.into_into_dart().into_dart(),
                self.identity.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::import_identity_response::FinalResponse {}
    impl flutter_rust_bridge::IntoIntoDart<
        crate::proto::import_identity_response::FinalResponse,
    > for crate::proto::import_identity_response::FinalResponse {
        fn into_into_dart(
            self,
        ) -> crate::proto::import_identity_response::FinalResponse {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::GetEvents {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.header.into_into_dart().into_dart(),
                self.block.into_into_dart().into_dart(),
                self.maybe_count.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::GetEvents {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::GetEvents>
    for crate::proto::GetEvents {
        fn into_into_dart(self) -> crate::proto::GetEvents {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::GetIdentityCommand {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.header.into_into_dart().into_dart(),
                self.owned.into_into_dart().into_dart(),
                self.id.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::GetIdentityCommand {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::GetIdentityCommand>
    for crate::proto::GetIdentityCommand {
        fn into_into_dart(self) -> crate::proto::GetIdentityCommand {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::GetMessagesCmd {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.header.into_into_dart().into_dart(),
                self.limit.into_into_dart().into_dart(),
                self.time_slice.into_into_dart().into_dart(),
                self.maybe_application.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::GetMessagesCmd {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::GetMessagesCmd>
    for crate::proto::GetMessagesCmd {
        fn into_into_dart(self) -> crate::proto::GetMessagesCmd {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::api::mdns::HostRecord {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.name.into_into_dart().into_dart(),
                self.addr.into_into_dart().into_dart(),
                self.port.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::api::mdns::HostRecord {}
    impl flutter_rust_bridge::IntoIntoDart<crate::api::mdns::HostRecord>
    for crate::api::mdns::HostRecord {
        fn into_into_dart(self) -> crate::api::mdns::HostRecord {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::get_identity_command::Id {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            match self {
                crate::proto::get_identity_command::Id::Identity(field0) => {
                    [0.into_dart(), field0.into_into_dart().into_dart()].into_dart()
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::get_identity_command::Id {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::get_identity_command::Id>
    for crate::proto::get_identity_command::Id {
        fn into_into_dart(self) -> crate::proto::get_identity_command::Id {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::IdentityResponse {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.header.into_into_dart().into_dart(),
                self.identity.into_into_dart().into_dart(),
                self.code.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::IdentityResponse {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::IdentityResponse>
    for crate::proto::IdentityResponse {
        fn into_into_dart(self) -> crate::proto::IdentityResponse {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::ImportIdentityCommand {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.header.into_into_dart().into_dart(),
                self.maybe_handle.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::ImportIdentityCommand {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::ImportIdentityCommand>
    for crate::proto::ImportIdentityCommand {
        fn into_into_dart(self) -> crate::proto::ImportIdentityCommand {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::ImportIdentityResponse {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.header.into_into_dart().into_dart(),
                self.code.into_into_dart().into_dart(),
                self.state.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::ImportIdentityResponse {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::ImportIdentityResponse>
    for crate::proto::ImportIdentityResponse {
        fn into_into_dart(self) -> crate::proto::ImportIdentityResponse {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for FrbWrapper<crate::api::mirror::IpAddr> {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            match self.0 {
                crate::api::mirror::IpAddr::V4(field0) => {
                    [0.into_dart(), field0.into_into_dart().into_dart()].into_dart()
                }
                crate::api::mirror::IpAddr::V6(field0) => {
                    [1.into_dart(), field0.into_into_dart().into_dart()].into_dart()
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<crate::api::mirror::IpAddr> {}
    impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<crate::api::mirror::IpAddr>>
    for crate::api::mirror::IpAddr {
        fn into_into_dart(self) -> FrbWrapper<crate::api::mirror::IpAddr> {
            self.into()
        }
    }
    impl flutter_rust_bridge::IntoDart
    for crate::proto::get_messages_cmd::MaybeApplication {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            match self {
                crate::proto::get_messages_cmd::MaybeApplication::Application(field0) => {
                    [0.into_dart(), field0.into_into_dart().into_dart()].into_dart()
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::get_messages_cmd::MaybeApplication {}
    impl flutter_rust_bridge::IntoIntoDart<
        crate::proto::get_messages_cmd::MaybeApplication,
    > for crate::proto::get_messages_cmd::MaybeApplication {
        fn into_into_dart(self) -> crate::proto::get_messages_cmd::MaybeApplication {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::get_events::MaybeCount {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            match self {
                crate::proto::get_events::MaybeCount::Count(field0) => {
                    [0.into_dart(), field0.into_into_dart().into_dart()].into_dart()
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::get_events::MaybeCount {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::get_events::MaybeCount>
    for crate::proto::get_events::MaybeCount {
        fn into_into_dart(self) -> crate::proto::get_events::MaybeCount {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::sb_event::MaybeEvent {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            match self {
                crate::proto::sb_event::MaybeEvent::NewMessage(field0) => {
                    [0.into_dart(), field0.into_into_dart().into_dart()].into_dart()
                }
                crate::proto::sb_event::MaybeEvent::NewIdentities(field0) => {
                    [1.into_dart(), field0.into_into_dart().into_dart()].into_dart()
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::sb_event::MaybeEvent {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::sb_event::MaybeEvent>
    for crate::proto::sb_event::MaybeEvent {
        fn into_into_dart(self) -> crate::proto::sb_event::MaybeEvent {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart
    for crate::proto::import_identity_command::MaybeHandle {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            match self {
                crate::proto::import_identity_command::MaybeHandle::Handle(field0) => {
                    [0.into_dart(), field0.into_into_dart().into_dart()].into_dart()
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::import_identity_command::MaybeHandle {}
    impl flutter_rust_bridge::IntoIntoDart<
        crate::proto::import_identity_command::MaybeHandle,
    > for crate::proto::import_identity_command::MaybeHandle {
        fn into_into_dart(self) -> crate::proto::import_identity_command::MaybeHandle {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::api::response::Message {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.from_fingerprint.into_into_dart().into_dart(),
                self.to_fingerprint.into_into_dart().into_dart(),
                self.application.into_into_dart().into_dart(),
                self.extension.into_into_dart().into_dart(),
                self.mime.into_into_dart().into_dart(),
                self.send_date.into_into_dart().into_dart(),
                self.receive_date.into_into_dart().into_dart(),
                self.is_file.into_into_dart().into_dart(),
                self.id.into_into_dart().into_dart(),
                self.body.into_into_dart().into_dart(),
                self.file_name.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::api::response::Message {}
    impl flutter_rust_bridge::IntoIntoDart<crate::api::response::Message>
    for crate::api::response::Message {
        fn into_into_dart(self) -> crate::api::response::Message {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::MessageResponse {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.header.into_into_dart().into_dart(),
                self.messsage.into_into_dart().into_dart(),
                self.code.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::MessageResponse {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::MessageResponse>
    for crate::proto::MessageResponse {
        fn into_into_dart(self) -> crate::proto::MessageResponse {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::MessageType {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            match self {
                Self::Advertise => 0.into_dart(),
                Self::DeclareHashes => 1.into_dart(),
                Self::BlockHeader => 2.into_dart(),
                Self::BlockSequence => 3.into_dart(),
                Self::ElectLeader => 4.into_dart(),
                Self::Upgrade => 5.into_dart(),
                Self::RoutingMetadata => 6.into_dart(),
                Self::IpAnnounce => 7.into_dart(),
                Self::Identity => 8.into_dart(),
                Self::Luid => 9.into_dart(),
                Self::JustUkes => 10.into_dart(),
                Self::Ack => 11.into_dart(),
                Self::Invalid => 12.into_dart(),
                Self::GetMessage => 13.into_dart(),
                Self::GetIdentity => 14.into_dart(),
                Self::SendMessage => 15.into_dart(),
                Self::Message => 16.into_dart(),
                Self::UnitResponse => 17.into_dart(),
                Self::CryptoMessage => 18.into_dart(),
                Self::PairingRequest => 19.into_dart(),
                Self::PairingInitiate => 20.into_dart(),
                Self::PairingCompleted => 21.into_dart(),
                Self::PairingAck => 22.into_dart(),
                Self::IdentityResponse => 23.into_dart(),
                Self::ApiIdentity => 24.into_dart(),
                Self::MessageResponse => 25.into_dart(),
                Self::ApiHeader => 26.into_dart(),
                Self::GenerateIdentity => 27.into_dart(),
                Self::ImportIdentity => 28.into_dart(),
                Self::ImportIdentityResponse => 29.into_dart(),
                Self::GenerateIdentityResponse => 30.into_dart(),
                Self::GetEvents => 31.into_dart(),
                Self::DesktopEvents => 32.into_dart(),
                Self::DesktopEvent => 33.into_dart(),
                Self::NoBodyMessage => 34.into_dart(),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::MessageType {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::MessageType>
    for crate::proto::MessageType {
        fn into_into_dart(self) -> crate::proto::MessageType {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::sb_event::NewIdentity {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [self.identities.into_into_dart().into_dart()].into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::sb_event::NewIdentity {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::sb_event::NewIdentity>
    for crate::proto::sb_event::NewIdentity {
        fn into_into_dart(self) -> crate::proto::sb_event::NewIdentity {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::sb_event::NewMessage {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [self.messages.into_into_dart().into_dart()].into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::sb_event::NewMessage {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::sb_event::NewMessage>
    for crate::proto::sb_event::NewMessage {
        fn into_into_dart(self) -> crate::proto::sb_event::NewMessage {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::sb_event::NoBodyMessage {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.from_fingerprint.into_into_dart().into_dart(),
                self.to_fingerprint.into_into_dart().into_dart(),
                self.application.into_into_dart().into_dart(),
                self.extension.into_into_dart().into_dart(),
                self.mime.into_into_dart().into_dart(),
                self.send_date.into_into_dart().into_dart(),
                self.receive_date.into_into_dart().into_dart(),
                self.is_file.into_into_dart().into_dart(),
                self.id.into_into_dart().into_dart(),
                self.file_name.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::sb_event::NoBodyMessage {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::sb_event::NoBodyMessage>
    for crate::proto::sb_event::NoBodyMessage {
        fn into_into_dart(self) -> crate::proto::sb_event::NoBodyMessage {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::PairingAck {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.session.into_into_dart().into_dart(),
                self.pubkey.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::PairingAck {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::PairingAck>
    for crate::proto::PairingAck {
        fn into_into_dart(self) -> crate::proto::PairingAck {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::PairingInitiate {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [self.pubkey.into_into_dart().into_dart()].into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::PairingInitiate {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::PairingInitiate>
    for crate::proto::PairingInitiate {
        fn into_into_dart(self) -> crate::proto::PairingInitiate {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::PairingRequest {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.session.into_into_dart().into_dart(),
                self.name.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::PairingRequest {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::PairingRequest>
    for crate::proto::PairingRequest {
        fn into_into_dart(self) -> crate::proto::PairingRequest {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::ProtoUuid {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.lower.into_into_dart().into_dart(),
                self.upper.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::ProtoUuid {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::ProtoUuid>
    for crate::proto::ProtoUuid {
        fn into_into_dart(self) -> crate::proto::ProtoUuid {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::SbEvent {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [self.maybe_event.into_into_dart().into_dart()].into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::SbEvent {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::SbEvent>
    for crate::proto::SbEvent {
        fn into_into_dart(self) -> crate::proto::SbEvent {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::SbEvents {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.header.into_into_dart().into_dart(),
                self.events.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::SbEvents {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::SbEvents>
    for crate::proto::SbEvents {
        fn into_into_dart(self) -> crate::proto::SbEvents {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::SendMessageCmd {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.header.into_into_dart().into_dart(),
                self.messages.into_into_dart().into_dart(),
                self.sign_identity.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::SendMessageCmd {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::SendMessageCmd>
    for crate::proto::SendMessageCmd {
        fn into_into_dart(self) -> crate::proto::SendMessageCmd {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::send_message_cmd::SignIdentity {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            match self {
                crate::proto::send_message_cmd::SignIdentity::Identity(field0) => {
                    [0.into_dart(), field0.into_into_dart().into_dart()].into_dart()
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::send_message_cmd::SignIdentity {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::send_message_cmd::SignIdentity>
    for crate::proto::send_message_cmd::SignIdentity {
        fn into_into_dart(self) -> crate::proto::send_message_cmd::SignIdentity {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart
    for crate::proto::import_identity_response::State {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            match self {
                crate::proto::import_identity_response::State::Final(field0) => {
                    [0.into_dart(), field0.into_into_dart().into_dart()].into_dart()
                }
                crate::proto::import_identity_response::State::Handle(field0) => {
                    [1.into_dart(), field0.into_into_dart().into_dart()].into_dart()
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::import_identity_response::State {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::import_identity_response::State>
    for crate::proto::import_identity_response::State {
        fn into_into_dart(self) -> crate::proto::import_identity_response::State {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::api_header::Stream {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            match self {
                crate::proto::api_header::Stream::StreamId(field0) => {
                    [0.into_dart(), field0.into_into_dart().into_dart()].into_dart()
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::api_header::Stream {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::api_header::Stream>
    for crate::proto::api_header::Stream {
        fn into_into_dart(self) -> crate::proto::api_header::Stream {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::get_messages_cmd::TimeRange {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.start.into_into_dart().into_dart(),
                self.end.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::get_messages_cmd::TimeRange {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::get_messages_cmd::TimeRange>
    for crate::proto::get_messages_cmd::TimeRange {
        fn into_into_dart(self) -> crate::proto::get_messages_cmd::TimeRange {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::get_messages_cmd::TimeSlice {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            match self {
                crate::proto::get_messages_cmd::TimeSlice::SendDate(field0) => {
                    [0.into_dart(), field0.into_into_dart().into_dart()].into_dart()
                }
                crate::proto::get_messages_cmd::TimeSlice::ReceiveDate(field0) => {
                    [1.into_dart(), field0.into_into_dart().into_dart()].into_dart()
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::get_messages_cmd::TimeSlice {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::get_messages_cmd::TimeSlice>
    for crate::proto::get_messages_cmd::TimeSlice {
        fn into_into_dart(self) -> crate::proto::get_messages_cmd::TimeSlice {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::TypePrefix {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [self.r#type.into_into_dart().into_dart()].into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::TypePrefix {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::TypePrefix>
    for crate::proto::TypePrefix {
        fn into_into_dart(self) -> crate::proto::TypePrefix {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart for crate::proto::UnitResponse {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            [
                self.header.into_into_dart().into_dart(),
                self.code.into_into_dart().into_dart(),
                self.unitresponse_maybe_message.into_into_dart().into_dart(),
            ]
                .into_dart()
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::UnitResponse {}
    impl flutter_rust_bridge::IntoIntoDart<crate::proto::UnitResponse>
    for crate::proto::UnitResponse {
        fn into_into_dart(self) -> crate::proto::UnitResponse {
            self
        }
    }
    impl flutter_rust_bridge::IntoDart
    for crate::proto::unit_response::UnitresponseMaybeMessage {
        fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
            match self {
                crate::proto::unit_response::UnitresponseMaybeMessage::MessageCode(
                    field0,
                ) => [0.into_dart(), field0.into_into_dart().into_dart()].into_dart(),
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::proto::unit_response::UnitresponseMaybeMessage {}
    impl flutter_rust_bridge::IntoIntoDart<
        crate::proto::unit_response::UnitresponseMaybeMessage,
    > for crate::proto::unit_response::UnitresponseMaybeMessage {
        fn into_into_dart(
            self,
        ) -> crate::proto::unit_response::UnitresponseMaybeMessage {
            self
        }
    }
    impl SseEncode for flutter_rust_bridge::for_generated::anyhow::Error {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <String>::sse_encode(
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(format_args!("{0:?}", self));
                    res
                }),
                serializer,
            );
        }
    }
    impl SseEncode for Ipv4Addr {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv4Addr>,
            >>::sse_encode(
                flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                    _,
                    MoiArc<_>,
                >(self),
                serializer,
            );
        }
    }
    impl SseEncode for Ipv6Addr {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv6Addr>,
            >>::sse_encode(
                flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                    _,
                    MoiArc<_>,
                >(self),
                serializer,
            );
        }
    }
    impl SseEncode
    for ::Pin<
        Box<dyn Future<Output = SbResult<ImportIdentityState>> + Send + 'async_trait>,
    > {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<
                        Box<
                            dyn Future<
                                Output = SbResult<ImportIdentityState>,
                            > + Send + 'async_trait,
                        >,
                    >,
                >,
            >>::sse_encode(
                flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                    _,
                    MoiArc<_>,
                >(self),
                serializer,
            );
        }
    }
    impl SseEncode
    for ::Pin<Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>>,
                >,
            >>::sse_encode(
                flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                    _,
                    MoiArc<_>,
                >(self),
                serializer,
            );
        }
    }
    impl SseEncode
    for ::Pin<Box<dyn Future<Output = SbResult<Vec<Identity>>> + Send + 'async_trait>> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<
                        Box<
                            dyn Future<
                                Output = SbResult<Vec<Identity>>,
                            > + Send + 'async_trait,
                        >,
                    >,
                >,
            >>::sse_encode(
                flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                    _,
                    MoiArc<_>,
                >(self),
                serializer,
            );
        }
    }
    impl SseEncode
    for ::Pin<Box<dyn Future<Output = SbResult<Vec<Message>>> + Send + 'async_trait>> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<
                        Box<
                            dyn Future<
                                Output = SbResult<Vec<Message>>,
                            > + Send + 'async_trait,
                        >,
                    >,
                >,
            >>::sse_encode(
                flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                    _,
                    MoiArc<_>,
                >(self),
                serializer,
            );
        }
    }
    impl SseEncode
    for ::Pin<Box<dyn Future<Output = SbResult<Vec<SbEvent>>> + Send + 'async_trait>> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<
                        Box<
                            dyn Future<
                                Output = SbResult<Vec<SbEvent>>,
                            > + Send + 'async_trait,
                        >,
                    >,
                >,
            >>::sse_encode(
                flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                    _,
                    MoiArc<_>,
                >(self),
                serializer,
            );
        }
    }
    impl SseEncode for SbResult<()> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbResult<()>>,
            >>::sse_encode(
                flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                    _,
                    MoiArc<_>,
                >(self),
                serializer,
            );
        }
    }
    impl SseEncode for SbResult<String> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbResult<String>>,
            >>::sse_encode(
                flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                    _,
                    MoiArc<_>,
                >(self),
                serializer,
            );
        }
    }
    impl SseEncode for SbSession {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbSession>,
            >>::sse_encode(
                flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                    _,
                    MoiArc<_>,
                >(self),
                serializer,
            );
        }
    }
    impl SseEncode for ServiceScanner {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ServiceScanner>,
            >>::sse_encode(
                flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<
                    _,
                    MoiArc<_>,
                >(self),
                serializer,
            );
        }
    }
    impl SseEncode for chrono::NaiveDateTime {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <i64>::sse_encode(self.and_utc().timestamp_micros(), serializer);
        }
    }
    impl SseEncode for flutter_rust_bridge::DartOpaque {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <usize>::sse_encode(self.encode(), serializer);
        }
    }
    impl SseEncode for std::collections::HashMap<String, Vec<u8>> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Vec<(String, Vec<u8>)>>::sse_encode(self.into_iter().collect(), serializer);
        }
    }
    impl SseEncode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv4Addr>,
    > {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            let (ptr, size) = self.sse_encode_raw();
            <usize>::sse_encode(ptr, serializer);
            <i32>::sse_encode(size, serializer);
        }
    }
    impl SseEncode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv6Addr>,
    > {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            let (ptr, size) = self.sse_encode_raw();
            <usize>::sse_encode(ptr, serializer);
            <i32>::sse_encode(size, serializer);
        }
    }
    impl SseEncode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
            ::Pin<
                Box<
                    dyn Future<
                        Output = SbResult<ImportIdentityState>,
                    > + Send + 'async_trait,
                >,
            >,
        >,
    > {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            let (ptr, size) = self.sse_encode_raw();
            <usize>::sse_encode(ptr, serializer);
            <i32>::sse_encode(size, serializer);
        }
    }
    impl SseEncode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
            ::Pin<Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>>,
        >,
    > {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            let (ptr, size) = self.sse_encode_raw();
            <usize>::sse_encode(ptr, serializer);
            <i32>::sse_encode(size, serializer);
        }
    }
    impl SseEncode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
            ::Pin<
                Box<dyn Future<Output = SbResult<Vec<Identity>>> + Send + 'async_trait>,
            >,
        >,
    > {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            let (ptr, size) = self.sse_encode_raw();
            <usize>::sse_encode(ptr, serializer);
            <i32>::sse_encode(size, serializer);
        }
    }
    impl SseEncode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
            ::Pin<Box<dyn Future<Output = SbResult<Vec<Message>>> + Send + 'async_trait>>,
        >,
    > {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            let (ptr, size) = self.sse_encode_raw();
            <usize>::sse_encode(ptr, serializer);
            <i32>::sse_encode(size, serializer);
        }
    }
    impl SseEncode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
            ::Pin<Box<dyn Future<Output = SbResult<Vec<SbEvent>>> + Send + 'async_trait>>,
        >,
    > {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            let (ptr, size) = self.sse_encode_raw();
            <usize>::sse_encode(ptr, serializer);
            <i32>::sse_encode(size, serializer);
        }
    }
    impl SseEncode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbResult<()>>,
    > {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            let (ptr, size) = self.sse_encode_raw();
            <usize>::sse_encode(ptr, serializer);
            <i32>::sse_encode(size, serializer);
        }
    }
    impl SseEncode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbResult<String>>,
    > {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            let (ptr, size) = self.sse_encode_raw();
            <usize>::sse_encode(ptr, serializer);
            <i32>::sse_encode(size, serializer);
        }
    }
    impl SseEncode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbSession>,
    > {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            let (ptr, size) = self.sse_encode_raw();
            <usize>::sse_encode(ptr, serializer);
            <i32>::sse_encode(size, serializer);
        }
    }
    impl SseEncode
    for RustOpaqueMoi<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ServiceScanner>,
    > {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            let (ptr, size) = self.sse_encode_raw();
            <usize>::sse_encode(ptr, serializer);
            <i32>::sse_encode(size, serializer);
        }
    }
    impl SseEncode for String {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Vec<u8>>::sse_encode(self.into_bytes(), serializer);
        }
    }
    impl SseEncode for uuid::Uuid {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Vec<u8>>::sse_encode(self.as_bytes().to_vec(), serializer);
        }
    }
    impl SseEncode for crate::proto::Ack {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.success, serializer);
            <i32>::sse_encode(self.status, serializer);
            <Option<
                crate::proto::ack::AckMaybeMessage,
            >>::sse_encode(self.ack_maybe_message, serializer);
        }
    }
    impl SseEncode for crate::proto::ack::AckMaybeMessage {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            match self {
                crate::proto::ack::AckMaybeMessage::Text(field0) => {
                    <i32>::sse_encode(0, serializer);
                    <String>::sse_encode(field0, serializer);
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseEncode for crate::proto::ApiHeader {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<crate::proto::ProtoUuid>>::sse_encode(self.session, serializer);
            <Option<
                crate::proto::api_header::Stream,
            >>::sse_encode(self.stream, serializer);
        }
    }
    impl SseEncode for crate::proto::ApiIdentity {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<crate::proto::ProtoUuid>>::sse_encode(self.fingerprint, serializer);
            <String>::sse_encode(self.name, serializer);
            <Vec<u8>>::sse_encode(self.public_key, serializer);
            <bool>::sse_encode(self.is_owned, serializer);
            <std::collections::HashMap<
                String,
                Vec<u8>,
            >>::sse_encode(self.extra, serializer);
            <Vec<u8>>::sse_encode(self.sig, serializer);
        }
    }
    impl SseEncode for crate::proto::ApiMessage {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<
                crate::proto::ProtoUuid,
            >>::sse_encode(self.from_fingerprint, serializer);
            <Option<
                crate::proto::ProtoUuid,
            >>::sse_encode(self.to_fingerprint, serializer);
            <String>::sse_encode(self.application, serializer);
            <String>::sse_encode(self.extension, serializer);
            <String>::sse_encode(self.mime, serializer);
            <i64>::sse_encode(self.send_date, serializer);
            <i64>::sse_encode(self.receive_date, serializer);
            <bool>::sse_encode(self.is_file, serializer);
            <Option<crate::proto::ProtoUuid>>::sse_encode(self.id, serializer);
            <Vec<u8>>::sse_encode(self.body, serializer);
            <String>::sse_encode(self.file_name, serializer);
        }
    }
    impl SseEncode for crate::api::types::B64SessionState {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <String>::sse_encode(self.secretkey, serializer);
            <String>::sse_encode(self.pubkey, serializer);
            <Option<String>>::sse_encode(self.remotekey, serializer);
        }
    }
    impl SseEncode for bool {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            serializer.cursor.write_u8(self as _).unwrap();
        }
    }
    impl SseEncode for crate::proto::CryptoMessage {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Vec<u8>>::sse_encode(self.nonce, serializer);
            <Vec<u8>>::sse_encode(self.encrypted, serializer);
        }
    }
    impl SseEncode for crate::proto::import_identity_response::FinalResponse {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<crate::proto::ProtoUuid>>::sse_encode(self.handle, serializer);
            <Option<crate::proto::ProtoUuid>>::sse_encode(self.identity, serializer);
        }
    }
    impl SseEncode for crate::proto::GetEvents {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<crate::proto::ApiHeader>>::sse_encode(self.header, serializer);
            <bool>::sse_encode(self.block, serializer);
            <Option<
                crate::proto::get_events::MaybeCount,
            >>::sse_encode(self.maybe_count, serializer);
        }
    }
    impl SseEncode for crate::proto::GetIdentityCommand {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<crate::proto::ApiHeader>>::sse_encode(self.header, serializer);
            <bool>::sse_encode(self.owned, serializer);
            <Option<
                crate::proto::get_identity_command::Id,
            >>::sse_encode(self.id, serializer);
        }
    }
    impl SseEncode for crate::proto::GetMessagesCmd {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<crate::proto::ApiHeader>>::sse_encode(self.header, serializer);
            <i32>::sse_encode(self.limit, serializer);
            <Option<
                crate::proto::get_messages_cmd::TimeSlice,
            >>::sse_encode(self.time_slice, serializer);
            <Option<
                crate::proto::get_messages_cmd::MaybeApplication,
            >>::sse_encode(self.maybe_application, serializer);
        }
    }
    impl SseEncode for crate::api::mdns::HostRecord {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <String>::sse_encode(self.name, serializer);
            <Vec<crate::api::mirror::IpAddr>>::sse_encode(self.addr, serializer);
            <u16>::sse_encode(self.port, serializer);
        }
    }
    impl SseEncode for i32 {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            serializer.cursor.write_i32::<NativeEndian>(self).unwrap();
        }
    }
    impl SseEncode for i64 {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            serializer.cursor.write_i64::<NativeEndian>(self).unwrap();
        }
    }
    impl SseEncode for crate::proto::get_identity_command::Id {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            match self {
                crate::proto::get_identity_command::Id::Identity(field0) => {
                    <i32>::sse_encode(0, serializer);
                    <crate::proto::ProtoUuid>::sse_encode(field0, serializer);
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseEncode for crate::proto::IdentityResponse {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<crate::proto::ApiHeader>>::sse_encode(self.header, serializer);
            <Vec<crate::proto::ApiIdentity>>::sse_encode(self.identity, serializer);
            <i32>::sse_encode(self.code, serializer);
        }
    }
    impl SseEncode for crate::proto::ImportIdentityCommand {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<crate::proto::ApiHeader>>::sse_encode(self.header, serializer);
            <Option<
                crate::proto::import_identity_command::MaybeHandle,
            >>::sse_encode(self.maybe_handle, serializer);
        }
    }
    impl SseEncode for crate::proto::ImportIdentityResponse {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<crate::proto::ApiHeader>>::sse_encode(self.header, serializer);
            <i32>::sse_encode(self.code, serializer);
            <Option<
                crate::proto::import_identity_response::State,
            >>::sse_encode(self.state, serializer);
        }
    }
    impl SseEncode for crate::api::mirror::IpAddr {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            match self {
                crate::api::mirror::IpAddr::V4(field0) => {
                    <i32>::sse_encode(0, serializer);
                    <Ipv4Addr>::sse_encode(field0, serializer);
                }
                crate::api::mirror::IpAddr::V6(field0) => {
                    <i32>::sse_encode(1, serializer);
                    <Ipv6Addr>::sse_encode(field0, serializer);
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseEncode for isize {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            serializer.cursor.write_i64::<NativeEndian>(self as _).unwrap();
        }
    }
    impl SseEncode for Vec<crate::proto::ApiIdentity> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <i32>::sse_encode(self.len() as _, serializer);
            for item in self {
                <crate::proto::ApiIdentity>::sse_encode(item, serializer);
            }
        }
    }
    impl SseEncode for Vec<crate::proto::ApiMessage> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <i32>::sse_encode(self.len() as _, serializer);
            for item in self {
                <crate::proto::ApiMessage>::sse_encode(item, serializer);
            }
        }
    }
    impl SseEncode for Vec<crate::api::mdns::HostRecord> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <i32>::sse_encode(self.len() as _, serializer);
            for item in self {
                <crate::api::mdns::HostRecord>::sse_encode(item, serializer);
            }
        }
    }
    impl SseEncode for Vec<crate::api::mirror::IpAddr> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <i32>::sse_encode(self.len() as _, serializer);
            for item in self {
                <crate::api::mirror::IpAddr>::sse_encode(item, serializer);
            }
        }
    }
    impl SseEncode for Vec<crate::api::response::Message> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <i32>::sse_encode(self.len() as _, serializer);
            for item in self {
                <crate::api::response::Message>::sse_encode(item, serializer);
            }
        }
    }
    impl SseEncode for Vec<crate::proto::sb_event::NoBodyMessage> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <i32>::sse_encode(self.len() as _, serializer);
            for item in self {
                <crate::proto::sb_event::NoBodyMessage>::sse_encode(item, serializer);
            }
        }
    }
    impl SseEncode for Vec<u8> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <i32>::sse_encode(self.len() as _, serializer);
            for item in self {
                <u8>::sse_encode(item, serializer);
            }
        }
    }
    impl SseEncode for Vec<(String, Vec<u8>)> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <i32>::sse_encode(self.len() as _, serializer);
            for item in self {
                <(String, Vec<u8>)>::sse_encode(item, serializer);
            }
        }
    }
    impl SseEncode for Vec<crate::proto::SbEvent> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <i32>::sse_encode(self.len() as _, serializer);
            for item in self {
                <crate::proto::SbEvent>::sse_encode(item, serializer);
            }
        }
    }
    impl SseEncode for crate::proto::get_messages_cmd::MaybeApplication {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            match self {
                crate::proto::get_messages_cmd::MaybeApplication::Application(
                    field0,
                ) => {
                    <i32>::sse_encode(0, serializer);
                    <String>::sse_encode(field0, serializer);
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseEncode for crate::proto::get_events::MaybeCount {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            match self {
                crate::proto::get_events::MaybeCount::Count(field0) => {
                    <i32>::sse_encode(0, serializer);
                    <u32>::sse_encode(field0, serializer);
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseEncode for crate::proto::sb_event::MaybeEvent {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            match self {
                crate::proto::sb_event::MaybeEvent::NewMessage(field0) => {
                    <i32>::sse_encode(0, serializer);
                    <crate::proto::sb_event::NewMessage>::sse_encode(field0, serializer);
                }
                crate::proto::sb_event::MaybeEvent::NewIdentities(field0) => {
                    <i32>::sse_encode(1, serializer);
                    <crate::proto::sb_event::NewIdentity>::sse_encode(
                        field0,
                        serializer,
                    );
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseEncode for crate::proto::import_identity_command::MaybeHandle {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            match self {
                crate::proto::import_identity_command::MaybeHandle::Handle(field0) => {
                    <i32>::sse_encode(0, serializer);
                    <crate::proto::ProtoUuid>::sse_encode(field0, serializer);
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseEncode for crate::api::response::Message {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<uuid::Uuid>>::sse_encode(self.from_fingerprint, serializer);
            <Option<uuid::Uuid>>::sse_encode(self.to_fingerprint, serializer);
            <String>::sse_encode(self.application, serializer);
            <String>::sse_encode(self.extension, serializer);
            <String>::sse_encode(self.mime, serializer);
            <i64>::sse_encode(self.send_date, serializer);
            <i64>::sse_encode(self.receive_date, serializer);
            <bool>::sse_encode(self.is_file, serializer);
            <Option<uuid::Uuid>>::sse_encode(self.id, serializer);
            <Vec<u8>>::sse_encode(self.body, serializer);
            <String>::sse_encode(self.file_name, serializer);
        }
    }
    impl SseEncode for crate::proto::MessageResponse {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<crate::proto::ApiHeader>>::sse_encode(self.header, serializer);
            <Vec<crate::proto::ApiMessage>>::sse_encode(self.messsage, serializer);
            <i32>::sse_encode(self.code, serializer);
        }
    }
    impl SseEncode for crate::proto::MessageType {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <i32>::sse_encode(
                match self {
                    crate::proto::MessageType::Advertise => 0,
                    crate::proto::MessageType::DeclareHashes => 1,
                    crate::proto::MessageType::BlockHeader => 2,
                    crate::proto::MessageType::BlockSequence => 3,
                    crate::proto::MessageType::ElectLeader => 4,
                    crate::proto::MessageType::Upgrade => 5,
                    crate::proto::MessageType::RoutingMetadata => 6,
                    crate::proto::MessageType::IpAnnounce => 7,
                    crate::proto::MessageType::Identity => 8,
                    crate::proto::MessageType::Luid => 9,
                    crate::proto::MessageType::JustUkes => 10,
                    crate::proto::MessageType::Ack => 11,
                    crate::proto::MessageType::Invalid => 12,
                    crate::proto::MessageType::GetMessage => 13,
                    crate::proto::MessageType::GetIdentity => 14,
                    crate::proto::MessageType::SendMessage => 15,
                    crate::proto::MessageType::Message => 16,
                    crate::proto::MessageType::UnitResponse => 17,
                    crate::proto::MessageType::CryptoMessage => 18,
                    crate::proto::MessageType::PairingRequest => 19,
                    crate::proto::MessageType::PairingInitiate => 20,
                    crate::proto::MessageType::PairingCompleted => 21,
                    crate::proto::MessageType::PairingAck => 22,
                    crate::proto::MessageType::IdentityResponse => 23,
                    crate::proto::MessageType::ApiIdentity => 24,
                    crate::proto::MessageType::MessageResponse => 25,
                    crate::proto::MessageType::ApiHeader => 26,
                    crate::proto::MessageType::GenerateIdentity => 27,
                    crate::proto::MessageType::ImportIdentity => 28,
                    crate::proto::MessageType::ImportIdentityResponse => 29,
                    crate::proto::MessageType::GenerateIdentityResponse => 30,
                    crate::proto::MessageType::GetEvents => 31,
                    crate::proto::MessageType::DesktopEvents => 32,
                    crate::proto::MessageType::DesktopEvent => 33,
                    crate::proto::MessageType::NoBodyMessage => 34,
                    _ => {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!("not implemented: {0}", format_args!("")),
                            );
                        };
                    }
                },
                serializer,
            );
        }
    }
    impl SseEncode for crate::proto::sb_event::NewIdentity {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Vec<crate::proto::ApiIdentity>>::sse_encode(self.identities, serializer);
        }
    }
    impl SseEncode for crate::proto::sb_event::NewMessage {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Vec<
                crate::proto::sb_event::NoBodyMessage,
            >>::sse_encode(self.messages, serializer);
        }
    }
    impl SseEncode for crate::proto::sb_event::NoBodyMessage {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<
                crate::proto::ProtoUuid,
            >>::sse_encode(self.from_fingerprint, serializer);
            <Option<
                crate::proto::ProtoUuid,
            >>::sse_encode(self.to_fingerprint, serializer);
            <String>::sse_encode(self.application, serializer);
            <String>::sse_encode(self.extension, serializer);
            <String>::sse_encode(self.mime, serializer);
            <i64>::sse_encode(self.send_date, serializer);
            <i64>::sse_encode(self.receive_date, serializer);
            <bool>::sse_encode(self.is_file, serializer);
            <Option<crate::proto::ProtoUuid>>::sse_encode(self.id, serializer);
            <String>::sse_encode(self.file_name, serializer);
        }
    }
    impl SseEncode for Option<String> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.is_some(), serializer);
            if let Some(value) = self {
                <String>::sse_encode(value, serializer);
            }
        }
    }
    impl SseEncode for Option<uuid::Uuid> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.is_some(), serializer);
            if let Some(value) = self {
                <uuid::Uuid>::sse_encode(value, serializer);
            }
        }
    }
    impl SseEncode for Option<SbSession> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.is_some(), serializer);
            if let Some(value) = self {
                <SbSession>::sse_encode(value, serializer);
            }
        }
    }
    impl SseEncode for Option<crate::proto::ack::AckMaybeMessage> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.is_some(), serializer);
            if let Some(value) = self {
                <crate::proto::ack::AckMaybeMessage>::sse_encode(value, serializer);
            }
        }
    }
    impl SseEncode for Option<crate::proto::ApiHeader> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.is_some(), serializer);
            if let Some(value) = self {
                <crate::proto::ApiHeader>::sse_encode(value, serializer);
            }
        }
    }
    impl SseEncode for Option<i32> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.is_some(), serializer);
            if let Some(value) = self {
                <i32>::sse_encode(value, serializer);
            }
        }
    }
    impl SseEncode for Option<crate::proto::get_identity_command::Id> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.is_some(), serializer);
            if let Some(value) = self {
                <crate::proto::get_identity_command::Id>::sse_encode(value, serializer);
            }
        }
    }
    impl SseEncode for Option<crate::proto::get_messages_cmd::MaybeApplication> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.is_some(), serializer);
            if let Some(value) = self {
                <crate::proto::get_messages_cmd::MaybeApplication>::sse_encode(
                    value,
                    serializer,
                );
            }
        }
    }
    impl SseEncode for Option<crate::proto::get_events::MaybeCount> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.is_some(), serializer);
            if let Some(value) = self {
                <crate::proto::get_events::MaybeCount>::sse_encode(value, serializer);
            }
        }
    }
    impl SseEncode for Option<crate::proto::sb_event::MaybeEvent> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.is_some(), serializer);
            if let Some(value) = self {
                <crate::proto::sb_event::MaybeEvent>::sse_encode(value, serializer);
            }
        }
    }
    impl SseEncode for Option<crate::proto::import_identity_command::MaybeHandle> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.is_some(), serializer);
            if let Some(value) = self {
                <crate::proto::import_identity_command::MaybeHandle>::sse_encode(
                    value,
                    serializer,
                );
            }
        }
    }
    impl SseEncode for Option<crate::proto::ProtoUuid> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.is_some(), serializer);
            if let Some(value) = self {
                <crate::proto::ProtoUuid>::sse_encode(value, serializer);
            }
        }
    }
    impl SseEncode for Option<crate::proto::send_message_cmd::SignIdentity> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.is_some(), serializer);
            if let Some(value) = self {
                <crate::proto::send_message_cmd::SignIdentity>::sse_encode(
                    value,
                    serializer,
                );
            }
        }
    }
    impl SseEncode for Option<crate::proto::import_identity_response::State> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.is_some(), serializer);
            if let Some(value) = self {
                <crate::proto::import_identity_response::State>::sse_encode(
                    value,
                    serializer,
                );
            }
        }
    }
    impl SseEncode for Option<crate::proto::api_header::Stream> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.is_some(), serializer);
            if let Some(value) = self {
                <crate::proto::api_header::Stream>::sse_encode(value, serializer);
            }
        }
    }
    impl SseEncode for Option<crate::proto::get_messages_cmd::TimeSlice> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.is_some(), serializer);
            if let Some(value) = self {
                <crate::proto::get_messages_cmd::TimeSlice>::sse_encode(
                    value,
                    serializer,
                );
            }
        }
    }
    impl SseEncode for Option<u32> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.is_some(), serializer);
            if let Some(value) = self {
                <u32>::sse_encode(value, serializer);
            }
        }
    }
    impl SseEncode for Option<crate::proto::unit_response::UnitresponseMaybeMessage> {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <bool>::sse_encode(self.is_some(), serializer);
            if let Some(value) = self {
                <crate::proto::unit_response::UnitresponseMaybeMessage>::sse_encode(
                    value,
                    serializer,
                );
            }
        }
    }
    impl SseEncode for crate::proto::PairingAck {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<crate::proto::ApiHeader>>::sse_encode(self.session, serializer);
            <Vec<u8>>::sse_encode(self.pubkey, serializer);
        }
    }
    impl SseEncode for crate::proto::PairingInitiate {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Vec<u8>>::sse_encode(self.pubkey, serializer);
        }
    }
    impl SseEncode for crate::proto::PairingRequest {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<crate::proto::ApiHeader>>::sse_encode(self.session, serializer);
            <String>::sse_encode(self.name, serializer);
        }
    }
    impl SseEncode for crate::proto::ProtoUuid {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <u64>::sse_encode(self.lower, serializer);
            <u64>::sse_encode(self.upper, serializer);
        }
    }
    impl SseEncode for (String, Vec<u8>) {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <String>::sse_encode(self.0, serializer);
            <Vec<u8>>::sse_encode(self.1, serializer);
        }
    }
    impl SseEncode for crate::proto::SbEvent {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<
                crate::proto::sb_event::MaybeEvent,
            >>::sse_encode(self.maybe_event, serializer);
        }
    }
    impl SseEncode for crate::proto::SbEvents {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<crate::proto::ApiHeader>>::sse_encode(self.header, serializer);
            <Vec<crate::proto::SbEvent>>::sse_encode(self.events, serializer);
        }
    }
    impl SseEncode for crate::proto::SendMessageCmd {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<crate::proto::ApiHeader>>::sse_encode(self.header, serializer);
            <Vec<crate::proto::ApiMessage>>::sse_encode(self.messages, serializer);
            <Option<
                crate::proto::send_message_cmd::SignIdentity,
            >>::sse_encode(self.sign_identity, serializer);
        }
    }
    impl SseEncode for crate::proto::send_message_cmd::SignIdentity {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            match self {
                crate::proto::send_message_cmd::SignIdentity::Identity(field0) => {
                    <i32>::sse_encode(0, serializer);
                    <crate::proto::ProtoUuid>::sse_encode(field0, serializer);
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseEncode for crate::proto::import_identity_response::State {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            match self {
                crate::proto::import_identity_response::State::Final(field0) => {
                    <i32>::sse_encode(0, serializer);
                    <crate::proto::import_identity_response::FinalResponse>::sse_encode(
                        field0,
                        serializer,
                    );
                }
                crate::proto::import_identity_response::State::Handle(field0) => {
                    <i32>::sse_encode(1, serializer);
                    <crate::proto::ProtoUuid>::sse_encode(field0, serializer);
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseEncode for crate::proto::api_header::Stream {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            match self {
                crate::proto::api_header::Stream::StreamId(field0) => {
                    <i32>::sse_encode(0, serializer);
                    <i32>::sse_encode(field0, serializer);
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseEncode for crate::proto::get_messages_cmd::TimeRange {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <i64>::sse_encode(self.start, serializer);
            <i64>::sse_encode(self.end, serializer);
        }
    }
    impl SseEncode for crate::proto::get_messages_cmd::TimeSlice {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            match self {
                crate::proto::get_messages_cmd::TimeSlice::SendDate(field0) => {
                    <i32>::sse_encode(0, serializer);
                    <crate::proto::get_messages_cmd::TimeRange>::sse_encode(
                        field0,
                        serializer,
                    );
                }
                crate::proto::get_messages_cmd::TimeSlice::ReceiveDate(field0) => {
                    <i32>::sse_encode(1, serializer);
                    <crate::proto::get_messages_cmd::TimeRange>::sse_encode(
                        field0,
                        serializer,
                    );
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseEncode for crate::proto::TypePrefix {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <i32>::sse_encode(self.r#type, serializer);
        }
    }
    impl SseEncode for u16 {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            serializer.cursor.write_u16::<NativeEndian>(self).unwrap();
        }
    }
    impl SseEncode for u32 {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            serializer.cursor.write_u32::<NativeEndian>(self).unwrap();
        }
    }
    impl SseEncode for u64 {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            serializer.cursor.write_u64::<NativeEndian>(self).unwrap();
        }
    }
    impl SseEncode for u8 {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            serializer.cursor.write_u8(self).unwrap();
        }
    }
    impl SseEncode for () {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {}
    }
    impl SseEncode for crate::proto::UnitResponse {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            <Option<crate::proto::ApiHeader>>::sse_encode(self.header, serializer);
            <i32>::sse_encode(self.code, serializer);
            <Option<
                crate::proto::unit_response::UnitresponseMaybeMessage,
            >>::sse_encode(self.unitresponse_maybe_message, serializer);
        }
    }
    impl SseEncode for crate::proto::unit_response::UnitresponseMaybeMessage {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            match self {
                crate::proto::unit_response::UnitresponseMaybeMessage::MessageCode(
                    field0,
                ) => {
                    <i32>::sse_encode(0, serializer);
                    <String>::sse_encode(field0, serializer);
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not implemented: {0}", format_args!("")),
                        );
                    };
                }
            }
        }
    }
    impl SseEncode for usize {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            serializer.cursor.write_u64::<NativeEndian>(self as _).unwrap();
        }
    }
    #[cfg(not(target_family = "wasm"))]
    mod io {
        use super::*;
        use crate::api::api::*;
        use crate::api::error::IntoRemoteErr;
        use crate::api::error::*;
        use crate::api::mdns::*;
        use crate::api::response::*;
        use crate::api::serialize::ToUuid;
        use crate::api::types::GetType;
        use crate::api::types::*;
        use crate::*;
        use flutter_rust_bridge::for_generated::byteorder::{
            NativeEndian, ReadBytesExt, WriteBytesExt,
        };
        use flutter_rust_bridge::for_generated::{
            transform_result_dco, Lifetimeable, Lockable,
        };
        use flutter_rust_bridge::{Handler, IntoIntoDart};
        pub trait NewWithNullPtr {
            fn new_with_null_ptr() -> Self;
        }
        impl<T> NewWithNullPtr for *mut T {
            fn new_with_null_ptr() -> Self {
                std::ptr::null_mut()
            }
        }
        #[no_mangle]
        pub extern "C" fn frb_get_rust_content_hash() -> i32 {
            FLUTTER_RUST_BRIDGE_CODEGEN_CONTENT_HASH
        }
        #[no_mangle]
        pub extern "C" fn frb_pde_ffi_dispatcher_primary(
            func_id: i32,
            port_: i64,
            ptr_: *mut u8,
            rust_vec_len_: i32,
            data_len_: i32,
        ) {
            pde_ffi_dispatcher_primary_impl(
                func_id,
                port_,
                ptr_,
                rust_vec_len_,
                data_len_,
            )
        }
        #[no_mangle]
        pub extern "C" fn frb_pde_ffi_dispatcher_sync(
            func_id: i32,
            ptr_: *mut u8,
            rust_vec_len_: i32,
            data_len_: i32,
        ) -> ::flutter_rust_bridge::for_generated::WireSyncRust2DartSse {
            pde_ffi_dispatcher_sync_impl(func_id, ptr_, rust_vec_len_, data_len_)
        }
        #[no_mangle]
        pub extern "C" fn dart_fn_deliver_output(
            call_id: i32,
            ptr_: *mut u8,
            rust_vec_len_: i32,
            data_len_: i32,
        ) {
            let message = unsafe {
                ::flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            FLUTTER_RUST_BRIDGE_HANDLER.dart_fn_handle_output(call_id, message)
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerIpv4Addr(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv4Addr>,
            >::increment_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerIpv4Addr(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv4Addr>,
            >::decrement_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerIpv6Addr(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv6Addr>,
            >::increment_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerIpv6Addr(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Ipv6Addr>,
            >::decrement_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPinBoxFutureOutputSbResultImportIdentityStateSendasync_trait(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<
                        Box<
                            dyn Future<
                                Output = SbResult<ImportIdentityState>,
                            > + Send + 'async_trait,
                        >,
                    >,
                >,
            >::increment_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPinBoxFutureOutputSbResultImportIdentityStateSendasync_trait(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<
                        Box<
                            dyn Future<
                                Output = SbResult<ImportIdentityState>,
                            > + Send + 'async_trait,
                        >,
                    >,
                >,
            >::decrement_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPinBoxFutureOutputSbResultSendasync_trait(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>>,
                >,
            >::increment_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPinBoxFutureOutputSbResultSendasync_trait(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<Box<dyn Future<Output = SbResult<()>> + Send + 'async_trait>>,
                >,
            >::decrement_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPinBoxFutureOutputSbResultVecIdentitySendasync_trait(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<
                        Box<
                            dyn Future<
                                Output = SbResult<Vec<Identity>>,
                            > + Send + 'async_trait,
                        >,
                    >,
                >,
            >::increment_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPinBoxFutureOutputSbResultVecIdentitySendasync_trait(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<
                        Box<
                            dyn Future<
                                Output = SbResult<Vec<Identity>>,
                            > + Send + 'async_trait,
                        >,
                    >,
                >,
            >::decrement_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPinBoxFutureOutputSbResultVecMessageSendasync_trait(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<
                        Box<
                            dyn Future<
                                Output = SbResult<Vec<Message>>,
                            > + Send + 'async_trait,
                        >,
                    >,
                >,
            >::increment_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPinBoxFutureOutputSbResultVecMessageSendasync_trait(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<
                        Box<
                            dyn Future<
                                Output = SbResult<Vec<Message>>,
                            > + Send + 'async_trait,
                        >,
                    >,
                >,
            >::decrement_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPinBoxFutureOutputSbResultVecSbEventSendasync_trait(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<
                        Box<
                            dyn Future<
                                Output = SbResult<Vec<SbEvent>>,
                            > + Send + 'async_trait,
                        >,
                    >,
                >,
            >::increment_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerPinBoxFutureOutputSbResultVecSbEventSendasync_trait(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
                    ::Pin<
                        Box<
                            dyn Future<
                                Output = SbResult<Vec<SbEvent>>,
                            > + Send + 'async_trait,
                        >,
                    >,
                >,
            >::decrement_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSbResult(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbResult<()>>,
            >::increment_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSbResult(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbResult<()>>,
            >::decrement_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSbResultString(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbResult<String>>,
            >::increment_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSbResultString(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbResult<String>>,
            >::decrement_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSbSession(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbSession>,
            >::increment_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSbSession(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<SbSession>,
            >::decrement_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerServiceScanner(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ServiceScanner>,
            >::increment_strong_count(ptr as _);
        }
        #[no_mangle]
        pub extern "C" fn frbgen_cry_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerServiceScanner(
            ptr: *const std::ffi::c_void,
        ) {
            MoiArc::<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ServiceScanner>,
            >::decrement_strong_count(ptr as _);
        }
    }
    #[cfg(not(target_family = "wasm"))]
    pub use io::*;
}
use error::{Error, SbResult};
pub mod api {
    #[cfg(feature = "flutter")]
    pub mod api {
        use std::sync::Arc;
        use chrono::NaiveDateTime;
        use flutter_rust_bridge::frb;
        pub use tokio::sync::RwLock;
        use uuid::Uuid;
        pub use super::types::{B64SessionState, ImportIdentityState, SessionTrait};
        pub use super::{error::SbResult, mdns::HostRecord};
        use crate::crypto::EncodeB64;
        pub use crate::crypto::SessionState;
        use crate::proto::SbEvent;
        pub use crate::response::{Identity, Message};
        pub struct SbSession(Arc<RwLock<dyn SessionTrait + Send + Sync>>);
        pub use async_trait::async_trait;
        impl HostRecord {
            pub async fn connect(
                self,
                state: B64SessionState,
            ) -> anyhow::Result<Option<SbSession>> {
                let proto = self.connect_impl().await?;
                if let Some(session) = proto
                    .key_exchange(SessionState::from_b64(state)?)
                    .await?
                {
                    Ok(Some(SbSession(Arc::new(RwLock::new(session)))))
                } else {
                    Ok(None)
                }
            }
        }
    }
    pub mod error {
        use std::array::TryFromSliceError;
        use crate::proto::{
            unit_response::UnitresponseMaybeMessage, RespCode, UnitResponse,
        };
        pub type SbResult<T> = std::result::Result<T, Error>;
        pub enum Error {
            #[error("Mdns error: {0}")]
            MdnsError(#[from] mdns_sd::Error),
            #[error("Protobuf decode error: {0}")]
            ProtoDecode(#[from] prost::DecodeError),
            #[error("Protobuf encode error: {0}")]
            ProtoEncode(#[from] prost::EncodeError),
            #[error("Unknown enum varient: {0}")]
            ProtoUnknownEnum(#[from] prost::UnknownEnumValue),
            #[error("IO error {0}")]
            IoError(#[from] std::io::Error),
            #[error("Message size error: {0}")]
            MessageSizeError(usize),
            #[error("CRC Mismatch")]
            CrcMismatch,
            #[error("Message type mismatch expected {expected} got {actual}")]
            TypeMismatch { expected: String, actual: String },
            #[error("crypto error: {0}")]
            Crypto(String),
            #[error("Buf length mismatch")]
            BufLengthError(#[from] TryFromSliceError),
            #[error("No addresses remaining")]
            NoAddr,
            #[error("Pairing failed")]
            PairingFailed,
            #[error("Coinscam error: {0}")]
            CoinError(#[from] bip39::Error),
            #[error("Possible mitm detected!")]
            MitmDetected,
            #[error("Corrupt header")]
            CorruptHeader,
            #[error("{0}")]
            RemoteError(String),
            #[error("{0}")]
            JsonError(#[from] serde_json::Error),
            #[error("{0}")]
            Generic(#[from] Box<dyn std::error::Error + Send + Sync>),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Error {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Error::MdnsError(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "MdnsError",
                            &__self_0,
                        )
                    }
                    Error::ProtoDecode(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ProtoDecode",
                            &__self_0,
                        )
                    }
                    Error::ProtoEncode(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ProtoEncode",
                            &__self_0,
                        )
                    }
                    Error::ProtoUnknownEnum(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ProtoUnknownEnum",
                            &__self_0,
                        )
                    }
                    Error::IoError(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "IoError",
                            &__self_0,
                        )
                    }
                    Error::MessageSizeError(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "MessageSizeError",
                            &__self_0,
                        )
                    }
                    Error::CrcMismatch => {
                        ::core::fmt::Formatter::write_str(f, "CrcMismatch")
                    }
                    Error::TypeMismatch { expected: __self_0, actual: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "TypeMismatch",
                            "expected",
                            __self_0,
                            "actual",
                            &__self_1,
                        )
                    }
                    Error::Crypto(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Crypto",
                            &__self_0,
                        )
                    }
                    Error::BufLengthError(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "BufLengthError",
                            &__self_0,
                        )
                    }
                    Error::NoAddr => ::core::fmt::Formatter::write_str(f, "NoAddr"),
                    Error::PairingFailed => {
                        ::core::fmt::Formatter::write_str(f, "PairingFailed")
                    }
                    Error::CoinError(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "CoinError",
                            &__self_0,
                        )
                    }
                    Error::MitmDetected => {
                        ::core::fmt::Formatter::write_str(f, "MitmDetected")
                    }
                    Error::CorruptHeader => {
                        ::core::fmt::Formatter::write_str(f, "CorruptHeader")
                    }
                    Error::RemoteError(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "RemoteError",
                            &__self_0,
                        )
                    }
                    Error::JsonError(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "JsonError",
                            &__self_0,
                        )
                    }
                    Error::Generic(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Generic",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        impl std::error::Error for Error {
            fn source(
                &self,
            ) -> ::core::option::Option<&(dyn std::error::Error + 'static)> {
                use thiserror::__private::AsDynError as _;
                #[allow(deprecated)]
                match self {
                    Error::MdnsError { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                    Error::ProtoDecode { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                    Error::ProtoEncode { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                    Error::ProtoUnknownEnum { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                    Error::IoError { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                    Error::MessageSizeError { .. } => ::core::option::Option::None,
                    Error::CrcMismatch { .. } => ::core::option::Option::None,
                    Error::TypeMismatch { .. } => ::core::option::Option::None,
                    Error::Crypto { .. } => ::core::option::Option::None,
                    Error::BufLengthError { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                    Error::NoAddr { .. } => ::core::option::Option::None,
                    Error::PairingFailed { .. } => ::core::option::Option::None,
                    Error::CoinError { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                    Error::MitmDetected { .. } => ::core::option::Option::None,
                    Error::CorruptHeader { .. } => ::core::option::Option::None,
                    Error::RemoteError { .. } => ::core::option::Option::None,
                    Error::JsonError { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                    Error::Generic { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        impl ::core::fmt::Display for Error {
            fn fmt(
                &self,
                __formatter: &mut ::core::fmt::Formatter,
            ) -> ::core::fmt::Result {
                use thiserror::__private::AsDisplay as _;
                #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
                match self {
                    Error::MdnsError(_0) => {
                        __formatter
                            .write_fmt(format_args!("Mdns error: {0}", _0.as_display()))
                    }
                    Error::ProtoDecode(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!("Protobuf decode error: {0}", _0.as_display()),
                            )
                    }
                    Error::ProtoEncode(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!("Protobuf encode error: {0}", _0.as_display()),
                            )
                    }
                    Error::ProtoUnknownEnum(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!("Unknown enum varient: {0}", _0.as_display()),
                            )
                    }
                    Error::IoError(_0) => {
                        __formatter
                            .write_fmt(format_args!("IO error {0}", _0.as_display()))
                    }
                    Error::MessageSizeError(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!("Message size error: {0}", _0.as_display()),
                            )
                    }
                    Error::CrcMismatch {} => __formatter.write_str("CRC Mismatch"),
                    Error::TypeMismatch { expected, actual } => {
                        __formatter
                            .write_fmt(
                                format_args!(
                                    "Message type mismatch expected {0} got {1}",
                                    expected.as_display(),
                                    actual.as_display(),
                                ),
                            )
                    }
                    Error::Crypto(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!("crypto error: {0}", _0.as_display()),
                            )
                    }
                    Error::BufLengthError(_0) => {
                        __formatter.write_str("Buf length mismatch")
                    }
                    Error::NoAddr {} => __formatter.write_str("No addresses remaining"),
                    Error::PairingFailed {} => __formatter.write_str("Pairing failed"),
                    Error::CoinError(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!("Coinscam error: {0}", _0.as_display()),
                            )
                    }
                    Error::MitmDetected {} => {
                        __formatter.write_str("Possible mitm detected!")
                    }
                    Error::CorruptHeader {} => __formatter.write_str("Corrupt header"),
                    Error::RemoteError(_0) => {
                        __formatter.write_fmt(format_args!("{0}", _0.as_display()))
                    }
                    Error::JsonError(_0) => {
                        __formatter.write_fmt(format_args!("{0}", _0.as_display()))
                    }
                    Error::Generic(_0) => {
                        __formatter.write_fmt(format_args!("{0}", _0.as_display()))
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        impl ::core::convert::From<mdns_sd::Error> for Error {
            #[allow(deprecated)]
            fn from(source: mdns_sd::Error) -> Self {
                Error::MdnsError { 0: source }
            }
        }
        #[allow(unused_qualifications)]
        impl ::core::convert::From<prost::DecodeError> for Error {
            #[allow(deprecated)]
            fn from(source: prost::DecodeError) -> Self {
                Error::ProtoDecode { 0: source }
            }
        }
        #[allow(unused_qualifications)]
        impl ::core::convert::From<prost::EncodeError> for Error {
            #[allow(deprecated)]
            fn from(source: prost::EncodeError) -> Self {
                Error::ProtoEncode { 0: source }
            }
        }
        #[allow(unused_qualifications)]
        impl ::core::convert::From<prost::UnknownEnumValue> for Error {
            #[allow(deprecated)]
            fn from(source: prost::UnknownEnumValue) -> Self {
                Error::ProtoUnknownEnum {
                    0: source,
                }
            }
        }
        #[allow(unused_qualifications)]
        impl ::core::convert::From<std::io::Error> for Error {
            #[allow(deprecated)]
            fn from(source: std::io::Error) -> Self {
                Error::IoError { 0: source }
            }
        }
        #[allow(unused_qualifications)]
        impl ::core::convert::From<TryFromSliceError> for Error {
            #[allow(deprecated)]
            fn from(source: TryFromSliceError) -> Self {
                Error::BufLengthError { 0: source }
            }
        }
        #[allow(unused_qualifications)]
        impl ::core::convert::From<bip39::Error> for Error {
            #[allow(deprecated)]
            fn from(source: bip39::Error) -> Self {
                Error::CoinError { 0: source }
            }
        }
        #[allow(unused_qualifications)]
        impl ::core::convert::From<serde_json::Error> for Error {
            #[allow(deprecated)]
            fn from(source: serde_json::Error) -> Self {
                Error::JsonError { 0: source }
            }
        }
        #[allow(unused_qualifications)]
        impl ::core::convert::From<Box<dyn std::error::Error + Send + Sync>> for Error {
            #[allow(deprecated)]
            fn from(source: Box<dyn std::error::Error + Send + Sync>) -> Self {
                Error::Generic { 0: source }
            }
        }
        pub trait IntoRemoteErr {
            fn into_remote_err(self) -> SbResult<()>;
        }
        impl IntoRemoteErr for UnitResponse {
            fn into_remote_err(self) -> SbResult<()> {
                if self.code() != RespCode::Ok {
                    Err(
                        self
                            .unitresponse_maybe_message
                            .map(|v| {
                                let UnitresponseMaybeMessage::MessageCode(v) = v;
                                Error::RemoteError(v)
                            })
                            .unwrap_or_else(|| Error::RemoteError("".to_owned())),
                    )
                } else {
                    Ok(())
                }
            }
        }
    }
    pub mod mdns {
        use std::sync::Arc;
        pub use std::{
            collections::{BTreeMap, BTreeSet},
            future::Future,
        };
        #[cfg(feature = "flutter")]
        use flutter_rust_bridge::{frb, DartFnFuture};
        pub use mdns_sd::{ServiceDaemon, ServiceEvent};
        pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
        use tokio_util::sync::CancellationToken;
        use crate::error::SbResult;
        pub type HostRecords<'a> = tokio::sync::RwLockReadGuard<
            'a,
            BTreeMap<String, HostRecord>,
        >;
        struct ServiceScannerInner {
            devices: tokio::sync::RwLock<BTreeMap<String, HostRecord>>,
        }
        pub struct ServiceScanner {
            inner: std::sync::Arc<ServiceScannerInner>,
            handle: Option<CancellationToken>,
        }
        pub struct HostRecord {
            pub name: String,
            pub addr: Vec<IpAddr>,
            pub port: u16,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for HostRecord {
            #[inline]
            fn clone(&self) -> HostRecord {
                HostRecord {
                    name: ::core::clone::Clone::clone(&self.name),
                    addr: ::core::clone::Clone::clone(&self.addr),
                    port: ::core::clone::Clone::clone(&self.port),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for HostRecord {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "HostRecord",
                    "name",
                    &self.name,
                    "addr",
                    &self.addr,
                    "port",
                    &&self.port,
                )
            }
        }
        #[cfg(feature = "flutter")]
        impl ServiceScanner {
            pub async fn discover_devices(
                &mut self,
                cb: impl Fn(Vec<HostRecord>) -> DartFnFuture<()> + Send + Sync + 'static,
            ) -> SbResult<()> {
                self.discover_devices_impl(std::sync::Arc::new(cb)).await
            }
            async fn discover_devices_impl(
                &mut self,
                cb: std::sync::Arc<
                    dyn Fn(Vec<HostRecord>) -> DartFnFuture<()> + Send + Sync + 'static,
                >,
            ) -> SbResult<()> {
                let s = self.inner.clone();
                let c = CancellationToken::new();
                self.handle = Some(c.clone());
                tokio::spawn(async move {
                    s.mdns_scan(
                            |res| {
                                let cb = cb.clone();
                                async move {
                                    cb(res.iter().map(|(_, v)| v.clone().into()).collect())
                                        .await;
                                    Ok(())
                                }
                            },
                            c,
                        )
                        .await
                });
                Ok(())
            }
            pub async fn stop_scan(&mut self) {
                if let Some(handle) = self.handle.take() {
                    handle.cancel()
                }
            }
        }
        impl ServiceScanner {
            pub fn new() -> Self {
                Self {
                    inner: Arc::new(ServiceScannerInner {
                        devices: tokio::sync::RwLock::new(BTreeMap::new()),
                    }),
                    handle: None,
                }
            }
            pub async fn mdns_scan<'b, F, Fut>(&'b mut self, cb: F) -> SbResult<()>
            where
                F: Fn(HostRecords<'b>) -> Fut,
                Fut: Future<Output = SbResult<()>>,
            {
                let c = CancellationToken::new();
                self.handle = Some(c.clone());
                self.inner.mdns_scan(cb, c).await
            }
        }
        impl ServiceScannerInner {
            async fn mdns_scan<'b, F, Fut>(
                &'b self,
                cb: F,
                token: CancellationToken,
            ) -> SbResult<()>
            where
                F: Fn(HostRecords<'b>) -> Fut,
                Fut: Future<Output = SbResult<()>>,
            {
                let mdns = ServiceDaemon::new()?;
                let service_type = "_sbd._tcp.local.";
                let receiver = mdns.browse(service_type)?;
                while let Some(event) = {
                    #[doc(hidden)]
                    mod __tokio_select_util {
                        pub(super) enum Out<_0, _1> {
                            _0(_0),
                            _1(_1),
                            Disabled,
                        }
                        pub(super) type Mask = u8;
                    }
                    use ::tokio::macros::support::Future;
                    use ::tokio::macros::support::Pin;
                    use ::tokio::macros::support::Poll::{Ready, Pending};
                    const BRANCHES: u32 = 2;
                    let mut disabled: __tokio_select_util::Mask = Default::default();
                    if !true {
                        let mask: __tokio_select_util::Mask = 1 << 0;
                        disabled |= mask;
                    }
                    if !true {
                        let mask: __tokio_select_util::Mask = 1 << 1;
                        disabled |= mask;
                    }
                    let mut output = {
                        let mut futures = (receiver.recv_async(), token.cancelled());
                        let mut futures = &mut futures;
                        ::tokio::macros::support::poll_fn(|cx| {
                                let mut is_pending = false;
                                let start = {
                                    ::tokio::macros::support::thread_rng_n(BRANCHES)
                                };
                                for i in 0..BRANCHES {
                                    let branch;
                                    #[allow(clippy::modulo_one)]
                                    {
                                        branch = (start + i) % BRANCHES;
                                    }
                                    match branch {
                                        #[allow(unreachable_code)]
                                        0 => {
                                            let mask = 1 << branch;
                                            if disabled & mask == mask {
                                                continue;
                                            }
                                            let (fut, ..) = &mut *futures;
                                            let mut fut = unsafe { Pin::new_unchecked(fut) };
                                            let out = match Future::poll(fut, cx) {
                                                Ready(out) => out,
                                                Pending => {
                                                    is_pending = true;
                                                    continue;
                                                }
                                            };
                                            disabled |= mask;
                                            #[allow(unused_variables)] #[allow(unused_mut)]
                                            match &out {
                                                event => {}
                                                _ => continue,
                                            }
                                            return Ready(__tokio_select_util::Out::_0(out));
                                        }
                                        #[allow(unreachable_code)]
                                        1 => {
                                            let mask = 1 << branch;
                                            if disabled & mask == mask {
                                                continue;
                                            }
                                            let (_, fut, ..) = &mut *futures;
                                            let mut fut = unsafe { Pin::new_unchecked(fut) };
                                            let out = match Future::poll(fut, cx) {
                                                Ready(out) => out,
                                                Pending => {
                                                    is_pending = true;
                                                    continue;
                                                }
                                            };
                                            disabled |= mask;
                                            #[allow(unused_variables)] #[allow(unused_mut)]
                                            match &out {
                                                _ => {}
                                                _ => continue,
                                            }
                                            return Ready(__tokio_select_util::Out::_1(out));
                                        }
                                        _ => {
                                            ::core::panicking::panic_fmt(
                                                format_args!(
                                                    "internal error: entered unreachable code: {0}",
                                                    format_args!(
                                                        "reaching this means there probably is an off by one bug",
                                                    ),
                                                ),
                                            );
                                        }
                                    }
                                }
                                if is_pending {
                                    Pending
                                } else {
                                    Ready(__tokio_select_util::Out::Disabled)
                                }
                            })
                            .await
                    };
                    match output {
                        __tokio_select_util::Out::_0(event) => Some(event),
                        __tokio_select_util::Out::_1(_) => None,
                        __tokio_select_util::Out::Disabled => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "all branches are disabled and there is no else branch",
                                ),
                            );
                        }
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "internal error: entered unreachable code: {0}",
                                    format_args!("failed to match bind"),
                                ),
                            );
                        }
                    }
                } {
                    match event {
                        Ok(ServiceEvent::ServiceResolved(info)) => {
                            self.devices
                                .write()
                                .await
                                .insert(
                                    info.get_fullname().to_owned(),
                                    HostRecord {
                                        name: info
                                            .get_fullname()
                                            .trim_end_matches(info.get_type())
                                            .trim_end_matches(".")
                                            .to_owned(),
                                        addr: info
                                            .get_addresses()
                                            .into_iter()
                                            .map(|v| v.clone())
                                            .collect::<BTreeSet<_>>()
                                            .into_iter()
                                            .map(|v| v.into())
                                            .collect(),
                                        port: info.get_port(),
                                    },
                                );
                        }
                        Ok(ServiceEvent::ServiceRemoved(_, fullname)) => {
                            self.devices.write().await.remove(&fullname);
                        }
                        _ => {}
                    }
                    cb(self.devices.read().await).await?;
                }
                Ok(())
            }
        }
    }
    #[cfg(feature = "flutter")]
    pub mod mirror {
        pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
        use flutter_rust_bridge::frb;
        pub enum Ip {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
        }
        const _: () = ();
        #[cfg(not(frb_expand))]
        impl __external_impl__49707634416464722369378111748433969 {
            pub const fn is_unspecified(&self) -> bool {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            pub const fn is_loopback(&self) -> bool {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            pub const fn is_multicast(&self) -> bool {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            pub fn to_string(&self) -> String {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
        }
        #[cfg(not(frb_expand))]
        pub struct __external_impl__49707634416464722369378111748433969(pub Ipv4Addr);
        const _: () = ();
        #[cfg(not(frb_expand))]
        impl __external_impl__497076364164647215619427836552447335 {
            pub const fn is_unspecified(&self) -> bool {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            pub const fn is_loopback(&self) -> bool {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            pub const fn is_multicast(&self) -> bool {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            pub fn to_string(&self) -> String {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
        }
        #[cfg(not(frb_expand))]
        pub struct __external_impl__497076364164647215619427836552447335(pub Ipv6Addr);
    }
    pub mod response {
        use std::{collections::HashMap, fmt};
        use serde::{ser::Error, Deserialize, Serialize};
        use uuid::Uuid;
        pub use crate::{
            error::{Error as CrateError, SbResult},
            proto::{ApiMessage, IdentityResponse, MessageResponse, RespCode},
            serialize::ToUuid,
        };
        pub trait PrettyPrint {
            fn print_output(&self) -> SbResult<String>;
        }
        pub struct Identity {
            pub fingerprint: Option<Uuid>,
            pub name: String,
            pub public_key: Vec<u8>,
            pub is_owned: bool,
            pub extra: HashMap<String, Vec<u8>>,
            pub sig: Vec<u8>,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Identity {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Identity",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "fingerprint",
                        &self.fingerprint,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "public_key",
                        &self.public_key,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "is_owned",
                        &self.is_owned,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "extra",
                        &self.extra,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "sig",
                        &self.sig,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Identity {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "fingerprint" => _serde::__private::Ok(__Field::__field0),
                                "name" => _serde::__private::Ok(__Field::__field1),
                                "public_key" => _serde::__private::Ok(__Field::__field2),
                                "is_owned" => _serde::__private::Ok(__Field::__field3),
                                "extra" => _serde::__private::Ok(__Field::__field4),
                                "sig" => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"fingerprint" => _serde::__private::Ok(__Field::__field0),
                                b"name" => _serde::__private::Ok(__Field::__field1),
                                b"public_key" => _serde::__private::Ok(__Field::__field2),
                                b"is_owned" => _serde::__private::Ok(__Field::__field3),
                                b"extra" => _serde::__private::Ok(__Field::__field4),
                                b"sig" => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Identity>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Identity;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Identity",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                Option<Uuid>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Identity with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Identity with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Vec<u8>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Identity with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Identity with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                HashMap<String, Vec<u8>>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Identity with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match _serde::de::SeqAccess::next_element::<
                                Vec<u8>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Identity with 6 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Identity {
                                fingerprint: __field0,
                                name: __field1,
                                public_key: __field2,
                                is_owned: __field3,
                                extra: __field4,
                                sig: __field5,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Option<Uuid>> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<bool> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                HashMap<String, Vec<u8>>,
                            > = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "fingerprint",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<Uuid>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "public_key",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Vec<u8>>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "is_owned",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("extra"),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                HashMap<String, Vec<u8>>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("sig"),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Vec<u8>>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("fingerprint")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("name")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("public_key")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("is_owned")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("extra")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("sig")?
                                }
                            };
                            _serde::__private::Ok(Identity {
                                fingerprint: __field0,
                                name: __field1,
                                public_key: __field2,
                                is_owned: __field3,
                                extra: __field4,
                                sig: __field5,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "fingerprint",
                        "name",
                        "public_key",
                        "is_owned",
                        "extra",
                        "sig",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Identity",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Identity>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Identity {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "fingerprint",
                    "name",
                    "public_key",
                    "is_owned",
                    "extra",
                    "sig",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.fingerprint,
                    &self.name,
                    &self.public_key,
                    &self.is_owned,
                    &self.extra,
                    &&self.sig,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Identity",
                    names,
                    values,
                )
            }
        }
        impl PrettyPrint for Vec<Identity> {
            fn print_output(&self) -> SbResult<String> {
                let s = serde_json::to_string_pretty(self)?;
                Ok(s)
            }
        }
        impl fmt::Display for Identity {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let s = serde_json::to_string_pretty(self)
                    .map_err(|e| fmt::Error::custom(e.to_string()))?;
                f.write_str(&s)
            }
        }
        pub struct Message {
            pub from_fingerprint: Option<Uuid>,
            pub to_fingerprint: Option<Uuid>,
            pub application: String,
            pub extension: String,
            pub mime: String,
            pub send_date: i64,
            pub receive_date: i64,
            pub is_file: bool,
            pub id: Option<Uuid>,
            pub body: Vec<u8>,
            pub file_name: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Message {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Message",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "from_fingerprint",
                        &self.from_fingerprint,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "to_fingerprint",
                        &self.to_fingerprint,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "application",
                        &self.application,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "extension",
                        &self.extension,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "mime",
                        &self.mime,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "send_date",
                        &self.send_date,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "receive_date",
                        &self.receive_date,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "is_file",
                        &self.is_file,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "body",
                        &self.body,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "file_name",
                        &self.file_name,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Message {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                        __field10,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                10u64 => _serde::__private::Ok(__Field::__field10),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "from_fingerprint" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                "to_fingerprint" => _serde::__private::Ok(__Field::__field1),
                                "application" => _serde::__private::Ok(__Field::__field2),
                                "extension" => _serde::__private::Ok(__Field::__field3),
                                "mime" => _serde::__private::Ok(__Field::__field4),
                                "send_date" => _serde::__private::Ok(__Field::__field5),
                                "receive_date" => _serde::__private::Ok(__Field::__field6),
                                "is_file" => _serde::__private::Ok(__Field::__field7),
                                "id" => _serde::__private::Ok(__Field::__field8),
                                "body" => _serde::__private::Ok(__Field::__field9),
                                "file_name" => _serde::__private::Ok(__Field::__field10),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"from_fingerprint" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                b"to_fingerprint" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                b"application" => _serde::__private::Ok(__Field::__field2),
                                b"extension" => _serde::__private::Ok(__Field::__field3),
                                b"mime" => _serde::__private::Ok(__Field::__field4),
                                b"send_date" => _serde::__private::Ok(__Field::__field5),
                                b"receive_date" => _serde::__private::Ok(__Field::__field6),
                                b"is_file" => _serde::__private::Ok(__Field::__field7),
                                b"id" => _serde::__private::Ok(__Field::__field8),
                                b"body" => _serde::__private::Ok(__Field::__field9),
                                b"file_name" => _serde::__private::Ok(__Field::__field10),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Message>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Message;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Message",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                Option<Uuid>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Message with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                Option<Uuid>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Message with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Message with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Message with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Message with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match _serde::de::SeqAccess::next_element::<
                                i64,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Message with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match _serde::de::SeqAccess::next_element::<
                                i64,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct Message with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct Message with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match _serde::de::SeqAccess::next_element::<
                                Option<Uuid>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct Message with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field9 = match _serde::de::SeqAccess::next_element::<
                                Vec<u8>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct Message with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field10 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            10usize,
                                            &"struct Message with 11 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Message {
                                from_fingerprint: __field0,
                                to_fingerprint: __field1,
                                application: __field2,
                                extension: __field3,
                                mime: __field4,
                                send_date: __field5,
                                receive_date: __field6,
                                is_file: __field7,
                                id: __field8,
                                body: __field9,
                                file_name: __field10,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Option<Uuid>> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Option<Uuid>> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<i64> = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<i64> = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<bool> = _serde::__private::None;
                            let mut __field8: _serde::__private::Option<Option<Uuid>> = _serde::__private::None;
                            let mut __field9: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
                            let mut __field10: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "from_fingerprint",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<Uuid>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "to_fingerprint",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<Uuid>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "application",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "extension",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("mime"),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "send_date",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i64>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "receive_date",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i64>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "is_file",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<Uuid>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field9 => {
                                        if _serde::__private::Option::is_some(&__field9) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("body"),
                                            );
                                        }
                                        __field9 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Vec<u8>>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field10 => {
                                        if _serde::__private::Option::is_some(&__field10) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "file_name",
                                                ),
                                            );
                                        }
                                        __field10 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("from_fingerprint")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("to_fingerprint")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("application")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("extension")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("mime")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("send_date")?
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("receive_date")?
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("is_file")?
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field9 = match __field9 {
                                _serde::__private::Some(__field9) => __field9,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("body")?
                                }
                            };
                            let __field10 = match __field10 {
                                _serde::__private::Some(__field10) => __field10,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("file_name")?
                                }
                            };
                            _serde::__private::Ok(Message {
                                from_fingerprint: __field0,
                                to_fingerprint: __field1,
                                application: __field2,
                                extension: __field3,
                                mime: __field4,
                                send_date: __field5,
                                receive_date: __field6,
                                is_file: __field7,
                                id: __field8,
                                body: __field9,
                                file_name: __field10,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "from_fingerprint",
                        "to_fingerprint",
                        "application",
                        "extension",
                        "mime",
                        "send_date",
                        "receive_date",
                        "is_file",
                        "id",
                        "body",
                        "file_name",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Message",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Message>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Message {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "from_fingerprint",
                    "to_fingerprint",
                    "application",
                    "extension",
                    "mime",
                    "send_date",
                    "receive_date",
                    "is_file",
                    "id",
                    "body",
                    "file_name",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.from_fingerprint,
                    &self.to_fingerprint,
                    &self.application,
                    &self.extension,
                    &self.mime,
                    &self.send_date,
                    &self.receive_date,
                    &self.is_file,
                    &self.id,
                    &self.body,
                    &&self.file_name,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Message",
                    names,
                    values,
                )
            }
        }
        impl fmt::Display for Message {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let s = serde_json::to_string_pretty(self)
                    .map_err(|e| fmt::Error::custom(e.to_string()))?;
                f.write_str(&s)
            }
        }
        impl PrettyPrint for Vec<Message> {
            fn print_output(&self) -> SbResult<String> {
                let s = serde_json::to_string_pretty(self)?;
                Ok(s)
            }
        }
        impl TryFrom<IdentityResponse> for Vec<Identity> {
            type Error = CrateError;
            fn try_from(
                value: IdentityResponse,
            ) -> std::result::Result<Self, Self::Error> {
                if let RespCode::Ok = value.code.try_into()? {
                    Ok(
                        value
                            .identity
                            .into_iter()
                            .map(|v| Identity {
                                fingerprint: v.fingerprint.map(|v| v.as_uuid()),
                                name: v.name,
                                public_key: v.public_key,
                                is_owned: v.is_owned,
                                extra: v.extra,
                                sig: v.sig,
                            })
                            .collect(),
                    )
                } else {
                    Err(CrateError::RemoteError(value.code.to_string()))
                }
            }
        }
        impl TryFrom<MessageResponse> for Vec<Message> {
            type Error = CrateError;
            fn try_from(
                value: MessageResponse,
            ) -> std::result::Result<Self, Self::Error> {
                if let RespCode::Ok = value.code.try_into()? {
                    Ok(
                        value
                            .messsage
                            .into_iter()
                            .map(|v| Message {
                                from_fingerprint: v.from_fingerprint.map(|v| v.as_uuid()),
                                to_fingerprint: v.to_fingerprint.map(|v| v.as_uuid()),
                                application: v.application,
                                mime: v.mime,
                                extension: v.extension,
                                send_date: v.send_date,
                                receive_date: v.receive_date,
                                is_file: v.is_file,
                                id: v.id.map(|v| v.as_uuid()),
                                body: v.body,
                                file_name: v.file_name,
                            })
                            .collect(),
                    )
                } else {
                    Err(CrateError::RemoteError(value.code.to_string()))
                }
            }
        }
        impl From<Message> for ApiMessage {
            fn from(v: Message) -> Self {
                ApiMessage {
                    from_fingerprint: v.from_fingerprint.map(|v| v.as_proto()),
                    to_fingerprint: v.to_fingerprint.map(|v| v.as_proto()),
                    application: v.application,
                    mime: v.mime,
                    extension: v.extension,
                    send_date: v.send_date,
                    receive_date: v.receive_date,
                    is_file: v.is_file,
                    id: v.id.map(|v| v.as_proto()),
                    body: v.body,
                    file_name: v.file_name,
                }
            }
        }
    }
    pub mod serialize {
        use std::io::{Read, Write};
        use crc::Crc;
        use prost::{bytes::BufMut, Message};
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use crate::constants::{MESSAGE_SIZE_CAP, TYPE_SIZE_CAP};
        use crate::error::{Error, IntoRemoteErr, SbResult};
        use crate::proto::{self, MessageType, TypePrefix, UnitResponse};
        use crate::types::GetType;
        use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
        const JAVA_ALG: crc::Algorithm<u32> = crc::Algorithm {
            width: 32,
            poly: 0x04C11DB7,
            init: 0xFFFFFFFF,
            refin: true,
            refout: true,
            xorout: 0xFFFFFFFF,
            check: 0xaee7,
            residue: 0x0000,
        };
        pub trait ToUuid {
            fn as_uuid(&self) -> uuid::Uuid;
            fn as_proto(&self) -> proto::ProtoUuid;
        }
        impl ToUuid for proto::ProtoUuid {
            fn as_uuid(&self) -> uuid::Uuid {
                uuid::Uuid::from_u64_pair(self.upper, self.lower)
            }
            fn as_proto(&self) -> proto::ProtoUuid {
                *self
            }
        }
        impl ToUuid for uuid::Uuid {
            fn as_uuid(&self) -> uuid::Uuid {
                *self
            }
            fn as_proto(&self) -> proto::ProtoUuid {
                let (upper, lower) = self.as_u64_pair();
                proto::ProtoUuid { upper, lower }
            }
        }
        pub struct ProtoStream<A>(A);
        pub struct TypedMessage<M>
        where
            M: Message + GetType + Default + Send,
        {
            pub message: M,
            pub message_type: MessageType,
        }
        #[automatically_derived]
        impl<M: ::core::fmt::Debug> ::core::fmt::Debug for TypedMessage<M>
        where
            M: Message + GetType + Default + Send,
        {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "TypedMessage",
                    "message",
                    &self.message,
                    "message_type",
                    &&self.message_type,
                )
            }
        }
        #[automatically_derived]
        impl<M: ::core::default::Default> ::core::default::Default for TypedMessage<M>
        where
            M: Message + GetType + Default + Send,
        {
            #[inline]
            fn default() -> TypedMessage<M> {
                TypedMessage {
                    message: ::core::default::Default::default(),
                    message_type: ::core::default::Default::default(),
                }
            }
        }
        impl<M> TypedMessage<M>
        where
            M: Message + GetType + Default + Send,
        {
            pub fn new_typed(message_type: MessageType) -> Self {
                Self {
                    message: M::default(),
                    message_type,
                }
            }
            pub fn new(message: M) -> Self {
                let message_type = M::get_type();
                Self { message, message_type }
            }
        }
        impl<M> Message for TypedMessage<M>
        where
            M: Message + GetType + Default + Send,
        {
            fn clear(&mut self) {
                self.message.clear()
            }
            fn encode_raw(&self, buf: &mut impl BufMut)
            where
                Self: Sized,
            {
                self.message.encode_raw(buf)
            }
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> std::result::Result<(), prost::DecodeError>
            where
                Self: Sized,
            {
                self.message.merge_field(tag, wire_type, buf, ctx)
            }
            fn encode(
                &self,
                buf: &mut impl BufMut,
            ) -> std::result::Result<(), prost::EncodeError>
            where
                Self: Sized,
            {
                self.message.encode(buf)
            }
            fn encoded_len(&self) -> usize {
                self.message.encoded_len()
            }
            fn encode_to_vec(&self) -> Vec<u8>
            where
                Self: Sized,
            {
                self.message.encode_to_vec()
            }
            fn merge(
                &mut self,
                buf: impl prost::bytes::Buf,
            ) -> std::result::Result<(), prost::DecodeError>
            where
                Self: Sized,
            {
                self.message.merge(buf)
            }
            fn decode(
                buf: impl prost::bytes::Buf,
            ) -> std::result::Result<Self, prost::DecodeError>
            where
                Self: Default,
            {
                Ok(Self::new(M::decode(buf)?))
            }
        }
        impl<A> ProtoStream<A>
        where
            A: Unpin + Send,
        {
            pub fn new(sock: A) -> Self {
                Self(sock)
            }
            pub fn write_message_sync<M>(&mut self, message: &M) -> SbResult<()>
            where
                M: Message + GetType + Default + Send,
                A: Write,
            {
                let crc = Crc::<u32>::new(&JAVA_ALG);
                let mut digest = crc.digest();
                let message = message.encode_to_vec();
                let size = message.len() as i32;
                let tp = TypePrefix {
                    r#type: M::get_type().into(),
                };
                let tp = tp.encode_to_vec();
                let typesize = tp.len() as i32;
                digest.update(&typesize.to_be_bytes());
                digest.update(&size.to_be_bytes());
                digest.update(&tp);
                digest.update(&message);
                self.0.write_i32::<BigEndian>(typesize)?;
                self.0.write_i32::<BigEndian>(size)?;
                self.0.write(&tp)?;
                self.0.write(&message)?;
                self.0.write_u32::<BigEndian>(digest.finalize())?;
                Ok(())
            }
            pub async fn write_message<M>(&mut self, message: &M) -> SbResult<()>
            where
                M: Message + GetType + Default + Send,
                A: AsyncWriteExt,
            {
                let crc = Crc::<u32>::new(&JAVA_ALG);
                let mut digest = crc.digest();
                let message = message.encode_to_vec();
                let size = message.len() as i32;
                let tp = TypePrefix {
                    r#type: M::get_type().into(),
                };
                let tp = tp.encode_to_vec();
                let typesize = tp.len() as i32;
                digest.update(&typesize.to_be_bytes());
                digest.update(&size.to_be_bytes());
                digest.update(&tp);
                digest.update(&message);
                self.0.write_i32(typesize).await?;
                self.0.write_i32(size).await?;
                self.0.write(&tp).await?;
                self.0.write(&message).await?;
                self.0.write_u32(digest.finalize()).await?;
                Ok(())
            }
            pub fn read_message_sync<M>(&mut self) -> SbResult<M>
            where
                M: Message + GetType + Default + Send,
                A: Read,
            {
                let crc = Crc::<u32>::new(&JAVA_ALG);
                let mut digest = crc.digest();
                let typesize: i32 = self.0.read_i32::<BigEndian>()?;
                let size = self.0.read_i32::<BigEndian>()?;
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!(
                                "receivied message sizes {0} {1}",
                                typesize,
                                size,
                            ),
                            lvl,
                            &(
                                "scatterbrain::api::serialize",
                                "scatterbrain::api::serialize",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                };
                digest.update(&typesize.to_be_bytes());
                digest.update(&size.to_be_bytes());
                if size > MESSAGE_SIZE_CAP as i32 {
                    return Err(Error::MessageSizeError(size as usize));
                }
                if typesize > TYPE_SIZE_CAP as i32 {
                    return Err(Error::MessageSizeError(typesize as usize));
                }
                let mut mb = ::alloc::vec::from_elem(0, typesize as usize);
                self.0.read(mb.as_mut_slice())?;
                digest.update(mb.as_slice());
                let tp = TypePrefix::decode(mb.as_slice())?;
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!(
                                "read type prefix: expected={0} got={1}",
                                M::get_type().as_str_name(),
                                tp.r#type().as_str_name(),
                            ),
                            lvl,
                            &(
                                "scatterbrain::api::serialize",
                                "scatterbrain::api::serialize",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                };
                if M::get_type() != tp.r#type() {
                    return Err(Error::TypeMismatch {
                        expected: M::get_type().as_str_name().to_owned(),
                        actual: tp.r#type().as_str_name().to_owned(),
                    });
                }
                let mut mb = ::alloc::vec::from_elem(0, size as usize);
                self.0.read(mb.as_mut_slice())?;
                digest.update(mb.as_slice());
                let m = M::decode(mb.as_slice())?;
                let crc = self.0.read_u32::<BigEndian>()?;
                let mycrc = digest.finalize();
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!("received CRC thiers={0} ours={1}", crc, mycrc),
                            lvl,
                            &(
                                "scatterbrain::api::serialize",
                                "scatterbrain::api::serialize",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                };
                if crc != mycrc {
                    return Err(Error::CrcMismatch);
                }
                Ok(m)
            }
            pub async fn read_message<M>(&mut self) -> SbResult<M>
            where
                M: Message + GetType + Default + Send,
                A: AsyncReadExt,
            {
                let crc = Crc::<u32>::new(&JAVA_ALG);
                let mut digest = crc.digest();
                let typesize = self.0.read_i32().await?;
                let size = self.0.read_i32().await?;
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!(
                                "receivied message sizes {0} {1}",
                                typesize,
                                size,
                            ),
                            lvl,
                            &(
                                "scatterbrain::api::serialize",
                                "scatterbrain::api::serialize",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                };
                digest.update(&typesize.to_be_bytes());
                digest.update(&size.to_be_bytes());
                if size > MESSAGE_SIZE_CAP as i32 {
                    return Err(Error::MessageSizeError(size as usize));
                }
                if typesize > TYPE_SIZE_CAP as i32 {
                    return Err(Error::MessageSizeError(typesize as usize));
                }
                let mut mb = ::alloc::vec::from_elem(0, typesize as usize);
                self.0.read(mb.as_mut_slice()).await?;
                digest.update(mb.as_slice());
                let tp = TypePrefix::decode(mb.as_slice())?;
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!(
                                "read type prefix: expected={0} got={1}",
                                M::get_type().as_str_name(),
                                tp.r#type().as_str_name(),
                            ),
                            lvl,
                            &(
                                "scatterbrain::api::serialize",
                                "scatterbrain::api::serialize",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                };
                if M::get_type() != tp.r#type() {
                    if tp.r#type() == MessageType::UnitResponse {
                        let mut mb = ::alloc::vec::from_elem(0, size as usize);
                        self.0.read(mb.as_mut_slice()).await?;
                        digest.update(mb.as_slice());
                        let m = UnitResponse::decode(mb.as_slice())?;
                        let crc = self.0.read_u32().await?;
                        let mycrc = digest.finalize();
                        {
                            let lvl = ::log::Level::Debug;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    format_args!(
                                        "received CRC thiers={0} ours={1}",
                                        crc,
                                        mycrc,
                                    ),
                                    lvl,
                                    &(
                                        "scatterbrain::api::serialize",
                                        "scatterbrain::api::serialize",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        };
                        if crc != mycrc {
                            return Err(Error::CrcMismatch);
                        }
                        m.into_remote_err()?;
                    }
                    return Err(Error::TypeMismatch {
                        expected: M::get_type().as_str_name().to_owned(),
                        actual: tp.r#type().as_str_name().to_owned(),
                    });
                }
                let mut mb = ::alloc::vec::from_elem(0, size as usize);
                self.0.read(mb.as_mut_slice()).await?;
                digest.update(mb.as_slice());
                let m = M::decode(mb.as_slice())?;
                let crc = self.0.read_u32().await?;
                let mycrc = digest.finalize();
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!("received CRC thiers={0} ours={1}", crc, mycrc),
                            lvl,
                            &(
                                "scatterbrain::api::serialize",
                                "scatterbrain::api::serialize",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                };
                if crc != mycrc {
                    return Err(Error::CrcMismatch);
                }
                Ok(m)
            }
        }
    }
    pub mod types {
        pub use crate::response::{Identity, Message};
        use chrono::NaiveDateTime;
        pub use serde::{Deserialize, Serialize};
        pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
        use uuid::Uuid;
        pub struct B64SessionState {
            pub secretkey: String,
            pub pubkey: String,
            pub remotekey: Option<String>,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for B64SessionState {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "B64SessionState",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "secretkey",
                        &self.secretkey,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "pubkey",
                        &self.pubkey,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "remotekey",
                        &self.remotekey,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for B64SessionState {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "secretkey" => _serde::__private::Ok(__Field::__field0),
                                "pubkey" => _serde::__private::Ok(__Field::__field1),
                                "remotekey" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"secretkey" => _serde::__private::Ok(__Field::__field0),
                                b"pubkey" => _serde::__private::Ok(__Field::__field1),
                                b"remotekey" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<B64SessionState>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = B64SessionState;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct B64SessionState",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct B64SessionState with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct B64SessionState with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct B64SessionState with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(B64SessionState {
                                secretkey: __field0,
                                pubkey: __field1,
                                remotekey: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "secretkey",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("pubkey"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "remotekey",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("secretkey")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("pubkey")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("remotekey")?
                                }
                            };
                            _serde::__private::Ok(B64SessionState {
                                secretkey: __field0,
                                pubkey: __field1,
                                remotekey: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "secretkey",
                        "pubkey",
                        "remotekey",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "B64SessionState",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<B64SessionState>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub use crate::proto::{
            Ack, CryptoMessage, GetEvents, GetIdentityCommand, GetMessagesCmd,
            IdentityResponse, ImportIdentityCommand, ImportIdentityResponse,
            MessageResponse, MessageType, PairingAck, PairingInitiate, PairingRequest,
            SbEvents, SendMessageCmd, TypePrefix, UnitResponse,
        };
        use super::api::SbResult;
        pub trait SessionTrait {
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn get_identity<'life0, 'async_trait>(
                &'life0 mut self,
                id: Option<Uuid>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = SbResult<Vec<Identity>>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait;
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn get_events<'life0, 'async_trait>(
                &'life0 mut self,
                block: bool,
                count: Option<u32>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = SbResult<Vec<SbEvent>>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait;
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn get_messages<'life0, 'async_trait>(
                &'life0 mut self,
                application: String,
                limit: Option<i32>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = SbResult<Vec<Message>>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait;
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn send_messages<'life0, 'async_trait>(
                &'life0 mut self,
                messages: Vec<Message>,
                sign_identity: Option<Uuid>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = SbResult<()>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait;
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn initiate_identity_import<'life0, 'async_trait>(
                &'life0 mut self,
                id: Option<Uuid>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = SbResult<ImportIdentityState>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait;
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn get_messages_send_date<'life0, 'async_trait>(
                &'life0 mut self,
                application: String,
                limit: Option<i32>,
                start_date: NaiveDateTime,
                end_date: NaiveDateTime,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = SbResult<Vec<Message>>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait;
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn get_messages_recieve_date<'life0, 'async_trait>(
                &'life0 mut self,
                application: String,
                limit: Option<i32>,
                start_date: NaiveDateTime,
                end_date: NaiveDateTime,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = SbResult<Vec<Message>>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait;
        }
        pub trait GetType {
            fn get_type() -> MessageType;
            fn get_type_message(&self) -> TypePrefix {
                TypePrefix {
                    r#type: Self::get_type().into(),
                }
            }
        }
        impl GetType for GetMessagesCmd {
            fn get_type() -> MessageType {
                MessageType::GetMessage
            }
        }
        impl GetType for GetIdentityCommand {
            fn get_type() -> MessageType {
                MessageType::GetIdentity
            }
        }
        impl GetType for SendMessageCmd {
            fn get_type() -> MessageType {
                MessageType::SendMessage
            }
        }
        impl GetType for MessageType {
            fn get_type() -> MessageType {
                MessageType::Message
            }
        }
        impl GetType for UnitResponse {
            fn get_type() -> MessageType {
                MessageType::UnitResponse
            }
        }
        impl GetType for CryptoMessage {
            fn get_type() -> MessageType {
                MessageType::CryptoMessage
            }
        }
        impl GetType for PairingRequest {
            fn get_type() -> MessageType {
                MessageType::PairingRequest
            }
        }
        impl GetType for PairingInitiate {
            fn get_type() -> MessageType {
                MessageType::PairingInitiate
            }
        }
        impl GetType for PairingAck {
            fn get_type() -> MessageType {
                MessageType::PairingAck
            }
        }
        impl GetType for Ack {
            fn get_type() -> MessageType {
                MessageType::Ack
            }
        }
        impl GetType for IdentityResponse {
            fn get_type() -> MessageType {
                MessageType::IdentityResponse
            }
        }
        impl GetType for MessageResponse {
            fn get_type() -> MessageType {
                MessageType::MessageResponse
            }
        }
        impl GetType for ImportIdentityCommand {
            fn get_type() -> MessageType {
                MessageType::ImportIdentity
            }
        }
        impl GetType for ImportIdentityResponse {
            fn get_type() -> MessageType {
                MessageType::ImportIdentityResponse
            }
        }
        impl GetType for GetEvents {
            fn get_type() -> MessageType {
                MessageType::GetEvents
            }
        }
        impl GetType for SbEvents {
            fn get_type() -> MessageType {
                MessageType::DesktopEvents
            }
        }
        pub enum ImportIdentityState {
            Initiated(Uuid),
            Complete(Uuid),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ImportIdentityState {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ImportIdentityState::Initiated(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Initiated",
                            &__self_0,
                        )
                    }
                    ImportIdentityState::Complete(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Complete",
                            &__self_0,
                        )
                    }
                }
            }
        }
    }
}
pub mod connection {
    use bip39::Mnemonic;
    use chrono::NaiveDateTime;
    use sodiumoxide::crypto::{generichash::self, kx::{client_session_keys, PublicKey}};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use uuid::Uuid;
    use crate::types::SessionTrait;
    pub use crate::{
        crypto::{CryptoMessageWrapper, Session, SessionState},
        error::{Error, IntoRemoteErr, SbResult},
        mdns::HostRecord,
        proto::{
            get_events::MaybeCount, get_identity_command::Id,
            get_messages_cmd::{MaybeApplication, TimeRange, TimeSlice},
            import_identity_command::MaybeHandle,
            import_identity_response::{FinalResponse, State},
            send_message_cmd::SignIdentity, Ack, CryptoMessage, GetEvents,
            GetIdentityCommand, GetMessagesCmd, IdentityResponse, ImportIdentityCommand,
            ImportIdentityResponse, MessageResponse, PairingAck, PairingInitiate,
            PairingRequest, SbEvent, SbEvents, SendMessageCmd, UnitResponse,
        },
        response::{Identity, Message},
        serialize::{ProtoStream, ToUuid},
        types::ImportIdentityState,
    };
    pub use std::{future::Future, net::SocketAddr};
    pub use tokio::net::TcpStream;
    impl From<SocketAddr> for HostRecord {
        fn from(value: SocketAddr) -> Self {
            Self {
                name: value.to_string(),
                addr: [value.ip()].into_iter().collect(),
                port: value.port(),
            }
        }
    }
    impl HostRecord {
        pub(crate) async fn connect_impl(self) -> SbResult<ProtoStream<TcpStream>> {
            for addr in self.addr {
                {
                    ::std::io::_print(
                        format_args!("attempting to connect to {0}\n", addr),
                    );
                };
                match TcpStream::connect((addr, self.port)).await {
                    Ok(c) => return Ok(ProtoStream::new(c)),
                    Err(err) => {
                        let lvl = ::log::Level::Warn;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!("Failed to connect to {0}: {1}", addr, err),
                                lvl,
                                &(
                                    "scatterbrain::connection",
                                    "scatterbrain::connection",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    }
                }
            }
            Err(Error::NoAddr)
        }
    }
    impl<A> SessionTrait for Session<A>
    where
        A: Unpin + Send + AsyncReadExt + AsyncWriteExt,
        Self: Sized,
    {
        #[allow(
            elided_named_lifetimes,
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::needless_arbitrary_self_type,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn get_identity<'life0, 'async_trait>(
            &'life0 mut self,
            id: Option<Uuid>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = SbResult<Vec<Identity>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    SbResult<Vec<Identity>>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let id = id;
                let __ret: SbResult<Vec<Identity>> = {
                    let cmd = GetIdentityCommand {
                        header: Some(__self.get_header()),
                        id: id.map(|v| Id::Identity(v.as_proto())),
                        owned: false,
                    };
                    __self.write_crypto(cmd).await?;
                    let id: IdentityResponse = __self.read_crypto().await?;
                    id.try_into()
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            elided_named_lifetimes,
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::needless_arbitrary_self_type,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn get_events<'life0, 'async_trait>(
            &'life0 mut self,
            block: bool,
            count: Option<u32>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = SbResult<Vec<SbEvent>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    SbResult<Vec<SbEvent>>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let block = block;
                let count = count;
                let __ret: SbResult<Vec<SbEvent>> = {
                    let cmd = GetEvents {
                        header: Some(__self.get_header()),
                        block,
                        maybe_count: count.map(|v| MaybeCount::Count(v)),
                    };
                    __self.write_crypto(cmd).await?;
                    let resp: SbEvents = __self.read_crypto().await?;
                    Ok(resp.events)
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            elided_named_lifetimes,
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::needless_arbitrary_self_type,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn get_messages<'life0, 'async_trait>(
            &'life0 mut self,
            application: String,
            limit: Option<i32>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = SbResult<Vec<Message>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    SbResult<Vec<Message>>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let application = application;
                let limit = limit;
                let __ret: SbResult<Vec<Message>> = {
                    let cmd = GetMessagesCmd {
                        header: Some(__self.get_header()),
                        time_slice: None,
                        maybe_application: Some(
                            MaybeApplication::Application(application),
                        ),
                        limit: limit.unwrap_or(-1),
                    };
                    __self.write_crypto(cmd).await?;
                    let m: MessageResponse = __self.read_crypto().await?;
                    m.try_into()
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            elided_named_lifetimes,
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::needless_arbitrary_self_type,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn send_messages<'life0, 'async_trait>(
            &'life0 mut self,
            messages: Vec<Message>,
            sign_identity: Option<Uuid>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = SbResult<()>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    SbResult<()>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let messages = messages;
                let sign_identity = sign_identity;
                let __ret: SbResult<()> = {
                    let cmd = SendMessageCmd {
                        header: Some(__self.get_header()),
                        messages: messages.into_iter().map(|v| v.into()).collect(),
                        sign_identity: sign_identity
                            .map(|v| SignIdentity::Identity(v.as_proto())),
                    };
                    __self.write_crypto(cmd).await?;
                    let m: UnitResponse = __self.read_crypto().await?;
                    m.into_remote_err()?;
                    Ok(())
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            elided_named_lifetimes,
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::needless_arbitrary_self_type,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn initiate_identity_import<'life0, 'async_trait>(
            &'life0 mut self,
            id: Option<Uuid>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = SbResult<ImportIdentityState>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    SbResult<ImportIdentityState>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let id = id;
                let __ret: SbResult<ImportIdentityState> = {
                    let cmd = ImportIdentityCommand {
                        header: Some(__self.get_header()),
                        maybe_handle: id.map(|v| MaybeHandle::Handle(v.as_proto())),
                    };
                    __self.write_crypto(cmd).await?;
                    let resp: ImportIdentityResponse = __self.read_crypto().await?;
                    let state = resp
                        .state
                        .ok_or_else(|| Error::RemoteError(
                            "Missing state field".to_owned(),
                        ))?;
                    let res = match state {
                        State::Handle(uuid) => {
                            ImportIdentityState::Initiated(uuid.as_uuid())
                        }
                        State::Final(FinalResponse { identity, .. }) => {
                            ImportIdentityState::Complete(
                                identity
                                    .ok_or_else(|| Error::RemoteError(
                                        "missing identity uuid".to_owned(),
                                    ))?
                                    .as_uuid(),
                            )
                        }
                    };
                    Ok(res)
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            elided_named_lifetimes,
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::needless_arbitrary_self_type,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn get_messages_send_date<'life0, 'async_trait>(
            &'life0 mut self,
            application: String,
            limit: Option<i32>,
            start_date: NaiveDateTime,
            end_date: NaiveDateTime,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = SbResult<Vec<Message>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    SbResult<Vec<Message>>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let application = application;
                let limit = limit;
                let start_date = start_date;
                let end_date = end_date;
                let __ret: SbResult<Vec<Message>> = {
                    let cmd = GetMessagesCmd {
                        header: Some(__self.get_header()),
                        time_slice: Some(
                            TimeSlice::SendDate(TimeRange {
                                start: start_date.and_utc().timestamp(),
                                end: end_date.and_utc().timestamp(),
                            }),
                        ),
                        maybe_application: Some(
                            MaybeApplication::Application(application),
                        ),
                        limit: limit.unwrap_or(-1),
                    };
                    __self.write_crypto(cmd).await?;
                    let m: MessageResponse = __self.read_crypto().await?;
                    m.try_into()
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            elided_named_lifetimes,
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::needless_arbitrary_self_type,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn get_messages_recieve_date<'life0, 'async_trait>(
            &'life0 mut self,
            application: String,
            limit: Option<i32>,
            start_date: NaiveDateTime,
            end_date: NaiveDateTime,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = SbResult<Vec<Message>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    SbResult<Vec<Message>>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let application = application;
                let limit = limit;
                let start_date = start_date;
                let end_date = end_date;
                let __ret: SbResult<Vec<Message>> = {
                    let cmd = GetMessagesCmd {
                        header: Some(__self.get_header()),
                        time_slice: Some(
                            TimeSlice::ReceiveDate(TimeRange {
                                start: start_date.and_utc().timestamp(),
                                end: end_date.and_utc().timestamp(),
                            }),
                        ),
                        maybe_application: Some(
                            MaybeApplication::Application(application),
                        ),
                        limit: limit.unwrap_or(-1),
                    };
                    __self.write_crypto(cmd).await?;
                    let m: MessageResponse = __self.read_crypto().await?;
                    m.try_into()
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
    impl<A> ProtoStream<A>
    where
        A: Unpin + Send + AsyncReadExt + AsyncWriteExt,
    {
        pub async fn key_exchange(
            mut self,
            state: SessionState,
        ) -> SbResult<Option<Session<A>>> {
            let i = PairingInitiate {
                pubkey: state.pubkey.0.iter().copied().collect(),
            };
            self.write_message(&i).await?;
            let v: PairingAck = self.read_message().await?;
            let session_id = v
                .session
                .ok_or_else(|| Error::CorruptHeader)?
                .session
                .ok_or_else(|| Error::CorruptHeader)?;
            let sp = PublicKey(
                v
                    .pubkey
                    .try_into()
                    .map_err(|_| Error::Crypto("pubkey wrong size".to_owned()))?,
            );
            let (rx, tx) = client_session_keys(&state.pubkey, &state.secretkey, &sp)
                .unwrap();
            if let Some(remotekey) = state.remotekey {
                if remotekey.0 != sp.0 {
                    return Err(Error::MitmDetected);
                }
                Ok(
                    Some(Session {
                        session: session_id.as_uuid(),
                        rx,
                        tx,
                        state: SessionState {
                            secretkey: state.secretkey,
                            pubkey: state.pubkey,
                            remotekey: Some(remotekey),
                        },
                        stream: self,
                    }),
                )
            } else {
                Ok(None)
            }
        }
        pub async fn pair<'a, F, Fut>(
            mut self,
            state: SessionState,
            app_name: String,
            cb: F,
        ) -> SbResult<Session<A>>
        where
            F: FnOnce(Mnemonic) -> Fut,
            Fut: Future<
                Output = std::result::Result<
                    bool,
                    Box<dyn std::error::Error + Send + Sync>,
                >,
            >,
        {
            let i = PairingInitiate {
                pubkey: state.pubkey.0.iter().copied().collect(),
            };
            self.write_message(&i).await?;
            let v: PairingAck = self.read_message().await?;
            let session_id = v
                .session
                .ok_or_else(|| Error::CorruptHeader)?
                .session
                .ok_or_else(|| Error::CorruptHeader)?;
            let sp = PublicKey(
                v
                    .pubkey
                    .try_into()
                    .map_err(|_| Error::Crypto("pubkey wrong size".to_owned()))?,
            );
            let (rx, tx) = client_session_keys(&state.pubkey, &state.secretkey, &sp)
                .unwrap();
            if let Some(remotekey) = state.remotekey {
                if remotekey.0 != sp.0 {
                    return Err(Error::MitmDetected);
                }
                Ok(Session {
                    session: session_id.as_uuid(),
                    rx,
                    tx,
                    state: SessionState {
                        secretkey: state.secretkey,
                        pubkey: state.pubkey,
                        remotekey: Some(remotekey),
                    },
                    stream: self,
                })
            } else {
                let mut pr = PairingRequest::default();
                pr.name = app_name;
                pr.session = v.session;
                let pr = CryptoMessageWrapper::new_message(&pr, &rx)?;
                self.write_message(pr.message()).await?;
                let fingerprint = generichash::hash(
                        &i.pubkey,
                        Some(generichash::DIGEST_MIN),
                        None,
                    )
                    .unwrap();
                let words = Mnemonic::from_entropy(fingerprint.as_ref())?;
                cb(words).await?;
                let v: CryptoMessage = self.read_message().await?;
                let v = CryptoMessageWrapper::new(v);
                let v: Ack = v.decrypt(&tx)?;
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!("got ack {0}", v.success),
                            lvl,
                            &(
                                "scatterbrain::connection",
                                "scatterbrain::connection",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                };
                if !v.success {
                    return Err(Error::PairingFailed);
                }
                Ok(Session {
                    session: session_id.as_uuid(),
                    rx,
                    tx,
                    state: SessionState {
                        secretkey: state.secretkey,
                        pubkey: state.pubkey,
                        remotekey: Some(sp),
                    },
                    stream: self,
                })
            }
        }
    }
}
pub mod constants {
    pub const MESSAGE_SIZE_CAP: usize = 1024 * 16;
    pub const TYPE_SIZE_CAP: usize = 1024 * 8;
}
pub mod crypto {
    use prost::{bytes::Buf, Message};
    use sodiumoxide::{
        base64,
        crypto::{
            kx::{PublicKey, SecretKey, SessionKey},
            secretbox::{open, seal, Key, Nonce},
        },
        randombytes::randombytes,
    };
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use uuid::Uuid;
    use serde::{Deserialize, Serialize};
    use sodiumoxide::crypto::kx::self;
    use crate::{
        error::{Error, SbResult},
        proto::{ApiHeader, CryptoMessage},
        serialize::{ProtoStream, ToUuid},
        types::{B64SessionState, GetType},
    };
    pub trait EncodeB64<T>
    where
        Self: Sized,
    {
        fn b64(&self) -> T;
        fn from_b64(val: T) -> SbResult<Self>;
    }
    pub struct SessionState {
        pub secretkey: SecretKey,
        pub pubkey: PublicKey,
        pub remotekey: Option<PublicKey>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for SessionState {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "SessionState",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "secretkey",
                    &self.secretkey,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "pubkey",
                    &self.pubkey,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "remotekey",
                    &self.remotekey,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for SessionState {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "secretkey" => _serde::__private::Ok(__Field::__field0),
                            "pubkey" => _serde::__private::Ok(__Field::__field1),
                            "remotekey" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"secretkey" => _serde::__private::Ok(__Field::__field0),
                            b"pubkey" => _serde::__private::Ok(__Field::__field1),
                            b"remotekey" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<SessionState>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = SessionState;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct SessionState",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            SecretKey,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct SessionState with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            PublicKey,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct SessionState with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Option<PublicKey>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct SessionState with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(SessionState {
                            secretkey: __field0,
                            pubkey: __field1,
                            remotekey: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<SecretKey> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<PublicKey> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<PublicKey>> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "secretkey",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<SecretKey>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("pubkey"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<PublicKey>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "remotekey",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<PublicKey>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("secretkey")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("pubkey")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("remotekey")?
                            }
                        };
                        _serde::__private::Ok(SessionState {
                            secretkey: __field0,
                            pubkey: __field1,
                            remotekey: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "secretkey",
                    "pubkey",
                    "remotekey",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "SessionState",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<SessionState>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for SessionState {
        #[inline]
        fn clone(&self) -> SessionState {
            SessionState {
                secretkey: ::core::clone::Clone::clone(&self.secretkey),
                pubkey: ::core::clone::Clone::clone(&self.pubkey),
                remotekey: ::core::clone::Clone::clone(&self.remotekey),
            }
        }
    }
    impl EncodeB64<B64SessionState> for SessionState {
        fn b64(&self) -> B64SessionState {
            let secretkey = base64::encode(&self.secretkey.0, base64::Variant::UrlSafe);
            let pubkey = base64::encode(&self.pubkey.0, base64::Variant::UrlSafe);
            let remotekey = self
                .remotekey
                .map(|v| base64::encode(&v.0, base64::Variant::UrlSafe));
            B64SessionState {
                secretkey,
                pubkey,
                remotekey,
            }
        }
        fn from_b64(val: B64SessionState) -> SbResult<Self> {
            let secretkey = base64::decode(&val.secretkey, base64::Variant::UrlSafe)
                .map_err(|_| Error::Crypto("failed to parse base64".to_owned()))?;
            let pubkey = base64::decode(&val.pubkey, base64::Variant::UrlSafe)
                .map_err(|_| Error::Crypto("failed to parse base64".to_owned()))?;
            let remotekey = val
                .remotekey
                .map(|v| {
                    base64::decode(&v, base64::Variant::UrlSafe)
                        .map_err(|_| Error::Crypto("failed to parse base64".to_owned()))
                })
                .transpose()?;
            Ok(Self {
                secretkey: SecretKey(
                    secretkey
                        .try_into()
                        .map_err(|_| Error::Crypto("Secret key wrong size".to_owned()))?,
                ),
                pubkey: PublicKey(
                    pubkey
                        .try_into()
                        .map_err(|_| Error::Crypto("Public key wrong size".to_owned()))?,
                ),
                remotekey: remotekey
                    .map(|k| {
                        Ok::<
                            PublicKey,
                            Error,
                        >(
                            PublicKey(
                                k
                                    .try_into()
                                    .map_err(|_| Error::Crypto(
                                        "Public key wrong size".to_owned(),
                                    ))?,
                            ),
                        )
                    })
                    .transpose()?,
            })
        }
    }
    impl Default for SessionState {
        fn default() -> Self {
            let (pubkey, secretkey) = kx::gen_keypair();
            Self {
                pubkey,
                secretkey,
                remotekey: None,
            }
        }
    }
    pub struct Session<A> {
        pub session: Uuid,
        pub rx: SessionKey,
        pub tx: SessionKey,
        pub state: SessionState,
        pub stream: ProtoStream<A>,
    }
    impl<A> Session<A>
    where
        A: Unpin + Send + AsyncReadExt + AsyncWriteExt,
    {
        pub(crate) fn get_header(&self) -> ApiHeader {
            ApiHeader {
                session: Some(self.session.as_proto()),
                stream: None,
            }
        }
        pub async fn write_crypto<M>(&mut self, message: M) -> SbResult<()>
        where
            M: Message + GetType + Default,
        {
            let cm = CryptoMessageWrapper::new_message(&message, &self.rx)?;
            self.stream.write_message(cm.message()).await
        }
        pub async fn read_crypto<M>(&mut self) -> SbResult<M>
        where
            M: Message + GetType + Default,
        {
            let cm: CryptoMessage = self.stream.read_message().await?;
            let w = CryptoMessageWrapper::new(cm);
            w.decrypt(&self.tx)
        }
    }
    pub fn hash_as_uuid(bytes: &[u8]) -> SbResult<Uuid> {
        let lower: [u8; 8] = bytes[..8].try_into()?;
        let upper: [u8; 8] = bytes[8..16].try_into()?;
        let lower = u64::from_be_bytes(lower);
        let upper = u64::from_be_bytes(upper);
        Ok(Uuid::from_u64_pair(upper, lower))
    }
    pub struct CryptoMessageWrapper(CryptoMessage);
    impl CryptoMessageWrapper {
        pub fn new(cm: CryptoMessage) -> Self {
            Self(cm)
        }
        pub fn message(&self) -> &'_ CryptoMessage {
            &self.0
        }
        pub fn new_message<M>(message: &M, key: &SessionKey) -> SbResult<Self>
        where
            M: Message + GetType + Default + Send,
        {
            let nonce = randombytes(24);
            let n: [u8; 24] = nonce.clone().try_into().unwrap();
            let n = Nonce(n);
            let mut m = Vec::new();
            ProtoStream::new(&mut m).write_message_sync(message)?;
            match (&m.len(), &0) {
                (left_val, right_val) => {
                    if *left_val == *right_val {
                        let kind = ::core::panicking::AssertKind::Ne;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            let m = seal(&m, &n, &Key(key.0));
            Ok(
                Self(CryptoMessage {
                    nonce,
                    encrypted: m,
                }),
            )
        }
        pub fn decrypt<M>(self, key: &SessionKey) -> SbResult<M>
        where
            M: Message + GetType + Default + Send,
        {
            let nonce: [u8; 24] = self
                .0
                .nonce
                .try_into()
                .map_err(|_| Error::Crypto("Nonce wrong size".to_owned()))?;
            let nonce = Nonce(nonce);
            let bytes = open(&self.0.encrypted, &nonce, &Key(key.0))
                .map_err(|_| Error::Crypto("Decrypt failed".to_owned()))?;
            let mut m = ProtoStream::new(bytes.reader());
            Ok(m.read_message_sync()?)
        }
    }
}
pub use api::error;
pub use api::mdns;
pub use api::response;
pub use api::serialize;
pub use api::types;
pub mod proto {
    /// Generic status response with optional error code
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct Ack {
        #[prost(bool, tag = "1")]
        pub success: bool,
        #[prost(int32, tag = "2")]
        pub status: i32,
        #[prost(oneof = "ack::AckMaybeMessage", tags = "3")]
        pub ack_maybe_message: ::core::option::Option<ack::AckMaybeMessage>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for Ack {
        #[inline]
        fn clone(&self) -> Ack {
            Ack {
                success: ::core::clone::Clone::clone(&self.success),
                status: ::core::clone::Clone::clone(&self.status),
                ack_maybe_message: ::core::clone::Clone::clone(&self.ack_maybe_message),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for Ack {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for Ack {
        #[inline]
        fn eq(&self, other: &Ack) -> bool {
            self.success == other.success && self.status == other.status
                && self.ack_maybe_message == other.ack_maybe_message
        }
    }
    impl ::prost::Message for Ack {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.success != false {
                ::prost::encoding::bool::encode(1u32, &self.success, buf);
            }
            if self.status != 0i32 {
                ::prost::encoding::int32::encode(2u32, &self.status, buf);
            }
            if let Some(ref oneof) = self.ack_maybe_message {
                oneof.encode(buf)
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "Ack";
            match tag {
                1u32 => {
                    let mut value = &mut self.success;
                    ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "success");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.status;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "status");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.ack_maybe_message;
                    ack::AckMaybeMessage::merge(value, tag, wire_type, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "ack_maybe_message");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.success != false {
                    ::prost::encoding::bool::encoded_len(1u32, &self.success)
                } else {
                    0
                }
                + if self.status != 0i32 {
                    ::prost::encoding::int32::encoded_len(2u32, &self.status)
                } else {
                    0
                }
                + self
                    .ack_maybe_message
                    .as_ref()
                    .map_or(0, ack::AckMaybeMessage::encoded_len)
        }
        fn clear(&mut self) {
            self.success = false;
            self.status = 0i32;
            self.ack_maybe_message = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for Ack {
        fn default() -> Self {
            Ack {
                success: false,
                status: 0i32,
                ack_maybe_message: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for Ack {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Ack");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.success)
                };
                builder.field("success", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.status)
                };
                builder.field("status", &wrapper)
            };
            let builder = {
                let wrapper = &self.ack_maybe_message;
                builder.field("ack_maybe_message", &wrapper)
            };
            builder.finish()
        }
    }
    /// Nested message and enum types in `Ack`.
    pub mod ack {
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub enum AckMaybeMessage {
            #[prost(string, tag = "3")]
            Text(::prost::alloc::string::String),
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for AckMaybeMessage {
            #[inline]
            fn clone(&self) -> AckMaybeMessage {
                match self {
                    AckMaybeMessage::Text(__self_0) => {
                        AckMaybeMessage::Text(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for AckMaybeMessage {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for AckMaybeMessage {
            #[inline]
            fn eq(&self, other: &AckMaybeMessage) -> bool {
                match (self, other) {
                    (
                        AckMaybeMessage::Text(__self_0),
                        AckMaybeMessage::Text(__arg1_0),
                    ) => __self_0 == __arg1_0,
                }
            }
        }
        impl AckMaybeMessage {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    AckMaybeMessage::Text(ref value) => {
                        ::prost::encoding::string::encode(3u32, &*value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<AckMaybeMessage>,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    3u32 => {
                        match field {
                            ::core::option::Option::Some(
                                AckMaybeMessage::Text(ref mut value),
                            ) => {
                                ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            AckMaybeMessage::Text(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("invalid AckMaybeMessage tag: {0}", tag),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    AckMaybeMessage::Text(ref value) => {
                        ::prost::encoding::string::encoded_len(3u32, &*value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for AckMaybeMessage {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    AckMaybeMessage::Text(ref value) => {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&*value)
                        };
                        f.debug_tuple("Text").field(&wrapper).finish()
                    }
                }
            }
        }
    }
    /// Protobuf has no native ProtoUuid type. This is 128 bit ProtoUuid.
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct ProtoUuid {
        #[prost(uint64, tag = "1")]
        pub lower: u64,
        #[prost(uint64, tag = "2")]
        pub upper: u64,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for ProtoUuid {
        #[inline]
        fn clone(&self) -> ProtoUuid {
            let _: ::core::clone::AssertParamIsClone<u64>;
            *self
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::Copy for ProtoUuid {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for ProtoUuid {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for ProtoUuid {
        #[inline]
        fn eq(&self, other: &ProtoUuid) -> bool {
            self.lower == other.lower && self.upper == other.upper
        }
    }
    impl ::prost::Message for ProtoUuid {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.lower != 0u64 {
                ::prost::encoding::uint64::encode(1u32, &self.lower, buf);
            }
            if self.upper != 0u64 {
                ::prost::encoding::uint64::encode(2u32, &self.upper, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "ProtoUuid";
            match tag {
                1u32 => {
                    let mut value = &mut self.lower;
                    ::prost::encoding::uint64::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "lower");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.upper;
                    ::prost::encoding::uint64::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "upper");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.lower != 0u64 {
                    ::prost::encoding::uint64::encoded_len(1u32, &self.lower)
                } else {
                    0
                }
                + if self.upper != 0u64 {
                    ::prost::encoding::uint64::encoded_len(2u32, &self.upper)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.lower = 0u64;
            self.upper = 0u64;
        }
    }
    impl ::core::default::Default for ProtoUuid {
        fn default() -> Self {
            ProtoUuid {
                lower: 0u64,
                upper: 0u64,
            }
        }
    }
    impl ::core::fmt::Debug for ProtoUuid {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("ProtoUuid");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.lower)
                };
                builder.field("lower", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.upper)
                };
                builder.field("upper", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct TypePrefix {
        #[prost(enumeration = "MessageType", tag = "1")]
        pub r#type: i32,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for TypePrefix {
        #[inline]
        fn clone(&self) -> TypePrefix {
            let _: ::core::clone::AssertParamIsClone<i32>;
            *self
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::Copy for TypePrefix {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for TypePrefix {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for TypePrefix {
        #[inline]
        fn eq(&self, other: &TypePrefix) -> bool {
            self.r#type == other.r#type
        }
    }
    impl ::prost::Message for TypePrefix {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.r#type != MessageType::default() as i32 {
                ::prost::encoding::int32::encode(1u32, &self.r#type, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "TypePrefix";
            match tag {
                1u32 => {
                    let mut value = &mut self.r#type;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "r#type");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.r#type != MessageType::default() as i32 {
                    ::prost::encoding::int32::encoded_len(1u32, &self.r#type)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.r#type = MessageType::default() as i32;
        }
    }
    impl ::core::default::Default for TypePrefix {
        fn default() -> Self {
            TypePrefix {
                r#type: MessageType::default() as i32,
            }
        }
    }
    impl ::core::fmt::Debug for TypePrefix {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("TypePrefix");
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a i32);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let res: ::core::result::Result<MessageType, _> = ::core::convert::TryFrom::try_from(
                                *self.0,
                            );
                            match res {
                                Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                            }
                        }
                    }
                    ScalarWrapper(&self.r#type)
                };
                builder.field("r#type", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl TypePrefix {
        ///Returns the enum value of `type`, or the default if the field is set to an invalid enum value.
        pub fn r#type(&self) -> MessageType {
            ::core::convert::TryFrom::try_from(self.r#type)
                .unwrap_or(MessageType::default())
        }
        ///Sets `type` to the provided enum value.
        pub fn set_type(&mut self, value: MessageType) {
            self.r#type = value as i32;
        }
    }
    /// used to advertise the presence of a scatterbrain device
    /// and to request the exchange of identities and blockdata
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct Advertise {
        /// ScatterType type = 1;
        #[prost(uint32, repeated, tag = "1")]
        pub provides: ::prost::alloc::vec::Vec<u32>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for Advertise {
        #[inline]
        fn clone(&self) -> Advertise {
            Advertise {
                provides: ::core::clone::Clone::clone(&self.provides),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for Advertise {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for Advertise {
        #[inline]
        fn eq(&self, other: &Advertise) -> bool {
            self.provides == other.provides
        }
    }
    impl ::prost::Message for Advertise {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            ::prost::encoding::uint32::encode_packed(1u32, &self.provides, buf);
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "Advertise";
            match tag {
                1u32 => {
                    let mut value = &mut self.provides;
                    ::prost::encoding::uint32::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "provides");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::prost::encoding::uint32::encoded_len_packed(1u32, &self.provides)
        }
        fn clear(&mut self) {
            self.provides.clear();
        }
    }
    impl ::core::default::Default for Advertise {
        fn default() -> Self {
            Advertise {
                provides: ::prost::alloc::vec::Vec::new(),
            }
        }
    }
    impl ::core::fmt::Debug for Advertise {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Advertise");
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<u32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.provides)
                };
                builder.field("provides", &wrapper)
            };
            builder.finish()
        }
    }
    /// Used for avoiding retransmitting messages already on remote peer.
    /// yes I am aware i should have used a merkle tree. I was sleepy.
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct DeclareHashes {
        /// ScatterType type = 1;
        #[prost(bool, tag = "1")]
        pub optout: bool,
        #[prost(bytes = "vec", repeated, tag = "2")]
        pub hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for DeclareHashes {
        #[inline]
        fn clone(&self) -> DeclareHashes {
            DeclareHashes {
                optout: ::core::clone::Clone::clone(&self.optout),
                hashes: ::core::clone::Clone::clone(&self.hashes),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for DeclareHashes {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for DeclareHashes {
        #[inline]
        fn eq(&self, other: &DeclareHashes) -> bool {
            self.optout == other.optout && self.hashes == other.hashes
        }
    }
    impl ::prost::Message for DeclareHashes {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.optout != false {
                ::prost::encoding::bool::encode(1u32, &self.optout, buf);
            }
            ::prost::encoding::bytes::encode_repeated(2u32, &self.hashes, buf);
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "DeclareHashes";
            match tag {
                1u32 => {
                    let mut value = &mut self.optout;
                    ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "optout");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.hashes;
                    ::prost::encoding::bytes::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "hashes");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.optout != false {
                    ::prost::encoding::bool::encoded_len(1u32, &self.optout)
                } else {
                    0
                } + ::prost::encoding::bytes::encoded_len_repeated(2u32, &self.hashes)
        }
        fn clear(&mut self) {
            self.optout = false;
            self.hashes.clear();
        }
    }
    impl ::core::default::Default for DeclareHashes {
        fn default() -> Self {
            DeclareHashes {
                optout: false,
                hashes: ::prost::alloc::vec::Vec::new(),
            }
        }
    }
    impl ::core::fmt::Debug for DeclareHashes {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("DeclareHashes");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.optout)
                };
                builder.field("optout", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(
                        &'a ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
                    );
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.hashes)
                };
                builder.field("hashes", &wrapper)
            };
            builder.finish()
        }
    }
    /// header + metadata for a file, message or data stream
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct BlockData {
        #[prost(message, repeated, tag = "2")]
        pub from_fingerprint: ::prost::alloc::vec::Vec<ProtoUuid>,
        #[prost(message, repeated, tag = "3")]
        pub to_fingerprint: ::prost::alloc::vec::Vec<ProtoUuid>,
        #[prost(string, tag = "4")]
        pub application: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub filename: ::prost::alloc::string::String,
        #[prost(uint32, tag = "6")]
        pub ttl: u32,
        #[prost(string, tag = "7")]
        pub extension: ::prost::alloc::string::String,
        #[prost(string, tag = "8")]
        pub mime: ::prost::alloc::string::String,
        #[prost(uint32, tag = "9")]
        pub sessionid: u32,
        #[prost(bool, tag = "10")]
        pub todisk: bool,
        #[prost(bool, tag = "11")]
        pub endofstream: bool,
        #[prost(uint64, tag = "12")]
        pub send_date: u64,
        /// size of this is fragment count
        #[prost(bytes = "vec", repeated, tag = "13")]
        pub nexthashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
        #[prost(bytes = "vec", tag = "14")]
        pub sig: ::prost::alloc::vec::Vec<u8>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for BlockData {
        #[inline]
        fn clone(&self) -> BlockData {
            BlockData {
                from_fingerprint: ::core::clone::Clone::clone(&self.from_fingerprint),
                to_fingerprint: ::core::clone::Clone::clone(&self.to_fingerprint),
                application: ::core::clone::Clone::clone(&self.application),
                filename: ::core::clone::Clone::clone(&self.filename),
                ttl: ::core::clone::Clone::clone(&self.ttl),
                extension: ::core::clone::Clone::clone(&self.extension),
                mime: ::core::clone::Clone::clone(&self.mime),
                sessionid: ::core::clone::Clone::clone(&self.sessionid),
                todisk: ::core::clone::Clone::clone(&self.todisk),
                endofstream: ::core::clone::Clone::clone(&self.endofstream),
                send_date: ::core::clone::Clone::clone(&self.send_date),
                nexthashes: ::core::clone::Clone::clone(&self.nexthashes),
                sig: ::core::clone::Clone::clone(&self.sig),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for BlockData {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for BlockData {
        #[inline]
        fn eq(&self, other: &BlockData) -> bool {
            self.from_fingerprint == other.from_fingerprint
                && self.to_fingerprint == other.to_fingerprint
                && self.application == other.application
                && self.filename == other.filename && self.ttl == other.ttl
                && self.extension == other.extension && self.mime == other.mime
                && self.sessionid == other.sessionid && self.todisk == other.todisk
                && self.endofstream == other.endofstream
                && self.send_date == other.send_date
                && self.nexthashes == other.nexthashes && self.sig == other.sig
        }
    }
    impl ::prost::Message for BlockData {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            for msg in &self.from_fingerprint {
                ::prost::encoding::message::encode(2u32, msg, buf);
            }
            for msg in &self.to_fingerprint {
                ::prost::encoding::message::encode(3u32, msg, buf);
            }
            if self.application != "" {
                ::prost::encoding::string::encode(4u32, &self.application, buf);
            }
            if self.filename != "" {
                ::prost::encoding::string::encode(5u32, &self.filename, buf);
            }
            if self.ttl != 0u32 {
                ::prost::encoding::uint32::encode(6u32, &self.ttl, buf);
            }
            if self.extension != "" {
                ::prost::encoding::string::encode(7u32, &self.extension, buf);
            }
            if self.mime != "" {
                ::prost::encoding::string::encode(8u32, &self.mime, buf);
            }
            if self.sessionid != 0u32 {
                ::prost::encoding::uint32::encode(9u32, &self.sessionid, buf);
            }
            if self.todisk != false {
                ::prost::encoding::bool::encode(10u32, &self.todisk, buf);
            }
            if self.endofstream != false {
                ::prost::encoding::bool::encode(11u32, &self.endofstream, buf);
            }
            if self.send_date != 0u64 {
                ::prost::encoding::uint64::encode(12u32, &self.send_date, buf);
            }
            ::prost::encoding::bytes::encode_repeated(13u32, &self.nexthashes, buf);
            if self.sig != b"" as &[u8] {
                ::prost::encoding::bytes::encode(14u32, &self.sig, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "BlockData";
            match tag {
                2u32 => {
                    let mut value = &mut self.from_fingerprint;
                    ::prost::encoding::message::merge_repeated(
                            wire_type,
                            value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "from_fingerprint");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.to_fingerprint;
                    ::prost::encoding::message::merge_repeated(
                            wire_type,
                            value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "to_fingerprint");
                            error
                        })
                }
                4u32 => {
                    let mut value = &mut self.application;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "application");
                            error
                        })
                }
                5u32 => {
                    let mut value = &mut self.filename;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "filename");
                            error
                        })
                }
                6u32 => {
                    let mut value = &mut self.ttl;
                    ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "ttl");
                            error
                        })
                }
                7u32 => {
                    let mut value = &mut self.extension;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "extension");
                            error
                        })
                }
                8u32 => {
                    let mut value = &mut self.mime;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "mime");
                            error
                        })
                }
                9u32 => {
                    let mut value = &mut self.sessionid;
                    ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "sessionid");
                            error
                        })
                }
                10u32 => {
                    let mut value = &mut self.todisk;
                    ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "todisk");
                            error
                        })
                }
                11u32 => {
                    let mut value = &mut self.endofstream;
                    ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "endofstream");
                            error
                        })
                }
                12u32 => {
                    let mut value = &mut self.send_date;
                    ::prost::encoding::uint64::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "send_date");
                            error
                        })
                }
                13u32 => {
                    let mut value = &mut self.nexthashes;
                    ::prost::encoding::bytes::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "nexthashes");
                            error
                        })
                }
                14u32 => {
                    let mut value = &mut self.sig;
                    ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "sig");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + ::prost::encoding::message::encoded_len_repeated(
                    2u32,
                    &self.from_fingerprint,
                )
                + ::prost::encoding::message::encoded_len_repeated(
                    3u32,
                    &self.to_fingerprint,
                )
                + if self.application != "" {
                    ::prost::encoding::string::encoded_len(4u32, &self.application)
                } else {
                    0
                }
                + if self.filename != "" {
                    ::prost::encoding::string::encoded_len(5u32, &self.filename)
                } else {
                    0
                }
                + if self.ttl != 0u32 {
                    ::prost::encoding::uint32::encoded_len(6u32, &self.ttl)
                } else {
                    0
                }
                + if self.extension != "" {
                    ::prost::encoding::string::encoded_len(7u32, &self.extension)
                } else {
                    0
                }
                + if self.mime != "" {
                    ::prost::encoding::string::encoded_len(8u32, &self.mime)
                } else {
                    0
                }
                + if self.sessionid != 0u32 {
                    ::prost::encoding::uint32::encoded_len(9u32, &self.sessionid)
                } else {
                    0
                }
                + if self.todisk != false {
                    ::prost::encoding::bool::encoded_len(10u32, &self.todisk)
                } else {
                    0
                }
                + if self.endofstream != false {
                    ::prost::encoding::bool::encoded_len(11u32, &self.endofstream)
                } else {
                    0
                }
                + if self.send_date != 0u64 {
                    ::prost::encoding::uint64::encoded_len(12u32, &self.send_date)
                } else {
                    0
                }
                + ::prost::encoding::bytes::encoded_len_repeated(13u32, &self.nexthashes)
                + if self.sig != b"" as &[u8] {
                    ::prost::encoding::bytes::encoded_len(14u32, &self.sig)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.from_fingerprint.clear();
            self.to_fingerprint.clear();
            self.application.clear();
            self.filename.clear();
            self.ttl = 0u32;
            self.extension.clear();
            self.mime.clear();
            self.sessionid = 0u32;
            self.todisk = false;
            self.endofstream = false;
            self.send_date = 0u64;
            self.nexthashes.clear();
            self.sig.clear();
        }
    }
    impl ::core::default::Default for BlockData {
        fn default() -> Self {
            BlockData {
                from_fingerprint: ::core::default::Default::default(),
                to_fingerprint: ::core::default::Default::default(),
                application: ::prost::alloc::string::String::new(),
                filename: ::prost::alloc::string::String::new(),
                ttl: 0u32,
                extension: ::prost::alloc::string::String::new(),
                mime: ::prost::alloc::string::String::new(),
                sessionid: 0u32,
                todisk: false,
                endofstream: false,
                send_date: 0u64,
                nexthashes: ::prost::alloc::vec::Vec::new(),
                sig: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for BlockData {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("BlockData");
            let builder = {
                let wrapper = &self.from_fingerprint;
                builder.field("from_fingerprint", &wrapper)
            };
            let builder = {
                let wrapper = &self.to_fingerprint;
                builder.field("to_fingerprint", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.application)
                };
                builder.field("application", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.filename)
                };
                builder.field("filename", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.ttl)
                };
                builder.field("ttl", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.extension)
                };
                builder.field("extension", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.mime)
                };
                builder.field("mime", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.sessionid)
                };
                builder.field("sessionid", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.todisk)
                };
                builder.field("todisk", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.endofstream)
                };
                builder.field("endofstream", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.send_date)
                };
                builder.field("send_date", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(
                        &'a ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
                    );
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.nexthashes)
                };
                builder.field("nexthashes", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.sig)
                };
                builder.field("sig", &wrapper)
            };
            builder.finish()
        }
    }
    /// body of file, message, or data stream.
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct BlockSequence {
        #[prost(uint32, tag = "2")]
        pub seqnum: u32,
        #[prost(bool, tag = "3")]
        pub end: bool,
        #[prost(oneof = "block_sequence::Data", tags = "4, 5")]
        pub data: ::core::option::Option<block_sequence::Data>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for BlockSequence {
        #[inline]
        fn clone(&self) -> BlockSequence {
            BlockSequence {
                seqnum: ::core::clone::Clone::clone(&self.seqnum),
                end: ::core::clone::Clone::clone(&self.end),
                data: ::core::clone::Clone::clone(&self.data),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for BlockSequence {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for BlockSequence {
        #[inline]
        fn eq(&self, other: &BlockSequence) -> bool {
            self.seqnum == other.seqnum && self.end == other.end
                && self.data == other.data
        }
    }
    impl ::prost::Message for BlockSequence {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.seqnum != 0u32 {
                ::prost::encoding::uint32::encode(2u32, &self.seqnum, buf);
            }
            if self.end != false {
                ::prost::encoding::bool::encode(3u32, &self.end, buf);
            }
            if let Some(ref oneof) = self.data {
                oneof.encode(buf)
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "BlockSequence";
            match tag {
                2u32 => {
                    let mut value = &mut self.seqnum;
                    ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "seqnum");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.end;
                    ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "end");
                            error
                        })
                }
                4u32 | 5u32 => {
                    let mut value = &mut self.data;
                    block_sequence::Data::merge(value, tag, wire_type, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "data");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.seqnum != 0u32 {
                    ::prost::encoding::uint32::encoded_len(2u32, &self.seqnum)
                } else {
                    0
                }
                + if self.end != false {
                    ::prost::encoding::bool::encoded_len(3u32, &self.end)
                } else {
                    0
                } + self.data.as_ref().map_or(0, block_sequence::Data::encoded_len)
        }
        fn clear(&mut self) {
            self.seqnum = 0u32;
            self.end = false;
            self.data = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for BlockSequence {
        fn default() -> Self {
            BlockSequence {
                seqnum: 0u32,
                end: false,
                data: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for BlockSequence {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("BlockSequence");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.seqnum)
                };
                builder.field("seqnum", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.end)
                };
                builder.field("end", &wrapper)
            };
            let builder = {
                let wrapper = &self.data;
                builder.field("data", &wrapper)
            };
            builder.finish()
        }
    }
    /// Nested message and enum types in `BlockSequence`.
    pub mod block_sequence {
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub enum Data {
            #[prost(bytes, tag = "4")]
            DataContents(::prost::alloc::vec::Vec<u8>),
            #[prost(bool, tag = "5")]
            DataNative(bool),
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Data {
            #[inline]
            fn clone(&self) -> Data {
                match self {
                    Data::DataContents(__self_0) => {
                        Data::DataContents(::core::clone::Clone::clone(__self_0))
                    }
                    Data::DataNative(__self_0) => {
                        Data::DataNative(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for Data {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Data {
            #[inline]
            fn eq(&self, other: &Data) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (Data::DataContents(__self_0), Data::DataContents(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Data::DataNative(__self_0), Data::DataNative(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        impl Data {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    Data::DataContents(ref value) => {
                        ::prost::encoding::bytes::encode(4u32, &*value, buf);
                    }
                    Data::DataNative(ref value) => {
                        ::prost::encoding::bool::encode(5u32, &*value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<Data>,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    4u32 => {
                        match field {
                            ::core::option::Option::Some(
                                Data::DataContents(ref mut value),
                            ) => {
                                ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            Data::DataContents(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    5u32 => {
                        match field {
                            ::core::option::Option::Some(
                                Data::DataNative(ref mut value),
                            ) => {
                                ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            Data::DataNative(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("invalid Data tag: {0}", tag),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    Data::DataContents(ref value) => {
                        ::prost::encoding::bytes::encoded_len(4u32, &*value)
                    }
                    Data::DataNative(ref value) => {
                        ::prost::encoding::bool::encoded_len(5u32, &*value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for Data {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Data::DataContents(ref value) => {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&*value)
                        };
                        f.debug_tuple("DataContents").field(&wrapper).finish()
                    }
                    Data::DataNative(ref value) => {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&*value)
                        };
                        f.debug_tuple("DataNative").field(&wrapper).finish()
                    }
                }
            }
        }
    }
    /// For declaring local router identity.
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct Luid {
        #[prost(oneof = "luid::Val", tags = "4, 5")]
        pub val: ::core::option::Option<luid::Val>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for Luid {
        #[inline]
        fn clone(&self) -> Luid {
            Luid {
                val: ::core::clone::Clone::clone(&self.val),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for Luid {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for Luid {
        #[inline]
        fn eq(&self, other: &Luid) -> bool {
            self.val == other.val
        }
    }
    impl ::prost::Message for Luid {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref oneof) = self.val {
                oneof.encode(buf)
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "Luid";
            match tag {
                4u32 | 5u32 => {
                    let mut value = &mut self.val;
                    luid::Val::merge(value, tag, wire_type, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "val");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.val.as_ref().map_or(0, luid::Val::encoded_len)
        }
        fn clear(&mut self) {
            self.val = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for Luid {
        fn default() -> Self {
            Luid {
                val: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for Luid {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Luid");
            let builder = {
                let wrapper = &self.val;
                builder.field("val", &wrapper)
            };
            builder.finish()
        }
    }
    /// Nested message and enum types in `Luid`.
    pub mod luid {
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct Hashed {
            #[prost(uint32, tag = "2")]
            pub protoversion: u32,
            #[prost(bytes = "vec", tag = "3")]
            pub hash: ::prost::alloc::vec::Vec<u8>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Hashed {
            #[inline]
            fn clone(&self) -> Hashed {
                Hashed {
                    protoversion: ::core::clone::Clone::clone(&self.protoversion),
                    hash: ::core::clone::Clone::clone(&self.hash),
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for Hashed {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Hashed {
            #[inline]
            fn eq(&self, other: &Hashed) -> bool {
                self.protoversion == other.protoversion && self.hash == other.hash
            }
        }
        impl ::prost::Message for Hashed {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if self.protoversion != 0u32 {
                    ::prost::encoding::uint32::encode(2u32, &self.protoversion, buf);
                }
                if self.hash != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(3u32, &self.hash, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "Hashed";
                match tag {
                    2u32 => {
                        let mut value = &mut self.protoversion;
                        ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "protoversion");
                                error
                            })
                    }
                    3u32 => {
                        let mut value = &mut self.hash;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "hash");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.protoversion != 0u32 {
                        ::prost::encoding::uint32::encoded_len(2u32, &self.protoversion)
                    } else {
                        0
                    }
                    + if self.hash != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(3u32, &self.hash)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.protoversion = 0u32;
                self.hash.clear();
            }
        }
        impl ::core::default::Default for Hashed {
            fn default() -> Self {
                Hashed {
                    protoversion: 0u32,
                    hash: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for Hashed {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Hashed");
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.protoversion)
                    };
                    builder.field("protoversion", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.hash)
                    };
                    builder.field("hash", &wrapper)
                };
                builder.finish()
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub enum Val {
            #[prost(message, tag = "4")]
            ValUuid(super::ProtoUuid),
            #[prost(message, tag = "5")]
            ValHash(Hashed),
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Val {
            #[inline]
            fn clone(&self) -> Val {
                match self {
                    Val::ValUuid(__self_0) => {
                        Val::ValUuid(::core::clone::Clone::clone(__self_0))
                    }
                    Val::ValHash(__self_0) => {
                        Val::ValHash(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for Val {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Val {
            #[inline]
            fn eq(&self, other: &Val) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (Val::ValUuid(__self_0), Val::ValUuid(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Val::ValHash(__self_0), Val::ValHash(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        impl Val {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    Val::ValUuid(ref value) => {
                        ::prost::encoding::message::encode(4u32, &*value, buf);
                    }
                    Val::ValHash(ref value) => {
                        ::prost::encoding::message::encode(5u32, &*value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<Val>,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    4u32 => {
                        match field {
                            ::core::option::Option::Some(Val::ValUuid(ref mut value)) => {
                                ::prost::encoding::message::merge(
                                    wire_type,
                                    value,
                                    buf,
                                    ctx,
                                )
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            Val::ValUuid(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    5u32 => {
                        match field {
                            ::core::option::Option::Some(Val::ValHash(ref mut value)) => {
                                ::prost::encoding::message::merge(
                                    wire_type,
                                    value,
                                    buf,
                                    ctx,
                                )
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            Val::ValHash(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("invalid Val tag: {0}", tag),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    Val::ValUuid(ref value) => {
                        ::prost::encoding::message::encoded_len(4u32, &*value)
                    }
                    Val::ValHash(ref value) => {
                        ::prost::encoding::message::encoded_len(5u32, &*value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for Val {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Val::ValUuid(ref value) => {
                        let wrapper = &*value;
                        f.debug_tuple("ValUuid").field(&wrapper).finish()
                    }
                    Val::ValHash(ref value) => {
                        let wrapper = &*value;
                        f.debug_tuple("ValHash").field(&wrapper).finish()
                    }
                }
            }
        }
    }
    /// For gossiping about wifi direct "ukes" in area. Currently unused
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct ExtraUke {
        #[prost(message, optional, tag = "2")]
        pub luid: ::core::option::Option<ProtoUuid>,
        #[prost(message, optional, tag = "3")]
        pub upgrade: ::core::option::Option<Upgrade>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for ExtraUke {
        #[inline]
        fn clone(&self) -> ExtraUke {
            ExtraUke {
                luid: ::core::clone::Clone::clone(&self.luid),
                upgrade: ::core::clone::Clone::clone(&self.upgrade),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for ExtraUke {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for ExtraUke {
        #[inline]
        fn eq(&self, other: &ExtraUke) -> bool {
            self.luid == other.luid && self.upgrade == other.upgrade
        }
    }
    impl ::prost::Message for ExtraUke {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.luid {
                ::prost::encoding::message::encode(2u32, msg, buf);
            }
            if let Some(ref msg) = self.upgrade {
                ::prost::encoding::message::encode(3u32, msg, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "ExtraUke";
            match tag {
                2u32 => {
                    let mut value = &mut self.luid;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "luid");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.upgrade;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "upgrade");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .luid
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(2u32, msg))
                + self
                    .upgrade
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(3u32, msg))
        }
        fn clear(&mut self) {
            self.luid = ::core::option::Option::None;
            self.upgrade = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for ExtraUke {
        fn default() -> Self {
            ExtraUke {
                luid: ::core::default::Default::default(),
                upgrade: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for ExtraUke {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("ExtraUke");
            let builder = {
                let wrapper = &self.luid;
                builder.field("luid", &wrapper)
            };
            let builder = {
                let wrapper = &self.upgrade;
                builder.field("upgrade", &wrapper)
            };
            builder.finish()
        }
    }
    /// Packet for voting in distributed elections. Used for bootstrapping
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct ElectLeader {
        #[prost(message, optional, tag = "2")]
        pub sender: ::core::option::Option<ProtoUuid>,
        #[prost(oneof = "elect_leader::Val", tags = "7, 8")]
        pub val: ::core::option::Option<elect_leader::Val>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for ElectLeader {
        #[inline]
        fn clone(&self) -> ElectLeader {
            ElectLeader {
                sender: ::core::clone::Clone::clone(&self.sender),
                val: ::core::clone::Clone::clone(&self.val),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for ElectLeader {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for ElectLeader {
        #[inline]
        fn eq(&self, other: &ElectLeader) -> bool {
            self.sender == other.sender && self.val == other.val
        }
    }
    impl ::prost::Message for ElectLeader {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.sender {
                ::prost::encoding::message::encode(2u32, msg, buf);
            }
            if let Some(ref oneof) = self.val {
                oneof.encode(buf)
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "ElectLeader";
            match tag {
                2u32 => {
                    let mut value = &mut self.sender;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "sender");
                            error
                        })
                }
                7u32 | 8u32 => {
                    let mut value = &mut self.val;
                    elect_leader::Val::merge(value, tag, wire_type, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "val");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .sender
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(2u32, msg))
                + self.val.as_ref().map_or(0, elect_leader::Val::encoded_len)
        }
        fn clear(&mut self) {
            self.sender = ::core::option::Option::None;
            self.val = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for ElectLeader {
        fn default() -> Self {
            ElectLeader {
                sender: ::core::default::Default::default(),
                val: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for ElectLeader {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("ElectLeader");
            let builder = {
                let wrapper = &self.sender;
                builder.field("sender", &wrapper)
            };
            let builder = {
                let wrapper = &self.val;
                builder.field("val", &wrapper)
            };
            builder.finish()
        }
    }
    /// Nested message and enum types in `ElectLeader`.
    pub mod elect_leader {
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct Body {
            #[prost(bytes = "vec", tag = "3")]
            pub salt: ::prost::alloc::vec::Vec<u8>,
            #[prost(uint32, tag = "4")]
            pub provides: u32,
            #[prost(message, optional, tag = "5")]
            pub tiebreaker_val: ::core::option::Option<super::ProtoUuid>,
            #[prost(oneof = "body::MaybeUpgrade", tags = "6")]
            pub maybe_upgrade: ::core::option::Option<body::MaybeUpgrade>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Body {
            #[inline]
            fn clone(&self) -> Body {
                Body {
                    salt: ::core::clone::Clone::clone(&self.salt),
                    provides: ::core::clone::Clone::clone(&self.provides),
                    tiebreaker_val: ::core::clone::Clone::clone(&self.tiebreaker_val),
                    maybe_upgrade: ::core::clone::Clone::clone(&self.maybe_upgrade),
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for Body {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Body {
            #[inline]
            fn eq(&self, other: &Body) -> bool {
                self.salt == other.salt && self.provides == other.provides
                    && self.tiebreaker_val == other.tiebreaker_val
                    && self.maybe_upgrade == other.maybe_upgrade
            }
        }
        impl ::prost::Message for Body {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if self.salt != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(3u32, &self.salt, buf);
                }
                if self.provides != 0u32 {
                    ::prost::encoding::uint32::encode(4u32, &self.provides, buf);
                }
                if let Some(ref msg) = self.tiebreaker_val {
                    ::prost::encoding::message::encode(5u32, msg, buf);
                }
                if let Some(ref oneof) = self.maybe_upgrade {
                    oneof.encode(buf)
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "Body";
                match tag {
                    3u32 => {
                        let mut value = &mut self.salt;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "salt");
                                error
                            })
                    }
                    4u32 => {
                        let mut value = &mut self.provides;
                        ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "provides");
                                error
                            })
                    }
                    5u32 => {
                        let mut value = &mut self.tiebreaker_val;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "tiebreaker_val");
                                error
                            })
                    }
                    6u32 => {
                        let mut value = &mut self.maybe_upgrade;
                        body::MaybeUpgrade::merge(value, tag, wire_type, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "maybe_upgrade");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.salt != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(3u32, &self.salt)
                    } else {
                        0
                    }
                    + if self.provides != 0u32 {
                        ::prost::encoding::uint32::encoded_len(4u32, &self.provides)
                    } else {
                        0
                    }
                    + self
                        .tiebreaker_val
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(5u32, msg),
                        )
                    + self
                        .maybe_upgrade
                        .as_ref()
                        .map_or(0, body::MaybeUpgrade::encoded_len)
            }
            fn clear(&mut self) {
                self.salt.clear();
                self.provides = 0u32;
                self.tiebreaker_val = ::core::option::Option::None;
                self.maybe_upgrade = ::core::option::Option::None;
            }
        }
        impl ::core::default::Default for Body {
            fn default() -> Self {
                Body {
                    salt: ::core::default::Default::default(),
                    provides: 0u32,
                    tiebreaker_val: ::core::default::Default::default(),
                    maybe_upgrade: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for Body {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Body");
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.salt)
                    };
                    builder.field("salt", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.provides)
                    };
                    builder.field("provides", &wrapper)
                };
                let builder = {
                    let wrapper = &self.tiebreaker_val;
                    builder.field("tiebreaker_val", &wrapper)
                };
                let builder = {
                    let wrapper = &self.maybe_upgrade;
                    builder.field("maybe_upgrade", &wrapper)
                };
                builder.finish()
            }
        }
        /// Nested message and enum types in `Body`.
        pub mod body {
            #[allow(clippy::derive_partial_eq_without_eq)]
            pub enum MaybeUpgrade {
                #[prost(message, tag = "6")]
                Upgrade(super::super::Upgrade),
            }
            #[automatically_derived]
            #[allow(clippy::derive_partial_eq_without_eq)]
            impl ::core::clone::Clone for MaybeUpgrade {
                #[inline]
                fn clone(&self) -> MaybeUpgrade {
                    match self {
                        MaybeUpgrade::Upgrade(__self_0) => {
                            MaybeUpgrade::Upgrade(::core::clone::Clone::clone(__self_0))
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(clippy::derive_partial_eq_without_eq)]
            impl ::core::marker::StructuralPartialEq for MaybeUpgrade {}
            #[automatically_derived]
            #[allow(clippy::derive_partial_eq_without_eq)]
            impl ::core::cmp::PartialEq for MaybeUpgrade {
                #[inline]
                fn eq(&self, other: &MaybeUpgrade) -> bool {
                    match (self, other) {
                        (
                            MaybeUpgrade::Upgrade(__self_0),
                            MaybeUpgrade::Upgrade(__arg1_0),
                        ) => __self_0 == __arg1_0,
                    }
                }
            }
            impl MaybeUpgrade {
                /// Encodes the message to a buffer.
                pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                    match *self {
                        MaybeUpgrade::Upgrade(ref value) => {
                            ::prost::encoding::message::encode(6u32, &*value, buf);
                        }
                    }
                }
                /// Decodes an instance of the message from a buffer, and merges it into self.
                pub fn merge(
                    field: &mut ::core::option::Option<MaybeUpgrade>,
                    tag: u32,
                    wire_type: ::prost::encoding::WireType,
                    buf: &mut impl ::prost::bytes::Buf,
                    ctx: ::prost::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::prost::DecodeError> {
                    match tag {
                        6u32 => {
                            match field {
                                ::core::option::Option::Some(
                                    MaybeUpgrade::Upgrade(ref mut value),
                                ) => {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                }
                                _ => {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                MaybeUpgrade::Upgrade(owned_value),
                                            );
                                        })
                                }
                            }
                        }
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "internal error: entered unreachable code: {0}",
                                    format_args!("invalid MaybeUpgrade tag: {0}", tag),
                                ),
                            );
                        }
                    }
                }
                /// Returns the encoded length of the message without a length delimiter.
                #[inline]
                pub fn encoded_len(&self) -> usize {
                    match *self {
                        MaybeUpgrade::Upgrade(ref value) => {
                            ::prost::encoding::message::encoded_len(6u32, &*value)
                        }
                    }
                }
            }
            impl ::core::fmt::Debug for MaybeUpgrade {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        MaybeUpgrade::Upgrade(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("Upgrade").field(&wrapper).finish()
                        }
                    }
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub enum Val {
            #[prost(message, tag = "7")]
            ValBody(Body),
            #[prost(bytes, tag = "8")]
            ValHash(::prost::alloc::vec::Vec<u8>),
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Val {
            #[inline]
            fn clone(&self) -> Val {
                match self {
                    Val::ValBody(__self_0) => {
                        Val::ValBody(::core::clone::Clone::clone(__self_0))
                    }
                    Val::ValHash(__self_0) => {
                        Val::ValHash(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for Val {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Val {
            #[inline]
            fn eq(&self, other: &Val) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (Val::ValBody(__self_0), Val::ValBody(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Val::ValHash(__self_0), Val::ValHash(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        impl Val {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    Val::ValBody(ref value) => {
                        ::prost::encoding::message::encode(7u32, &*value, buf);
                    }
                    Val::ValHash(ref value) => {
                        ::prost::encoding::bytes::encode(8u32, &*value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<Val>,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    7u32 => {
                        match field {
                            ::core::option::Option::Some(Val::ValBody(ref mut value)) => {
                                ::prost::encoding::message::merge(
                                    wire_type,
                                    value,
                                    buf,
                                    ctx,
                                )
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            Val::ValBody(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    8u32 => {
                        match field {
                            ::core::option::Option::Some(Val::ValHash(ref mut value)) => {
                                ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            Val::ValHash(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("invalid Val tag: {0}", tag),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    Val::ValBody(ref value) => {
                        ::prost::encoding::message::encoded_len(7u32, &*value)
                    }
                    Val::ValHash(ref value) => {
                        ::prost::encoding::bytes::encoded_len(8u32, &*value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for Val {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Val::ValBody(ref value) => {
                        let wrapper = &*value;
                        f.debug_tuple("ValBody").field(&wrapper).finish()
                    }
                    Val::ValHash(ref value) => {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&*value)
                        };
                        f.debug_tuple("ValHash").field(&wrapper).finish()
                    }
                }
            }
        }
    }
    /// used to request switching to a different physical transport
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct Upgrade {
        #[prost(uint32, tag = "2")]
        pub provides: u32,
        #[prost(uint32, tag = "3")]
        pub sessionid: u32,
        #[prost(map = "string, string", tag = "4")]
        pub metadata: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        #[prost(message, optional, tag = "5")]
        pub from: ::core::option::Option<ProtoUuid>,
        #[prost(enumeration = "Role", tag = "6")]
        pub role: i32,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for Upgrade {
        #[inline]
        fn clone(&self) -> Upgrade {
            Upgrade {
                provides: ::core::clone::Clone::clone(&self.provides),
                sessionid: ::core::clone::Clone::clone(&self.sessionid),
                metadata: ::core::clone::Clone::clone(&self.metadata),
                from: ::core::clone::Clone::clone(&self.from),
                role: ::core::clone::Clone::clone(&self.role),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for Upgrade {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for Upgrade {
        #[inline]
        fn eq(&self, other: &Upgrade) -> bool {
            self.provides == other.provides && self.sessionid == other.sessionid
                && self.metadata == other.metadata && self.from == other.from
                && self.role == other.role
        }
    }
    impl ::prost::Message for Upgrade {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.provides != 0u32 {
                ::prost::encoding::uint32::encode(2u32, &self.provides, buf);
            }
            if self.sessionid != 0u32 {
                ::prost::encoding::uint32::encode(3u32, &self.sessionid, buf);
            }
            ::prost::encoding::hash_map::encode(
                ::prost::encoding::string::encode,
                ::prost::encoding::string::encoded_len,
                ::prost::encoding::string::encode,
                ::prost::encoding::string::encoded_len,
                4u32,
                &self.metadata,
                buf,
            );
            if let Some(ref msg) = self.from {
                ::prost::encoding::message::encode(5u32, msg, buf);
            }
            if self.role != Role::default() as i32 {
                ::prost::encoding::int32::encode(6u32, &self.role, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "Upgrade";
            match tag {
                2u32 => {
                    let mut value = &mut self.provides;
                    ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "provides");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.sessionid;
                    ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "sessionid");
                            error
                        })
                }
                4u32 => {
                    let mut value = &mut self.metadata;
                    ::prost::encoding::hash_map::merge(
                            ::prost::encoding::string::merge,
                            ::prost::encoding::string::merge,
                            &mut value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "metadata");
                            error
                        })
                }
                5u32 => {
                    let mut value = &mut self.from;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "from");
                            error
                        })
                }
                6u32 => {
                    let mut value = &mut self.role;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "role");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.provides != 0u32 {
                    ::prost::encoding::uint32::encoded_len(2u32, &self.provides)
                } else {
                    0
                }
                + if self.sessionid != 0u32 {
                    ::prost::encoding::uint32::encoded_len(3u32, &self.sessionid)
                } else {
                    0
                }
                + ::prost::encoding::hash_map::encoded_len(
                    ::prost::encoding::string::encoded_len,
                    ::prost::encoding::string::encoded_len,
                    4u32,
                    &self.metadata,
                )
                + self
                    .from
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(5u32, msg))
                + if self.role != Role::default() as i32 {
                    ::prost::encoding::int32::encoded_len(6u32, &self.role)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.provides = 0u32;
            self.sessionid = 0u32;
            self.metadata.clear();
            self.from = ::core::option::Option::None;
            self.role = Role::default() as i32;
        }
    }
    impl ::core::default::Default for Upgrade {
        fn default() -> Self {
            Upgrade {
                provides: 0u32,
                sessionid: 0u32,
                metadata: ::core::default::Default::default(),
                from: ::core::default::Default::default(),
                role: Role::default() as i32,
            }
        }
    }
    impl ::core::fmt::Debug for Upgrade {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Upgrade");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.provides)
                };
                builder.field("provides", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.sessionid)
                };
                builder.field("sessionid", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct MapWrapper<'a>(
                        &'a ::std::collections::HashMap<
                            ::prost::alloc::string::String,
                            ::prost::alloc::string::String,
                        >,
                    );
                    impl<'a> ::core::fmt::Debug for MapWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            #[allow(non_snake_case)]
                            fn KeyWrapper<T>(v: T) -> T {
                                v
                            }
                            #[allow(non_snake_case)]
                            fn ValueWrapper<T>(v: T) -> T {
                                v
                            }
                            let mut builder = f.debug_map();
                            for (k, v) in self.0 {
                                builder.entry(&KeyWrapper(k), &ValueWrapper(v));
                            }
                            builder.finish()
                        }
                    }
                    MapWrapper(&self.metadata)
                };
                builder.field("metadata", &wrapper)
            };
            let builder = {
                let wrapper = &self.from;
                builder.field("from", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a i32);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let res: ::core::result::Result<Role, _> = ::core::convert::TryFrom::try_from(
                                *self.0,
                            );
                            match res {
                                Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                            }
                        }
                    }
                    ScalarWrapper(&self.role)
                };
                builder.field("role", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl Upgrade {
        ///Returns the enum value of `role`, or the default if the field is set to an invalid enum value.
        pub fn role(&self) -> Role {
            ::core::convert::TryFrom::try_from(self.role).unwrap_or(Role::default())
        }
        ///Sets `role` to the provided enum value.
        pub fn set_role(&mut self, value: Role) {
            self.role = value as i32;
        }
    }
    /// a user identity with name and one or more keys
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct Identity {
        #[prost(oneof = "identity::Message", tags = "5, 6")]
        pub message: ::core::option::Option<identity::Message>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for Identity {
        #[inline]
        fn clone(&self) -> Identity {
            Identity {
                message: ::core::clone::Clone::clone(&self.message),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for Identity {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for Identity {
        #[inline]
        fn eq(&self, other: &Identity) -> bool {
            self.message == other.message
        }
    }
    impl ::prost::Message for Identity {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref oneof) = self.message {
                oneof.encode(buf)
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "Identity";
            match tag {
                5u32 | 6u32 => {
                    let mut value = &mut self.message;
                    identity::Message::merge(value, tag, wire_type, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "message");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.message.as_ref().map_or(0, identity::Message::encoded_len)
        }
        fn clear(&mut self) {
            self.message = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for Identity {
        fn default() -> Self {
            Identity {
                message: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for Identity {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Identity");
            let builder = {
                let wrapper = &self.message;
                builder.field("message", &wrapper)
            };
            builder.finish()
        }
    }
    /// Nested message and enum types in `Identity`.
    pub mod identity {
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct Body {
            #[prost(string, tag = "2")]
            pub givenname: ::prost::alloc::string::String,
            #[prost(map = "string, bytes", tag = "3")]
            pub keys: ::std::collections::HashMap<
                ::prost::alloc::string::String,
                ::prost::alloc::vec::Vec<u8>,
            >,
            #[prost(bytes = "vec", tag = "4")]
            pub sig: ::prost::alloc::vec::Vec<u8>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Body {
            #[inline]
            fn clone(&self) -> Body {
                Body {
                    givenname: ::core::clone::Clone::clone(&self.givenname),
                    keys: ::core::clone::Clone::clone(&self.keys),
                    sig: ::core::clone::Clone::clone(&self.sig),
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for Body {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Body {
            #[inline]
            fn eq(&self, other: &Body) -> bool {
                self.givenname == other.givenname && self.keys == other.keys
                    && self.sig == other.sig
            }
        }
        impl ::prost::Message for Body {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if self.givenname != "" {
                    ::prost::encoding::string::encode(2u32, &self.givenname, buf);
                }
                ::prost::encoding::hash_map::encode(
                    ::prost::encoding::string::encode,
                    ::prost::encoding::string::encoded_len,
                    ::prost::encoding::bytes::encode,
                    ::prost::encoding::bytes::encoded_len,
                    3u32,
                    &self.keys,
                    buf,
                );
                if self.sig != b"" as &[u8] {
                    ::prost::encoding::bytes::encode(4u32, &self.sig, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "Body";
                match tag {
                    2u32 => {
                        let mut value = &mut self.givenname;
                        ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "givenname");
                                error
                            })
                    }
                    3u32 => {
                        let mut value = &mut self.keys;
                        ::prost::encoding::hash_map::merge(
                                ::prost::encoding::string::merge,
                                ::prost::encoding::bytes::merge,
                                &mut value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "keys");
                                error
                            })
                    }
                    4u32 => {
                        let mut value = &mut self.sig;
                        ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "sig");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.givenname != "" {
                        ::prost::encoding::string::encoded_len(2u32, &self.givenname)
                    } else {
                        0
                    }
                    + ::prost::encoding::hash_map::encoded_len(
                        ::prost::encoding::string::encoded_len,
                        ::prost::encoding::bytes::encoded_len,
                        3u32,
                        &self.keys,
                    )
                    + if self.sig != b"" as &[u8] {
                        ::prost::encoding::bytes::encoded_len(4u32, &self.sig)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.givenname.clear();
                self.keys.clear();
                self.sig.clear();
            }
        }
        impl ::core::default::Default for Body {
            fn default() -> Self {
                Body {
                    givenname: ::prost::alloc::string::String::new(),
                    keys: ::core::default::Default::default(),
                    sig: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for Body {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Body");
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.givenname)
                    };
                    builder.field("givenname", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        struct MapWrapper<'a>(&'a dyn ::core::fmt::Debug);
                        impl<'a> ::core::fmt::Debug for MapWrapper<'a> {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                self.0.fmt(f)
                            }
                        }
                        MapWrapper(&self.keys)
                    };
                    builder.field("keys", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.sig)
                    };
                    builder.field("sig", &wrapper)
                };
                builder.finish()
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub enum Message {
            #[prost(bool, tag = "5")]
            End(bool),
            #[prost(message, tag = "6")]
            Val(Body),
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Message {
            #[inline]
            fn clone(&self) -> Message {
                match self {
                    Message::End(__self_0) => {
                        Message::End(::core::clone::Clone::clone(__self_0))
                    }
                    Message::Val(__self_0) => {
                        Message::Val(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for Message {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Message {
            #[inline]
            fn eq(&self, other: &Message) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (Message::End(__self_0), Message::End(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Message::Val(__self_0), Message::Val(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        impl Message {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    Message::End(ref value) => {
                        ::prost::encoding::bool::encode(5u32, &*value, buf);
                    }
                    Message::Val(ref value) => {
                        ::prost::encoding::message::encode(6u32, &*value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<Message>,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    5u32 => {
                        match field {
                            ::core::option::Option::Some(Message::End(ref mut value)) => {
                                ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            Message::End(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    6u32 => {
                        match field {
                            ::core::option::Option::Some(Message::Val(ref mut value)) => {
                                ::prost::encoding::message::merge(
                                    wire_type,
                                    value,
                                    buf,
                                    ctx,
                                )
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            Message::Val(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("invalid Message tag: {0}", tag),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    Message::End(ref value) => {
                        ::prost::encoding::bool::encoded_len(5u32, &*value)
                    }
                    Message::Val(ref value) => {
                        ::prost::encoding::message::encoded_len(6u32, &*value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for Message {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Message::End(ref value) => {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&*value)
                        };
                        f.debug_tuple("End").field(&wrapper).finish()
                    }
                    Message::Val(ref value) => {
                        let wrapper = &*value;
                        f.debug_tuple("Val").field(&wrapper).finish()
                    }
                }
            }
        }
    }
    /// Currently unused, part of old "gossip" based wifi direct bootstrap method
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct JustUkes {
        #[prost(message, repeated, tag = "1")]
        pub ukes: ::prost::alloc::vec::Vec<ExtraUke>,
        #[prost(bool, tag = "2")]
        pub too_small: bool,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for JustUkes {
        #[inline]
        fn clone(&self) -> JustUkes {
            JustUkes {
                ukes: ::core::clone::Clone::clone(&self.ukes),
                too_small: ::core::clone::Clone::clone(&self.too_small),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for JustUkes {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for JustUkes {
        #[inline]
        fn eq(&self, other: &JustUkes) -> bool {
            self.ukes == other.ukes && self.too_small == other.too_small
        }
    }
    impl ::prost::Message for JustUkes {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            for msg in &self.ukes {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            if self.too_small != false {
                ::prost::encoding::bool::encode(2u32, &self.too_small, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "JustUkes";
            match tag {
                1u32 => {
                    let mut value = &mut self.ukes;
                    ::prost::encoding::message::merge_repeated(
                            wire_type,
                            value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "ukes");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.too_small;
                    ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "too_small");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::prost::encoding::message::encoded_len_repeated(1u32, &self.ukes)
                + if self.too_small != false {
                    ::prost::encoding::bool::encoded_len(2u32, &self.too_small)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.ukes.clear();
            self.too_small = false;
        }
    }
    impl ::core::default::Default for JustUkes {
        fn default() -> Self {
            JustUkes {
                ukes: ::core::default::Default::default(),
                too_small: false,
            }
        }
    }
    impl ::core::fmt::Debug for JustUkes {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("JustUkes");
            let builder = {
                let wrapper = &self.ukes;
                builder.field("ukes", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.too_small)
                };
                builder.field("too_small", &wrapper)
            };
            builder.finish()
        }
    }
    /// arbitrary metadata about the network
    /// used for building more complex routing algorithms someday
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct RoutingMetadata {
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<ProtoUuid>,
        #[prost(bool, tag = "2")]
        pub endofstream: bool,
        #[prost(map = "string, bytes", tag = "3")]
        pub keyval: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::vec::Vec<u8>,
        >,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for RoutingMetadata {
        #[inline]
        fn clone(&self) -> RoutingMetadata {
            RoutingMetadata {
                id: ::core::clone::Clone::clone(&self.id),
                endofstream: ::core::clone::Clone::clone(&self.endofstream),
                keyval: ::core::clone::Clone::clone(&self.keyval),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for RoutingMetadata {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for RoutingMetadata {
        #[inline]
        fn eq(&self, other: &RoutingMetadata) -> bool {
            self.id == other.id && self.endofstream == other.endofstream
                && self.keyval == other.keyval
        }
    }
    impl ::prost::Message for RoutingMetadata {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.id {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            if self.endofstream != false {
                ::prost::encoding::bool::encode(2u32, &self.endofstream, buf);
            }
            ::prost::encoding::hash_map::encode(
                ::prost::encoding::string::encode,
                ::prost::encoding::string::encoded_len,
                ::prost::encoding::bytes::encode,
                ::prost::encoding::bytes::encoded_len,
                3u32,
                &self.keyval,
                buf,
            );
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "RoutingMetadata";
            match tag {
                1u32 => {
                    let mut value = &mut self.id;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "id");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.endofstream;
                    ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "endofstream");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.keyval;
                    ::prost::encoding::hash_map::merge(
                            ::prost::encoding::string::merge,
                            ::prost::encoding::bytes::merge,
                            &mut value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "keyval");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .id
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + if self.endofstream != false {
                    ::prost::encoding::bool::encoded_len(2u32, &self.endofstream)
                } else {
                    0
                }
                + ::prost::encoding::hash_map::encoded_len(
                    ::prost::encoding::string::encoded_len,
                    ::prost::encoding::bytes::encoded_len,
                    3u32,
                    &self.keyval,
                )
        }
        fn clear(&mut self) {
            self.id = ::core::option::Option::None;
            self.endofstream = false;
            self.keyval.clear();
        }
    }
    impl ::core::default::Default for RoutingMetadata {
        fn default() -> Self {
            RoutingMetadata {
                id: ::core::default::Default::default(),
                endofstream: false,
                keyval: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for RoutingMetadata {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("RoutingMetadata");
            let builder = {
                let wrapper = &self.id;
                builder.field("id", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.endofstream)
                };
                builder.field("endofstream", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct MapWrapper<'a>(&'a dyn ::core::fmt::Debug);
                    impl<'a> ::core::fmt::Debug for MapWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            self.0.fmt(f)
                        }
                    }
                    MapWrapper(&self.keyval)
                };
                builder.field("keyval", &wrapper)
            };
            builder.finish()
        }
    }
    /// Used for informing new wifi direct peers about neighbors
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct IpAnnounceItem {
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<ProtoUuid>,
        #[prost(string, tag = "2")]
        pub address: ::prost::alloc::string::String,
        #[prost(uint32, tag = "3")]
        pub port: u32,
        #[prost(bool, tag = "4")]
        pub uke: bool,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for IpAnnounceItem {
        #[inline]
        fn clone(&self) -> IpAnnounceItem {
            IpAnnounceItem {
                id: ::core::clone::Clone::clone(&self.id),
                address: ::core::clone::Clone::clone(&self.address),
                port: ::core::clone::Clone::clone(&self.port),
                uke: ::core::clone::Clone::clone(&self.uke),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for IpAnnounceItem {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for IpAnnounceItem {
        #[inline]
        fn eq(&self, other: &IpAnnounceItem) -> bool {
            self.id == other.id && self.address == other.address
                && self.port == other.port && self.uke == other.uke
        }
    }
    impl ::prost::Message for IpAnnounceItem {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.id {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            if self.address != "" {
                ::prost::encoding::string::encode(2u32, &self.address, buf);
            }
            if self.port != 0u32 {
                ::prost::encoding::uint32::encode(3u32, &self.port, buf);
            }
            if self.uke != false {
                ::prost::encoding::bool::encode(4u32, &self.uke, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "IpAnnounceItem";
            match tag {
                1u32 => {
                    let mut value = &mut self.id;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "id");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.address;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "address");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.port;
                    ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "port");
                            error
                        })
                }
                4u32 => {
                    let mut value = &mut self.uke;
                    ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "uke");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .id
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + if self.address != "" {
                    ::prost::encoding::string::encoded_len(2u32, &self.address)
                } else {
                    0
                }
                + if self.port != 0u32 {
                    ::prost::encoding::uint32::encoded_len(3u32, &self.port)
                } else {
                    0
                }
                + if self.uke != false {
                    ::prost::encoding::bool::encoded_len(4u32, &self.uke)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.id = ::core::option::Option::None;
            self.address.clear();
            self.port = 0u32;
            self.uke = false;
        }
    }
    impl ::core::default::Default for IpAnnounceItem {
        fn default() -> Self {
            IpAnnounceItem {
                id: ::core::default::Default::default(),
                address: ::prost::alloc::string::String::new(),
                port: 0u32,
                uke: false,
            }
        }
    }
    impl ::core::fmt::Debug for IpAnnounceItem {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("IpAnnounceItem");
            let builder = {
                let wrapper = &self.id;
                builder.field("id", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.address)
                };
                builder.field("address", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.port)
                };
                builder.field("port", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.uke)
                };
                builder.field("uke", &wrapper)
            };
            builder.finish()
        }
    }
    /// Used for informing new wifi direct peers about neighbors
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct IpAnnounce {
        #[prost(message, optional, tag = "1")]
        pub self_: ::core::option::Option<ProtoUuid>,
        #[prost(message, repeated, tag = "2")]
        pub items: ::prost::alloc::vec::Vec<IpAnnounceItem>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for IpAnnounce {
        #[inline]
        fn clone(&self) -> IpAnnounce {
            IpAnnounce {
                self_: ::core::clone::Clone::clone(&self.self_),
                items: ::core::clone::Clone::clone(&self.items),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for IpAnnounce {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for IpAnnounce {
        #[inline]
        fn eq(&self, other: &IpAnnounce) -> bool {
            self.self_ == other.self_ && self.items == other.items
        }
    }
    impl ::prost::Message for IpAnnounce {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.self_ {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            for msg in &self.items {
                ::prost::encoding::message::encode(2u32, msg, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "IpAnnounce";
            match tag {
                1u32 => {
                    let mut value = &mut self.self_;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "self_");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.items;
                    ::prost::encoding::message::merge_repeated(
                            wire_type,
                            value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "items");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .self_
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + ::prost::encoding::message::encoded_len_repeated(2u32, &self.items)
        }
        fn clear(&mut self) {
            self.self_ = ::core::option::Option::None;
            self.items.clear();
        }
    }
    impl ::core::default::Default for IpAnnounce {
        fn default() -> Self {
            IpAnnounce {
                self_: ::core::default::Default::default(),
                items: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for IpAnnounce {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("IpAnnounce");
            let builder = {
                let wrapper = &self.self_;
                builder.field("self_", &wrapper)
            };
            let builder = {
                let wrapper = &self.items;
                builder.field("items", &wrapper)
            };
            builder.finish()
        }
    }
    /// Header for all messages sent in the context of a session
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct ApiHeader {
        /// ScatterType type = 1;
        #[prost(message, optional, tag = "1")]
        pub session: ::core::option::Option<ProtoUuid>,
        #[prost(oneof = "api_header::Stream", tags = "2")]
        pub stream: ::core::option::Option<api_header::Stream>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for ApiHeader {
        #[inline]
        fn clone(&self) -> ApiHeader {
            let _: ::core::clone::AssertParamIsClone<::core::option::Option<ProtoUuid>>;
            let _: ::core::clone::AssertParamIsClone<
                ::core::option::Option<api_header::Stream>,
            >;
            *self
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::Copy for ApiHeader {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for ApiHeader {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for ApiHeader {
        #[inline]
        fn eq(&self, other: &ApiHeader) -> bool {
            self.session == other.session && self.stream == other.stream
        }
    }
    impl ::prost::Message for ApiHeader {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.session {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            if let Some(ref oneof) = self.stream {
                oneof.encode(buf)
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "ApiHeader";
            match tag {
                1u32 => {
                    let mut value = &mut self.session;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "session");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.stream;
                    api_header::Stream::merge(value, tag, wire_type, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "stream");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .session
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + self.stream.as_ref().map_or(0, api_header::Stream::encoded_len)
        }
        fn clear(&mut self) {
            self.session = ::core::option::Option::None;
            self.stream = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for ApiHeader {
        fn default() -> Self {
            ApiHeader {
                session: ::core::default::Default::default(),
                stream: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for ApiHeader {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("ApiHeader");
            let builder = {
                let wrapper = &self.session;
                builder.field("session", &wrapper)
            };
            let builder = {
                let wrapper = &self.stream;
                builder.field("stream", &wrapper)
            };
            builder.finish()
        }
    }
    /// Nested message and enum types in `ApiHeader`.
    pub mod api_header {
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub enum Stream {
            #[prost(int32, tag = "2")]
            StreamId(i32),
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Stream {
            #[inline]
            fn clone(&self) -> Stream {
                let _: ::core::clone::AssertParamIsClone<i32>;
                *self
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::Copy for Stream {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for Stream {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Stream {
            #[inline]
            fn eq(&self, other: &Stream) -> bool {
                match (self, other) {
                    (Stream::StreamId(__self_0), Stream::StreamId(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                }
            }
        }
        impl Stream {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    Stream::StreamId(ref value) => {
                        ::prost::encoding::int32::encode(2u32, &*value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<Stream>,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    2u32 => {
                        match field {
                            ::core::option::Option::Some(
                                Stream::StreamId(ref mut value),
                            ) => {
                                ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            Stream::StreamId(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("invalid Stream tag: {0}", tag),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    Stream::StreamId(ref value) => {
                        ::prost::encoding::int32::encoded_len(2u32, &*value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for Stream {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Stream::StreamId(ref value) => {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&*value)
                        };
                        f.debug_tuple("StreamId").field(&wrapper).finish()
                    }
                }
            }
        }
    }
    /// Sent by a desktop app to initiate a pairing request, offering
    /// public key
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct PairingInitiate {
        #[prost(bytes = "vec", tag = "1")]
        pub pubkey: ::prost::alloc::vec::Vec<u8>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for PairingInitiate {
        #[inline]
        fn clone(&self) -> PairingInitiate {
            PairingInitiate {
                pubkey: ::core::clone::Clone::clone(&self.pubkey),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for PairingInitiate {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for PairingInitiate {
        #[inline]
        fn eq(&self, other: &PairingInitiate) -> bool {
            self.pubkey == other.pubkey
        }
    }
    impl ::prost::Message for PairingInitiate {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.pubkey != b"" as &[u8] {
                ::prost::encoding::bytes::encode(1u32, &self.pubkey, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "PairingInitiate";
            match tag {
                1u32 => {
                    let mut value = &mut self.pubkey;
                    ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "pubkey");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.pubkey != b"" as &[u8] {
                    ::prost::encoding::bytes::encoded_len(1u32, &self.pubkey)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.pubkey.clear();
        }
    }
    impl ::core::default::Default for PairingInitiate {
        fn default() -> Self {
            PairingInitiate {
                pubkey: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for PairingInitiate {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("PairingInitiate");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.pubkey)
                };
                builder.field("pubkey", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct PairingAck {
        #[prost(message, optional, tag = "1")]
        pub session: ::core::option::Option<ApiHeader>,
        #[prost(bytes = "vec", tag = "2")]
        pub pubkey: ::prost::alloc::vec::Vec<u8>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for PairingAck {
        #[inline]
        fn clone(&self) -> PairingAck {
            PairingAck {
                session: ::core::clone::Clone::clone(&self.session),
                pubkey: ::core::clone::Clone::clone(&self.pubkey),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for PairingAck {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for PairingAck {
        #[inline]
        fn eq(&self, other: &PairingAck) -> bool {
            self.session == other.session && self.pubkey == other.pubkey
        }
    }
    impl ::prost::Message for PairingAck {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.session {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            if self.pubkey != b"" as &[u8] {
                ::prost::encoding::bytes::encode(2u32, &self.pubkey, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "PairingAck";
            match tag {
                1u32 => {
                    let mut value = &mut self.session;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "session");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.pubkey;
                    ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "pubkey");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .session
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + if self.pubkey != b"" as &[u8] {
                    ::prost::encoding::bytes::encoded_len(2u32, &self.pubkey)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.session = ::core::option::Option::None;
            self.pubkey.clear();
        }
    }
    impl ::core::default::Default for PairingAck {
        fn default() -> Self {
            PairingAck {
                session: ::core::default::Default::default(),
                pubkey: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for PairingAck {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("PairingAck");
            let builder = {
                let wrapper = &self.session;
                builder.field("session", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.pubkey)
                };
                builder.field("pubkey", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct PairingRequest {
        #[prost(message, optional, tag = "1")]
        pub session: ::core::option::Option<ApiHeader>,
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for PairingRequest {
        #[inline]
        fn clone(&self) -> PairingRequest {
            PairingRequest {
                session: ::core::clone::Clone::clone(&self.session),
                name: ::core::clone::Clone::clone(&self.name),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for PairingRequest {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for PairingRequest {
        #[inline]
        fn eq(&self, other: &PairingRequest) -> bool {
            self.session == other.session && self.name == other.name
        }
    }
    impl ::prost::Message for PairingRequest {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.session {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            if self.name != "" {
                ::prost::encoding::string::encode(2u32, &self.name, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "PairingRequest";
            match tag {
                1u32 => {
                    let mut value = &mut self.session;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "session");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.name;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "name");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .session
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + if self.name != "" {
                    ::prost::encoding::string::encoded_len(2u32, &self.name)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.session = ::core::option::Option::None;
            self.name.clear();
        }
    }
    impl ::core::default::Default for PairingRequest {
        fn default() -> Self {
            PairingRequest {
                session: ::core::default::Default::default(),
                name: ::prost::alloc::string::String::new(),
            }
        }
    }
    impl ::core::fmt::Debug for PairingRequest {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("PairingRequest");
            let builder = {
                let wrapper = &self.session;
                builder.field("session", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.name)
                };
                builder.field("name", &wrapper)
            };
            builder.finish()
        }
    }
    /// Response sent once by the device and once by the desktop app to indicate
    /// whether the button has been pressed
    /// this is sent as a CryptoMessage
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct UnitResponse {
        #[prost(message, optional, tag = "1")]
        pub header: ::core::option::Option<ApiHeader>,
        #[prost(enumeration = "RespCode", tag = "2")]
        pub code: i32,
        #[prost(oneof = "unit_response::UnitresponseMaybeMessage", tags = "3")]
        pub unitresponse_maybe_message: ::core::option::Option<
            unit_response::UnitresponseMaybeMessage,
        >,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for UnitResponse {
        #[inline]
        fn clone(&self) -> UnitResponse {
            UnitResponse {
                header: ::core::clone::Clone::clone(&self.header),
                code: ::core::clone::Clone::clone(&self.code),
                unitresponse_maybe_message: ::core::clone::Clone::clone(
                    &self.unitresponse_maybe_message,
                ),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for UnitResponse {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for UnitResponse {
        #[inline]
        fn eq(&self, other: &UnitResponse) -> bool {
            self.header == other.header && self.code == other.code
                && self.unitresponse_maybe_message == other.unitresponse_maybe_message
        }
    }
    impl ::prost::Message for UnitResponse {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.header {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            if self.code != RespCode::default() as i32 {
                ::prost::encoding::int32::encode(2u32, &self.code, buf);
            }
            if let Some(ref oneof) = self.unitresponse_maybe_message {
                oneof.encode(buf)
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "UnitResponse";
            match tag {
                1u32 => {
                    let mut value = &mut self.header;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "header");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.code;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "code");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.unitresponse_maybe_message;
                    unit_response::UnitresponseMaybeMessage::merge(
                            value,
                            tag,
                            wire_type,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "unitresponse_maybe_message");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .header
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + if self.code != RespCode::default() as i32 {
                    ::prost::encoding::int32::encoded_len(2u32, &self.code)
                } else {
                    0
                }
                + self
                    .unitresponse_maybe_message
                    .as_ref()
                    .map_or(0, unit_response::UnitresponseMaybeMessage::encoded_len)
        }
        fn clear(&mut self) {
            self.header = ::core::option::Option::None;
            self.code = RespCode::default() as i32;
            self.unitresponse_maybe_message = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for UnitResponse {
        fn default() -> Self {
            UnitResponse {
                header: ::core::default::Default::default(),
                code: RespCode::default() as i32,
                unitresponse_maybe_message: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for UnitResponse {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("UnitResponse");
            let builder = {
                let wrapper = &self.header;
                builder.field("header", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a i32);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let res: ::core::result::Result<RespCode, _> = ::core::convert::TryFrom::try_from(
                                *self.0,
                            );
                            match res {
                                Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                            }
                        }
                    }
                    ScalarWrapper(&self.code)
                };
                builder.field("code", &wrapper)
            };
            let builder = {
                let wrapper = &self.unitresponse_maybe_message;
                builder.field("unitresponse_maybe_message", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl UnitResponse {
        ///Returns the enum value of `code`, or the default if the field is set to an invalid enum value.
        pub fn code(&self) -> RespCode {
            ::core::convert::TryFrom::try_from(self.code).unwrap_or(RespCode::default())
        }
        ///Sets `code` to the provided enum value.
        pub fn set_code(&mut self, value: RespCode) {
            self.code = value as i32;
        }
    }
    /// Nested message and enum types in `UnitResponse`.
    pub mod unit_response {
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub enum UnitresponseMaybeMessage {
            #[prost(string, tag = "3")]
            MessageCode(::prost::alloc::string::String),
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for UnitresponseMaybeMessage {
            #[inline]
            fn clone(&self) -> UnitresponseMaybeMessage {
                match self {
                    UnitresponseMaybeMessage::MessageCode(__self_0) => {
                        UnitresponseMaybeMessage::MessageCode(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for UnitresponseMaybeMessage {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for UnitresponseMaybeMessage {
            #[inline]
            fn eq(&self, other: &UnitresponseMaybeMessage) -> bool {
                match (self, other) {
                    (
                        UnitresponseMaybeMessage::MessageCode(__self_0),
                        UnitresponseMaybeMessage::MessageCode(__arg1_0),
                    ) => __self_0 == __arg1_0,
                }
            }
        }
        impl UnitresponseMaybeMessage {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    UnitresponseMaybeMessage::MessageCode(ref value) => {
                        ::prost::encoding::string::encode(3u32, &*value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<UnitresponseMaybeMessage>,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    3u32 => {
                        match field {
                            ::core::option::Option::Some(
                                UnitresponseMaybeMessage::MessageCode(ref mut value),
                            ) => {
                                ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            UnitresponseMaybeMessage::MessageCode(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!(
                                    "invalid UnitresponseMaybeMessage tag: {0}",
                                    tag,
                                ),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    UnitresponseMaybeMessage::MessageCode(ref value) => {
                        ::prost::encoding::string::encoded_len(3u32, &*value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for UnitresponseMaybeMessage {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    UnitresponseMaybeMessage::MessageCode(ref value) => {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&*value)
                        };
                        f.debug_tuple("MessageCode").field(&wrapper).finish()
                    }
                }
            }
        }
    }
    /// Wrapper for libsodium secret boxes (authenticated encryption)
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct CryptoMessage {
        #[prost(bytes = "vec", tag = "1")]
        pub nonce: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes = "vec", tag = "2")]
        pub encrypted: ::prost::alloc::vec::Vec<u8>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for CryptoMessage {
        #[inline]
        fn clone(&self) -> CryptoMessage {
            CryptoMessage {
                nonce: ::core::clone::Clone::clone(&self.nonce),
                encrypted: ::core::clone::Clone::clone(&self.encrypted),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for CryptoMessage {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for CryptoMessage {
        #[inline]
        fn eq(&self, other: &CryptoMessage) -> bool {
            self.nonce == other.nonce && self.encrypted == other.encrypted
        }
    }
    impl ::prost::Message for CryptoMessage {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.nonce != b"" as &[u8] {
                ::prost::encoding::bytes::encode(1u32, &self.nonce, buf);
            }
            if self.encrypted != b"" as &[u8] {
                ::prost::encoding::bytes::encode(2u32, &self.encrypted, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "CryptoMessage";
            match tag {
                1u32 => {
                    let mut value = &mut self.nonce;
                    ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "nonce");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.encrypted;
                    ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "encrypted");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.nonce != b"" as &[u8] {
                    ::prost::encoding::bytes::encoded_len(1u32, &self.nonce)
                } else {
                    0
                }
                + if self.encrypted != b"" as &[u8] {
                    ::prost::encoding::bytes::encoded_len(2u32, &self.encrypted)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.nonce.clear();
            self.encrypted.clear();
        }
    }
    impl ::core::default::Default for CryptoMessage {
        fn default() -> Self {
            CryptoMessage {
                nonce: ::core::default::Default::default(),
                encrypted: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for CryptoMessage {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("CryptoMessage");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.nonce)
                };
                builder.field("nonce", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.encrypted)
                };
                builder.field("encrypted", &wrapper)
            };
            builder.finish()
        }
    }
    /// Scattermessage stored on device
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct ApiMessage {
        #[prost(message, optional, tag = "1")]
        pub from_fingerprint: ::core::option::Option<ProtoUuid>,
        #[prost(message, optional, tag = "2")]
        pub to_fingerprint: ::core::option::Option<ProtoUuid>,
        #[prost(string, tag = "3")]
        pub application: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub extension: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub mime: ::prost::alloc::string::String,
        #[prost(int64, tag = "6")]
        pub send_date: i64,
        #[prost(int64, tag = "7")]
        pub receive_date: i64,
        #[prost(bool, tag = "8")]
        pub is_file: bool,
        #[prost(message, optional, tag = "9")]
        pub id: ::core::option::Option<ProtoUuid>,
        #[prost(bytes = "vec", tag = "10")]
        pub body: ::prost::alloc::vec::Vec<u8>,
        #[prost(string, tag = "11")]
        pub file_name: ::prost::alloc::string::String,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for ApiMessage {
        #[inline]
        fn clone(&self) -> ApiMessage {
            ApiMessage {
                from_fingerprint: ::core::clone::Clone::clone(&self.from_fingerprint),
                to_fingerprint: ::core::clone::Clone::clone(&self.to_fingerprint),
                application: ::core::clone::Clone::clone(&self.application),
                extension: ::core::clone::Clone::clone(&self.extension),
                mime: ::core::clone::Clone::clone(&self.mime),
                send_date: ::core::clone::Clone::clone(&self.send_date),
                receive_date: ::core::clone::Clone::clone(&self.receive_date),
                is_file: ::core::clone::Clone::clone(&self.is_file),
                id: ::core::clone::Clone::clone(&self.id),
                body: ::core::clone::Clone::clone(&self.body),
                file_name: ::core::clone::Clone::clone(&self.file_name),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for ApiMessage {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for ApiMessage {
        #[inline]
        fn eq(&self, other: &ApiMessage) -> bool {
            self.from_fingerprint == other.from_fingerprint
                && self.to_fingerprint == other.to_fingerprint
                && self.application == other.application
                && self.extension == other.extension && self.mime == other.mime
                && self.send_date == other.send_date
                && self.receive_date == other.receive_date
                && self.is_file == other.is_file && self.id == other.id
                && self.body == other.body && self.file_name == other.file_name
        }
    }
    impl ::prost::Message for ApiMessage {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.from_fingerprint {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            if let Some(ref msg) = self.to_fingerprint {
                ::prost::encoding::message::encode(2u32, msg, buf);
            }
            if self.application != "" {
                ::prost::encoding::string::encode(3u32, &self.application, buf);
            }
            if self.extension != "" {
                ::prost::encoding::string::encode(4u32, &self.extension, buf);
            }
            if self.mime != "" {
                ::prost::encoding::string::encode(5u32, &self.mime, buf);
            }
            if self.send_date != 0i64 {
                ::prost::encoding::int64::encode(6u32, &self.send_date, buf);
            }
            if self.receive_date != 0i64 {
                ::prost::encoding::int64::encode(7u32, &self.receive_date, buf);
            }
            if self.is_file != false {
                ::prost::encoding::bool::encode(8u32, &self.is_file, buf);
            }
            if let Some(ref msg) = self.id {
                ::prost::encoding::message::encode(9u32, msg, buf);
            }
            if self.body != b"" as &[u8] {
                ::prost::encoding::bytes::encode(10u32, &self.body, buf);
            }
            if self.file_name != "" {
                ::prost::encoding::string::encode(11u32, &self.file_name, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "ApiMessage";
            match tag {
                1u32 => {
                    let mut value = &mut self.from_fingerprint;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "from_fingerprint");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.to_fingerprint;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "to_fingerprint");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.application;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "application");
                            error
                        })
                }
                4u32 => {
                    let mut value = &mut self.extension;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "extension");
                            error
                        })
                }
                5u32 => {
                    let mut value = &mut self.mime;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "mime");
                            error
                        })
                }
                6u32 => {
                    let mut value = &mut self.send_date;
                    ::prost::encoding::int64::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "send_date");
                            error
                        })
                }
                7u32 => {
                    let mut value = &mut self.receive_date;
                    ::prost::encoding::int64::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "receive_date");
                            error
                        })
                }
                8u32 => {
                    let mut value = &mut self.is_file;
                    ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "is_file");
                            error
                        })
                }
                9u32 => {
                    let mut value = &mut self.id;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "id");
                            error
                        })
                }
                10u32 => {
                    let mut value = &mut self.body;
                    ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "body");
                            error
                        })
                }
                11u32 => {
                    let mut value = &mut self.file_name;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "file_name");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .from_fingerprint
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + self
                    .to_fingerprint
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(2u32, msg))
                + if self.application != "" {
                    ::prost::encoding::string::encoded_len(3u32, &self.application)
                } else {
                    0
                }
                + if self.extension != "" {
                    ::prost::encoding::string::encoded_len(4u32, &self.extension)
                } else {
                    0
                }
                + if self.mime != "" {
                    ::prost::encoding::string::encoded_len(5u32, &self.mime)
                } else {
                    0
                }
                + if self.send_date != 0i64 {
                    ::prost::encoding::int64::encoded_len(6u32, &self.send_date)
                } else {
                    0
                }
                + if self.receive_date != 0i64 {
                    ::prost::encoding::int64::encoded_len(7u32, &self.receive_date)
                } else {
                    0
                }
                + if self.is_file != false {
                    ::prost::encoding::bool::encoded_len(8u32, &self.is_file)
                } else {
                    0
                }
                + self
                    .id
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(9u32, msg))
                + if self.body != b"" as &[u8] {
                    ::prost::encoding::bytes::encoded_len(10u32, &self.body)
                } else {
                    0
                }
                + if self.file_name != "" {
                    ::prost::encoding::string::encoded_len(11u32, &self.file_name)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.from_fingerprint = ::core::option::Option::None;
            self.to_fingerprint = ::core::option::Option::None;
            self.application.clear();
            self.extension.clear();
            self.mime.clear();
            self.send_date = 0i64;
            self.receive_date = 0i64;
            self.is_file = false;
            self.id = ::core::option::Option::None;
            self.body.clear();
            self.file_name.clear();
        }
    }
    impl ::core::default::Default for ApiMessage {
        fn default() -> Self {
            ApiMessage {
                from_fingerprint: ::core::default::Default::default(),
                to_fingerprint: ::core::default::Default::default(),
                application: ::prost::alloc::string::String::new(),
                extension: ::prost::alloc::string::String::new(),
                mime: ::prost::alloc::string::String::new(),
                send_date: 0i64,
                receive_date: 0i64,
                is_file: false,
                id: ::core::default::Default::default(),
                body: ::core::default::Default::default(),
                file_name: ::prost::alloc::string::String::new(),
            }
        }
    }
    impl ::core::fmt::Debug for ApiMessage {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("ApiMessage");
            let builder = {
                let wrapper = &self.from_fingerprint;
                builder.field("from_fingerprint", &wrapper)
            };
            let builder = {
                let wrapper = &self.to_fingerprint;
                builder.field("to_fingerprint", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.application)
                };
                builder.field("application", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.extension)
                };
                builder.field("extension", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.mime)
                };
                builder.field("mime", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.send_date)
                };
                builder.field("send_date", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.receive_date)
                };
                builder.field("receive_date", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.is_file)
                };
                builder.field("is_file", &wrapper)
            };
            let builder = {
                let wrapper = &self.id;
                builder.field("id", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.body)
                };
                builder.field("body", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.file_name)
                };
                builder.field("file_name", &wrapper)
            };
            builder.finish()
        }
    }
    /// Identity stored on device
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct ApiIdentity {
        #[prost(message, optional, tag = "1")]
        pub fingerprint: ::core::option::Option<ProtoUuid>,
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
        #[prost(bytes = "vec", tag = "3")]
        pub public_key: ::prost::alloc::vec::Vec<u8>,
        #[prost(bool, tag = "4")]
        pub is_owned: bool,
        #[prost(map = "string, bytes", tag = "5")]
        pub extra: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::vec::Vec<u8>,
        >,
        #[prost(bytes = "vec", tag = "6")]
        pub sig: ::prost::alloc::vec::Vec<u8>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for ApiIdentity {
        #[inline]
        fn clone(&self) -> ApiIdentity {
            ApiIdentity {
                fingerprint: ::core::clone::Clone::clone(&self.fingerprint),
                name: ::core::clone::Clone::clone(&self.name),
                public_key: ::core::clone::Clone::clone(&self.public_key),
                is_owned: ::core::clone::Clone::clone(&self.is_owned),
                extra: ::core::clone::Clone::clone(&self.extra),
                sig: ::core::clone::Clone::clone(&self.sig),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for ApiIdentity {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for ApiIdentity {
        #[inline]
        fn eq(&self, other: &ApiIdentity) -> bool {
            self.fingerprint == other.fingerprint && self.name == other.name
                && self.public_key == other.public_key && self.is_owned == other.is_owned
                && self.extra == other.extra && self.sig == other.sig
        }
    }
    impl ::prost::Message for ApiIdentity {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.fingerprint {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            if self.name != "" {
                ::prost::encoding::string::encode(2u32, &self.name, buf);
            }
            if self.public_key != b"" as &[u8] {
                ::prost::encoding::bytes::encode(3u32, &self.public_key, buf);
            }
            if self.is_owned != false {
                ::prost::encoding::bool::encode(4u32, &self.is_owned, buf);
            }
            ::prost::encoding::hash_map::encode(
                ::prost::encoding::string::encode,
                ::prost::encoding::string::encoded_len,
                ::prost::encoding::bytes::encode,
                ::prost::encoding::bytes::encoded_len,
                5u32,
                &self.extra,
                buf,
            );
            if self.sig != b"" as &[u8] {
                ::prost::encoding::bytes::encode(6u32, &self.sig, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "ApiIdentity";
            match tag {
                1u32 => {
                    let mut value = &mut self.fingerprint;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "fingerprint");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.name;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "name");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.public_key;
                    ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "public_key");
                            error
                        })
                }
                4u32 => {
                    let mut value = &mut self.is_owned;
                    ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "is_owned");
                            error
                        })
                }
                5u32 => {
                    let mut value = &mut self.extra;
                    ::prost::encoding::hash_map::merge(
                            ::prost::encoding::string::merge,
                            ::prost::encoding::bytes::merge,
                            &mut value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "extra");
                            error
                        })
                }
                6u32 => {
                    let mut value = &mut self.sig;
                    ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "sig");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .fingerprint
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + if self.name != "" {
                    ::prost::encoding::string::encoded_len(2u32, &self.name)
                } else {
                    0
                }
                + if self.public_key != b"" as &[u8] {
                    ::prost::encoding::bytes::encoded_len(3u32, &self.public_key)
                } else {
                    0
                }
                + if self.is_owned != false {
                    ::prost::encoding::bool::encoded_len(4u32, &self.is_owned)
                } else {
                    0
                }
                + ::prost::encoding::hash_map::encoded_len(
                    ::prost::encoding::string::encoded_len,
                    ::prost::encoding::bytes::encoded_len,
                    5u32,
                    &self.extra,
                )
                + if self.sig != b"" as &[u8] {
                    ::prost::encoding::bytes::encoded_len(6u32, &self.sig)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.fingerprint = ::core::option::Option::None;
            self.name.clear();
            self.public_key.clear();
            self.is_owned = false;
            self.extra.clear();
            self.sig.clear();
        }
    }
    impl ::core::default::Default for ApiIdentity {
        fn default() -> Self {
            ApiIdentity {
                fingerprint: ::core::default::Default::default(),
                name: ::prost::alloc::string::String::new(),
                public_key: ::core::default::Default::default(),
                is_owned: false,
                extra: ::core::default::Default::default(),
                sig: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for ApiIdentity {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("ApiIdentity");
            let builder = {
                let wrapper = &self.fingerprint;
                builder.field("fingerprint", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.name)
                };
                builder.field("name", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.public_key)
                };
                builder.field("public_key", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.is_owned)
                };
                builder.field("is_owned", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct MapWrapper<'a>(&'a dyn ::core::fmt::Debug);
                    impl<'a> ::core::fmt::Debug for MapWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            self.0.fmt(f)
                        }
                    }
                    MapWrapper(&self.extra)
                };
                builder.field("extra", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.sig)
                };
                builder.field("sig", &wrapper)
            };
            builder.finish()
        }
    }
    /// command to send messages
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct SendMessageCmd {
        #[prost(message, optional, tag = "1")]
        pub header: ::core::option::Option<ApiHeader>,
        #[prost(message, repeated, tag = "2")]
        pub messages: ::prost::alloc::vec::Vec<ApiMessage>,
        #[prost(oneof = "send_message_cmd::SignIdentity", tags = "3")]
        pub sign_identity: ::core::option::Option<send_message_cmd::SignIdentity>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for SendMessageCmd {
        #[inline]
        fn clone(&self) -> SendMessageCmd {
            SendMessageCmd {
                header: ::core::clone::Clone::clone(&self.header),
                messages: ::core::clone::Clone::clone(&self.messages),
                sign_identity: ::core::clone::Clone::clone(&self.sign_identity),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for SendMessageCmd {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for SendMessageCmd {
        #[inline]
        fn eq(&self, other: &SendMessageCmd) -> bool {
            self.header == other.header && self.messages == other.messages
                && self.sign_identity == other.sign_identity
        }
    }
    impl ::prost::Message for SendMessageCmd {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.header {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            for msg in &self.messages {
                ::prost::encoding::message::encode(2u32, msg, buf);
            }
            if let Some(ref oneof) = self.sign_identity {
                oneof.encode(buf)
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "SendMessageCmd";
            match tag {
                1u32 => {
                    let mut value = &mut self.header;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "header");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.messages;
                    ::prost::encoding::message::merge_repeated(
                            wire_type,
                            value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "messages");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.sign_identity;
                    send_message_cmd::SignIdentity::merge(
                            value,
                            tag,
                            wire_type,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "sign_identity");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .header
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + ::prost::encoding::message::encoded_len_repeated(2u32, &self.messages)
                + self
                    .sign_identity
                    .as_ref()
                    .map_or(0, send_message_cmd::SignIdentity::encoded_len)
        }
        fn clear(&mut self) {
            self.header = ::core::option::Option::None;
            self.messages.clear();
            self.sign_identity = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for SendMessageCmd {
        fn default() -> Self {
            SendMessageCmd {
                header: ::core::default::Default::default(),
                messages: ::core::default::Default::default(),
                sign_identity: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for SendMessageCmd {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("SendMessageCmd");
            let builder = {
                let wrapper = &self.header;
                builder.field("header", &wrapper)
            };
            let builder = {
                let wrapper = &self.messages;
                builder.field("messages", &wrapper)
            };
            let builder = {
                let wrapper = &self.sign_identity;
                builder.field("sign_identity", &wrapper)
            };
            builder.finish()
        }
    }
    /// Nested message and enum types in `SendMessageCmd`.
    pub mod send_message_cmd {
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub enum SignIdentity {
            #[prost(message, tag = "3")]
            Identity(super::ProtoUuid),
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for SignIdentity {
            #[inline]
            fn clone(&self) -> SignIdentity {
                let _: ::core::clone::AssertParamIsClone<super::ProtoUuid>;
                *self
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::Copy for SignIdentity {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for SignIdentity {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for SignIdentity {
            #[inline]
            fn eq(&self, other: &SignIdentity) -> bool {
                match (self, other) {
                    (
                        SignIdentity::Identity(__self_0),
                        SignIdentity::Identity(__arg1_0),
                    ) => __self_0 == __arg1_0,
                }
            }
        }
        impl SignIdentity {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    SignIdentity::Identity(ref value) => {
                        ::prost::encoding::message::encode(3u32, &*value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<SignIdentity>,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    3u32 => {
                        match field {
                            ::core::option::Option::Some(
                                SignIdentity::Identity(ref mut value),
                            ) => {
                                ::prost::encoding::message::merge(
                                    wire_type,
                                    value,
                                    buf,
                                    ctx,
                                )
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            SignIdentity::Identity(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("invalid SignIdentity tag: {0}", tag),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    SignIdentity::Identity(ref value) => {
                        ::prost::encoding::message::encoded_len(3u32, &*value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for SignIdentity {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    SignIdentity::Identity(ref value) => {
                        let wrapper = &*value;
                        f.debug_tuple("Identity").field(&wrapper).finish()
                    }
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct GetMessagesCmd {
        #[prost(message, optional, tag = "1")]
        pub header: ::core::option::Option<ApiHeader>,
        #[prost(int32, tag = "6")]
        pub limit: i32,
        #[prost(oneof = "get_messages_cmd::TimeSlice", tags = "2, 3")]
        pub time_slice: ::core::option::Option<get_messages_cmd::TimeSlice>,
        #[prost(oneof = "get_messages_cmd::MaybeApplication", tags = "5")]
        pub maybe_application: ::core::option::Option<
            get_messages_cmd::MaybeApplication,
        >,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for GetMessagesCmd {
        #[inline]
        fn clone(&self) -> GetMessagesCmd {
            GetMessagesCmd {
                header: ::core::clone::Clone::clone(&self.header),
                limit: ::core::clone::Clone::clone(&self.limit),
                time_slice: ::core::clone::Clone::clone(&self.time_slice),
                maybe_application: ::core::clone::Clone::clone(&self.maybe_application),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for GetMessagesCmd {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for GetMessagesCmd {
        #[inline]
        fn eq(&self, other: &GetMessagesCmd) -> bool {
            self.header == other.header && self.limit == other.limit
                && self.time_slice == other.time_slice
                && self.maybe_application == other.maybe_application
        }
    }
    impl ::prost::Message for GetMessagesCmd {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.header {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            if let Some(ref oneof) = self.time_slice {
                oneof.encode(buf)
            }
            if let Some(ref oneof) = self.maybe_application {
                oneof.encode(buf)
            }
            if self.limit != 0i32 {
                ::prost::encoding::int32::encode(6u32, &self.limit, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "GetMessagesCmd";
            match tag {
                1u32 => {
                    let mut value = &mut self.header;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "header");
                            error
                        })
                }
                2u32 | 3u32 => {
                    let mut value = &mut self.time_slice;
                    get_messages_cmd::TimeSlice::merge(value, tag, wire_type, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "time_slice");
                            error
                        })
                }
                5u32 => {
                    let mut value = &mut self.maybe_application;
                    get_messages_cmd::MaybeApplication::merge(
                            value,
                            tag,
                            wire_type,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "maybe_application");
                            error
                        })
                }
                6u32 => {
                    let mut value = &mut self.limit;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "limit");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .header
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + self
                    .time_slice
                    .as_ref()
                    .map_or(0, get_messages_cmd::TimeSlice::encoded_len)
                + self
                    .maybe_application
                    .as_ref()
                    .map_or(0, get_messages_cmd::MaybeApplication::encoded_len)
                + if self.limit != 0i32 {
                    ::prost::encoding::int32::encoded_len(6u32, &self.limit)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.header = ::core::option::Option::None;
            self.time_slice = ::core::option::Option::None;
            self.maybe_application = ::core::option::Option::None;
            self.limit = 0i32;
        }
    }
    impl ::core::default::Default for GetMessagesCmd {
        fn default() -> Self {
            GetMessagesCmd {
                header: ::core::default::Default::default(),
                time_slice: ::core::default::Default::default(),
                maybe_application: ::core::default::Default::default(),
                limit: 0i32,
            }
        }
    }
    impl ::core::fmt::Debug for GetMessagesCmd {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("GetMessagesCmd");
            let builder = {
                let wrapper = &self.header;
                builder.field("header", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.limit)
                };
                builder.field("limit", &wrapper)
            };
            let builder = {
                let wrapper = &self.time_slice;
                builder.field("time_slice", &wrapper)
            };
            let builder = {
                let wrapper = &self.maybe_application;
                builder.field("maybe_application", &wrapper)
            };
            builder.finish()
        }
    }
    /// Nested message and enum types in `GetMessagesCmd`.
    pub mod get_messages_cmd {
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct Start {
            #[prost(int64, tag = "1")]
            pub start: i64,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Start {
            #[inline]
            fn clone(&self) -> Start {
                let _: ::core::clone::AssertParamIsClone<i64>;
                *self
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::Copy for Start {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for Start {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Start {
            #[inline]
            fn eq(&self, other: &Start) -> bool {
                self.start == other.start
            }
        }
        impl ::prost::Message for Start {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if self.start != 0i64 {
                    ::prost::encoding::int64::encode(1u32, &self.start, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "Start";
                match tag {
                    1u32 => {
                        let mut value = &mut self.start;
                        ::prost::encoding::int64::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "start");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.start != 0i64 {
                        ::prost::encoding::int64::encoded_len(1u32, &self.start)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.start = 0i64;
            }
        }
        impl ::core::default::Default for Start {
            fn default() -> Self {
                Start { start: 0i64 }
            }
        }
        impl ::core::fmt::Debug for Start {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Start");
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.start)
                    };
                    builder.field("start", &wrapper)
                };
                builder.finish()
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct End {
            #[prost(int64, tag = "1")]
            pub end: i64,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for End {
            #[inline]
            fn clone(&self) -> End {
                let _: ::core::clone::AssertParamIsClone<i64>;
                *self
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::Copy for End {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for End {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for End {
            #[inline]
            fn eq(&self, other: &End) -> bool {
                self.end == other.end
            }
        }
        impl ::prost::Message for End {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if self.end != 0i64 {
                    ::prost::encoding::int64::encode(1u32, &self.end, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "End";
                match tag {
                    1u32 => {
                        let mut value = &mut self.end;
                        ::prost::encoding::int64::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "end");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.end != 0i64 {
                        ::prost::encoding::int64::encoded_len(1u32, &self.end)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.end = 0i64;
            }
        }
        impl ::core::default::Default for End {
            fn default() -> Self {
                End { end: 0i64 }
            }
        }
        impl ::core::fmt::Debug for End {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("End");
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.end)
                    };
                    builder.field("end", &wrapper)
                };
                builder.finish()
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct TimeRange {
            #[prost(int64, tag = "1")]
            pub start: i64,
            #[prost(int64, tag = "2")]
            pub end: i64,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for TimeRange {
            #[inline]
            fn clone(&self) -> TimeRange {
                let _: ::core::clone::AssertParamIsClone<i64>;
                *self
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::Copy for TimeRange {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for TimeRange {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for TimeRange {
            #[inline]
            fn eq(&self, other: &TimeRange) -> bool {
                self.start == other.start && self.end == other.end
            }
        }
        impl ::prost::Message for TimeRange {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if self.start != 0i64 {
                    ::prost::encoding::int64::encode(1u32, &self.start, buf);
                }
                if self.end != 0i64 {
                    ::prost::encoding::int64::encode(2u32, &self.end, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "TimeRange";
                match tag {
                    1u32 => {
                        let mut value = &mut self.start;
                        ::prost::encoding::int64::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "start");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.end;
                        ::prost::encoding::int64::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "end");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.start != 0i64 {
                        ::prost::encoding::int64::encoded_len(1u32, &self.start)
                    } else {
                        0
                    }
                    + if self.end != 0i64 {
                        ::prost::encoding::int64::encoded_len(2u32, &self.end)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.start = 0i64;
                self.end = 0i64;
            }
        }
        impl ::core::default::Default for TimeRange {
            fn default() -> Self {
                TimeRange {
                    start: 0i64,
                    end: 0i64,
                }
            }
        }
        impl ::core::fmt::Debug for TimeRange {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("TimeRange");
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.start)
                    };
                    builder.field("start", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.end)
                    };
                    builder.field("end", &wrapper)
                };
                builder.finish()
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub enum TimeSlice {
            #[prost(message, tag = "2")]
            SendDate(TimeRange),
            #[prost(message, tag = "3")]
            ReceiveDate(TimeRange),
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for TimeSlice {
            #[inline]
            fn clone(&self) -> TimeSlice {
                let _: ::core::clone::AssertParamIsClone<TimeRange>;
                *self
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::Copy for TimeSlice {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for TimeSlice {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for TimeSlice {
            #[inline]
            fn eq(&self, other: &TimeSlice) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            TimeSlice::SendDate(__self_0),
                            TimeSlice::SendDate(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            TimeSlice::ReceiveDate(__self_0),
                            TimeSlice::ReceiveDate(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        impl TimeSlice {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    TimeSlice::SendDate(ref value) => {
                        ::prost::encoding::message::encode(2u32, &*value, buf);
                    }
                    TimeSlice::ReceiveDate(ref value) => {
                        ::prost::encoding::message::encode(3u32, &*value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<TimeSlice>,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    2u32 => {
                        match field {
                            ::core::option::Option::Some(
                                TimeSlice::SendDate(ref mut value),
                            ) => {
                                ::prost::encoding::message::merge(
                                    wire_type,
                                    value,
                                    buf,
                                    ctx,
                                )
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            TimeSlice::SendDate(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    3u32 => {
                        match field {
                            ::core::option::Option::Some(
                                TimeSlice::ReceiveDate(ref mut value),
                            ) => {
                                ::prost::encoding::message::merge(
                                    wire_type,
                                    value,
                                    buf,
                                    ctx,
                                )
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            TimeSlice::ReceiveDate(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("invalid TimeSlice tag: {0}", tag),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    TimeSlice::SendDate(ref value) => {
                        ::prost::encoding::message::encoded_len(2u32, &*value)
                    }
                    TimeSlice::ReceiveDate(ref value) => {
                        ::prost::encoding::message::encoded_len(3u32, &*value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for TimeSlice {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    TimeSlice::SendDate(ref value) => {
                        let wrapper = &*value;
                        f.debug_tuple("SendDate").field(&wrapper).finish()
                    }
                    TimeSlice::ReceiveDate(ref value) => {
                        let wrapper = &*value;
                        f.debug_tuple("ReceiveDate").field(&wrapper).finish()
                    }
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub enum MaybeApplication {
            #[prost(string, tag = "5")]
            Application(::prost::alloc::string::String),
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for MaybeApplication {
            #[inline]
            fn clone(&self) -> MaybeApplication {
                match self {
                    MaybeApplication::Application(__self_0) => {
                        MaybeApplication::Application(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for MaybeApplication {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for MaybeApplication {
            #[inline]
            fn eq(&self, other: &MaybeApplication) -> bool {
                match (self, other) {
                    (
                        MaybeApplication::Application(__self_0),
                        MaybeApplication::Application(__arg1_0),
                    ) => __self_0 == __arg1_0,
                }
            }
        }
        impl MaybeApplication {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    MaybeApplication::Application(ref value) => {
                        ::prost::encoding::string::encode(5u32, &*value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<MaybeApplication>,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    5u32 => {
                        match field {
                            ::core::option::Option::Some(
                                MaybeApplication::Application(ref mut value),
                            ) => {
                                ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            MaybeApplication::Application(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("invalid MaybeApplication tag: {0}", tag),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    MaybeApplication::Application(ref value) => {
                        ::prost::encoding::string::encoded_len(5u32, &*value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for MaybeApplication {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    MaybeApplication::Application(ref value) => {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&*value)
                        };
                        f.debug_tuple("Application").field(&wrapper).finish()
                    }
                }
            }
        }
    }
    /// Get an identity by id
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct GetIdentityCommand {
        #[prost(message, optional, tag = "1")]
        pub header: ::core::option::Option<ApiHeader>,
        #[prost(bool, tag = "3")]
        pub owned: bool,
        #[prost(oneof = "get_identity_command::Id", tags = "2")]
        pub id: ::core::option::Option<get_identity_command::Id>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for GetIdentityCommand {
        #[inline]
        fn clone(&self) -> GetIdentityCommand {
            let _: ::core::clone::AssertParamIsClone<::core::option::Option<ApiHeader>>;
            let _: ::core::clone::AssertParamIsClone<bool>;
            let _: ::core::clone::AssertParamIsClone<
                ::core::option::Option<get_identity_command::Id>,
            >;
            *self
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::Copy for GetIdentityCommand {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for GetIdentityCommand {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for GetIdentityCommand {
        #[inline]
        fn eq(&self, other: &GetIdentityCommand) -> bool {
            self.header == other.header && self.owned == other.owned
                && self.id == other.id
        }
    }
    impl ::prost::Message for GetIdentityCommand {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.header {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            if let Some(ref oneof) = self.id {
                oneof.encode(buf)
            }
            if self.owned != false {
                ::prost::encoding::bool::encode(3u32, &self.owned, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "GetIdentityCommand";
            match tag {
                1u32 => {
                    let mut value = &mut self.header;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "header");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.id;
                    get_identity_command::Id::merge(value, tag, wire_type, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "id");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.owned;
                    ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "owned");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .header
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + self.id.as_ref().map_or(0, get_identity_command::Id::encoded_len)
                + if self.owned != false {
                    ::prost::encoding::bool::encoded_len(3u32, &self.owned)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.header = ::core::option::Option::None;
            self.id = ::core::option::Option::None;
            self.owned = false;
        }
    }
    impl ::core::default::Default for GetIdentityCommand {
        fn default() -> Self {
            GetIdentityCommand {
                header: ::core::default::Default::default(),
                id: ::core::default::Default::default(),
                owned: false,
            }
        }
    }
    impl ::core::fmt::Debug for GetIdentityCommand {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("GetIdentityCommand");
            let builder = {
                let wrapper = &self.header;
                builder.field("header", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.owned)
                };
                builder.field("owned", &wrapper)
            };
            let builder = {
                let wrapper = &self.id;
                builder.field("id", &wrapper)
            };
            builder.finish()
        }
    }
    /// Nested message and enum types in `GetIdentityCommand`.
    pub mod get_identity_command {
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub enum Id {
            #[prost(message, tag = "2")]
            Identity(super::ProtoUuid),
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for Id {
            #[inline]
            fn clone(&self) -> Id {
                let _: ::core::clone::AssertParamIsClone<super::ProtoUuid>;
                *self
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::Copy for Id {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for Id {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for Id {
            #[inline]
            fn eq(&self, other: &Id) -> bool {
                match (self, other) {
                    (Id::Identity(__self_0), Id::Identity(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                }
            }
        }
        impl Id {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    Id::Identity(ref value) => {
                        ::prost::encoding::message::encode(2u32, &*value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<Id>,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    2u32 => {
                        match field {
                            ::core::option::Option::Some(Id::Identity(ref mut value)) => {
                                ::prost::encoding::message::merge(
                                    wire_type,
                                    value,
                                    buf,
                                    ctx,
                                )
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            Id::Identity(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("invalid Id tag: {0}", tag),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    Id::Identity(ref value) => {
                        ::prost::encoding::message::encoded_len(2u32, &*value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for Id {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Id::Identity(ref value) => {
                        let wrapper = &*value;
                        f.debug_tuple("Identity").field(&wrapper).finish()
                    }
                }
            }
        }
    }
    /// Identity with response code
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct IdentityResponse {
        #[prost(message, optional, tag = "1")]
        pub header: ::core::option::Option<ApiHeader>,
        #[prost(message, repeated, tag = "2")]
        pub identity: ::prost::alloc::vec::Vec<ApiIdentity>,
        #[prost(enumeration = "RespCode", tag = "3")]
        pub code: i32,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for IdentityResponse {
        #[inline]
        fn clone(&self) -> IdentityResponse {
            IdentityResponse {
                header: ::core::clone::Clone::clone(&self.header),
                identity: ::core::clone::Clone::clone(&self.identity),
                code: ::core::clone::Clone::clone(&self.code),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for IdentityResponse {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for IdentityResponse {
        #[inline]
        fn eq(&self, other: &IdentityResponse) -> bool {
            self.header == other.header && self.identity == other.identity
                && self.code == other.code
        }
    }
    impl ::prost::Message for IdentityResponse {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.header {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            for msg in &self.identity {
                ::prost::encoding::message::encode(2u32, msg, buf);
            }
            if self.code != RespCode::default() as i32 {
                ::prost::encoding::int32::encode(3u32, &self.code, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "IdentityResponse";
            match tag {
                1u32 => {
                    let mut value = &mut self.header;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "header");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.identity;
                    ::prost::encoding::message::merge_repeated(
                            wire_type,
                            value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "identity");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.code;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "code");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .header
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + ::prost::encoding::message::encoded_len_repeated(2u32, &self.identity)
                + if self.code != RespCode::default() as i32 {
                    ::prost::encoding::int32::encoded_len(3u32, &self.code)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.header = ::core::option::Option::None;
            self.identity.clear();
            self.code = RespCode::default() as i32;
        }
    }
    impl ::core::default::Default for IdentityResponse {
        fn default() -> Self {
            IdentityResponse {
                header: ::core::default::Default::default(),
                identity: ::core::default::Default::default(),
                code: RespCode::default() as i32,
            }
        }
    }
    impl ::core::fmt::Debug for IdentityResponse {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("IdentityResponse");
            let builder = {
                let wrapper = &self.header;
                builder.field("header", &wrapper)
            };
            let builder = {
                let wrapper = &self.identity;
                builder.field("identity", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a i32);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let res: ::core::result::Result<RespCode, _> = ::core::convert::TryFrom::try_from(
                                *self.0,
                            );
                            match res {
                                Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                            }
                        }
                    }
                    ScalarWrapper(&self.code)
                };
                builder.field("code", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl IdentityResponse {
        ///Returns the enum value of `code`, or the default if the field is set to an invalid enum value.
        pub fn code(&self) -> RespCode {
            ::core::convert::TryFrom::try_from(self.code).unwrap_or(RespCode::default())
        }
        ///Sets `code` to the provided enum value.
        pub fn set_code(&mut self, value: RespCode) {
            self.code = value as i32;
        }
    }
    /// Message with response code
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct MessageResponse {
        #[prost(message, optional, tag = "1")]
        pub header: ::core::option::Option<ApiHeader>,
        #[prost(message, repeated, tag = "2")]
        pub messsage: ::prost::alloc::vec::Vec<ApiMessage>,
        #[prost(enumeration = "RespCode", tag = "3")]
        pub code: i32,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for MessageResponse {
        #[inline]
        fn clone(&self) -> MessageResponse {
            MessageResponse {
                header: ::core::clone::Clone::clone(&self.header),
                messsage: ::core::clone::Clone::clone(&self.messsage),
                code: ::core::clone::Clone::clone(&self.code),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for MessageResponse {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for MessageResponse {
        #[inline]
        fn eq(&self, other: &MessageResponse) -> bool {
            self.header == other.header && self.messsage == other.messsage
                && self.code == other.code
        }
    }
    impl ::prost::Message for MessageResponse {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.header {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            for msg in &self.messsage {
                ::prost::encoding::message::encode(2u32, msg, buf);
            }
            if self.code != RespCode::default() as i32 {
                ::prost::encoding::int32::encode(3u32, &self.code, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "MessageResponse";
            match tag {
                1u32 => {
                    let mut value = &mut self.header;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "header");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.messsage;
                    ::prost::encoding::message::merge_repeated(
                            wire_type,
                            value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "messsage");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.code;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "code");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .header
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + ::prost::encoding::message::encoded_len_repeated(2u32, &self.messsage)
                + if self.code != RespCode::default() as i32 {
                    ::prost::encoding::int32::encoded_len(3u32, &self.code)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.header = ::core::option::Option::None;
            self.messsage.clear();
            self.code = RespCode::default() as i32;
        }
    }
    impl ::core::default::Default for MessageResponse {
        fn default() -> Self {
            MessageResponse {
                header: ::core::default::Default::default(),
                messsage: ::core::default::Default::default(),
                code: RespCode::default() as i32,
            }
        }
    }
    impl ::core::fmt::Debug for MessageResponse {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("MessageResponse");
            let builder = {
                let wrapper = &self.header;
                builder.field("header", &wrapper)
            };
            let builder = {
                let wrapper = &self.messsage;
                builder.field("messsage", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a i32);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let res: ::core::result::Result<RespCode, _> = ::core::convert::TryFrom::try_from(
                                *self.0,
                            );
                            match res {
                                Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                            }
                        }
                    }
                    ScalarWrapper(&self.code)
                };
                builder.field("code", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl MessageResponse {
        ///Returns the enum value of `code`, or the default if the field is set to an invalid enum value.
        pub fn code(&self) -> RespCode {
            ::core::convert::TryFrom::try_from(self.code).unwrap_or(RespCode::default())
        }
        ///Sets `code` to the provided enum value.
        pub fn set_code(&mut self, value: RespCode) {
            self.code = value as i32;
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct GenerateIdentityCommand {
        #[prost(message, optional, tag = "1")]
        pub header: ::core::option::Option<ApiHeader>,
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for GenerateIdentityCommand {
        #[inline]
        fn clone(&self) -> GenerateIdentityCommand {
            GenerateIdentityCommand {
                header: ::core::clone::Clone::clone(&self.header),
                name: ::core::clone::Clone::clone(&self.name),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for GenerateIdentityCommand {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for GenerateIdentityCommand {
        #[inline]
        fn eq(&self, other: &GenerateIdentityCommand) -> bool {
            self.header == other.header && self.name == other.name
        }
    }
    impl ::prost::Message for GenerateIdentityCommand {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.header {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            if self.name != "" {
                ::prost::encoding::string::encode(2u32, &self.name, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "GenerateIdentityCommand";
            match tag {
                1u32 => {
                    let mut value = &mut self.header;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "header");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.name;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "name");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .header
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + if self.name != "" {
                    ::prost::encoding::string::encoded_len(2u32, &self.name)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.header = ::core::option::Option::None;
            self.name.clear();
        }
    }
    impl ::core::default::Default for GenerateIdentityCommand {
        fn default() -> Self {
            GenerateIdentityCommand {
                header: ::core::default::Default::default(),
                name: ::prost::alloc::string::String::new(),
            }
        }
    }
    impl ::core::fmt::Debug for GenerateIdentityCommand {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("GenerateIdentityCommand");
            let builder = {
                let wrapper = &self.header;
                builder.field("header", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.name)
                };
                builder.field("name", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct ImportIdentityCommand {
        #[prost(message, optional, tag = "1")]
        pub header: ::core::option::Option<ApiHeader>,
        #[prost(oneof = "import_identity_command::MaybeHandle", tags = "2")]
        pub maybe_handle: ::core::option::Option<import_identity_command::MaybeHandle>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for ImportIdentityCommand {
        #[inline]
        fn clone(&self) -> ImportIdentityCommand {
            let _: ::core::clone::AssertParamIsClone<::core::option::Option<ApiHeader>>;
            let _: ::core::clone::AssertParamIsClone<
                ::core::option::Option<import_identity_command::MaybeHandle>,
            >;
            *self
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::Copy for ImportIdentityCommand {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for ImportIdentityCommand {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for ImportIdentityCommand {
        #[inline]
        fn eq(&self, other: &ImportIdentityCommand) -> bool {
            self.header == other.header && self.maybe_handle == other.maybe_handle
        }
    }
    impl ::prost::Message for ImportIdentityCommand {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.header {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            if let Some(ref oneof) = self.maybe_handle {
                oneof.encode(buf)
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "ImportIdentityCommand";
            match tag {
                1u32 => {
                    let mut value = &mut self.header;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "header");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.maybe_handle;
                    import_identity_command::MaybeHandle::merge(
                            value,
                            tag,
                            wire_type,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "maybe_handle");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .header
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + self
                    .maybe_handle
                    .as_ref()
                    .map_or(0, import_identity_command::MaybeHandle::encoded_len)
        }
        fn clear(&mut self) {
            self.header = ::core::option::Option::None;
            self.maybe_handle = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for ImportIdentityCommand {
        fn default() -> Self {
            ImportIdentityCommand {
                header: ::core::default::Default::default(),
                maybe_handle: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for ImportIdentityCommand {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("ImportIdentityCommand");
            let builder = {
                let wrapper = &self.header;
                builder.field("header", &wrapper)
            };
            let builder = {
                let wrapper = &self.maybe_handle;
                builder.field("maybe_handle", &wrapper)
            };
            builder.finish()
        }
    }
    /// Nested message and enum types in `ImportIdentityCommand`.
    pub mod import_identity_command {
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub enum MaybeHandle {
            #[prost(message, tag = "2")]
            Handle(super::ProtoUuid),
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for MaybeHandle {
            #[inline]
            fn clone(&self) -> MaybeHandle {
                let _: ::core::clone::AssertParamIsClone<super::ProtoUuid>;
                *self
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::Copy for MaybeHandle {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for MaybeHandle {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for MaybeHandle {
            #[inline]
            fn eq(&self, other: &MaybeHandle) -> bool {
                match (self, other) {
                    (MaybeHandle::Handle(__self_0), MaybeHandle::Handle(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                }
            }
        }
        impl MaybeHandle {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    MaybeHandle::Handle(ref value) => {
                        ::prost::encoding::message::encode(2u32, &*value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<MaybeHandle>,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    2u32 => {
                        match field {
                            ::core::option::Option::Some(
                                MaybeHandle::Handle(ref mut value),
                            ) => {
                                ::prost::encoding::message::merge(
                                    wire_type,
                                    value,
                                    buf,
                                    ctx,
                                )
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            MaybeHandle::Handle(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("invalid MaybeHandle tag: {0}", tag),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    MaybeHandle::Handle(ref value) => {
                        ::prost::encoding::message::encoded_len(2u32, &*value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for MaybeHandle {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    MaybeHandle::Handle(ref value) => {
                        let wrapper = &*value;
                        f.debug_tuple("Handle").field(&wrapper).finish()
                    }
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct GenerateIdentityResponse {
        #[prost(message, optional, tag = "1")]
        pub header: ::core::option::Option<ApiHeader>,
        #[prost(message, optional, tag = "2")]
        pub identity: ::core::option::Option<ProtoUuid>,
        #[prost(enumeration = "RespCode", tag = "3")]
        pub code: i32,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for GenerateIdentityResponse {
        #[inline]
        fn clone(&self) -> GenerateIdentityResponse {
            let _: ::core::clone::AssertParamIsClone<::core::option::Option<ApiHeader>>;
            let _: ::core::clone::AssertParamIsClone<::core::option::Option<ProtoUuid>>;
            let _: ::core::clone::AssertParamIsClone<i32>;
            *self
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::Copy for GenerateIdentityResponse {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for GenerateIdentityResponse {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for GenerateIdentityResponse {
        #[inline]
        fn eq(&self, other: &GenerateIdentityResponse) -> bool {
            self.header == other.header && self.identity == other.identity
                && self.code == other.code
        }
    }
    impl ::prost::Message for GenerateIdentityResponse {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.header {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            if let Some(ref msg) = self.identity {
                ::prost::encoding::message::encode(2u32, msg, buf);
            }
            if self.code != RespCode::default() as i32 {
                ::prost::encoding::int32::encode(3u32, &self.code, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "GenerateIdentityResponse";
            match tag {
                1u32 => {
                    let mut value = &mut self.header;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "header");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.identity;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "identity");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.code;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "code");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .header
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + self
                    .identity
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(2u32, msg))
                + if self.code != RespCode::default() as i32 {
                    ::prost::encoding::int32::encoded_len(3u32, &self.code)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.header = ::core::option::Option::None;
            self.identity = ::core::option::Option::None;
            self.code = RespCode::default() as i32;
        }
    }
    impl ::core::default::Default for GenerateIdentityResponse {
        fn default() -> Self {
            GenerateIdentityResponse {
                header: ::core::default::Default::default(),
                identity: ::core::default::Default::default(),
                code: RespCode::default() as i32,
            }
        }
    }
    impl ::core::fmt::Debug for GenerateIdentityResponse {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("GenerateIdentityResponse");
            let builder = {
                let wrapper = &self.header;
                builder.field("header", &wrapper)
            };
            let builder = {
                let wrapper = &self.identity;
                builder.field("identity", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a i32);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let res: ::core::result::Result<RespCode, _> = ::core::convert::TryFrom::try_from(
                                *self.0,
                            );
                            match res {
                                Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                            }
                        }
                    }
                    ScalarWrapper(&self.code)
                };
                builder.field("code", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl GenerateIdentityResponse {
        ///Returns the enum value of `code`, or the default if the field is set to an invalid enum value.
        pub fn code(&self) -> RespCode {
            ::core::convert::TryFrom::try_from(self.code).unwrap_or(RespCode::default())
        }
        ///Sets `code` to the provided enum value.
        pub fn set_code(&mut self, value: RespCode) {
            self.code = value as i32;
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct ImportIdentityResponse {
        #[prost(message, optional, tag = "1")]
        pub header: ::core::option::Option<ApiHeader>,
        #[prost(enumeration = "RespCode", tag = "2")]
        pub code: i32,
        #[prost(oneof = "import_identity_response::State", tags = "3, 4")]
        pub state: ::core::option::Option<import_identity_response::State>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for ImportIdentityResponse {
        #[inline]
        fn clone(&self) -> ImportIdentityResponse {
            let _: ::core::clone::AssertParamIsClone<::core::option::Option<ApiHeader>>;
            let _: ::core::clone::AssertParamIsClone<i32>;
            let _: ::core::clone::AssertParamIsClone<
                ::core::option::Option<import_identity_response::State>,
            >;
            *self
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::Copy for ImportIdentityResponse {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for ImportIdentityResponse {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for ImportIdentityResponse {
        #[inline]
        fn eq(&self, other: &ImportIdentityResponse) -> bool {
            self.header == other.header && self.code == other.code
                && self.state == other.state
        }
    }
    impl ::prost::Message for ImportIdentityResponse {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.header {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            if self.code != RespCode::default() as i32 {
                ::prost::encoding::int32::encode(2u32, &self.code, buf);
            }
            if let Some(ref oneof) = self.state {
                oneof.encode(buf)
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "ImportIdentityResponse";
            match tag {
                1u32 => {
                    let mut value = &mut self.header;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "header");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.code;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "code");
                            error
                        })
                }
                3u32 | 4u32 => {
                    let mut value = &mut self.state;
                    import_identity_response::State::merge(
                            value,
                            tag,
                            wire_type,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "state");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .header
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + if self.code != RespCode::default() as i32 {
                    ::prost::encoding::int32::encoded_len(2u32, &self.code)
                } else {
                    0
                }
                + self
                    .state
                    .as_ref()
                    .map_or(0, import_identity_response::State::encoded_len)
        }
        fn clear(&mut self) {
            self.header = ::core::option::Option::None;
            self.code = RespCode::default() as i32;
            self.state = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for ImportIdentityResponse {
        fn default() -> Self {
            ImportIdentityResponse {
                header: ::core::default::Default::default(),
                code: RespCode::default() as i32,
                state: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for ImportIdentityResponse {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("ImportIdentityResponse");
            let builder = {
                let wrapper = &self.header;
                builder.field("header", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a i32);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let res: ::core::result::Result<RespCode, _> = ::core::convert::TryFrom::try_from(
                                *self.0,
                            );
                            match res {
                                Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                            }
                        }
                    }
                    ScalarWrapper(&self.code)
                };
                builder.field("code", &wrapper)
            };
            let builder = {
                let wrapper = &self.state;
                builder.field("state", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl ImportIdentityResponse {
        ///Returns the enum value of `code`, or the default if the field is set to an invalid enum value.
        pub fn code(&self) -> RespCode {
            ::core::convert::TryFrom::try_from(self.code).unwrap_or(RespCode::default())
        }
        ///Sets `code` to the provided enum value.
        pub fn set_code(&mut self, value: RespCode) {
            self.code = value as i32;
        }
    }
    /// Nested message and enum types in `ImportIdentityResponse`.
    pub mod import_identity_response {
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct FinalResponse {
            #[prost(message, optional, tag = "5")]
            pub handle: ::core::option::Option<super::ProtoUuid>,
            #[prost(message, optional, tag = "6")]
            pub identity: ::core::option::Option<super::ProtoUuid>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for FinalResponse {
            #[inline]
            fn clone(&self) -> FinalResponse {
                let _: ::core::clone::AssertParamIsClone<
                    ::core::option::Option<super::ProtoUuid>,
                >;
                let _: ::core::clone::AssertParamIsClone<
                    ::core::option::Option<super::ProtoUuid>,
                >;
                *self
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::Copy for FinalResponse {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for FinalResponse {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for FinalResponse {
            #[inline]
            fn eq(&self, other: &FinalResponse) -> bool {
                self.handle == other.handle && self.identity == other.identity
            }
        }
        impl ::prost::Message for FinalResponse {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if let Some(ref msg) = self.handle {
                    ::prost::encoding::message::encode(5u32, msg, buf);
                }
                if let Some(ref msg) = self.identity {
                    ::prost::encoding::message::encode(6u32, msg, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "FinalResponse";
                match tag {
                    5u32 => {
                        let mut value = &mut self.handle;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "handle");
                                error
                            })
                    }
                    6u32 => {
                        let mut value = &mut self.identity;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "identity");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + self
                        .handle
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(5u32, msg),
                        )
                    + self
                        .identity
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(6u32, msg),
                        )
            }
            fn clear(&mut self) {
                self.handle = ::core::option::Option::None;
                self.identity = ::core::option::Option::None;
            }
        }
        impl ::core::default::Default for FinalResponse {
            fn default() -> Self {
                FinalResponse {
                    handle: ::core::default::Default::default(),
                    identity: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for FinalResponse {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("FinalResponse");
                let builder = {
                    let wrapper = &self.handle;
                    builder.field("handle", &wrapper)
                };
                let builder = {
                    let wrapper = &self.identity;
                    builder.field("identity", &wrapper)
                };
                builder.finish()
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub enum State {
            #[prost(message, tag = "3")]
            Final(FinalResponse),
            #[prost(message, tag = "4")]
            Handle(super::ProtoUuid),
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for State {
            #[inline]
            fn clone(&self) -> State {
                let _: ::core::clone::AssertParamIsClone<FinalResponse>;
                let _: ::core::clone::AssertParamIsClone<super::ProtoUuid>;
                *self
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::Copy for State {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for State {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for State {
            #[inline]
            fn eq(&self, other: &State) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (State::Final(__self_0), State::Final(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (State::Handle(__self_0), State::Handle(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        impl State {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    State::Final(ref value) => {
                        ::prost::encoding::message::encode(3u32, &*value, buf);
                    }
                    State::Handle(ref value) => {
                        ::prost::encoding::message::encode(4u32, &*value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<State>,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    3u32 => {
                        match field {
                            ::core::option::Option::Some(State::Final(ref mut value)) => {
                                ::prost::encoding::message::merge(
                                    wire_type,
                                    value,
                                    buf,
                                    ctx,
                                )
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            State::Final(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    4u32 => {
                        match field {
                            ::core::option::Option::Some(
                                State::Handle(ref mut value),
                            ) => {
                                ::prost::encoding::message::merge(
                                    wire_type,
                                    value,
                                    buf,
                                    ctx,
                                )
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            State::Handle(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("invalid State tag: {0}", tag),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    State::Final(ref value) => {
                        ::prost::encoding::message::encoded_len(3u32, &*value)
                    }
                    State::Handle(ref value) => {
                        ::prost::encoding::message::encoded_len(4u32, &*value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for State {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    State::Final(ref value) => {
                        let wrapper = &*value;
                        f.debug_tuple("Final").field(&wrapper).finish()
                    }
                    State::Handle(ref value) => {
                        let wrapper = &*value;
                        f.debug_tuple("Handle").field(&wrapper).finish()
                    }
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct GetEvents {
        #[prost(message, optional, tag = "1")]
        pub header: ::core::option::Option<ApiHeader>,
        #[prost(bool, tag = "2")]
        pub block: bool,
        #[prost(oneof = "get_events::MaybeCount", tags = "3")]
        pub maybe_count: ::core::option::Option<get_events::MaybeCount>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for GetEvents {
        #[inline]
        fn clone(&self) -> GetEvents {
            let _: ::core::clone::AssertParamIsClone<::core::option::Option<ApiHeader>>;
            let _: ::core::clone::AssertParamIsClone<bool>;
            let _: ::core::clone::AssertParamIsClone<
                ::core::option::Option<get_events::MaybeCount>,
            >;
            *self
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::Copy for GetEvents {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for GetEvents {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for GetEvents {
        #[inline]
        fn eq(&self, other: &GetEvents) -> bool {
            self.header == other.header && self.block == other.block
                && self.maybe_count == other.maybe_count
        }
    }
    impl ::prost::Message for GetEvents {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.header {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            if self.block != false {
                ::prost::encoding::bool::encode(2u32, &self.block, buf);
            }
            if let Some(ref oneof) = self.maybe_count {
                oneof.encode(buf)
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "GetEvents";
            match tag {
                1u32 => {
                    let mut value = &mut self.header;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "header");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.block;
                    ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "block");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.maybe_count;
                    get_events::MaybeCount::merge(value, tag, wire_type, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "maybe_count");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .header
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + if self.block != false {
                    ::prost::encoding::bool::encoded_len(2u32, &self.block)
                } else {
                    0
                }
                + self
                    .maybe_count
                    .as_ref()
                    .map_or(0, get_events::MaybeCount::encoded_len)
        }
        fn clear(&mut self) {
            self.header = ::core::option::Option::None;
            self.block = false;
            self.maybe_count = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for GetEvents {
        fn default() -> Self {
            GetEvents {
                header: ::core::default::Default::default(),
                block: false,
                maybe_count: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for GetEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("GetEvents");
            let builder = {
                let wrapper = &self.header;
                builder.field("header", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.block)
                };
                builder.field("block", &wrapper)
            };
            let builder = {
                let wrapper = &self.maybe_count;
                builder.field("maybe_count", &wrapper)
            };
            builder.finish()
        }
    }
    /// Nested message and enum types in `GetEvents`.
    pub mod get_events {
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub enum MaybeCount {
            #[prost(uint32, tag = "3")]
            Count(u32),
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for MaybeCount {
            #[inline]
            fn clone(&self) -> MaybeCount {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::Copy for MaybeCount {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for MaybeCount {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for MaybeCount {
            #[inline]
            fn eq(&self, other: &MaybeCount) -> bool {
                match (self, other) {
                    (MaybeCount::Count(__self_0), MaybeCount::Count(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                }
            }
        }
        impl MaybeCount {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    MaybeCount::Count(ref value) => {
                        ::prost::encoding::uint32::encode(3u32, &*value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<MaybeCount>,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    3u32 => {
                        match field {
                            ::core::option::Option::Some(
                                MaybeCount::Count(ref mut value),
                            ) => {
                                ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::uint32::merge(wire_type, value, buf, ctx)
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            MaybeCount::Count(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("invalid MaybeCount tag: {0}", tag),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    MaybeCount::Count(ref value) => {
                        ::prost::encoding::uint32::encoded_len(3u32, &*value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for MaybeCount {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    MaybeCount::Count(ref value) => {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&*value)
                        };
                        f.debug_tuple("Count").field(&wrapper).finish()
                    }
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct SbEvents {
        #[prost(message, optional, tag = "1")]
        pub header: ::core::option::Option<ApiHeader>,
        #[prost(message, repeated, tag = "2")]
        pub events: ::prost::alloc::vec::Vec<SbEvent>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for SbEvents {
        #[inline]
        fn clone(&self) -> SbEvents {
            SbEvents {
                header: ::core::clone::Clone::clone(&self.header),
                events: ::core::clone::Clone::clone(&self.events),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for SbEvents {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for SbEvents {
        #[inline]
        fn eq(&self, other: &SbEvents) -> bool {
            self.header == other.header && self.events == other.events
        }
    }
    impl ::prost::Message for SbEvents {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref msg) = self.header {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
            for msg in &self.events {
                ::prost::encoding::message::encode(2u32, msg, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "SbEvents";
            match tag {
                1u32 => {
                    let mut value = &mut self.header;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "header");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.events;
                    ::prost::encoding::message::merge_repeated(
                            wire_type,
                            value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "events");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .header
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(1u32, msg))
                + ::prost::encoding::message::encoded_len_repeated(2u32, &self.events)
        }
        fn clear(&mut self) {
            self.header = ::core::option::Option::None;
            self.events.clear();
        }
    }
    impl ::core::default::Default for SbEvents {
        fn default() -> Self {
            SbEvents {
                header: ::core::default::Default::default(),
                events: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for SbEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("SbEvents");
            let builder = {
                let wrapper = &self.header;
                builder.field("header", &wrapper)
            };
            let builder = {
                let wrapper = &self.events;
                builder.field("events", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    pub struct SbEvent {
        #[prost(oneof = "sb_event::MaybeEvent", tags = "21, 2")]
        pub maybe_event: ::core::option::Option<sb_event::MaybeEvent>,
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for SbEvent {
        #[inline]
        fn clone(&self) -> SbEvent {
            SbEvent {
                maybe_event: ::core::clone::Clone::clone(&self.maybe_event),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::marker::StructuralPartialEq for SbEvent {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for SbEvent {
        #[inline]
        fn eq(&self, other: &SbEvent) -> bool {
            self.maybe_event == other.maybe_event
        }
    }
    impl ::prost::Message for SbEvent {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref oneof) = self.maybe_event {
                oneof.encode(buf)
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "SbEvent";
            match tag {
                21u32 | 2u32 => {
                    let mut value = &mut self.maybe_event;
                    sb_event::MaybeEvent::merge(value, tag, wire_type, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "maybe_event");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.maybe_event.as_ref().map_or(0, sb_event::MaybeEvent::encoded_len)
        }
        fn clear(&mut self) {
            self.maybe_event = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for SbEvent {
        fn default() -> Self {
            SbEvent {
                maybe_event: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for SbEvent {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("SbEvent");
            let builder = {
                let wrapper = &self.maybe_event;
                builder.field("maybe_event", &wrapper)
            };
            builder.finish()
        }
    }
    /// Nested message and enum types in `SbEvent`.
    pub mod sb_event {
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct NewMessage {
            #[prost(message, repeated, tag = "1")]
            pub messages: ::prost::alloc::vec::Vec<NoBodyMessage>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for NewMessage {
            #[inline]
            fn clone(&self) -> NewMessage {
                NewMessage {
                    messages: ::core::clone::Clone::clone(&self.messages),
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for NewMessage {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for NewMessage {
            #[inline]
            fn eq(&self, other: &NewMessage) -> bool {
                self.messages == other.messages
            }
        }
        impl ::prost::Message for NewMessage {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                for msg in &self.messages {
                    ::prost::encoding::message::encode(1u32, msg, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "NewMessage";
                match tag {
                    1u32 => {
                        let mut value = &mut self.messages;
                        ::prost::encoding::message::merge_repeated(
                                wire_type,
                                value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "messages");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + ::prost::encoding::message::encoded_len_repeated(
                        1u32,
                        &self.messages,
                    )
            }
            fn clear(&mut self) {
                self.messages.clear();
            }
        }
        impl ::core::default::Default for NewMessage {
            fn default() -> Self {
                NewMessage {
                    messages: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for NewMessage {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("NewMessage");
                let builder = {
                    let wrapper = &self.messages;
                    builder.field("messages", &wrapper)
                };
                builder.finish()
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct NoBodyMessage {
            #[prost(message, optional, tag = "1")]
            pub from_fingerprint: ::core::option::Option<super::ProtoUuid>,
            #[prost(message, optional, tag = "2")]
            pub to_fingerprint: ::core::option::Option<super::ProtoUuid>,
            #[prost(string, tag = "3")]
            pub application: ::prost::alloc::string::String,
            #[prost(string, tag = "4")]
            pub extension: ::prost::alloc::string::String,
            #[prost(string, tag = "5")]
            pub mime: ::prost::alloc::string::String,
            #[prost(int64, tag = "6")]
            pub send_date: i64,
            #[prost(int64, tag = "7")]
            pub receive_date: i64,
            #[prost(bool, tag = "8")]
            pub is_file: bool,
            #[prost(message, optional, tag = "9")]
            pub id: ::core::option::Option<super::ProtoUuid>,
            #[prost(string, tag = "10")]
            pub file_name: ::prost::alloc::string::String,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for NoBodyMessage {
            #[inline]
            fn clone(&self) -> NoBodyMessage {
                NoBodyMessage {
                    from_fingerprint: ::core::clone::Clone::clone(
                        &self.from_fingerprint,
                    ),
                    to_fingerprint: ::core::clone::Clone::clone(&self.to_fingerprint),
                    application: ::core::clone::Clone::clone(&self.application),
                    extension: ::core::clone::Clone::clone(&self.extension),
                    mime: ::core::clone::Clone::clone(&self.mime),
                    send_date: ::core::clone::Clone::clone(&self.send_date),
                    receive_date: ::core::clone::Clone::clone(&self.receive_date),
                    is_file: ::core::clone::Clone::clone(&self.is_file),
                    id: ::core::clone::Clone::clone(&self.id),
                    file_name: ::core::clone::Clone::clone(&self.file_name),
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for NoBodyMessage {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for NoBodyMessage {
            #[inline]
            fn eq(&self, other: &NoBodyMessage) -> bool {
                self.from_fingerprint == other.from_fingerprint
                    && self.to_fingerprint == other.to_fingerprint
                    && self.application == other.application
                    && self.extension == other.extension && self.mime == other.mime
                    && self.send_date == other.send_date
                    && self.receive_date == other.receive_date
                    && self.is_file == other.is_file && self.id == other.id
                    && self.file_name == other.file_name
            }
        }
        impl ::prost::Message for NoBodyMessage {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if let Some(ref msg) = self.from_fingerprint {
                    ::prost::encoding::message::encode(1u32, msg, buf);
                }
                if let Some(ref msg) = self.to_fingerprint {
                    ::prost::encoding::message::encode(2u32, msg, buf);
                }
                if self.application != "" {
                    ::prost::encoding::string::encode(3u32, &self.application, buf);
                }
                if self.extension != "" {
                    ::prost::encoding::string::encode(4u32, &self.extension, buf);
                }
                if self.mime != "" {
                    ::prost::encoding::string::encode(5u32, &self.mime, buf);
                }
                if self.send_date != 0i64 {
                    ::prost::encoding::int64::encode(6u32, &self.send_date, buf);
                }
                if self.receive_date != 0i64 {
                    ::prost::encoding::int64::encode(7u32, &self.receive_date, buf);
                }
                if self.is_file != false {
                    ::prost::encoding::bool::encode(8u32, &self.is_file, buf);
                }
                if let Some(ref msg) = self.id {
                    ::prost::encoding::message::encode(9u32, msg, buf);
                }
                if self.file_name != "" {
                    ::prost::encoding::string::encode(10u32, &self.file_name, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "NoBodyMessage";
                match tag {
                    1u32 => {
                        let mut value = &mut self.from_fingerprint;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "from_fingerprint");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.to_fingerprint;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "to_fingerprint");
                                error
                            })
                    }
                    3u32 => {
                        let mut value = &mut self.application;
                        ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "application");
                                error
                            })
                    }
                    4u32 => {
                        let mut value = &mut self.extension;
                        ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "extension");
                                error
                            })
                    }
                    5u32 => {
                        let mut value = &mut self.mime;
                        ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "mime");
                                error
                            })
                    }
                    6u32 => {
                        let mut value = &mut self.send_date;
                        ::prost::encoding::int64::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "send_date");
                                error
                            })
                    }
                    7u32 => {
                        let mut value = &mut self.receive_date;
                        ::prost::encoding::int64::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "receive_date");
                                error
                            })
                    }
                    8u32 => {
                        let mut value = &mut self.is_file;
                        ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "is_file");
                                error
                            })
                    }
                    9u32 => {
                        let mut value = &mut self.id;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "id");
                                error
                            })
                    }
                    10u32 => {
                        let mut value = &mut self.file_name;
                        ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "file_name");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + self
                        .from_fingerprint
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(1u32, msg),
                        )
                    + self
                        .to_fingerprint
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(2u32, msg),
                        )
                    + if self.application != "" {
                        ::prost::encoding::string::encoded_len(3u32, &self.application)
                    } else {
                        0
                    }
                    + if self.extension != "" {
                        ::prost::encoding::string::encoded_len(4u32, &self.extension)
                    } else {
                        0
                    }
                    + if self.mime != "" {
                        ::prost::encoding::string::encoded_len(5u32, &self.mime)
                    } else {
                        0
                    }
                    + if self.send_date != 0i64 {
                        ::prost::encoding::int64::encoded_len(6u32, &self.send_date)
                    } else {
                        0
                    }
                    + if self.receive_date != 0i64 {
                        ::prost::encoding::int64::encoded_len(7u32, &self.receive_date)
                    } else {
                        0
                    }
                    + if self.is_file != false {
                        ::prost::encoding::bool::encoded_len(8u32, &self.is_file)
                    } else {
                        0
                    }
                    + self
                        .id
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(9u32, msg),
                        )
                    + if self.file_name != "" {
                        ::prost::encoding::string::encoded_len(10u32, &self.file_name)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.from_fingerprint = ::core::option::Option::None;
                self.to_fingerprint = ::core::option::Option::None;
                self.application.clear();
                self.extension.clear();
                self.mime.clear();
                self.send_date = 0i64;
                self.receive_date = 0i64;
                self.is_file = false;
                self.id = ::core::option::Option::None;
                self.file_name.clear();
            }
        }
        impl ::core::default::Default for NoBodyMessage {
            fn default() -> Self {
                NoBodyMessage {
                    from_fingerprint: ::core::default::Default::default(),
                    to_fingerprint: ::core::default::Default::default(),
                    application: ::prost::alloc::string::String::new(),
                    extension: ::prost::alloc::string::String::new(),
                    mime: ::prost::alloc::string::String::new(),
                    send_date: 0i64,
                    receive_date: 0i64,
                    is_file: false,
                    id: ::core::default::Default::default(),
                    file_name: ::prost::alloc::string::String::new(),
                }
            }
        }
        impl ::core::fmt::Debug for NoBodyMessage {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("NoBodyMessage");
                let builder = {
                    let wrapper = &self.from_fingerprint;
                    builder.field("from_fingerprint", &wrapper)
                };
                let builder = {
                    let wrapper = &self.to_fingerprint;
                    builder.field("to_fingerprint", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.application)
                    };
                    builder.field("application", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.extension)
                    };
                    builder.field("extension", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.mime)
                    };
                    builder.field("mime", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.send_date)
                    };
                    builder.field("send_date", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.receive_date)
                    };
                    builder.field("receive_date", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.is_file)
                    };
                    builder.field("is_file", &wrapper)
                };
                let builder = {
                    let wrapper = &self.id;
                    builder.field("id", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.file_name)
                    };
                    builder.field("file_name", &wrapper)
                };
                builder.finish()
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub struct NewIdentity {
            #[prost(message, repeated, tag = "1")]
            pub identities: ::prost::alloc::vec::Vec<super::ApiIdentity>,
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for NewIdentity {
            #[inline]
            fn clone(&self) -> NewIdentity {
                NewIdentity {
                    identities: ::core::clone::Clone::clone(&self.identities),
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for NewIdentity {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for NewIdentity {
            #[inline]
            fn eq(&self, other: &NewIdentity) -> bool {
                self.identities == other.identities
            }
        }
        impl ::prost::Message for NewIdentity {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                for msg in &self.identities {
                    ::prost::encoding::message::encode(1u32, msg, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "NewIdentity";
                match tag {
                    1u32 => {
                        let mut value = &mut self.identities;
                        ::prost::encoding::message::merge_repeated(
                                wire_type,
                                value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "identities");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + ::prost::encoding::message::encoded_len_repeated(
                        1u32,
                        &self.identities,
                    )
            }
            fn clear(&mut self) {
                self.identities.clear();
            }
        }
        impl ::core::default::Default for NewIdentity {
            fn default() -> Self {
                NewIdentity {
                    identities: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for NewIdentity {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("NewIdentity");
                let builder = {
                    let wrapper = &self.identities;
                    builder.field("identities", &wrapper)
                };
                builder.finish()
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub enum MaybeEvent {
            #[prost(message, tag = "21")]
            NewMessage(NewMessage),
            #[prost(message, tag = "2")]
            NewIdentities(NewIdentity),
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::clone::Clone for MaybeEvent {
            #[inline]
            fn clone(&self) -> MaybeEvent {
                match self {
                    MaybeEvent::NewMessage(__self_0) => {
                        MaybeEvent::NewMessage(::core::clone::Clone::clone(__self_0))
                    }
                    MaybeEvent::NewIdentities(__self_0) => {
                        MaybeEvent::NewIdentities(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::marker::StructuralPartialEq for MaybeEvent {}
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        impl ::core::cmp::PartialEq for MaybeEvent {
            #[inline]
            fn eq(&self, other: &MaybeEvent) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            MaybeEvent::NewMessage(__self_0),
                            MaybeEvent::NewMessage(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            MaybeEvent::NewIdentities(__self_0),
                            MaybeEvent::NewIdentities(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        impl MaybeEvent {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    MaybeEvent::NewMessage(ref value) => {
                        ::prost::encoding::message::encode(21u32, &*value, buf);
                    }
                    MaybeEvent::NewIdentities(ref value) => {
                        ::prost::encoding::message::encode(2u32, &*value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<MaybeEvent>,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    21u32 => {
                        match field {
                            ::core::option::Option::Some(
                                MaybeEvent::NewMessage(ref mut value),
                            ) => {
                                ::prost::encoding::message::merge(
                                    wire_type,
                                    value,
                                    buf,
                                    ctx,
                                )
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            MaybeEvent::NewMessage(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    2u32 => {
                        match field {
                            ::core::option::Option::Some(
                                MaybeEvent::NewIdentities(ref mut value),
                            ) => {
                                ::prost::encoding::message::merge(
                                    wire_type,
                                    value,
                                    buf,
                                    ctx,
                                )
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            MaybeEvent::NewIdentities(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("invalid MaybeEvent tag: {0}", tag),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    MaybeEvent::NewMessage(ref value) => {
                        ::prost::encoding::message::encoded_len(21u32, &*value)
                    }
                    MaybeEvent::NewIdentities(ref value) => {
                        ::prost::encoding::message::encoded_len(2u32, &*value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for MaybeEvent {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    MaybeEvent::NewMessage(ref value) => {
                        let wrapper = &*value;
                        f.debug_tuple("NewMessage").field(&wrapper).finish()
                    }
                    MaybeEvent::NewIdentities(ref value) => {
                        let wrapper = &*value;
                        f.debug_tuple("NewIdentities").field(&wrapper).finish()
                    }
                }
            }
        }
    }
    #[repr(i32)]
    pub enum MessageType {
        /// p2p types
        Advertise = 0,
        DeclareHashes = 1,
        BlockHeader = 2,
        BlockSequence = 3,
        ElectLeader = 4,
        Upgrade = 5,
        RoutingMetadata = 6,
        IpAnnounce = 7,
        Identity = 8,
        Luid = 9,
        JustUkes = 10,
        Ack = 11,
        Invalid = 12,
        /// desktop types
        GetMessage = 51,
        GetIdentity = 52,
        SendMessage = 53,
        Message = 54,
        UnitResponse = 55,
        CryptoMessage = 56,
        PairingRequest = 57,
        PairingInitiate = 58,
        PairingCompleted = 59,
        PairingAck = 60,
        IdentityResponse = 61,
        ApiIdentity = 62,
        MessageResponse = 63,
        ApiHeader = 64,
        GenerateIdentity = 65,
        ImportIdentity = 66,
        ImportIdentityResponse = 67,
        GenerateIdentityResponse = 68,
        GetEvents = 69,
        DesktopEvents = 70,
        DesktopEvent = 71,
        NoBodyMessage = 72,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MessageType {
        #[inline]
        fn clone(&self) -> MessageType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for MessageType {}
    #[automatically_derived]
    impl ::core::fmt::Debug for MessageType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    MessageType::Advertise => "Advertise",
                    MessageType::DeclareHashes => "DeclareHashes",
                    MessageType::BlockHeader => "BlockHeader",
                    MessageType::BlockSequence => "BlockSequence",
                    MessageType::ElectLeader => "ElectLeader",
                    MessageType::Upgrade => "Upgrade",
                    MessageType::RoutingMetadata => "RoutingMetadata",
                    MessageType::IpAnnounce => "IpAnnounce",
                    MessageType::Identity => "Identity",
                    MessageType::Luid => "Luid",
                    MessageType::JustUkes => "JustUkes",
                    MessageType::Ack => "Ack",
                    MessageType::Invalid => "Invalid",
                    MessageType::GetMessage => "GetMessage",
                    MessageType::GetIdentity => "GetIdentity",
                    MessageType::SendMessage => "SendMessage",
                    MessageType::Message => "Message",
                    MessageType::UnitResponse => "UnitResponse",
                    MessageType::CryptoMessage => "CryptoMessage",
                    MessageType::PairingRequest => "PairingRequest",
                    MessageType::PairingInitiate => "PairingInitiate",
                    MessageType::PairingCompleted => "PairingCompleted",
                    MessageType::PairingAck => "PairingAck",
                    MessageType::IdentityResponse => "IdentityResponse",
                    MessageType::ApiIdentity => "ApiIdentity",
                    MessageType::MessageResponse => "MessageResponse",
                    MessageType::ApiHeader => "ApiHeader",
                    MessageType::GenerateIdentity => "GenerateIdentity",
                    MessageType::ImportIdentity => "ImportIdentity",
                    MessageType::ImportIdentityResponse => "ImportIdentityResponse",
                    MessageType::GenerateIdentityResponse => "GenerateIdentityResponse",
                    MessageType::GetEvents => "GetEvents",
                    MessageType::DesktopEvents => "DesktopEvents",
                    MessageType::DesktopEvent => "DesktopEvent",
                    MessageType::NoBodyMessage => "NoBodyMessage",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MessageType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MessageType {
        #[inline]
        fn eq(&self, other: &MessageType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for MessageType {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for MessageType {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for MessageType {
        #[inline]
        fn partial_cmp(
            &self,
            other: &MessageType,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for MessageType {
        #[inline]
        fn cmp(&self, other: &MessageType) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    impl MessageType {
        ///Returns `true` if `value` is a variant of `MessageType`.
        pub fn is_valid(value: i32) -> bool {
            match value {
                0 => true,
                1 => true,
                2 => true,
                3 => true,
                4 => true,
                5 => true,
                6 => true,
                7 => true,
                8 => true,
                9 => true,
                10 => true,
                11 => true,
                12 => true,
                51 => true,
                52 => true,
                53 => true,
                54 => true,
                55 => true,
                56 => true,
                57 => true,
                58 => true,
                59 => true,
                60 => true,
                61 => true,
                62 => true,
                63 => true,
                64 => true,
                65 => true,
                66 => true,
                67 => true,
                68 => true,
                69 => true,
                70 => true,
                71 => true,
                72 => true,
                _ => false,
            }
        }
        #[deprecated = "Use the TryFrom<i32> implementation instead"]
        ///Converts an `i32` to a `MessageType`, or `None` if `value` is not a valid variant.
        pub fn from_i32(value: i32) -> ::core::option::Option<MessageType> {
            match value {
                0 => ::core::option::Option::Some(MessageType::Advertise),
                1 => ::core::option::Option::Some(MessageType::DeclareHashes),
                2 => ::core::option::Option::Some(MessageType::BlockHeader),
                3 => ::core::option::Option::Some(MessageType::BlockSequence),
                4 => ::core::option::Option::Some(MessageType::ElectLeader),
                5 => ::core::option::Option::Some(MessageType::Upgrade),
                6 => ::core::option::Option::Some(MessageType::RoutingMetadata),
                7 => ::core::option::Option::Some(MessageType::IpAnnounce),
                8 => ::core::option::Option::Some(MessageType::Identity),
                9 => ::core::option::Option::Some(MessageType::Luid),
                10 => ::core::option::Option::Some(MessageType::JustUkes),
                11 => ::core::option::Option::Some(MessageType::Ack),
                12 => ::core::option::Option::Some(MessageType::Invalid),
                51 => ::core::option::Option::Some(MessageType::GetMessage),
                52 => ::core::option::Option::Some(MessageType::GetIdentity),
                53 => ::core::option::Option::Some(MessageType::SendMessage),
                54 => ::core::option::Option::Some(MessageType::Message),
                55 => ::core::option::Option::Some(MessageType::UnitResponse),
                56 => ::core::option::Option::Some(MessageType::CryptoMessage),
                57 => ::core::option::Option::Some(MessageType::PairingRequest),
                58 => ::core::option::Option::Some(MessageType::PairingInitiate),
                59 => ::core::option::Option::Some(MessageType::PairingCompleted),
                60 => ::core::option::Option::Some(MessageType::PairingAck),
                61 => ::core::option::Option::Some(MessageType::IdentityResponse),
                62 => ::core::option::Option::Some(MessageType::ApiIdentity),
                63 => ::core::option::Option::Some(MessageType::MessageResponse),
                64 => ::core::option::Option::Some(MessageType::ApiHeader),
                65 => ::core::option::Option::Some(MessageType::GenerateIdentity),
                66 => ::core::option::Option::Some(MessageType::ImportIdentity),
                67 => ::core::option::Option::Some(MessageType::ImportIdentityResponse),
                68 => ::core::option::Option::Some(MessageType::GenerateIdentityResponse),
                69 => ::core::option::Option::Some(MessageType::GetEvents),
                70 => ::core::option::Option::Some(MessageType::DesktopEvents),
                71 => ::core::option::Option::Some(MessageType::DesktopEvent),
                72 => ::core::option::Option::Some(MessageType::NoBodyMessage),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl ::core::default::Default for MessageType {
        fn default() -> MessageType {
            MessageType::Advertise
        }
    }
    impl ::core::convert::From<MessageType> for i32 {
        fn from(value: MessageType) -> i32 {
            value as i32
        }
    }
    impl ::core::convert::TryFrom<i32> for MessageType {
        type Error = ::prost::UnknownEnumValue;
        fn try_from(value: i32) -> ::core::result::Result<MessageType, Self::Error> {
            match value {
                0 => ::core::result::Result::Ok(MessageType::Advertise),
                1 => ::core::result::Result::Ok(MessageType::DeclareHashes),
                2 => ::core::result::Result::Ok(MessageType::BlockHeader),
                3 => ::core::result::Result::Ok(MessageType::BlockSequence),
                4 => ::core::result::Result::Ok(MessageType::ElectLeader),
                5 => ::core::result::Result::Ok(MessageType::Upgrade),
                6 => ::core::result::Result::Ok(MessageType::RoutingMetadata),
                7 => ::core::result::Result::Ok(MessageType::IpAnnounce),
                8 => ::core::result::Result::Ok(MessageType::Identity),
                9 => ::core::result::Result::Ok(MessageType::Luid),
                10 => ::core::result::Result::Ok(MessageType::JustUkes),
                11 => ::core::result::Result::Ok(MessageType::Ack),
                12 => ::core::result::Result::Ok(MessageType::Invalid),
                51 => ::core::result::Result::Ok(MessageType::GetMessage),
                52 => ::core::result::Result::Ok(MessageType::GetIdentity),
                53 => ::core::result::Result::Ok(MessageType::SendMessage),
                54 => ::core::result::Result::Ok(MessageType::Message),
                55 => ::core::result::Result::Ok(MessageType::UnitResponse),
                56 => ::core::result::Result::Ok(MessageType::CryptoMessage),
                57 => ::core::result::Result::Ok(MessageType::PairingRequest),
                58 => ::core::result::Result::Ok(MessageType::PairingInitiate),
                59 => ::core::result::Result::Ok(MessageType::PairingCompleted),
                60 => ::core::result::Result::Ok(MessageType::PairingAck),
                61 => ::core::result::Result::Ok(MessageType::IdentityResponse),
                62 => ::core::result::Result::Ok(MessageType::ApiIdentity),
                63 => ::core::result::Result::Ok(MessageType::MessageResponse),
                64 => ::core::result::Result::Ok(MessageType::ApiHeader),
                65 => ::core::result::Result::Ok(MessageType::GenerateIdentity),
                66 => ::core::result::Result::Ok(MessageType::ImportIdentity),
                67 => ::core::result::Result::Ok(MessageType::ImportIdentityResponse),
                68 => ::core::result::Result::Ok(MessageType::GenerateIdentityResponse),
                69 => ::core::result::Result::Ok(MessageType::GetEvents),
                70 => ::core::result::Result::Ok(MessageType::DesktopEvents),
                71 => ::core::result::Result::Ok(MessageType::DesktopEvent),
                72 => ::core::result::Result::Ok(MessageType::NoBodyMessage),
                _ => ::core::result::Result::Err(::prost::UnknownEnumValue(value)),
            }
        }
    }
    impl MessageType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MessageType::Advertise => "ADVERTISE",
                MessageType::DeclareHashes => "DECLARE_HASHES",
                MessageType::BlockHeader => "BLOCK_HEADER",
                MessageType::BlockSequence => "BLOCK_SEQUENCE",
                MessageType::ElectLeader => "ELECT_LEADER",
                MessageType::Upgrade => "UPGRADE",
                MessageType::RoutingMetadata => "ROUTING_METADATA",
                MessageType::IpAnnounce => "IP_ANNOUNCE",
                MessageType::Identity => "IDENTITY",
                MessageType::Luid => "LUID",
                MessageType::JustUkes => "JUST_UKES",
                MessageType::Ack => "ACK",
                MessageType::Invalid => "INVALID",
                MessageType::GetMessage => "GET_MESSAGE",
                MessageType::GetIdentity => "GET_IDENTITY",
                MessageType::SendMessage => "SEND_MESSAGE",
                MessageType::Message => "MESSAGE",
                MessageType::UnitResponse => "UNIT_RESPONSE",
                MessageType::CryptoMessage => "CRYPTO_MESSAGE",
                MessageType::PairingRequest => "PAIRING_REQUEST",
                MessageType::PairingInitiate => "PAIRING_INITIATE",
                MessageType::PairingCompleted => "PAIRING_COMPLETED",
                MessageType::PairingAck => "PAIRING_ACK",
                MessageType::IdentityResponse => "IDENTITY_RESPONSE",
                MessageType::ApiIdentity => "API_IDENTITY",
                MessageType::MessageResponse => "MESSAGE_RESPONSE",
                MessageType::ApiHeader => "API_HEADER",
                MessageType::GenerateIdentity => "GENERATE_IDENTITY",
                MessageType::ImportIdentity => "IMPORT_IDENTITY",
                MessageType::ImportIdentityResponse => "IMPORT_IDENTITY_RESPONSE",
                MessageType::GenerateIdentityResponse => "GENERATE_IDENTITY_RESPONSE",
                MessageType::GetEvents => "GET_EVENTS",
                MessageType::DesktopEvents => "DESKTOP_EVENTS",
                MessageType::DesktopEvent => "DESKTOP_EVENT",
                MessageType::NoBodyMessage => "NO_BODY_MESSAGE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ADVERTISE" => Some(Self::Advertise),
                "DECLARE_HASHES" => Some(Self::DeclareHashes),
                "BLOCK_HEADER" => Some(Self::BlockHeader),
                "BLOCK_SEQUENCE" => Some(Self::BlockSequence),
                "ELECT_LEADER" => Some(Self::ElectLeader),
                "UPGRADE" => Some(Self::Upgrade),
                "ROUTING_METADATA" => Some(Self::RoutingMetadata),
                "IP_ANNOUNCE" => Some(Self::IpAnnounce),
                "IDENTITY" => Some(Self::Identity),
                "LUID" => Some(Self::Luid),
                "JUST_UKES" => Some(Self::JustUkes),
                "ACK" => Some(Self::Ack),
                "INVALID" => Some(Self::Invalid),
                "GET_MESSAGE" => Some(Self::GetMessage),
                "GET_IDENTITY" => Some(Self::GetIdentity),
                "SEND_MESSAGE" => Some(Self::SendMessage),
                "MESSAGE" => Some(Self::Message),
                "UNIT_RESPONSE" => Some(Self::UnitResponse),
                "CRYPTO_MESSAGE" => Some(Self::CryptoMessage),
                "PAIRING_REQUEST" => Some(Self::PairingRequest),
                "PAIRING_INITIATE" => Some(Self::PairingInitiate),
                "PAIRING_COMPLETED" => Some(Self::PairingCompleted),
                "PAIRING_ACK" => Some(Self::PairingAck),
                "IDENTITY_RESPONSE" => Some(Self::IdentityResponse),
                "API_IDENTITY" => Some(Self::ApiIdentity),
                "MESSAGE_RESPONSE" => Some(Self::MessageResponse),
                "API_HEADER" => Some(Self::ApiHeader),
                "GENERATE_IDENTITY" => Some(Self::GenerateIdentity),
                "IMPORT_IDENTITY" => Some(Self::ImportIdentity),
                "IMPORT_IDENTITY_RESPONSE" => Some(Self::ImportIdentityResponse),
                "GENERATE_IDENTITY_RESPONSE" => Some(Self::GenerateIdentityResponse),
                "GET_EVENTS" => Some(Self::GetEvents),
                "DESKTOP_EVENTS" => Some(Self::DesktopEvents),
                "DESKTOP_EVENT" => Some(Self::DesktopEvent),
                "NO_BODY_MESSAGE" => Some(Self::NoBodyMessage),
                _ => None,
            }
        }
    }
    /// Symmetry breaking role used for bootstrapping to another protocol
    #[repr(i32)]
    pub enum Role {
        Uke = 0,
        Seme = 1,
        SuperUke = 2,
        SuperSeme = 3,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Role {
        #[inline]
        fn clone(&self) -> Role {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Role {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Role {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Role::Uke => "Uke",
                    Role::Seme => "Seme",
                    Role::SuperUke => "SuperUke",
                    Role::SuperSeme => "SuperSeme",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Role {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Role {
        #[inline]
        fn eq(&self, other: &Role) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Role {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Role {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Role {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Role,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Role {
        #[inline]
        fn cmp(&self, other: &Role) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    impl Role {
        ///Returns `true` if `value` is a variant of `Role`.
        pub fn is_valid(value: i32) -> bool {
            match value {
                0 => true,
                1 => true,
                2 => true,
                3 => true,
                _ => false,
            }
        }
        #[deprecated = "Use the TryFrom<i32> implementation instead"]
        ///Converts an `i32` to a `Role`, or `None` if `value` is not a valid variant.
        pub fn from_i32(value: i32) -> ::core::option::Option<Role> {
            match value {
                0 => ::core::option::Option::Some(Role::Uke),
                1 => ::core::option::Option::Some(Role::Seme),
                2 => ::core::option::Option::Some(Role::SuperUke),
                3 => ::core::option::Option::Some(Role::SuperSeme),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl ::core::default::Default for Role {
        fn default() -> Role {
            Role::Uke
        }
    }
    impl ::core::convert::From<Role> for i32 {
        fn from(value: Role) -> i32 {
            value as i32
        }
    }
    impl ::core::convert::TryFrom<i32> for Role {
        type Error = ::prost::UnknownEnumValue;
        fn try_from(value: i32) -> ::core::result::Result<Role, Self::Error> {
            match value {
                0 => ::core::result::Result::Ok(Role::Uke),
                1 => ::core::result::Result::Ok(Role::Seme),
                2 => ::core::result::Result::Ok(Role::SuperUke),
                3 => ::core::result::Result::Ok(Role::SuperSeme),
                _ => ::core::result::Result::Err(::prost::UnknownEnumValue(value)),
            }
        }
    }
    impl Role {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Role::Uke => "UKE",
                Role::Seme => "SEME",
                Role::SuperUke => "SUPER_UKE",
                Role::SuperSeme => "SUPER_SEME",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UKE" => Some(Self::Uke),
                "SEME" => Some(Self::Seme),
                "SUPER_UKE" => Some(Self::SuperUke),
                "SUPER_SEME" => Some(Self::SuperSeme),
                _ => None,
            }
        }
    }
    /// Response code for pairing operations
    #[repr(i32)]
    pub enum RespCode {
        Ok = 0,
        Err = 1,
        Denied = 2,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RespCode {
        #[inline]
        fn clone(&self) -> RespCode {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for RespCode {}
    #[automatically_derived]
    impl ::core::fmt::Debug for RespCode {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    RespCode::Ok => "Ok",
                    RespCode::Err => "Err",
                    RespCode::Denied => "Denied",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RespCode {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RespCode {
        #[inline]
        fn eq(&self, other: &RespCode) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for RespCode {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for RespCode {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for RespCode {
        #[inline]
        fn partial_cmp(
            &self,
            other: &RespCode,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for RespCode {
        #[inline]
        fn cmp(&self, other: &RespCode) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    impl RespCode {
        ///Returns `true` if `value` is a variant of `RespCode`.
        pub fn is_valid(value: i32) -> bool {
            match value {
                0 => true,
                1 => true,
                2 => true,
                _ => false,
            }
        }
        #[deprecated = "Use the TryFrom<i32> implementation instead"]
        ///Converts an `i32` to a `RespCode`, or `None` if `value` is not a valid variant.
        pub fn from_i32(value: i32) -> ::core::option::Option<RespCode> {
            match value {
                0 => ::core::option::Option::Some(RespCode::Ok),
                1 => ::core::option::Option::Some(RespCode::Err),
                2 => ::core::option::Option::Some(RespCode::Denied),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl ::core::default::Default for RespCode {
        fn default() -> RespCode {
            RespCode::Ok
        }
    }
    impl ::core::convert::From<RespCode> for i32 {
        fn from(value: RespCode) -> i32 {
            value as i32
        }
    }
    impl ::core::convert::TryFrom<i32> for RespCode {
        type Error = ::prost::UnknownEnumValue;
        fn try_from(value: i32) -> ::core::result::Result<RespCode, Self::Error> {
            match value {
                0 => ::core::result::Result::Ok(RespCode::Ok),
                1 => ::core::result::Result::Ok(RespCode::Err),
                2 => ::core::result::Result::Ok(RespCode::Denied),
                _ => ::core::result::Result::Err(::prost::UnknownEnumValue(value)),
            }
        }
    }
    impl RespCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RespCode::Ok => "OK",
                RespCode::Err => "ERR",
                RespCode::Denied => "DENIED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OK" => Some(Self::Ok),
                "ERR" => Some(Self::Err),
                "DENIED" => Some(Self::Denied),
                _ => None,
            }
        }
    }
}
pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
pub fn init() -> SbResult<()> {
    sodiumoxide::init().map_err(|_| Error::Crypto("Failed to init".to_owned()))?;
    Ok(())
}
