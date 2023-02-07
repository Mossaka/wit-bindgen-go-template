#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::path::PathBuf;
use anyhow::{Context, Result};
use host::add_to_linker;
use wasi_cap_std_sync::WasiCtxBuilder;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store, WasmBacktraceDetails,
};
use crate::handler::{Method, Request};
#[allow(clippy::all)]
pub mod handler {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    /// The HTTP URI of the current request.
    pub type Uri<'a> = &'a str;
    /// The HTTP parameter queries, represented as a list of (name, value) pairs.
    pub type Params<'a> = &'a [(&'a str, &'a str)];
    /// The HTTP method.
    #[component(enum)]
    pub enum Method {
        #[component(name = "get")]
        Get,
        #[component(name = "post")]
        Post,
        #[component(name = "put")]
        Put,
        #[component(name = "delete")]
        Delete,
        #[component(name = "patch")]
        Patch,
        #[component(name = "head")]
        Head,
        #[component(name = "options")]
        Options,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Method {
        #[inline]
        fn clone(&self) -> Method {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Method {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Method {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Method {
        #[inline]
        fn eq(&self, other: &Method) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Method {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Method {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    unsafe impl wasmtime::component::Lower for Method {
        #[inline]
        fn lower<T>(
            &self,
            store: &mut wasmtime::StoreContextMut<T>,
            options: &wasmtime::component::__internal::Options,
            dst: &mut std::mem::MaybeUninit<Self::Lower>,
        ) -> wasmtime::component::__internal::anyhow::Result<()> {
            match self {
                Self::Get => {
                    {
                        #[allow(unused_unsafe)]
                        {
                            unsafe {
                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                m.map(|p| &raw mut (*p).tag)
                            }
                        }
                    }
                    .write(wasmtime::ValRaw::u32(0u32));
                    unsafe {
                        wasmtime::component::__internal::lower_payload(
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).payload)
                                    }
                                }
                            },
                            |payload| {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = payload;
                                        m.map(|p| &raw mut (*p).Get)
                                    }
                                }
                            },
                            |dst| Ok(()),
                        )
                    }
                }
                Self::Post => {
                    {
                        #[allow(unused_unsafe)]
                        {
                            unsafe {
                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                m.map(|p| &raw mut (*p).tag)
                            }
                        }
                    }
                    .write(wasmtime::ValRaw::u32(1u32));
                    unsafe {
                        wasmtime::component::__internal::lower_payload(
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).payload)
                                    }
                                }
                            },
                            |payload| {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = payload;
                                        m.map(|p| &raw mut (*p).Post)
                                    }
                                }
                            },
                            |dst| Ok(()),
                        )
                    }
                }
                Self::Put => {
                    {
                        #[allow(unused_unsafe)]
                        {
                            unsafe {
                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                m.map(|p| &raw mut (*p).tag)
                            }
                        }
                    }
                    .write(wasmtime::ValRaw::u32(2u32));
                    unsafe {
                        wasmtime::component::__internal::lower_payload(
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).payload)
                                    }
                                }
                            },
                            |payload| {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = payload;
                                        m.map(|p| &raw mut (*p).Put)
                                    }
                                }
                            },
                            |dst| Ok(()),
                        )
                    }
                }
                Self::Delete => {
                    {
                        #[allow(unused_unsafe)]
                        {
                            unsafe {
                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                m.map(|p| &raw mut (*p).tag)
                            }
                        }
                    }
                    .write(wasmtime::ValRaw::u32(3u32));
                    unsafe {
                        wasmtime::component::__internal::lower_payload(
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).payload)
                                    }
                                }
                            },
                            |payload| {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = payload;
                                        m.map(|p| &raw mut (*p).Delete)
                                    }
                                }
                            },
                            |dst| Ok(()),
                        )
                    }
                }
                Self::Patch => {
                    {
                        #[allow(unused_unsafe)]
                        {
                            unsafe {
                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                m.map(|p| &raw mut (*p).tag)
                            }
                        }
                    }
                    .write(wasmtime::ValRaw::u32(4u32));
                    unsafe {
                        wasmtime::component::__internal::lower_payload(
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).payload)
                                    }
                                }
                            },
                            |payload| {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = payload;
                                        m.map(|p| &raw mut (*p).Patch)
                                    }
                                }
                            },
                            |dst| Ok(()),
                        )
                    }
                }
                Self::Head => {
                    {
                        #[allow(unused_unsafe)]
                        {
                            unsafe {
                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                m.map(|p| &raw mut (*p).tag)
                            }
                        }
                    }
                    .write(wasmtime::ValRaw::u32(5u32));
                    unsafe {
                        wasmtime::component::__internal::lower_payload(
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).payload)
                                    }
                                }
                            },
                            |payload| {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = payload;
                                        m.map(|p| &raw mut (*p).Head)
                                    }
                                }
                            },
                            |dst| Ok(()),
                        )
                    }
                }
                Self::Options => {
                    {
                        #[allow(unused_unsafe)]
                        {
                            unsafe {
                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                m.map(|p| &raw mut (*p).tag)
                            }
                        }
                    }
                    .write(wasmtime::ValRaw::u32(6u32));
                    unsafe {
                        wasmtime::component::__internal::lower_payload(
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).payload)
                                    }
                                }
                            },
                            |payload| {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = payload;
                                        m.map(|p| &raw mut (*p).Options)
                                    }
                                }
                            },
                            |dst| Ok(()),
                        )
                    }
                }
            }
        }
        #[inline]
        fn store<T>(
            &self,
            memory: &mut wasmtime::component::__internal::MemoryMut<'_, T>,
            mut offset: usize,
        ) -> wasmtime::component::__internal::anyhow::Result<()> {
            if true {
                if !(offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0)
                {
                    :: core :: panicking :: panic ("assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0")
                };
            };
            match self {
                Self::Get => {
                    *memory.get::<1usize>(offset) = 0u8.to_le_bytes();
                    Ok(())
                }
                Self::Post => {
                    *memory.get::<1usize>(offset) = 1u8.to_le_bytes();
                    Ok(())
                }
                Self::Put => {
                    *memory.get::<1usize>(offset) = 2u8.to_le_bytes();
                    Ok(())
                }
                Self::Delete => {
                    *memory.get::<1usize>(offset) = 3u8.to_le_bytes();
                    Ok(())
                }
                Self::Patch => {
                    *memory.get::<1usize>(offset) = 4u8.to_le_bytes();
                    Ok(())
                }
                Self::Head => {
                    *memory.get::<1usize>(offset) = 5u8.to_le_bytes();
                    Ok(())
                }
                Self::Options => {
                    *memory.get::<1usize>(offset) = 6u8.to_le_bytes();
                    Ok(())
                }
            }
        }
    }
    unsafe impl wasmtime::component::Lift for Method {
        #[inline]
        fn lift(
            store: &wasmtime::component::__internal::StoreOpaque,
            options: &wasmtime::component::__internal::Options,
            src: &Self::Lower,
        ) -> wasmtime::component::__internal::anyhow::Result<Self> {
            Ok(match src.tag.get_u32() {
                0u32 => Self::Get,
                1u32 => Self::Post,
                2u32 => Self::Put,
                3u32 => Self::Delete,
                4u32 => Self::Patch,
                5u32 => Self::Head,
                6u32 => Self::Options,
                discrim => {
                    return ::anyhow::__private::Err(::anyhow::Error::msg({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["unexpected discriminant: "],
                            &[::core::fmt::ArgumentV1::new_display(&discrim)],
                        ));
                        res
                    }))
                }
            })
        }
        #[inline]
        fn load(
            memory: &wasmtime::component::__internal::Memory,
            bytes: &[u8],
        ) -> wasmtime::component::__internal::anyhow::Result<Self> {
            let align = <Self as wasmtime::component::ComponentType>::ALIGN32;
            if true {
                if !((bytes.as_ptr() as usize) % (align as usize) == 0) {
                    ::core::panicking::panic(
                        "assertion failed: (bytes.as_ptr() as usize) % (align as usize) == 0",
                    )
                };
            };
            let discrim = bytes[0];
            let payload_offset =
                <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32;
            let payload = &bytes[payload_offset..];
            Ok(match discrim {
                0u8 => Self::Get,
                1u8 => Self::Post,
                2u8 => Self::Put,
                3u8 => Self::Delete,
                4u8 => Self::Patch,
                5u8 => Self::Head,
                6u8 => Self::Options,
                discrim => {
                    return ::anyhow::__private::Err(::anyhow::Error::msg({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["unexpected discriminant: "],
                            &[::core::fmt::ArgumentV1::new_display(&discrim)],
                        ));
                        res
                    }))
                }
            })
        }
    }
    const _: () = {
        #[doc(hidden)]
        #[repr(C)]
        pub struct LowerMethod {
            tag: wasmtime::ValRaw,
            payload: LowerPayloadMethod,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for LowerMethod {
            #[inline]
            fn clone(&self) -> LowerMethod {
                let _: ::core::clone::AssertParamIsClone<wasmtime::ValRaw>;
                let _: ::core::clone::AssertParamIsClone<LowerPayloadMethod>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for LowerMethod {}
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[repr(C)]
        union LowerPayloadMethod {
            Get: [wasmtime::ValRaw; 0],
            Post: [wasmtime::ValRaw; 0],
            Put: [wasmtime::ValRaw; 0],
            Delete: [wasmtime::ValRaw; 0],
            Patch: [wasmtime::ValRaw; 0],
            Head: [wasmtime::ValRaw; 0],
            Options: [wasmtime::ValRaw; 0],
        }
        #[automatically_derived]
        #[allow(non_snake_case)]
        impl ::core::clone::Clone for LowerPayloadMethod {
            #[inline]
            fn clone(&self) -> LowerPayloadMethod {
                let _: ::core::clone::AssertParamIsCopy<Self>;
                *self
            }
        }
        #[automatically_derived]
        #[allow(non_snake_case)]
        impl ::core::marker::Copy for LowerPayloadMethod {}
        unsafe impl wasmtime::component::ComponentType for Method {
            type Lower = LowerMethod;
            #[inline]
            fn typecheck(
                ty: &wasmtime::component::__internal::InterfaceType,
                types: &wasmtime::component::__internal::ComponentTypes,
            ) -> wasmtime::component::__internal::anyhow::Result<()> {
                wasmtime::component::__internal::typecheck_enum(
                    ty,
                    types,
                    &["get", "post", "put", "delete", "patch", "head", "options"],
                )
            }
            const ABI: wasmtime::component::__internal::CanonicalAbiInfo =
                wasmtime::component::__internal::CanonicalAbiInfo::variant_static(&[
                    None, None, None, None, None, None, None,
                ]);
        }
        unsafe impl wasmtime::component::__internal::ComponentVariant for Method {
            const CASES: &'static [Option<wasmtime::component::__internal::CanonicalAbiInfo>] =
                &[None, None, None, None, None, None, None];
        }
    };
    impl core::fmt::Debug for Method {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Method::Get => f.debug_tuple("Method::Get").finish(),
                Method::Post => f.debug_tuple("Method::Post").finish(),
                Method::Put => f.debug_tuple("Method::Put").finish(),
                Method::Delete => f.debug_tuple("Method::Delete").finish(),
                Method::Patch => f.debug_tuple("Method::Patch").finish(),
                Method::Head => f.debug_tuple("Method::Head").finish(),
                Method::Options => f.debug_tuple("Method::Options").finish(),
            }
        }
    }
    /// The HTTP status code.
    pub type HttpStatus = u16;
    /// HTTP errors returned by the runtime.
    #[component(variant)]
    pub enum HttpError {
        #[component(name = "invalid-url")]
        InvalidUrl(String),
        #[component(name = "timeout-error")]
        TimeoutError(String),
        #[component(name = "protocol-error")]
        ProtocolError(String),
        #[component(name = "status-error")]
        StatusError(u16),
        #[component(name = "unexpected-error")]
        UnexpectedError(String),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for HttpError {
        #[inline]
        fn clone(&self) -> HttpError {
            match self {
                HttpError::InvalidUrl(__self_0) => {
                    HttpError::InvalidUrl(::core::clone::Clone::clone(__self_0))
                }
                HttpError::TimeoutError(__self_0) => {
                    HttpError::TimeoutError(::core::clone::Clone::clone(__self_0))
                }
                HttpError::ProtocolError(__self_0) => {
                    HttpError::ProtocolError(::core::clone::Clone::clone(__self_0))
                }
                HttpError::StatusError(__self_0) => {
                    HttpError::StatusError(::core::clone::Clone::clone(__self_0))
                }
                HttpError::UnexpectedError(__self_0) => {
                    HttpError::UnexpectedError(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    unsafe impl wasmtime::component::Lower for HttpError {
        #[inline]
        fn lower<T>(
            &self,
            store: &mut wasmtime::StoreContextMut<T>,
            options: &wasmtime::component::__internal::Options,
            dst: &mut std::mem::MaybeUninit<Self::Lower>,
        ) -> wasmtime::component::__internal::anyhow::Result<()> {
            match self {
                Self::InvalidUrl(value) => {
                    {
                        #[allow(unused_unsafe)]
                        {
                            unsafe {
                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                m.map(|p| &raw mut (*p).tag)
                            }
                        }
                    }
                    .write(wasmtime::ValRaw::u32(0u32));
                    unsafe {
                        wasmtime::component::__internal::lower_payload(
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).payload)
                                    }
                                }
                            },
                            |payload| {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = payload;
                                        m.map(|p| &raw mut (*p).InvalidUrl)
                                    }
                                }
                            },
                            |dst| value.lower(store, options, dst),
                        )
                    }
                }
                Self::TimeoutError(value) => {
                    {
                        #[allow(unused_unsafe)]
                        {
                            unsafe {
                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                m.map(|p| &raw mut (*p).tag)
                            }
                        }
                    }
                    .write(wasmtime::ValRaw::u32(1u32));
                    unsafe {
                        wasmtime::component::__internal::lower_payload(
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).payload)
                                    }
                                }
                            },
                            |payload| {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = payload;
                                        m.map(|p| &raw mut (*p).TimeoutError)
                                    }
                                }
                            },
                            |dst| value.lower(store, options, dst),
                        )
                    }
                }
                Self::ProtocolError(value) => {
                    {
                        #[allow(unused_unsafe)]
                        {
                            unsafe {
                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                m.map(|p| &raw mut (*p).tag)
                            }
                        }
                    }
                    .write(wasmtime::ValRaw::u32(2u32));
                    unsafe {
                        wasmtime::component::__internal::lower_payload(
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).payload)
                                    }
                                }
                            },
                            |payload| {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = payload;
                                        m.map(|p| &raw mut (*p).ProtocolError)
                                    }
                                }
                            },
                            |dst| value.lower(store, options, dst),
                        )
                    }
                }
                Self::StatusError(value) => {
                    {
                        #[allow(unused_unsafe)]
                        {
                            unsafe {
                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                m.map(|p| &raw mut (*p).tag)
                            }
                        }
                    }
                    .write(wasmtime::ValRaw::u32(3u32));
                    unsafe {
                        wasmtime::component::__internal::lower_payload(
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).payload)
                                    }
                                }
                            },
                            |payload| {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = payload;
                                        m.map(|p| &raw mut (*p).StatusError)
                                    }
                                }
                            },
                            |dst| value.lower(store, options, dst),
                        )
                    }
                }
                Self::UnexpectedError(value) => {
                    {
                        #[allow(unused_unsafe)]
                        {
                            unsafe {
                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                m.map(|p| &raw mut (*p).tag)
                            }
                        }
                    }
                    .write(wasmtime::ValRaw::u32(4u32));
                    unsafe {
                        wasmtime::component::__internal::lower_payload(
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).payload)
                                    }
                                }
                            },
                            |payload| {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = payload;
                                        m.map(|p| &raw mut (*p).UnexpectedError)
                                    }
                                }
                            },
                            |dst| value.lower(store, options, dst),
                        )
                    }
                }
            }
        }
        #[inline]
        fn store<T>(
            &self,
            memory: &mut wasmtime::component::__internal::MemoryMut<'_, T>,
            mut offset: usize,
        ) -> wasmtime::component::__internal::anyhow::Result<()> {
            if true {
                if !(offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0)
                {
                    :: core :: panicking :: panic ("assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0")
                };
            };
            match self {
                Self::InvalidUrl(value) => {
                    *memory.get::<1usize>(offset) = 0u8.to_le_bytes();
                    value . store (memory , offset + < Self as wasmtime :: component :: __internal :: ComponentVariant > :: PAYLOAD_OFFSET32)
                }
                Self::TimeoutError(value) => {
                    *memory.get::<1usize>(offset) = 1u8.to_le_bytes();
                    value . store (memory , offset + < Self as wasmtime :: component :: __internal :: ComponentVariant > :: PAYLOAD_OFFSET32)
                }
                Self::ProtocolError(value) => {
                    *memory.get::<1usize>(offset) = 2u8.to_le_bytes();
                    value . store (memory , offset + < Self as wasmtime :: component :: __internal :: ComponentVariant > :: PAYLOAD_OFFSET32)
                }
                Self::StatusError(value) => {
                    *memory.get::<1usize>(offset) = 3u8.to_le_bytes();
                    value . store (memory , offset + < Self as wasmtime :: component :: __internal :: ComponentVariant > :: PAYLOAD_OFFSET32)
                }
                Self::UnexpectedError(value) => {
                    *memory.get::<1usize>(offset) = 4u8.to_le_bytes();
                    value . store (memory , offset + < Self as wasmtime :: component :: __internal :: ComponentVariant > :: PAYLOAD_OFFSET32)
                }
            }
        }
    }
    unsafe impl wasmtime::component::Lift for HttpError {
        #[inline]
        fn lift(
            store: &wasmtime::component::__internal::StoreOpaque,
            options: &wasmtime::component::__internal::Options,
            src: &Self::Lower,
        ) -> wasmtime::component::__internal::anyhow::Result<Self> {
            Ok(match src.tag.get_u32() {
                0u32 => Self::InvalidUrl(<String as wasmtime::component::Lift>::lift(
                    store,
                    options,
                    unsafe { &src.payload.InvalidUrl },
                )?),
                1u32 => Self::TimeoutError(<String as wasmtime::component::Lift>::lift(
                    store,
                    options,
                    unsafe { &src.payload.TimeoutError },
                )?),
                2u32 => Self::ProtocolError(<String as wasmtime::component::Lift>::lift(
                    store,
                    options,
                    unsafe { &src.payload.ProtocolError },
                )?),
                3u32 => Self::StatusError(<u16 as wasmtime::component::Lift>::lift(
                    store,
                    options,
                    unsafe { &src.payload.StatusError },
                )?),
                4u32 => Self::UnexpectedError(<String as wasmtime::component::Lift>::lift(
                    store,
                    options,
                    unsafe { &src.payload.UnexpectedError },
                )?),
                discrim => {
                    return ::anyhow::__private::Err(::anyhow::Error::msg({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["unexpected discriminant: "],
                            &[::core::fmt::ArgumentV1::new_display(&discrim)],
                        ));
                        res
                    }))
                }
            })
        }
        #[inline]
        fn load(
            memory: &wasmtime::component::__internal::Memory,
            bytes: &[u8],
        ) -> wasmtime::component::__internal::anyhow::Result<Self> {
            let align = <Self as wasmtime::component::ComponentType>::ALIGN32;
            if true {
                if !((bytes.as_ptr() as usize) % (align as usize) == 0) {
                    ::core::panicking::panic(
                        "assertion failed: (bytes.as_ptr() as usize) % (align as usize) == 0",
                    )
                };
            };
            let discrim = bytes[0];
            let payload_offset =
                <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32;
            let payload = &bytes[payload_offset..];
            Ok(match discrim {
                0u8 => Self::InvalidUrl(<String as wasmtime::component::Lift>::load(
                    memory,
                    &payload[..<String as wasmtime::component::ComponentType>::SIZE32],
                )?),
                1u8 => Self::TimeoutError(<String as wasmtime::component::Lift>::load(
                    memory,
                    &payload[..<String as wasmtime::component::ComponentType>::SIZE32],
                )?),
                2u8 => Self::ProtocolError(<String as wasmtime::component::Lift>::load(
                    memory,
                    &payload[..<String as wasmtime::component::ComponentType>::SIZE32],
                )?),
                3u8 => Self::StatusError(<u16 as wasmtime::component::Lift>::load(
                    memory,
                    &payload[..<u16 as wasmtime::component::ComponentType>::SIZE32],
                )?),
                4u8 => Self::UnexpectedError(<String as wasmtime::component::Lift>::load(
                    memory,
                    &payload[..<String as wasmtime::component::ComponentType>::SIZE32],
                )?),
                discrim => {
                    return ::anyhow::__private::Err(::anyhow::Error::msg({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["unexpected discriminant: "],
                            &[::core::fmt::ArgumentV1::new_display(&discrim)],
                        ));
                        res
                    }))
                }
            })
        }
    }
    const _: () = {
        #[doc(hidden)]
        #[repr(C)]
        pub struct LowerHttpError<T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy> {
            tag: wasmtime::ValRaw,
            payload: LowerPayloadHttpError<T0, T1, T2, T3, T4>,
        }
        #[automatically_derived]
        impl<
                T0: ::core::clone::Clone + Copy,
                T1: ::core::clone::Clone + Copy,
                T2: ::core::clone::Clone + Copy,
                T3: ::core::clone::Clone + Copy,
                T4: ::core::clone::Clone + Copy,
            > ::core::clone::Clone for LowerHttpError<T0, T1, T2, T3, T4>
        {
            #[inline]
            fn clone(&self) -> LowerHttpError<T0, T1, T2, T3, T4> {
                LowerHttpError {
                    tag: ::core::clone::Clone::clone(&self.tag),
                    payload: ::core::clone::Clone::clone(&self.payload),
                }
            }
        }
        #[automatically_derived]
        impl<
                T0: ::core::marker::Copy + Copy,
                T1: ::core::marker::Copy + Copy,
                T2: ::core::marker::Copy + Copy,
                T3: ::core::marker::Copy + Copy,
                T4: ::core::marker::Copy + Copy,
            > ::core::marker::Copy for LowerHttpError<T0, T1, T2, T3, T4>
        {
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[repr(C)]
        union LowerPayloadHttpError<T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy> {
            InvalidUrl: T0,
            TimeoutError: T1,
            ProtocolError: T2,
            StatusError: T3,
            UnexpectedError: T4,
        }
        #[automatically_derived]
        #[allow(non_snake_case)]
        impl<
                T0: ::core::marker::Copy + ::core::clone::Clone + Copy,
                T1: ::core::marker::Copy + ::core::clone::Clone + Copy,
                T2: ::core::marker::Copy + ::core::clone::Clone + Copy,
                T3: ::core::marker::Copy + ::core::clone::Clone + Copy,
                T4: ::core::marker::Copy + ::core::clone::Clone + Copy,
            > ::core::clone::Clone for LowerPayloadHttpError<T0, T1, T2, T3, T4>
        {
            #[inline]
            fn clone(&self) -> LowerPayloadHttpError<T0, T1, T2, T3, T4> {
                let _: ::core::clone::AssertParamIsCopy<Self>;
                *self
            }
        }
        #[automatically_derived]
        #[allow(non_snake_case)]
        impl<
                T0: ::core::marker::Copy + Copy,
                T1: ::core::marker::Copy + Copy,
                T2: ::core::marker::Copy + Copy,
                T3: ::core::marker::Copy + Copy,
                T4: ::core::marker::Copy + Copy,
            > ::core::marker::Copy for LowerPayloadHttpError<T0, T1, T2, T3, T4>
        {
        }
        unsafe impl wasmtime::component::ComponentType for HttpError {
            type Lower = LowerHttpError<
                <String as wasmtime::component::ComponentType>::Lower,
                <String as wasmtime::component::ComponentType>::Lower,
                <String as wasmtime::component::ComponentType>::Lower,
                <u16 as wasmtime::component::ComponentType>::Lower,
                <String as wasmtime::component::ComponentType>::Lower,
            >;
            #[inline]
            fn typecheck(
                ty: &wasmtime::component::__internal::InterfaceType,
                types: &wasmtime::component::__internal::ComponentTypes,
            ) -> wasmtime::component::__internal::anyhow::Result<()> {
                wasmtime::component::__internal::typecheck_variant(
                    ty,
                    types,
                    &[
                        (
                            "invalid-url",
                            Some(<String as wasmtime::component::ComponentType>::typecheck),
                        ),
                        (
                            "timeout-error",
                            Some(<String as wasmtime::component::ComponentType>::typecheck),
                        ),
                        (
                            "protocol-error",
                            Some(<String as wasmtime::component::ComponentType>::typecheck),
                        ),
                        (
                            "status-error",
                            Some(<u16 as wasmtime::component::ComponentType>::typecheck),
                        ),
                        (
                            "unexpected-error",
                            Some(<String as wasmtime::component::ComponentType>::typecheck),
                        ),
                    ],
                )
            }
            const ABI: wasmtime::component::__internal::CanonicalAbiInfo =
                wasmtime::component::__internal::CanonicalAbiInfo::variant_static(&[
                    Some(<String as wasmtime::component::ComponentType>::ABI),
                    Some(<String as wasmtime::component::ComponentType>::ABI),
                    Some(<String as wasmtime::component::ComponentType>::ABI),
                    Some(<u16 as wasmtime::component::ComponentType>::ABI),
                    Some(<String as wasmtime::component::ComponentType>::ABI),
                ]);
        }
        unsafe impl wasmtime::component::__internal::ComponentVariant for HttpError {
            const CASES: &'static [Option<wasmtime::component::__internal::CanonicalAbiInfo>] = &[
                Some(<String as wasmtime::component::ComponentType>::ABI),
                Some(<String as wasmtime::component::ComponentType>::ABI),
                Some(<String as wasmtime::component::ComponentType>::ABI),
                Some(<u16 as wasmtime::component::ComponentType>::ABI),
                Some(<String as wasmtime::component::ComponentType>::ABI),
            ];
        }
    };
    impl core::fmt::Debug for HttpError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                HttpError::InvalidUrl(e) => {
                    f.debug_tuple("HttpError::InvalidUrl").field(e).finish()
                }
                HttpError::TimeoutError(e) => {
                    f.debug_tuple("HttpError::TimeoutError").field(e).finish()
                }
                HttpError::ProtocolError(e) => {
                    f.debug_tuple("HttpError::ProtocolError").field(e).finish()
                }
                HttpError::StatusError(e) => {
                    f.debug_tuple("HttpError::StatusError").field(e).finish()
                }
                HttpError::UnexpectedError(e) => f
                    .debug_tuple("HttpError::UnexpectedError")
                    .field(e)
                    .finish(),
            }
        }
    }
    impl core::fmt::Display for HttpError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &[::core::fmt::ArgumentV1::new_debug(&self)],
            ))
        }
    }
    impl std::error::Error for HttpError {}
    /// The HTTP headers represented as a list of (name, value) pairs.
    pub type HeadersParam<'a> = &'a [(&'a str, &'a str)];
    /// The HTTP headers represented as a list of (name, value) pairs.
    pub type HeadersResult = Vec<(String, String)>;
    /// The HTTP body.
    pub type BodyParam<'a> = &'a [u8];
    /// The HTTP body.
    pub type BodyResult = Vec<u8>;
    /// An HTTP response.
    #[component(record)]
    pub struct Response {
        #[component(name = "status")]
        pub status: HttpStatus,
        #[component(name = "headers")]
        pub headers: Option<HeadersResult>,
        #[component(name = "body")]
        pub body: Option<BodyResult>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Response {
        #[inline]
        fn clone(&self) -> Response {
            Response {
                status: ::core::clone::Clone::clone(&self.status),
                headers: ::core::clone::Clone::clone(&self.headers),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    unsafe impl wasmtime::component::Lower for Response {
        #[inline]
        fn lower<T>(
            &self,
            store: &mut wasmtime::StoreContextMut<T>,
            options: &wasmtime::component::__internal::Options,
            dst: &mut std::mem::MaybeUninit<Self::Lower>,
        ) -> wasmtime::component::__internal::anyhow::Result<()> {
            wasmtime::component::Lower::lower(&self.status, store, options, {
                #[allow(unused_unsafe)]
                {
                    unsafe {
                        use ::wasmtime::component::__internal::MaybeUninitExt;
                        let m: &mut std::mem::MaybeUninit<_> = dst;
                        m.map(|p| &raw mut (*p).status)
                    }
                }
            })?;
            wasmtime::component::Lower::lower(&self.headers, store, options, {
                #[allow(unused_unsafe)]
                {
                    unsafe {
                        use ::wasmtime::component::__internal::MaybeUninitExt;
                        let m: &mut std::mem::MaybeUninit<_> = dst;
                        m.map(|p| &raw mut (*p).headers)
                    }
                }
            })?;
            wasmtime::component::Lower::lower(&self.body, store, options, {
                #[allow(unused_unsafe)]
                {
                    unsafe {
                        use ::wasmtime::component::__internal::MaybeUninitExt;
                        let m: &mut std::mem::MaybeUninit<_> = dst;
                        m.map(|p| &raw mut (*p).body)
                    }
                }
            })?;
            Ok(())
        }
        #[inline]
        fn store<T>(
            &self,
            memory: &mut wasmtime::component::__internal::MemoryMut<'_, T>,
            mut offset: usize,
        ) -> wasmtime::component::__internal::anyhow::Result<()> {
            if true {
                if !(offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0)
                {
                    :: core :: panicking :: panic ("assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0")
                };
            };
            wasmtime::component::Lower::store(
                &self.status,
                memory,
                <HttpStatus as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.headers,
                memory,
                <Option<HeadersResult> as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.body,
                memory,
                <Option<BodyResult> as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            Ok(())
        }
    }
    unsafe impl wasmtime::component::Lift for Response {
        #[inline]
        fn lift(
            store: &wasmtime::component::__internal::StoreOpaque,
            options: &wasmtime::component::__internal::Options,
            src: &Self::Lower,
        ) -> wasmtime::component::__internal::anyhow::Result<Self> {
            Ok(Self {
                status: <HttpStatus as wasmtime::component::Lift>::lift(
                    store,
                    options,
                    &src.status,
                )?,
                headers: <Option<HeadersResult> as wasmtime::component::Lift>::lift(
                    store,
                    options,
                    &src.headers,
                )?,
                body: <Option<BodyResult> as wasmtime::component::Lift>::lift(
                    store, options, &src.body,
                )?,
            })
        }
        #[inline]
        fn load(
            memory: &wasmtime::component::__internal::Memory,
            bytes: &[u8],
        ) -> wasmtime::component::__internal::anyhow::Result<Self> {
            if true {
                if !((bytes.as_ptr() as usize)
                    % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize)
                    == 0)
                {
                    :: core :: panicking :: panic ("assertion failed: (bytes.as_ptr() as usize) %\\n        (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0")
                };
            };
            let mut offset = 0;
            Ok(Self {
                status: <HttpStatus as wasmtime::component::Lift>::load(
                    memory,
                    &bytes[<HttpStatus as wasmtime::component::ComponentType>::ABI
                        .next_field32_size(&mut offset)..]
                        [..<HttpStatus as wasmtime::component::ComponentType>::SIZE32],
                )?,
                headers: <Option<HeadersResult> as wasmtime::component::Lift>::load(
                    memory,
                    &bytes[<Option<HeadersResult> as wasmtime::component::ComponentType>::ABI
                        .next_field32_size(&mut offset)..]
                        [..<Option<HeadersResult> as wasmtime::component::ComponentType>::SIZE32],
                )?,
                body: <Option<BodyResult> as wasmtime::component::Lift>::load(
                    memory,
                    &bytes[<Option<BodyResult> as wasmtime::component::ComponentType>::ABI
                        .next_field32_size(&mut offset)..]
                        [..<Option<BodyResult> as wasmtime::component::ComponentType>::SIZE32],
                )?,
            })
        }
    }
    const _: () = {
        #[doc(hidden)]
        #[repr(C)]
        pub struct LowerResponse<T0: Copy, T1: Copy, T2: Copy> {
            status: T0,
            headers: T1,
            body: T2,
            _align: [wasmtime::ValRaw; 0],
        }
        #[automatically_derived]
        impl<
                T0: ::core::clone::Clone + Copy,
                T1: ::core::clone::Clone + Copy,
                T2: ::core::clone::Clone + Copy,
            > ::core::clone::Clone for LowerResponse<T0, T1, T2>
        {
            #[inline]
            fn clone(&self) -> LowerResponse<T0, T1, T2> {
                LowerResponse {
                    status: ::core::clone::Clone::clone(&self.status),
                    headers: ::core::clone::Clone::clone(&self.headers),
                    body: ::core::clone::Clone::clone(&self.body),
                    _align: ::core::clone::Clone::clone(&self._align),
                }
            }
        }
        #[automatically_derived]
        impl<
                T0: ::core::marker::Copy + Copy,
                T1: ::core::marker::Copy + Copy,
                T2: ::core::marker::Copy + Copy,
            > ::core::marker::Copy for LowerResponse<T0, T1, T2>
        {
        }
        unsafe impl wasmtime::component::ComponentType for Response {
            type Lower = LowerResponse<
                <HttpStatus as wasmtime::component::ComponentType>::Lower,
                <Option<HeadersResult> as wasmtime::component::ComponentType>::Lower,
                <Option<BodyResult> as wasmtime::component::ComponentType>::Lower,
            >;
            const ABI: wasmtime::component::__internal::CanonicalAbiInfo =
                wasmtime::component::__internal::CanonicalAbiInfo::record_static(&[
                    <HttpStatus as wasmtime::component::ComponentType>::ABI,
                    <Option<HeadersResult> as wasmtime::component::ComponentType>::ABI,
                    <Option<BodyResult> as wasmtime::component::ComponentType>::ABI,
                ]);
            #[inline]
            fn typecheck(
                ty: &wasmtime::component::__internal::InterfaceType,
                types: &wasmtime::component::__internal::ComponentTypes,
            ) -> wasmtime::component::__internal::anyhow::Result<()> {
                wasmtime :: component :: __internal :: typecheck_record (ty , types , & [("status" , < HttpStatus as wasmtime :: component :: ComponentType > :: typecheck) , ("headers" , < Option < HeadersResult > as wasmtime :: component :: ComponentType > :: typecheck) , ("body" , < Option < BodyResult > as wasmtime :: component :: ComponentType > :: typecheck)])
            }
        }
    };
    impl core::fmt::Debug for Response {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Response")
                .field("status", &self.status)
                .field("headers", &self.headers)
                .field("body", &self.body)
                .finish()
        }
    }
    /// An HTTP request.
    #[component(record)]
    pub struct Request<'a> {
        #[component(name = "method")]
        pub method: Method,
        #[component(name = "uri")]
        pub uri: Uri<'a>,
        #[component(name = "headers")]
        pub headers: HeadersParam<'a>,
        #[component(name = "params")]
        pub params: Params<'a>,
        #[component(name = "body")]
        pub body: Option<BodyParam<'a>>,
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for Request<'a> {
        #[inline]
        fn clone(&self) -> Request<'a> {
            Request {
                method: ::core::clone::Clone::clone(&self.method),
                uri: ::core::clone::Clone::clone(&self.uri),
                headers: ::core::clone::Clone::clone(&self.headers),
                params: ::core::clone::Clone::clone(&self.params),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    unsafe impl<'a> wasmtime::component::Lower for Request<'a> {
        #[inline]
        fn lower<T>(
            &self,
            store: &mut wasmtime::StoreContextMut<T>,
            options: &wasmtime::component::__internal::Options,
            dst: &mut std::mem::MaybeUninit<Self::Lower>,
        ) -> wasmtime::component::__internal::anyhow::Result<()> {
            wasmtime::component::Lower::lower(&self.method, store, options, {
                #[allow(unused_unsafe)]
                {
                    unsafe {
                        use ::wasmtime::component::__internal::MaybeUninitExt;
                        let m: &mut std::mem::MaybeUninit<_> = dst;
                        m.map(|p| &raw mut (*p).method)
                    }
                }
            })?;
            wasmtime::component::Lower::lower(&self.uri, store, options, {
                #[allow(unused_unsafe)]
                {
                    unsafe {
                        use ::wasmtime::component::__internal::MaybeUninitExt;
                        let m: &mut std::mem::MaybeUninit<_> = dst;
                        m.map(|p| &raw mut (*p).uri)
                    }
                }
            })?;
            wasmtime::component::Lower::lower(&self.headers, store, options, {
                #[allow(unused_unsafe)]
                {
                    unsafe {
                        use ::wasmtime::component::__internal::MaybeUninitExt;
                        let m: &mut std::mem::MaybeUninit<_> = dst;
                        m.map(|p| &raw mut (*p).headers)
                    }
                }
            })?;
            wasmtime::component::Lower::lower(&self.params, store, options, {
                #[allow(unused_unsafe)]
                {
                    unsafe {
                        use ::wasmtime::component::__internal::MaybeUninitExt;
                        let m: &mut std::mem::MaybeUninit<_> = dst;
                        m.map(|p| &raw mut (*p).params)
                    }
                }
            })?;
            wasmtime::component::Lower::lower(&self.body, store, options, {
                #[allow(unused_unsafe)]
                {
                    unsafe {
                        use ::wasmtime::component::__internal::MaybeUninitExt;
                        let m: &mut std::mem::MaybeUninit<_> = dst;
                        m.map(|p| &raw mut (*p).body)
                    }
                }
            })?;
            Ok(())
        }
        #[inline]
        fn store<T>(
            &self,
            memory: &mut wasmtime::component::__internal::MemoryMut<'_, T>,
            mut offset: usize,
        ) -> wasmtime::component::__internal::anyhow::Result<()> {
            if true {
                if !(offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0)
                {
                    :: core :: panicking :: panic ("assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0")
                };
            };
            wasmtime::component::Lower::store(
                &self.method,
                memory,
                <Method as wasmtime::component::ComponentType>::ABI.next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.uri,
                memory,
                <Uri<'a> as wasmtime::component::ComponentType>::ABI.next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.headers,
                memory,
                <HeadersParam<'a> as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.params,
                memory,
                <Params<'a> as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.body,
                memory,
                <Option<BodyParam<'a>> as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            Ok(())
        }
    }
    const _: () = {
        #[doc(hidden)]
        #[repr(C)]
        pub struct LowerRequest<T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy> {
            method: T0,
            uri: T1,
            headers: T2,
            params: T3,
            body: T4,
            _align: [wasmtime::ValRaw; 0],
        }
        #[automatically_derived]
        impl<
                T0: ::core::clone::Clone + Copy,
                T1: ::core::clone::Clone + Copy,
                T2: ::core::clone::Clone + Copy,
                T3: ::core::clone::Clone + Copy,
                T4: ::core::clone::Clone + Copy,
            > ::core::clone::Clone for LowerRequest<T0, T1, T2, T3, T4>
        {
            #[inline]
            fn clone(&self) -> LowerRequest<T0, T1, T2, T3, T4> {
                LowerRequest {
                    method: ::core::clone::Clone::clone(&self.method),
                    uri: ::core::clone::Clone::clone(&self.uri),
                    headers: ::core::clone::Clone::clone(&self.headers),
                    params: ::core::clone::Clone::clone(&self.params),
                    body: ::core::clone::Clone::clone(&self.body),
                    _align: ::core::clone::Clone::clone(&self._align),
                }
            }
        }
        #[automatically_derived]
        impl<
                T0: ::core::marker::Copy + Copy,
                T1: ::core::marker::Copy + Copy,
                T2: ::core::marker::Copy + Copy,
                T3: ::core::marker::Copy + Copy,
                T4: ::core::marker::Copy + Copy,
            > ::core::marker::Copy for LowerRequest<T0, T1, T2, T3, T4>
        {
        }
        unsafe impl<'a> wasmtime::component::ComponentType for Request<'a> {
            type Lower = LowerRequest<
                <Method as wasmtime::component::ComponentType>::Lower,
                <Uri<'a> as wasmtime::component::ComponentType>::Lower,
                <HeadersParam<'a> as wasmtime::component::ComponentType>::Lower,
                <Params<'a> as wasmtime::component::ComponentType>::Lower,
                <Option<BodyParam<'a>> as wasmtime::component::ComponentType>::Lower,
            >;
            const ABI: wasmtime::component::__internal::CanonicalAbiInfo =
                wasmtime::component::__internal::CanonicalAbiInfo::record_static(&[
                    <Method as wasmtime::component::ComponentType>::ABI,
                    <Uri<'a> as wasmtime::component::ComponentType>::ABI,
                    <HeadersParam<'a> as wasmtime::component::ComponentType>::ABI,
                    <Params<'a> as wasmtime::component::ComponentType>::ABI,
                    <Option<BodyParam<'a>> as wasmtime::component::ComponentType>::ABI,
                ]);
            #[inline]
            fn typecheck(
                ty: &wasmtime::component::__internal::InterfaceType,
                types: &wasmtime::component::__internal::ComponentTypes,
            ) -> wasmtime::component::__internal::anyhow::Result<()> {
                wasmtime :: component :: __internal :: typecheck_record (ty , types , & [("method" , < Method as wasmtime :: component :: ComponentType > :: typecheck) , ("uri" , < Uri < 'a > as wasmtime :: component :: ComponentType > :: typecheck) , ("headers" , < HeadersParam < 'a > as wasmtime :: component :: ComponentType > :: typecheck) , ("params" , < Params < 'a > as wasmtime :: component :: ComponentType > :: typecheck) , ("body" , < Option < BodyParam < 'a > > as wasmtime :: component :: ComponentType > :: typecheck)])
            }
        }
    };
    impl<'a> core::fmt::Debug for Request<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Request")
                .field("method", &self.method)
                .field("uri", &self.uri)
                .field("headers", &self.headers)
                .field("params", &self.params)
                .field("body", &self.body)
                .finish()
        }
    }
    pub struct Handler {
        handle_http: wasmtime::component::Func,
    }
    impl Handler {
        pub fn new(
            __exports: &mut wasmtime::component::ExportInstance<'_, '_>,
        ) -> anyhow::Result<Handler> {
            let handle_http = *__exports
                .typed_func::<(Request<'_>,), (Result<Response, HttpError>,)>("handle-http")?
                .func();
            Ok(Handler { handle_http })
        }
        pub async fn call_handle_http<S: wasmtime::AsContextMut>(
            &self,
            mut store: S,
            arg0: Request<'_>,
        ) -> anyhow::Result<Result<Response, HttpError>>
        where
            <S as wasmtime::AsContext>::Data: Send,
        {
            let callee = unsafe {
                wasmtime :: component :: TypedFunc :: < (Request < '_ > ,) , (Result < Response , HttpError > ,) > :: new_unchecked (self . handle_http)
            };
            let (ret0,) = callee.call_async(store.as_context_mut(), (arg0,)).await?;
            callee.post_return_async(store.as_context_mut()).await?;
            Ok(ret0)
        }
    }
}
pub struct Serverless {
    handler: handler::Handler,
}
const _: () = { use wasmtime :: component :: __internal :: anyhow ; impl Serverless { # [doc = " Instantiates the provided `module` using the specified"] # [doc = " parameters, wrapping up the result in a structure that"] # [doc = " translates between wasm and the host."] pub async fn instantiate_async < T : Send > (mut store : impl wasmtime :: AsContextMut < Data = T > , component : & wasmtime :: component :: Component , linker : & wasmtime :: component :: Linker < T >) -> anyhow :: Result < (Self , wasmtime :: component :: Instance) > { let instance = linker . instantiate_async (& mut store , component) . await ? ; Ok ((Self :: new (store , & instance) ? , instance)) } # [doc = " Instantiates a pre-instantiated module using the specified"] # [doc = " parameters, wrapping up the result in a structure that"] # [doc = " translates between wasm and the host."] pub async fn instantiate_pre < T : Send > (mut store : impl wasmtime :: AsContextMut < Data = T > , instance_pre : & wasmtime :: component :: InstancePre < T >) -> anyhow :: Result < (Self , wasmtime :: component :: Instance) > { let instance = instance_pre . instantiate_async (& mut store) . await ? ; Ok ((Self :: new (store , & instance) ? , instance)) } # [doc = " Low-level creation wrapper for wrapping up the exports"] # [doc = " of the `instance` provided in this structure of wasm"] # [doc = " exports."] # [doc = ""] # [doc = " This function will extract exports from the `instance`"] # [doc = " defined within `store` and wrap them all up in the"] # [doc = " returned structure which can be used to interact with"] # [doc = " the wasm module."] pub fn new (mut store : impl wasmtime :: AsContextMut , instance : & wasmtime :: component :: Instance) -> anyhow :: Result < Self > { let mut store = store . as_context_mut () ; let mut exports = instance . exports (& mut store) ; let mut __exports = exports . root () ; let handler = handler :: Handler :: new (& mut __exports . instance ("handler") . ok_or_else (| | :: anyhow :: __private :: must_use ({ let error = :: anyhow :: __private :: format_err (:: core :: fmt :: Arguments :: new_v1 (& ["exported instance `handler` not present"] , & [])) ; error })) ?) ? ; Ok (Serverless { handler }) } pub fn handler (& self) -> & handler :: Handler { & self . handler } } };
const _ : & str = "interface wasi-http {\n\t/// The HTTP status code.\n\ttype http-status = u16\n\n\t/// The HTTP body.\n\ttype body = list<u8>\n\n\t/// The HTTP headers represented as a list of (name, value) pairs.\n\ttype headers = list<tuple<string, string>>\n\n\t/// The HTTP parameter queries, represented as a list of (name, value) pairs.\n\ttype params = list<tuple<string, string>>\n\n\t/// The HTTP URI of the current request.\n\ttype uri = string\n\n\t/// The HTTP method.\n\tenum method {\n\t\tget,\n\t\tpost,\n\t\tput,\n\t\tdelete,\n\t\tpatch,\n\t\thead,\n\t\toptions,\n\t}\n\n\t/// An HTTP request.\n\trecord request {\n\t\tmethod: method,\n\t\turi: uri,\n\t\theaders: headers,\n\t\tparams: params,\n\t\tbody: option<body>,\n\t}\n\n\t/// An HTTP response.\n\trecord response {\n\t\tstatus: http-status,\n\t\theaders: option<headers>,\n\t\tbody: option<body>,\n\t}\n\n\t/// HTTP errors returned by the runtime.\n\tvariant http-error {\n\t\tinvalid-url(string),\n\t\ttimeout-error(string),\n\t\tprotocol-error(string),\n\t\tstatus-error(u16),\n\t\tunexpected-error(string)\n\t}\n\n\thandle-http: func(req: request) -> result<response, http-error>\n}\n\ndefault world serverless {\n\texport handler: self.wasi-http\n}" ;
fn main() -> Result<()> {
    let body = async {
        let input = PathBuf::from(
            std::env::args()
                .collect::<Vec<String>>()
                .get(1)
                .context("must provide an input file")?,
        );
        let mut config = Config::new();
        config.cache_config_load_default()?;
        config.wasm_backtrace_details(WasmBacktraceDetails::Enable);
        config.wasm_component_model(true);
        config.async_support(true);
        let engine = Engine::new(&config)?;
        let mut linker = Linker::new(&engine);
        let mut store = Store::new(
            &engine,
            WasiCtxBuilder::new()
                .inherit_stdin()
                .inherit_stdout()
                .build(),
        );
        add_to_linker(&mut linker, |x| x)?;
        let component = Component::from_file(&engine, &input)?;
        let (s, _) = Serverless::instantiate_async(&mut store, &component, &linker).await?;
        let handler = s.handler();
        let mut req = Request {
            method: Method::Get,
            uri: "http://localhost:8080/",
            headers: &<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([("content-type", "application/json")]),
            ),
            params: &<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([("name", "world")]),
            ),
            body: None,
        };
        let res = handler.call_handle_http(&mut store, req).await?;
        match res {
            Ok(res) => {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_debug(&res)],
                ));
            }
            Err(err) => {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_debug(&err)],
                ));
            }
        }
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(&["hello world!\n"], &[]));
        };
        Ok(())
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
