#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::path::PathBuf;
use clap::{command, Parser};
mod file {
    use std::path::PathBuf;
    use bon::{bon, builder, Builder};
    use serde::{Deserialize, Serialize};
    use serde_yaml::Value;
    pub struct Become {
        #[serde(rename = "become", deserialize_with = "bool_or_yes")]
        pub flag: Option<bool>,
        #[serde(rename = "become_user")]
        pub user: Option<String>,
        #[serde(rename = "become_method")]
        pub method: Option<String>,
        #[serde(rename = "become_flags")]
        pub flags: Option<String>,
        #[serde(rename = "become_exe")]
        pub exe: Option<String>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Become {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "Become",
                "flag",
                &self.flag,
                "user",
                &self.user,
                "method",
                &self.method,
                "flags",
                &self.flags,
                "exe",
                &&self.exe,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Become {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Become",
                    false as usize + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "become",
                    &self.flag,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "become_user",
                    &self.user,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "become_method",
                    &self.method,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "become_flags",
                    &self.flags,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "become_exe",
                    &self.exe,
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
        impl<'de> _serde::Deserialize<'de> for Become {
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
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
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
                            "become" => _serde::__private::Ok(__Field::__field0),
                            "become_user" => _serde::__private::Ok(__Field::__field1),
                            "become_method" => _serde::__private::Ok(__Field::__field2),
                            "become_flags" => _serde::__private::Ok(__Field::__field3),
                            "become_exe" => _serde::__private::Ok(__Field::__field4),
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
                            b"become" => _serde::__private::Ok(__Field::__field0),
                            b"become_user" => _serde::__private::Ok(__Field::__field1),
                            b"become_method" => _serde::__private::Ok(__Field::__field2),
                            b"become_flags" => _serde::__private::Ok(__Field::__field3),
                            b"become_exe" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
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
                    marker: _serde::__private::PhantomData<Become>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Become;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Become",
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
                        let __field0 = match {
                            #[doc(hidden)]
                            struct __DeserializeWith<'de> {
                                value: Option<bool>,
                                phantom: _serde::__private::PhantomData<Become>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            #[automatically_derived]
                            impl<'de> _serde::Deserialize<'de>
                            for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::__private::Ok(__DeserializeWith {
                                        value: bool_or_yes(__deserializer)?,
                                        phantom: _serde::__private::PhantomData,
                                        lifetime: _serde::__private::PhantomData,
                                    })
                                }
                            }
                            _serde::__private::Option::map(
                                _serde::de::SeqAccess::next_element::<
                                    __DeserializeWith<'de>,
                                >(&mut __seq)?,
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Become with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Become with 5 elements",
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
                                        &"struct Become with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Become with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct Become with 5 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Become {
                            flag: __field0,
                            user: __field1,
                            method: __field2,
                            flags: __field3,
                            exe: __field4,
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
                        let mut __field0: _serde::__private::Option<Option<bool>> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("become"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some({
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: Option<bool>,
                                            phantom: _serde::__private::PhantomData<Become>,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private::Ok(__DeserializeWith {
                                                    value: bool_or_yes(__deserializer)?,
                                                    phantom: _serde::__private::PhantomData,
                                                    lifetime: _serde::__private::PhantomData,
                                                })
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "become_user",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "become_method",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "become_flags",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "become_exe",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
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
                                return _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field("become"),
                                );
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("become_user")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("become_method")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("become_flags")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("become_exe")?
                            }
                        };
                        _serde::__private::Ok(Become {
                            flag: __field0,
                            user: __field1,
                            method: __field2,
                            flags: __field3,
                            exe: __field4,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "become",
                    "become_user",
                    "become_method",
                    "become_flags",
                    "become_exe",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Become",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Become>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::default::Default for Become {
        #[inline]
        fn default() -> Become {
            Become {
                flag: ::core::default::Default::default(),
                user: ::core::default::Default::default(),
                method: ::core::default::Default::default(),
                flags: ::core::default::Default::default(),
                exe: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl Become {
        ///Create an instance of [`Become`] using the builder syntax
        #[inline(always)]
        #[allow(clippy::inline_always, clippy::use_self, clippy::missing_const_for_fn)]
        #[allow(deprecated)]
        pub fn builder() -> BecomeBuilder {
            BecomeBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: ::core::default::Default::default(),
            }
        }
    }
    #[allow(unnameable_types, unreachable_pub, clippy::redundant_pub_crate)]
    /**Tools for manipulating the type state of [`BecomeBuilder`].

See the [detailed guide](https://bon-rs.com/guide/typestate-api) that describes how all the pieces here fit together.*/
    #[allow(deprecated)]
    mod become_builder {
        #[doc(inline)]
        pub use ::bon::__::{IsSet, IsUnset};
        use ::bon::__::{Set, Unset};
        mod sealed {
            pub struct Sealed;
        }
        /**Builder's type state specifies if members are set or not (unset).

You can use the associated types of this trait to control the state of individual members with the [`IsSet`] and [`IsUnset`] traits. You can change the state of the members with the `Set*` structs available in this module.*/
        pub trait State: ::core::marker::Sized {
            /**Type state of the member `flag`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type Flag;
            /**Type state of the member `user`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type User;
            /**Type state of the member `method`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type Method;
            /**Type state of the member `flags`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type Flags;
            /**Type state of the member `exe`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type Exe;
            #[doc(hidden)]
            const SEALED: sealed::Sealed;
        }
        /**Marker trait that indicates that all required members are set.

In this state, you can finish building by calling the method [`BecomeBuilder::build()`](super::BecomeBuilder::build())*/
        pub trait IsComplete: State {
            #[doc(hidden)]
            const SEALED: sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> IsComplete for S {
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[deprecated = "this should not be used directly; it is an implementation detail; use the Set* type aliases to control the state of members instead"]
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        mod members {
            pub struct flag(());
            pub struct user(());
            pub struct method(());
            pub struct flags(());
            pub struct exe(());
        }
        /// Represents a [`State`] that has [`IsUnset`] implemented for all members.
        ///
        /// This is the initial state of the builder before any setters are called.
        pub struct Empty(());
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::Flag`].

The state for all other members is left the same as in the input state.*/
        pub struct SetFlag<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::User`].

The state for all other members is left the same as in the input state.*/
        pub struct SetUser<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::Method`].

The state for all other members is left the same as in the input state.*/
        pub struct SetMethod<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::Flags`].

The state for all other members is left the same as in the input state.*/
        pub struct SetFlags<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::Exe`].

The state for all other members is left the same as in the input state.*/
        pub struct SetExe<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        #[doc(hidden)]
        impl State for Empty {
            type Flag = Unset<members::flag>;
            type User = Unset<members::user>;
            type Method = Unset<members::method>;
            type Flags = Unset<members::flags>;
            type Exe = Unset<members::exe>;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetFlag<S> {
            type Flag = Set<members::flag>;
            type User = S::User;
            type Method = S::Method;
            type Flags = S::Flags;
            type Exe = S::Exe;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetUser<S> {
            type Flag = S::Flag;
            type User = Set<members::user>;
            type Method = S::Method;
            type Flags = S::Flags;
            type Exe = S::Exe;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetMethod<S> {
            type Flag = S::Flag;
            type User = S::User;
            type Method = Set<members::method>;
            type Flags = S::Flags;
            type Exe = S::Exe;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetFlags<S> {
            type Flag = S::Flag;
            type User = S::User;
            type Method = S::Method;
            type Flags = Set<members::flags>;
            type Exe = S::Exe;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetExe<S> {
            type Flag = S::Flag;
            type User = S::User;
            type Method = S::Method;
            type Flags = S::Flags;
            type Exe = Set<members::exe>;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
    }
    #[must_use = "the builder does nothing until you call `build()` on it to finish building"]
    ///Use builder syntax to set the inputs and finish with [`build()`](Self::build()).
    #[allow(unused_parens)]
    #[allow(clippy::struct_field_names, clippy::type_complexity)]
    #[allow(deprecated)]
    pub struct BecomeBuilder<S: become_builder::State = become_builder::Empty> {
        #[doc(hidden)]
        #[deprecated = "this field should not be used directly; it's an implementation detail, and if you access it directly, you may break some internal unsafe invariants; if you found yourself needing it, then you are probably doing something wrong; feel free to open an issue/discussion in our GitHub repository (https://github.com/elastio/bon) or ask for help in our Discord server (https://bon-rs.com/discord)"]
        __unsafe_private_phantom: ::core::marker::PhantomData<
            (fn() -> S, fn() -> ::core::marker::PhantomData<Become>),
        >,
        #[doc(hidden)]
        #[deprecated = "this field should not be used directly; it's an implementation detail, and if you access it directly, you may break some internal unsafe invariants; if you found yourself needing it, then you are probably doing something wrong; feel free to open an issue/discussion in our GitHub repository (https://github.com/elastio/bon) or ask for help in our Discord server (https://bon-rs.com/discord)"]
        __unsafe_private_named: (
            ::core::option::Option<bool>,
            ::core::option::Option<String>,
            ::core::option::Option<String>,
            ::core::option::Option<String>,
            ::core::option::Option<String>,
        ),
    }
    #[allow(unused_parens)]
    #[automatically_derived]
    #[allow(deprecated)]
    impl<S: become_builder::State> BecomeBuilder<S> {
        /// Finish building and return the requested object
        #[inline(always)]
        #[allow(
            clippy::inline_always,
            clippy::future_not_send,
            clippy::missing_const_for_fn,
        )]
        #[must_use = "building a struct without using it is likely a bug"]
        pub fn build(self) -> Become
        where
            S: become_builder::IsComplete,
        {
            let flag: Option<bool> = self.__unsafe_private_named.0;
            let user: Option<String> = self.__unsafe_private_named.1;
            let method: Option<String> = self.__unsafe_private_named.2;
            let flags: Option<String> = self.__unsafe_private_named.3;
            let exe: Option<String> = self.__unsafe_private_named.4;
            Become {
                flag,
                user,
                method,
                flags,
                exe,
            }
        }
        /**_**Optional** ([Some](Self::flag()) / [Option](Self::maybe_flag()) setters)._

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn flag(self, value: bool) -> BecomeBuilder<become_builder::SetFlag<S>>
        where
            S::Flag: become_builder::IsUnset,
        {
            self.maybe_flag(Some(value))
        }
        /**_**Optional** ([Some](Self::flag()) / [Option](Self::maybe_flag()) setters)._

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn maybe_flag(
            mut self,
            value: Option<bool>,
        ) -> BecomeBuilder<become_builder::SetFlag<S>>
        where
            S::Flag: become_builder::IsUnset,
        {
            self.__unsafe_private_named.0 = value;
            BecomeBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
        /**_**Optional** ([Some](Self::user()) / [Option](Self::maybe_user()) setters)._

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn user(self, value: String) -> BecomeBuilder<become_builder::SetUser<S>>
        where
            S::User: become_builder::IsUnset,
        {
            self.maybe_user(Some(value))
        }
        /**_**Optional** ([Some](Self::user()) / [Option](Self::maybe_user()) setters)._

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn maybe_user(
            mut self,
            value: Option<String>,
        ) -> BecomeBuilder<become_builder::SetUser<S>>
        where
            S::User: become_builder::IsUnset,
        {
            self.__unsafe_private_named.1 = value;
            BecomeBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
        /**_**Optional** ([Some](Self::method()) / [Option](Self::maybe_method()) setters)._

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn method(self, value: String) -> BecomeBuilder<become_builder::SetMethod<S>>
        where
            S::Method: become_builder::IsUnset,
        {
            self.maybe_method(Some(value))
        }
        /**_**Optional** ([Some](Self::method()) / [Option](Self::maybe_method()) setters)._

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn maybe_method(
            mut self,
            value: Option<String>,
        ) -> BecomeBuilder<become_builder::SetMethod<S>>
        where
            S::Method: become_builder::IsUnset,
        {
            self.__unsafe_private_named.2 = value;
            BecomeBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
        /**_**Optional** ([Some](Self::flags()) / [Option](Self::maybe_flags()) setters)._

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn flags(self, value: String) -> BecomeBuilder<become_builder::SetFlags<S>>
        where
            S::Flags: become_builder::IsUnset,
        {
            self.maybe_flags(Some(value))
        }
        /**_**Optional** ([Some](Self::flags()) / [Option](Self::maybe_flags()) setters)._

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn maybe_flags(
            mut self,
            value: Option<String>,
        ) -> BecomeBuilder<become_builder::SetFlags<S>>
        where
            S::Flags: become_builder::IsUnset,
        {
            self.__unsafe_private_named.3 = value;
            BecomeBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
        /**_**Optional** ([Some](Self::exe()) / [Option](Self::maybe_exe()) setters)._

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn exe(self, value: String) -> BecomeBuilder<become_builder::SetExe<S>>
        where
            S::Exe: become_builder::IsUnset,
        {
            self.maybe_exe(Some(value))
        }
        /**_**Optional** ([Some](Self::exe()) / [Option](Self::maybe_exe()) setters)._

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn maybe_exe(
            mut self,
            value: Option<String>,
        ) -> BecomeBuilder<become_builder::SetExe<S>>
        where
            S::Exe: become_builder::IsUnset,
        {
            self.__unsafe_private_named.4 = value;
            BecomeBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
    }
    fn bool_or_yes<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;
        if let Some(bool) = value.as_bool() {
            return Ok(Some(bool));
        }
        if let Some(string) = value.as_str() {
            if string == "yes" {
                return Ok(Some(true));
            } else if string == "no" {
                return Ok(Some(false));
            } else {
                return Err(serde::de::Error::custom("expected bool or yes/no"));
            }
        }
        Ok(None)
    }
    pub struct Playbook {
        tasks: Vec<Play>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Playbook {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Playbook",
                "tasks",
                &&self.tasks,
            )
        }
    }
    #[automatically_derived]
    impl Playbook {
        ///Create an instance of [`Playbook`] using the builder syntax
        #[inline(always)]
        #[allow(clippy::inline_always, clippy::use_self, clippy::missing_const_for_fn)]
        #[allow(deprecated)]
        pub fn builder() -> PlaybookBuilder {
            PlaybookBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: ::core::default::Default::default(),
            }
        }
    }
    #[allow(unnameable_types, unreachable_pub, clippy::redundant_pub_crate)]
    /**Tools for manipulating the type state of [`PlaybookBuilder`].

See the [detailed guide](https://bon-rs.com/guide/typestate-api) that describes how all the pieces here fit together.*/
    #[allow(deprecated)]
    mod playbook_builder {
        #[doc(inline)]
        pub use ::bon::__::{IsSet, IsUnset};
        use ::bon::__::{Set, Unset};
        mod sealed {
            pub struct Sealed;
        }
        /**Builder's type state specifies if members are set or not (unset).

You can use the associated types of this trait to control the state of individual members with the [`IsSet`] and [`IsUnset`] traits. You can change the state of the members with the `Set*` structs available in this module.*/
        pub trait State: ::core::marker::Sized {
            /**Type state of the member `tasks`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type Tasks;
            #[doc(hidden)]
            const SEALED: sealed::Sealed;
        }
        /**Marker trait that indicates that all required members are set.

In this state, you can finish building by calling the method [`PlaybookBuilder::build()`](super::PlaybookBuilder::build())*/
        pub trait IsComplete: State {
            #[doc(hidden)]
            const SEALED: sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> IsComplete for S
        where
            S::Tasks: IsSet,
        {
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[deprecated = "this should not be used directly; it is an implementation detail; use the Set* type aliases to control the state of members instead"]
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        mod members {
            pub struct tasks(());
        }
        /// Represents a [`State`] that has [`IsUnset`] implemented for all members.
        ///
        /// This is the initial state of the builder before any setters are called.
        pub struct Empty(());
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::Tasks`].

The state for all other members is left the same as in the input state.*/
        pub struct SetTasks<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        #[doc(hidden)]
        impl State for Empty {
            type Tasks = Unset<members::tasks>;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetTasks<S> {
            type Tasks = Set<members::tasks>;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
    }
    #[must_use = "the builder does nothing until you call `build()` on it to finish building"]
    ///Use builder syntax to set the inputs and finish with [`build()`](Self::build()).
    #[allow(unused_parens)]
    #[allow(clippy::struct_field_names, clippy::type_complexity)]
    #[allow(deprecated)]
    pub struct PlaybookBuilder<S: playbook_builder::State = playbook_builder::Empty> {
        #[doc(hidden)]
        #[deprecated = "this field should not be used directly; it's an implementation detail, and if you access it directly, you may break some internal unsafe invariants; if you found yourself needing it, then you are probably doing something wrong; feel free to open an issue/discussion in our GitHub repository (https://github.com/elastio/bon) or ask for help in our Discord server (https://bon-rs.com/discord)"]
        __unsafe_private_phantom: ::core::marker::PhantomData<
            (fn() -> S, fn() -> ::core::marker::PhantomData<Playbook>),
        >,
        #[doc(hidden)]
        #[deprecated = "this field should not be used directly; it's an implementation detail, and if you access it directly, you may break some internal unsafe invariants; if you found yourself needing it, then you are probably doing something wrong; feel free to open an issue/discussion in our GitHub repository (https://github.com/elastio/bon) or ask for help in our Discord server (https://bon-rs.com/discord)"]
        __unsafe_private_named: (::core::option::Option<Vec<Play>>,),
    }
    #[allow(unused_parens)]
    #[automatically_derived]
    #[allow(deprecated)]
    impl<S: playbook_builder::State> PlaybookBuilder<S> {
        /// Finish building and return the requested object
        #[inline(always)]
        #[allow(
            clippy::inline_always,
            clippy::future_not_send,
            clippy::missing_const_for_fn,
        )]
        #[must_use = "building a struct without using it is likely a bug"]
        pub fn build(self) -> Playbook
        where
            S: playbook_builder::IsComplete,
        {
            let tasks: Vec<Play> = unsafe {
                ::core::option::Option::unwrap_unchecked(self.__unsafe_private_named.0)
            };
            Playbook { tasks }
        }
        /**_**Required.**_

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn tasks(
            mut self,
            value: Vec<Play>,
        ) -> PlaybookBuilder<playbook_builder::SetTasks<S>>
        where
            S::Tasks: playbook_builder::IsUnset,
        {
            self.__unsafe_private_named.0 = ::core::option::Option::Some(value);
            PlaybookBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
    }
    impl<'de> Deserialize<'de> for Playbook {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let tasks = Vec::<Play>::deserialize(deserializer)?;
            Ok(Playbook { tasks })
        }
    }
    impl Serialize for Playbook {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.tasks.serialize(serializer)
        }
    }
    pub struct Task {
        pub name: String,
        #[serde(flatten)]
        pub becomes: Option<Become>,
        #[serde(flatten)]
        pub action: Action,
        #[serde(flatten)]
        pub _extra_fields: Value,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Task {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Task",
                "name",
                &self.name,
                "becomes",
                &self.becomes,
                "action",
                &self.action,
                "_extra_fields",
                &&self._extra_fields,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Task {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_map(
                    __serializer,
                    _serde::__private::None,
                )?;
                _serde::ser::SerializeMap::serialize_entry(
                    &mut __serde_state,
                    "name",
                    &self.name,
                )?;
                _serde::Serialize::serialize(
                    &&self.becomes,
                    _serde::__private::ser::FlatMapSerializer(&mut __serde_state),
                )?;
                _serde::Serialize::serialize(
                    &&self.action,
                    _serde::__private::ser::FlatMapSerializer(&mut __serde_state),
                )?;
                _serde::Serialize::serialize(
                    &&self._extra_fields,
                    _serde::__private::ser::FlatMapSerializer(&mut __serde_state),
                )?;
                _serde::ser::SerializeMap::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Task {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field<'de> {
                    __field0,
                    __other(_serde::__private::de::Content<'de>),
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::Bool(__value),
                            ),
                        )
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::I8(__value)),
                        )
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I16(__value),
                            ),
                        )
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I32(__value),
                            ),
                        )
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I64(__value),
                            ),
                        )
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::U8(__value)),
                        )
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U16(__value),
                            ),
                        )
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U32(__value),
                            ),
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U64(__value),
                            ),
                        )
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::F32(__value),
                            ),
                        )
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::F64(__value),
                            ),
                        )
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::Char(__value),
                            ),
                        )
                    }
                    fn visit_unit<__E>(
                        self,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::Unit),
                        )
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
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
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = _serde::__private::de::Content::ByteBuf(
                                    __value.to_vec(),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(
                                    __value,
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
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
                    marker: _serde::__private::PhantomData<Task>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Task;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Task",
                        )
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
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<
                                (
                                    _serde::__private::de::Content,
                                    _serde::__private::de::Content,
                                ),
                            >,
                        >::new();
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__other(__name) => {
                                    __collect
                                        .push(
                                            _serde::__private::Some((
                                                __name,
                                                _serde::de::MapAccess::next_value(&mut __map)?,
                                            )),
                                        );
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("name")?
                            }
                        };
                        let __field1: Option<Become> = _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        )?;
                        let __field2: Action = _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        )?;
                        let __field3: Value = _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        )?;
                        _serde::__private::Ok(Task {
                            name: __field0,
                            becomes: __field1,
                            action: __field2,
                            _extra_fields: __field3,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Task>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl Task {
        ///Create an instance of [`Task`] using the builder syntax
        #[inline(always)]
        #[allow(clippy::inline_always, clippy::use_self, clippy::missing_const_for_fn)]
        #[allow(deprecated)]
        pub fn builder() -> TaskBuilder {
            TaskBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: ::core::default::Default::default(),
            }
        }
    }
    #[allow(unnameable_types, unreachable_pub, clippy::redundant_pub_crate)]
    /**Tools for manipulating the type state of [`TaskBuilder`].

See the [detailed guide](https://bon-rs.com/guide/typestate-api) that describes how all the pieces here fit together.*/
    #[allow(deprecated)]
    mod task_builder {
        #[doc(inline)]
        pub use ::bon::__::{IsSet, IsUnset};
        use ::bon::__::{Set, Unset};
        mod sealed {
            pub struct Sealed;
        }
        /**Builder's type state specifies if members are set or not (unset).

You can use the associated types of this trait to control the state of individual members with the [`IsSet`] and [`IsUnset`] traits. You can change the state of the members with the `Set*` structs available in this module.*/
        pub trait State: ::core::marker::Sized {
            /**Type state of the member `name`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type Name;
            /**Type state of the member `becomes`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type Becomes;
            /**Type state of the member `action`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type Action;
            /**Type state of the member `extra_fields`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type ExtraFields;
            #[doc(hidden)]
            const SEALED: sealed::Sealed;
        }
        /**Marker trait that indicates that all required members are set.

In this state, you can finish building by calling the method [`TaskBuilder::build()`](super::TaskBuilder::build())*/
        pub trait IsComplete: State {
            #[doc(hidden)]
            const SEALED: sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> IsComplete for S
        where
            S::Name: IsSet,
            S::Action: IsSet,
            S::ExtraFields: IsSet,
        {
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[deprecated = "this should not be used directly; it is an implementation detail; use the Set* type aliases to control the state of members instead"]
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        mod members {
            pub struct name(());
            pub struct becomes(());
            pub struct action(());
            pub struct extra_fields(());
        }
        /// Represents a [`State`] that has [`IsUnset`] implemented for all members.
        ///
        /// This is the initial state of the builder before any setters are called.
        pub struct Empty(());
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::Name`].

The state for all other members is left the same as in the input state.*/
        pub struct SetName<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::Becomes`].

The state for all other members is left the same as in the input state.*/
        pub struct SetBecomes<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::Action`].

The state for all other members is left the same as in the input state.*/
        pub struct SetAction<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::ExtraFields`].

The state for all other members is left the same as in the input state.*/
        pub struct SetExtraFields<S: State = Empty>(
            ::core::marker::PhantomData<fn() -> S>,
        );
        #[doc(hidden)]
        impl State for Empty {
            type Name = Unset<members::name>;
            type Becomes = Unset<members::becomes>;
            type Action = Unset<members::action>;
            type ExtraFields = Unset<members::extra_fields>;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetName<S> {
            type Name = Set<members::name>;
            type Becomes = S::Becomes;
            type Action = S::Action;
            type ExtraFields = S::ExtraFields;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetBecomes<S> {
            type Name = S::Name;
            type Becomes = Set<members::becomes>;
            type Action = S::Action;
            type ExtraFields = S::ExtraFields;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetAction<S> {
            type Name = S::Name;
            type Becomes = S::Becomes;
            type Action = Set<members::action>;
            type ExtraFields = S::ExtraFields;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetExtraFields<S> {
            type Name = S::Name;
            type Becomes = S::Becomes;
            type Action = S::Action;
            type ExtraFields = Set<members::extra_fields>;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
    }
    #[must_use = "the builder does nothing until you call `build()` on it to finish building"]
    ///Use builder syntax to set the inputs and finish with [`build()`](Self::build()).
    #[allow(unused_parens)]
    #[allow(clippy::struct_field_names, clippy::type_complexity)]
    #[allow(deprecated)]
    pub struct TaskBuilder<S: task_builder::State = task_builder::Empty> {
        #[doc(hidden)]
        #[deprecated = "this field should not be used directly; it's an implementation detail, and if you access it directly, you may break some internal unsafe invariants; if you found yourself needing it, then you are probably doing something wrong; feel free to open an issue/discussion in our GitHub repository (https://github.com/elastio/bon) or ask for help in our Discord server (https://bon-rs.com/discord)"]
        __unsafe_private_phantom: ::core::marker::PhantomData<
            (fn() -> S, fn() -> ::core::marker::PhantomData<Task>),
        >,
        #[doc(hidden)]
        #[deprecated = "this field should not be used directly; it's an implementation detail, and if you access it directly, you may break some internal unsafe invariants; if you found yourself needing it, then you are probably doing something wrong; feel free to open an issue/discussion in our GitHub repository (https://github.com/elastio/bon) or ask for help in our Discord server (https://bon-rs.com/discord)"]
        __unsafe_private_named: (
            ::core::option::Option<String>,
            ::core::option::Option<Become>,
            ::core::option::Option<Action>,
            ::core::option::Option<Value>,
        ),
    }
    #[allow(unused_parens)]
    #[automatically_derived]
    #[allow(deprecated)]
    impl<S: task_builder::State> TaskBuilder<S> {
        /// Finish building and return the requested object
        #[inline(always)]
        #[allow(
            clippy::inline_always,
            clippy::future_not_send,
            clippy::missing_const_for_fn,
        )]
        #[must_use = "building a struct without using it is likely a bug"]
        pub fn build(self) -> Task
        where
            S: task_builder::IsComplete,
        {
            let name: String = unsafe {
                ::core::option::Option::unwrap_unchecked(self.__unsafe_private_named.0)
            };
            let becomes: Option<Become> = self.__unsafe_private_named.1;
            let action: Action = unsafe {
                ::core::option::Option::unwrap_unchecked(self.__unsafe_private_named.2)
            };
            let _extra_fields: Value = unsafe {
                ::core::option::Option::unwrap_unchecked(self.__unsafe_private_named.3)
            };
            Task {
                name,
                becomes,
                action,
                _extra_fields,
            }
        }
        /**_**Required.**_

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn name(mut self, value: String) -> TaskBuilder<task_builder::SetName<S>>
        where
            S::Name: task_builder::IsUnset,
        {
            self.__unsafe_private_named.0 = ::core::option::Option::Some(value);
            TaskBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
        /**_**Optional** ([Some](Self::becomes()) / [Option](Self::maybe_becomes()) setters)._

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn becomes(self, value: Become) -> TaskBuilder<task_builder::SetBecomes<S>>
        where
            S::Becomes: task_builder::IsUnset,
        {
            self.maybe_becomes(Some(value))
        }
        /**_**Optional** ([Some](Self::becomes()) / [Option](Self::maybe_becomes()) setters)._

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn maybe_becomes(
            mut self,
            value: Option<Become>,
        ) -> TaskBuilder<task_builder::SetBecomes<S>>
        where
            S::Becomes: task_builder::IsUnset,
        {
            self.__unsafe_private_named.1 = value;
            TaskBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
        /**_**Required.**_

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn action(mut self, value: Action) -> TaskBuilder<task_builder::SetAction<S>>
        where
            S::Action: task_builder::IsUnset,
        {
            self.__unsafe_private_named.2 = ::core::option::Option::Some(value);
            TaskBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
        /**_**Required.**_

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn extra_fields(
            mut self,
            value: Value,
        ) -> TaskBuilder<task_builder::SetExtraFields<S>>
        where
            S::ExtraFields: task_builder::IsUnset,
        {
            self.__unsafe_private_named.3 = ::core::option::Option::Some(value);
            TaskBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
    }
    pub struct Block {
        pub name: String,
        #[serde(flatten)]
        pub becomes: Option<Become>,
        pub block: Vec<Box<Task>>,
        #[serde(flatten)]
        _extra_fields: Value,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Block {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Block",
                "name",
                &self.name,
                "becomes",
                &self.becomes,
                "block",
                &self.block,
                "_extra_fields",
                &&self._extra_fields,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Block {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_map(
                    __serializer,
                    _serde::__private::None,
                )?;
                _serde::ser::SerializeMap::serialize_entry(
                    &mut __serde_state,
                    "name",
                    &self.name,
                )?;
                _serde::Serialize::serialize(
                    &&self.becomes,
                    _serde::__private::ser::FlatMapSerializer(&mut __serde_state),
                )?;
                _serde::ser::SerializeMap::serialize_entry(
                    &mut __serde_state,
                    "block",
                    &self.block,
                )?;
                _serde::Serialize::serialize(
                    &&self._extra_fields,
                    _serde::__private::ser::FlatMapSerializer(&mut __serde_state),
                )?;
                _serde::ser::SerializeMap::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Block {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field<'de> {
                    __field0,
                    __field2,
                    __other(_serde::__private::de::Content<'de>),
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::Bool(__value),
                            ),
                        )
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::I8(__value)),
                        )
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I16(__value),
                            ),
                        )
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I32(__value),
                            ),
                        )
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I64(__value),
                            ),
                        )
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::U8(__value)),
                        )
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U16(__value),
                            ),
                        )
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U32(__value),
                            ),
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U64(__value),
                            ),
                        )
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::F32(__value),
                            ),
                        )
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::F64(__value),
                            ),
                        )
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::Char(__value),
                            ),
                        )
                    }
                    fn visit_unit<__E>(
                        self,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::Unit),
                        )
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            "block" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
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
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            b"block" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::ByteBuf(
                                    __value.to_vec(),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            "block" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            b"block" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(
                                    __value,
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
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
                    marker: _serde::__private::PhantomData<Block>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Block;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Block",
                        )
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
                        let mut __field2: _serde::__private::Option<Vec<Box<Task>>> = _serde::__private::None;
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<
                                (
                                    _serde::__private::de::Content,
                                    _serde::__private::de::Content,
                                ),
                            >,
                        >::new();
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("block"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<Box<Task>>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__other(__name) => {
                                    __collect
                                        .push(
                                            _serde::__private::Some((
                                                __name,
                                                _serde::de::MapAccess::next_value(&mut __map)?,
                                            )),
                                        );
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("name")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("block")?
                            }
                        };
                        let __field1: Option<Become> = _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        )?;
                        let __field3: Value = _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        )?;
                        _serde::__private::Ok(Block {
                            name: __field0,
                            becomes: __field1,
                            block: __field2,
                            _extra_fields: __field3,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Block>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl Block {
        ///Create an instance of [`Block`] using the builder syntax
        #[inline(always)]
        #[allow(clippy::inline_always, clippy::use_self, clippy::missing_const_for_fn)]
        #[allow(deprecated)]
        pub fn builder() -> BlockBuilder {
            BlockBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: ::core::default::Default::default(),
            }
        }
    }
    #[allow(unnameable_types, unreachable_pub, clippy::redundant_pub_crate)]
    /**Tools for manipulating the type state of [`BlockBuilder`].

See the [detailed guide](https://bon-rs.com/guide/typestate-api) that describes how all the pieces here fit together.*/
    #[allow(deprecated)]
    mod block_builder {
        #[doc(inline)]
        pub use ::bon::__::{IsSet, IsUnset};
        use ::bon::__::{Set, Unset};
        mod sealed {
            pub struct Sealed;
        }
        /**Builder's type state specifies if members are set or not (unset).

You can use the associated types of this trait to control the state of individual members with the [`IsSet`] and [`IsUnset`] traits. You can change the state of the members with the `Set*` structs available in this module.*/
        pub trait State: ::core::marker::Sized {
            /**Type state of the member `name`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type Name;
            /**Type state of the member `becomes`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type Becomes;
            /**Type state of the member `block`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type Block;
            /**Type state of the member `extra_fields`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type ExtraFields;
            #[doc(hidden)]
            const SEALED: sealed::Sealed;
        }
        /**Marker trait that indicates that all required members are set.

In this state, you can finish building by calling the method [`BlockBuilder::build()`](super::BlockBuilder::build())*/
        pub trait IsComplete: State {
            #[doc(hidden)]
            const SEALED: sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> IsComplete for S
        where
            S::Name: IsSet,
            S::Block: IsSet,
            S::ExtraFields: IsSet,
        {
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[deprecated = "this should not be used directly; it is an implementation detail; use the Set* type aliases to control the state of members instead"]
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        mod members {
            pub struct name(());
            pub struct becomes(());
            pub struct block(());
            pub struct extra_fields(());
        }
        /// Represents a [`State`] that has [`IsUnset`] implemented for all members.
        ///
        /// This is the initial state of the builder before any setters are called.
        pub struct Empty(());
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::Name`].

The state for all other members is left the same as in the input state.*/
        pub struct SetName<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::Becomes`].

The state for all other members is left the same as in the input state.*/
        pub struct SetBecomes<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::Block`].

The state for all other members is left the same as in the input state.*/
        pub struct SetBlock<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::ExtraFields`].

The state for all other members is left the same as in the input state.*/
        pub struct SetExtraFields<S: State = Empty>(
            ::core::marker::PhantomData<fn() -> S>,
        );
        #[doc(hidden)]
        impl State for Empty {
            type Name = Unset<members::name>;
            type Becomes = Unset<members::becomes>;
            type Block = Unset<members::block>;
            type ExtraFields = Unset<members::extra_fields>;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetName<S> {
            type Name = Set<members::name>;
            type Becomes = S::Becomes;
            type Block = S::Block;
            type ExtraFields = S::ExtraFields;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetBecomes<S> {
            type Name = S::Name;
            type Becomes = Set<members::becomes>;
            type Block = S::Block;
            type ExtraFields = S::ExtraFields;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetBlock<S> {
            type Name = S::Name;
            type Becomes = S::Becomes;
            type Block = Set<members::block>;
            type ExtraFields = S::ExtraFields;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetExtraFields<S> {
            type Name = S::Name;
            type Becomes = S::Becomes;
            type Block = S::Block;
            type ExtraFields = Set<members::extra_fields>;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
    }
    #[must_use = "the builder does nothing until you call `build()` on it to finish building"]
    ///Use builder syntax to set the inputs and finish with [`build()`](Self::build()).
    #[allow(unused_parens)]
    #[allow(clippy::struct_field_names, clippy::type_complexity)]
    #[allow(deprecated)]
    pub struct BlockBuilder<S: block_builder::State = block_builder::Empty> {
        #[doc(hidden)]
        #[deprecated = "this field should not be used directly; it's an implementation detail, and if you access it directly, you may break some internal unsafe invariants; if you found yourself needing it, then you are probably doing something wrong; feel free to open an issue/discussion in our GitHub repository (https://github.com/elastio/bon) or ask for help in our Discord server (https://bon-rs.com/discord)"]
        __unsafe_private_phantom: ::core::marker::PhantomData<
            (fn() -> S, fn() -> ::core::marker::PhantomData<Block>),
        >,
        #[doc(hidden)]
        #[deprecated = "this field should not be used directly; it's an implementation detail, and if you access it directly, you may break some internal unsafe invariants; if you found yourself needing it, then you are probably doing something wrong; feel free to open an issue/discussion in our GitHub repository (https://github.com/elastio/bon) or ask for help in our Discord server (https://bon-rs.com/discord)"]
        __unsafe_private_named: (
            ::core::option::Option<String>,
            ::core::option::Option<Become>,
            ::core::option::Option<Vec<Box<Task>>>,
            ::core::option::Option<Value>,
        ),
    }
    #[allow(unused_parens)]
    #[automatically_derived]
    #[allow(deprecated)]
    impl<S: block_builder::State> BlockBuilder<S> {
        /// Finish building and return the requested object
        #[inline(always)]
        #[allow(
            clippy::inline_always,
            clippy::future_not_send,
            clippy::missing_const_for_fn,
        )]
        #[must_use = "building a struct without using it is likely a bug"]
        pub fn build(self) -> Block
        where
            S: block_builder::IsComplete,
        {
            let name: String = unsafe {
                ::core::option::Option::unwrap_unchecked(self.__unsafe_private_named.0)
            };
            let becomes: Option<Become> = self.__unsafe_private_named.1;
            let block: Vec<Box<Task>> = unsafe {
                ::core::option::Option::unwrap_unchecked(self.__unsafe_private_named.2)
            };
            let _extra_fields: Value = unsafe {
                ::core::option::Option::unwrap_unchecked(self.__unsafe_private_named.3)
            };
            Block {
                name,
                becomes,
                block,
                _extra_fields,
            }
        }
        /**_**Required.**_

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn name(mut self, value: String) -> BlockBuilder<block_builder::SetName<S>>
        where
            S::Name: block_builder::IsUnset,
        {
            self.__unsafe_private_named.0 = ::core::option::Option::Some(value);
            BlockBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
        /**_**Optional** ([Some](Self::becomes()) / [Option](Self::maybe_becomes()) setters)._

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn becomes(self, value: Become) -> BlockBuilder<block_builder::SetBecomes<S>>
        where
            S::Becomes: block_builder::IsUnset,
        {
            self.maybe_becomes(Some(value))
        }
        /**_**Optional** ([Some](Self::becomes()) / [Option](Self::maybe_becomes()) setters)._

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn maybe_becomes(
            mut self,
            value: Option<Become>,
        ) -> BlockBuilder<block_builder::SetBecomes<S>>
        where
            S::Becomes: block_builder::IsUnset,
        {
            self.__unsafe_private_named.1 = value;
            BlockBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
        /**_**Required.**_

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn block(
            mut self,
            value: Vec<Box<Task>>,
        ) -> BlockBuilder<block_builder::SetBlock<S>>
        where
            S::Block: block_builder::IsUnset,
        {
            self.__unsafe_private_named.2 = ::core::option::Option::Some(value);
            BlockBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
        /**_**Required.**_

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn extra_fields(
            mut self,
            value: Value,
        ) -> BlockBuilder<block_builder::SetExtraFields<S>>
        where
            S::ExtraFields: block_builder::IsUnset,
        {
            self.__unsafe_private_named.3 = ::core::option::Option::Some(value);
            BlockBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
    }
    pub struct Play {
        pub name: String,
        #[serde(flatten)]
        pub becomes: Option<Become>,
        tasks: Vec<Box<Task>>,
        #[serde(flatten)]
        _extra_fields: Value,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Play {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Play",
                "name",
                &self.name,
                "becomes",
                &self.becomes,
                "tasks",
                &self.tasks,
                "_extra_fields",
                &&self._extra_fields,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Play {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_map(
                    __serializer,
                    _serde::__private::None,
                )?;
                _serde::ser::SerializeMap::serialize_entry(
                    &mut __serde_state,
                    "name",
                    &self.name,
                )?;
                _serde::Serialize::serialize(
                    &&self.becomes,
                    _serde::__private::ser::FlatMapSerializer(&mut __serde_state),
                )?;
                _serde::ser::SerializeMap::serialize_entry(
                    &mut __serde_state,
                    "tasks",
                    &self.tasks,
                )?;
                _serde::Serialize::serialize(
                    &&self._extra_fields,
                    _serde::__private::ser::FlatMapSerializer(&mut __serde_state),
                )?;
                _serde::ser::SerializeMap::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Play {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field<'de> {
                    __field0,
                    __field2,
                    __other(_serde::__private::de::Content<'de>),
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::Bool(__value),
                            ),
                        )
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::I8(__value)),
                        )
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I16(__value),
                            ),
                        )
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I32(__value),
                            ),
                        )
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I64(__value),
                            ),
                        )
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::U8(__value)),
                        )
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U16(__value),
                            ),
                        )
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U32(__value),
                            ),
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U64(__value),
                            ),
                        )
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::F32(__value),
                            ),
                        )
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::F64(__value),
                            ),
                        )
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::Char(__value),
                            ),
                        )
                    }
                    fn visit_unit<__E>(
                        self,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::Unit),
                        )
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            "tasks" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
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
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            b"tasks" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::ByteBuf(
                                    __value.to_vec(),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            "tasks" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            b"tasks" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(
                                    __value,
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
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
                    marker: _serde::__private::PhantomData<Play>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Play;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Play",
                        )
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
                        let mut __field2: _serde::__private::Option<Vec<Box<Task>>> = _serde::__private::None;
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<
                                (
                                    _serde::__private::de::Content,
                                    _serde::__private::de::Content,
                                ),
                            >,
                        >::new();
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("tasks"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<Box<Task>>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__other(__name) => {
                                    __collect
                                        .push(
                                            _serde::__private::Some((
                                                __name,
                                                _serde::de::MapAccess::next_value(&mut __map)?,
                                            )),
                                        );
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("name")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("tasks")?
                            }
                        };
                        let __field1: Option<Become> = _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        )?;
                        let __field3: Value = _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        )?;
                        _serde::__private::Ok(Play {
                            name: __field0,
                            becomes: __field1,
                            tasks: __field2,
                            _extra_fields: __field3,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Play>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl Play {
        ///Create an instance of [`Play`] using the builder syntax
        #[inline(always)]
        #[allow(clippy::inline_always, clippy::use_self, clippy::missing_const_for_fn)]
        #[allow(deprecated)]
        pub fn builder() -> PlayBuilder {
            PlayBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: ::core::default::Default::default(),
            }
        }
    }
    #[allow(unnameable_types, unreachable_pub, clippy::redundant_pub_crate)]
    /**Tools for manipulating the type state of [`PlayBuilder`].

See the [detailed guide](https://bon-rs.com/guide/typestate-api) that describes how all the pieces here fit together.*/
    #[allow(deprecated)]
    mod play_builder {
        #[doc(inline)]
        pub use ::bon::__::{IsSet, IsUnset};
        use ::bon::__::{Set, Unset};
        mod sealed {
            pub struct Sealed;
        }
        /**Builder's type state specifies if members are set or not (unset).

You can use the associated types of this trait to control the state of individual members with the [`IsSet`] and [`IsUnset`] traits. You can change the state of the members with the `Set*` structs available in this module.*/
        pub trait State: ::core::marker::Sized {
            /**Type state of the member `name`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type Name;
            /**Type state of the member `becomes`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type Becomes;
            /**Type state of the member `tasks`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type Tasks;
            /**Type state of the member `extra_fields`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type ExtraFields;
            #[doc(hidden)]
            const SEALED: sealed::Sealed;
        }
        /**Marker trait that indicates that all required members are set.

In this state, you can finish building by calling the method [`PlayBuilder::build()`](super::PlayBuilder::build())*/
        pub trait IsComplete: State {
            #[doc(hidden)]
            const SEALED: sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> IsComplete for S
        where
            S::Name: IsSet,
            S::Tasks: IsSet,
            S::ExtraFields: IsSet,
        {
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[deprecated = "this should not be used directly; it is an implementation detail; use the Set* type aliases to control the state of members instead"]
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        mod members {
            pub struct name(());
            pub struct becomes(());
            pub struct tasks(());
            pub struct extra_fields(());
        }
        /// Represents a [`State`] that has [`IsUnset`] implemented for all members.
        ///
        /// This is the initial state of the builder before any setters are called.
        pub struct Empty(());
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::Name`].

The state for all other members is left the same as in the input state.*/
        pub struct SetName<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::Becomes`].

The state for all other members is left the same as in the input state.*/
        pub struct SetBecomes<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::Tasks`].

The state for all other members is left the same as in the input state.*/
        pub struct SetTasks<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::ExtraFields`].

The state for all other members is left the same as in the input state.*/
        pub struct SetExtraFields<S: State = Empty>(
            ::core::marker::PhantomData<fn() -> S>,
        );
        #[doc(hidden)]
        impl State for Empty {
            type Name = Unset<members::name>;
            type Becomes = Unset<members::becomes>;
            type Tasks = Unset<members::tasks>;
            type ExtraFields = Unset<members::extra_fields>;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetName<S> {
            type Name = Set<members::name>;
            type Becomes = S::Becomes;
            type Tasks = S::Tasks;
            type ExtraFields = S::ExtraFields;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetBecomes<S> {
            type Name = S::Name;
            type Becomes = Set<members::becomes>;
            type Tasks = S::Tasks;
            type ExtraFields = S::ExtraFields;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetTasks<S> {
            type Name = S::Name;
            type Becomes = S::Becomes;
            type Tasks = Set<members::tasks>;
            type ExtraFields = S::ExtraFields;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetExtraFields<S> {
            type Name = S::Name;
            type Becomes = S::Becomes;
            type Tasks = S::Tasks;
            type ExtraFields = Set<members::extra_fields>;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
    }
    #[must_use = "the builder does nothing until you call `build()` on it to finish building"]
    ///Use builder syntax to set the inputs and finish with [`build()`](Self::build()).
    #[allow(unused_parens)]
    #[allow(clippy::struct_field_names, clippy::type_complexity)]
    #[allow(deprecated)]
    pub struct PlayBuilder<S: play_builder::State = play_builder::Empty> {
        #[doc(hidden)]
        #[deprecated = "this field should not be used directly; it's an implementation detail, and if you access it directly, you may break some internal unsafe invariants; if you found yourself needing it, then you are probably doing something wrong; feel free to open an issue/discussion in our GitHub repository (https://github.com/elastio/bon) or ask for help in our Discord server (https://bon-rs.com/discord)"]
        __unsafe_private_phantom: ::core::marker::PhantomData<
            (fn() -> S, fn() -> ::core::marker::PhantomData<Play>),
        >,
        #[doc(hidden)]
        #[deprecated = "this field should not be used directly; it's an implementation detail, and if you access it directly, you may break some internal unsafe invariants; if you found yourself needing it, then you are probably doing something wrong; feel free to open an issue/discussion in our GitHub repository (https://github.com/elastio/bon) or ask for help in our Discord server (https://bon-rs.com/discord)"]
        __unsafe_private_named: (
            ::core::option::Option<String>,
            ::core::option::Option<Become>,
            ::core::option::Option<Vec<Box<Task>>>,
            ::core::option::Option<Value>,
        ),
    }
    #[allow(unused_parens)]
    #[automatically_derived]
    #[allow(deprecated)]
    impl<S: play_builder::State> PlayBuilder<S> {
        /// Finish building and return the requested object
        #[inline(always)]
        #[allow(
            clippy::inline_always,
            clippy::future_not_send,
            clippy::missing_const_for_fn,
        )]
        #[must_use = "building a struct without using it is likely a bug"]
        pub fn build(self) -> Play
        where
            S: play_builder::IsComplete,
        {
            let name: String = unsafe {
                ::core::option::Option::unwrap_unchecked(self.__unsafe_private_named.0)
            };
            let becomes: Option<Become> = self.__unsafe_private_named.1;
            let tasks: Vec<Box<Task>> = unsafe {
                ::core::option::Option::unwrap_unchecked(self.__unsafe_private_named.2)
            };
            let _extra_fields: Value = unsafe {
                ::core::option::Option::unwrap_unchecked(self.__unsafe_private_named.3)
            };
            Play {
                name,
                becomes,
                tasks,
                _extra_fields,
            }
        }
        /**_**Required.**_

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn name(mut self, value: String) -> PlayBuilder<play_builder::SetName<S>>
        where
            S::Name: play_builder::IsUnset,
        {
            self.__unsafe_private_named.0 = ::core::option::Option::Some(value);
            PlayBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
        /**_**Optional** ([Some](Self::becomes()) / [Option](Self::maybe_becomes()) setters)._

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn becomes(self, value: Become) -> PlayBuilder<play_builder::SetBecomes<S>>
        where
            S::Becomes: play_builder::IsUnset,
        {
            self.maybe_becomes(Some(value))
        }
        /**_**Optional** ([Some](Self::becomes()) / [Option](Self::maybe_becomes()) setters)._

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn maybe_becomes(
            mut self,
            value: Option<Become>,
        ) -> PlayBuilder<play_builder::SetBecomes<S>>
        where
            S::Becomes: play_builder::IsUnset,
        {
            self.__unsafe_private_named.1 = value;
            PlayBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
        /**_**Required.**_

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn tasks(
            mut self,
            value: Vec<Box<Task>>,
        ) -> PlayBuilder<play_builder::SetTasks<S>>
        where
            S::Tasks: play_builder::IsUnset,
        {
            self.__unsafe_private_named.2 = ::core::option::Option::Some(value);
            PlayBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
        /**_**Required.**_

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn extra_fields(
            mut self,
            value: Value,
        ) -> PlayBuilder<play_builder::SetExtraFields<S>>
        where
            S::ExtraFields: play_builder::IsUnset,
        {
            self.__unsafe_private_named.3 = ::core::option::Option::Some(value);
            PlayBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
    }
    pub struct Role {
        pub name: String,
        #[serde(flatten)]
        pub becomes: Option<Become>,
        #[serde(flatten)]
        _extra_fields: Value,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Role {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Role",
                "name",
                &self.name,
                "becomes",
                &self.becomes,
                "_extra_fields",
                &&self._extra_fields,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Role {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_map(
                    __serializer,
                    _serde::__private::None,
                )?;
                _serde::ser::SerializeMap::serialize_entry(
                    &mut __serde_state,
                    "name",
                    &self.name,
                )?;
                _serde::Serialize::serialize(
                    &&self.becomes,
                    _serde::__private::ser::FlatMapSerializer(&mut __serde_state),
                )?;
                _serde::Serialize::serialize(
                    &&self._extra_fields,
                    _serde::__private::ser::FlatMapSerializer(&mut __serde_state),
                )?;
                _serde::ser::SerializeMap::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Role {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field<'de> {
                    __field0,
                    __other(_serde::__private::de::Content<'de>),
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::Bool(__value),
                            ),
                        )
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::I8(__value)),
                        )
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I16(__value),
                            ),
                        )
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I32(__value),
                            ),
                        )
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I64(__value),
                            ),
                        )
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::U8(__value)),
                        )
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U16(__value),
                            ),
                        )
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U32(__value),
                            ),
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U64(__value),
                            ),
                        )
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::F32(__value),
                            ),
                        )
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::F64(__value),
                            ),
                        )
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::Char(__value),
                            ),
                        )
                    }
                    fn visit_unit<__E>(
                        self,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::Unit),
                        )
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
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
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = _serde::__private::de::Content::ByteBuf(
                                    __value.to_vec(),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(
                                    __value,
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
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
                    marker: _serde::__private::PhantomData<Role>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Role;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Role",
                        )
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
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<
                                (
                                    _serde::__private::de::Content,
                                    _serde::__private::de::Content,
                                ),
                            >,
                        >::new();
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__other(__name) => {
                                    __collect
                                        .push(
                                            _serde::__private::Some((
                                                __name,
                                                _serde::de::MapAccess::next_value(&mut __map)?,
                                            )),
                                        );
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("name")?
                            }
                        };
                        let __field1: Option<Become> = _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        )?;
                        let __field2: Value = _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        )?;
                        _serde::__private::Ok(Role {
                            name: __field0,
                            becomes: __field1,
                            _extra_fields: __field2,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Role>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl Role {
        ///Create an instance of [`Role`] using the builder syntax
        #[inline(always)]
        #[allow(clippy::inline_always, clippy::use_self, clippy::missing_const_for_fn)]
        #[allow(deprecated)]
        pub fn builder() -> RoleBuilder {
            RoleBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: ::core::default::Default::default(),
            }
        }
    }
    #[allow(unnameable_types, unreachable_pub, clippy::redundant_pub_crate)]
    /**Tools for manipulating the type state of [`RoleBuilder`].

See the [detailed guide](https://bon-rs.com/guide/typestate-api) that describes how all the pieces here fit together.*/
    #[allow(deprecated)]
    mod role_builder {
        #[doc(inline)]
        pub use ::bon::__::{IsSet, IsUnset};
        use ::bon::__::{Set, Unset};
        mod sealed {
            pub struct Sealed;
        }
        /**Builder's type state specifies if members are set or not (unset).

You can use the associated types of this trait to control the state of individual members with the [`IsSet`] and [`IsUnset`] traits. You can change the state of the members with the `Set*` structs available in this module.*/
        pub trait State: ::core::marker::Sized {
            /**Type state of the member `name`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type Name;
            /**Type state of the member `becomes`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type Becomes;
            /**Type state of the member `extra_fields`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type ExtraFields;
            #[doc(hidden)]
            const SEALED: sealed::Sealed;
        }
        /**Marker trait that indicates that all required members are set.

In this state, you can finish building by calling the method [`RoleBuilder::build()`](super::RoleBuilder::build())*/
        pub trait IsComplete: State {
            #[doc(hidden)]
            const SEALED: sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> IsComplete for S
        where
            S::Name: IsSet,
            S::ExtraFields: IsSet,
        {
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[deprecated = "this should not be used directly; it is an implementation detail; use the Set* type aliases to control the state of members instead"]
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        mod members {
            pub struct name(());
            pub struct becomes(());
            pub struct extra_fields(());
        }
        /// Represents a [`State`] that has [`IsUnset`] implemented for all members.
        ///
        /// This is the initial state of the builder before any setters are called.
        pub struct Empty(());
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::Name`].

The state for all other members is left the same as in the input state.*/
        pub struct SetName<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::Becomes`].

The state for all other members is left the same as in the input state.*/
        pub struct SetBecomes<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::ExtraFields`].

The state for all other members is left the same as in the input state.*/
        pub struct SetExtraFields<S: State = Empty>(
            ::core::marker::PhantomData<fn() -> S>,
        );
        #[doc(hidden)]
        impl State for Empty {
            type Name = Unset<members::name>;
            type Becomes = Unset<members::becomes>;
            type ExtraFields = Unset<members::extra_fields>;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetName<S> {
            type Name = Set<members::name>;
            type Becomes = S::Becomes;
            type ExtraFields = S::ExtraFields;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetBecomes<S> {
            type Name = S::Name;
            type Becomes = Set<members::becomes>;
            type ExtraFields = S::ExtraFields;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetExtraFields<S> {
            type Name = S::Name;
            type Becomes = S::Becomes;
            type ExtraFields = Set<members::extra_fields>;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
    }
    #[must_use = "the builder does nothing until you call `build()` on it to finish building"]
    ///Use builder syntax to set the inputs and finish with [`build()`](Self::build()).
    #[allow(unused_parens)]
    #[allow(clippy::struct_field_names, clippy::type_complexity)]
    #[allow(deprecated)]
    pub struct RoleBuilder<S: role_builder::State = role_builder::Empty> {
        #[doc(hidden)]
        #[deprecated = "this field should not be used directly; it's an implementation detail, and if you access it directly, you may break some internal unsafe invariants; if you found yourself needing it, then you are probably doing something wrong; feel free to open an issue/discussion in our GitHub repository (https://github.com/elastio/bon) or ask for help in our Discord server (https://bon-rs.com/discord)"]
        __unsafe_private_phantom: ::core::marker::PhantomData<
            (fn() -> S, fn() -> ::core::marker::PhantomData<Role>),
        >,
        #[doc(hidden)]
        #[deprecated = "this field should not be used directly; it's an implementation detail, and if you access it directly, you may break some internal unsafe invariants; if you found yourself needing it, then you are probably doing something wrong; feel free to open an issue/discussion in our GitHub repository (https://github.com/elastio/bon) or ask for help in our Discord server (https://bon-rs.com/discord)"]
        __unsafe_private_named: (
            ::core::option::Option<String>,
            ::core::option::Option<Become>,
            ::core::option::Option<Value>,
        ),
    }
    #[allow(unused_parens)]
    #[automatically_derived]
    #[allow(deprecated)]
    impl<S: role_builder::State> RoleBuilder<S> {
        /// Finish building and return the requested object
        #[inline(always)]
        #[allow(
            clippy::inline_always,
            clippy::future_not_send,
            clippy::missing_const_for_fn,
        )]
        #[must_use = "building a struct without using it is likely a bug"]
        pub fn build(self) -> Role
        where
            S: role_builder::IsComplete,
        {
            let name: String = unsafe {
                ::core::option::Option::unwrap_unchecked(self.__unsafe_private_named.0)
            };
            let becomes: Option<Become> = self.__unsafe_private_named.1;
            let _extra_fields: Value = unsafe {
                ::core::option::Option::unwrap_unchecked(self.__unsafe_private_named.2)
            };
            Role {
                name,
                becomes,
                _extra_fields,
            }
        }
        /**_**Required.**_

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn name(mut self, value: String) -> RoleBuilder<role_builder::SetName<S>>
        where
            S::Name: role_builder::IsUnset,
        {
            self.__unsafe_private_named.0 = ::core::option::Option::Some(value);
            RoleBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
        /**_**Optional** ([Some](Self::becomes()) / [Option](Self::maybe_becomes()) setters)._

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn becomes(self, value: Become) -> RoleBuilder<role_builder::SetBecomes<S>>
        where
            S::Becomes: role_builder::IsUnset,
        {
            self.maybe_becomes(Some(value))
        }
        /**_**Optional** ([Some](Self::becomes()) / [Option](Self::maybe_becomes()) setters)._

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn maybe_becomes(
            mut self,
            value: Option<Become>,
        ) -> RoleBuilder<role_builder::SetBecomes<S>>
        where
            S::Becomes: role_builder::IsUnset,
        {
            self.__unsafe_private_named.1 = value;
            RoleBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
        /**_**Required.**_

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn extra_fields(
            mut self,
            value: Value,
        ) -> RoleBuilder<role_builder::SetExtraFields<S>>
        where
            S::ExtraFields: role_builder::IsUnset,
        {
            self.__unsafe_private_named.2 = ::core::option::Option::Some(value);
            RoleBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
    }
    #[serde(rename_all = "snake_case")]
    pub enum Action {
        IncludeTasks(IncludeTask),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Action {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Action::IncludeTasks(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IncludeTasks",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Action {
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
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
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
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 1",
                                    ),
                                )
                            }
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
                            "include_tasks" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
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
                            b"include_tasks" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
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
                    marker: _serde::__private::PhantomData<Action>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Action;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum Action",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::__private::Result::map(
                                    _serde::de::VariantAccess::newtype_variant::<
                                        IncludeTask,
                                    >(__variant),
                                    Action::IncludeTasks,
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["include_tasks"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Action",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Action>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Action {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Action::IncludeTasks(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Action",
                            0u32,
                            "include_tasks",
                            __field0,
                        )
                    }
                }
            }
        }
    };
    impl Action {
        #[doc(hidden)]
        #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools)]
        fn __orig_include_tasks(file: String) -> Self {
            Action::IncludeTasks(file)
        }
        #[inline(always)]
        #[allow(clippy::inline_always, clippy::use_self, clippy::missing_const_for_fn)]
        #[allow(deprecated)]
        pub fn include_tasks() -> ActionIncludeTasksBuilder {
            ActionIncludeTasksBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: ::core::default::Default::default(),
            }
        }
    }
    #[allow(unnameable_types, unreachable_pub, clippy::redundant_pub_crate)]
    /**Tools for manipulating the type state of [`ActionIncludeTasksBuilder`].

See the [detailed guide](https://bon-rs.com/guide/typestate-api) that describes how all the pieces here fit together.*/
    #[allow(deprecated)]
    mod action_include_tasks_builder {
        #[doc(inline)]
        pub use ::bon::__::{IsSet, IsUnset};
        use ::bon::__::{Set, Unset};
        mod sealed {
            pub struct Sealed;
        }
        /**Builder's type state specifies if members are set or not (unset).

You can use the associated types of this trait to control the state of individual members with the [`IsSet`] and [`IsUnset`] traits. You can change the state of the members with the `Set*` structs available in this module.*/
        pub trait State: ::core::marker::Sized {
            /**Type state of the member `file`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type File;
            #[doc(hidden)]
            const SEALED: sealed::Sealed;
        }
        /**Marker trait that indicates that all required members are set.

In this state, you can finish building by calling the method [`ActionIncludeTasksBuilder::call()`](super::ActionIncludeTasksBuilder::call())*/
        pub trait IsComplete: State {
            #[doc(hidden)]
            const SEALED: sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> IsComplete for S
        where
            S::File: IsSet,
        {
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[deprecated = "this should not be used directly; it is an implementation detail; use the Set* type aliases to control the state of members instead"]
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        mod members {
            pub struct file(());
        }
        /// Represents a [`State`] that has [`IsUnset`] implemented for all members.
        ///
        /// This is the initial state of the builder before any setters are called.
        pub struct Empty(());
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::File`].

The state for all other members is left the same as in the input state.*/
        pub struct SetFile<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        #[doc(hidden)]
        impl State for Empty {
            type File = Unset<members::file>;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetFile<S> {
            type File = Set<members::file>;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
    }
    #[must_use = "the builder does nothing until you call `call()` on it to finish building"]
    ///Use builder syntax to set the inputs and finish with [`call()`](Self::call()).
    #[allow(unused_parens)]
    #[allow(clippy::struct_field_names, clippy::type_complexity)]
    #[allow(deprecated)]
    pub struct ActionIncludeTasksBuilder<
        S: action_include_tasks_builder::State = action_include_tasks_builder::Empty,
    > {
        #[doc(hidden)]
        #[deprecated = "this field should not be used directly; it's an implementation detail, and if you access it directly, you may break some internal unsafe invariants; if you found yourself needing it, then you are probably doing something wrong; feel free to open an issue/discussion in our GitHub repository (https://github.com/elastio/bon) or ask for help in our Discord server (https://bon-rs.com/discord)"]
        __unsafe_private_phantom: ::core::marker::PhantomData<
            (fn() -> S, fn() -> ::core::marker::PhantomData<Action>),
        >,
        #[doc(hidden)]
        #[deprecated = "this field should not be used directly; it's an implementation detail, and if you access it directly, you may break some internal unsafe invariants; if you found yourself needing it, then you are probably doing something wrong; feel free to open an issue/discussion in our GitHub repository (https://github.com/elastio/bon) or ask for help in our Discord server (https://bon-rs.com/discord)"]
        __unsafe_private_named: (::core::option::Option<String>,),
    }
    #[allow(unused_parens)]
    #[automatically_derived]
    #[allow(deprecated)]
    impl<S: action_include_tasks_builder::State> ActionIncludeTasksBuilder<S> {
        /// Finishes building and performs the requested action.
        #[inline(always)]
        #[allow(
            clippy::inline_always,
            clippy::future_not_send,
            clippy::missing_const_for_fn,
        )]
        pub fn call(self) -> Action
        where
            S: action_include_tasks_builder::IsComplete,
        {
            let file: String = unsafe {
                ::core::option::Option::unwrap_unchecked(self.__unsafe_private_named.0)
            };
            <Action>::__orig_include_tasks(file)
        }
        /**_**Required.**_

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn file(
            mut self,
            value: String,
        ) -> ActionIncludeTasksBuilder<action_include_tasks_builder::SetFile<S>>
        where
            S::File: action_include_tasks_builder::IsUnset,
        {
            self.__unsafe_private_named.0 = ::core::option::Option::Some(value);
            ActionIncludeTasksBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
    }
    pub struct IncludeTask {
        #[serde(flatten)]
        file: PathBuf,
        #[serde(skip)]
        tasks: Vec<Box<Task>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for IncludeTask {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "IncludeTask",
                "file",
                &self.file,
                "tasks",
                &&self.tasks,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for IncludeTask {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field<'de> {
                    __other(_serde::__private::de::Content<'de>),
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::Bool(__value),
                            ),
                        )
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::I8(__value)),
                        )
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I16(__value),
                            ),
                        )
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I32(__value),
                            ),
                        )
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I64(__value),
                            ),
                        )
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::U8(__value)),
                        )
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U16(__value),
                            ),
                        )
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U32(__value),
                            ),
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U64(__value),
                            ),
                        )
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::F32(__value),
                            ),
                        )
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::F64(__value),
                            ),
                        )
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::Char(__value),
                            ),
                        )
                    }
                    fn visit_unit<__E>(
                        self,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::Unit),
                        )
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
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
                            _ => {
                                let __value = _serde::__private::de::Content::ByteBuf(
                                    __value.to_vec(),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(
                                    __value,
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
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
                    marker: _serde::__private::PhantomData<IncludeTask>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = IncludeTask;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct IncludeTask",
                        )
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<
                                (
                                    _serde::__private::de::Content,
                                    _serde::__private::de::Content,
                                ),
                            >,
                        >::new();
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__other(__name) => {
                                    __collect
                                        .push(
                                            _serde::__private::Some((
                                                __name,
                                                _serde::de::MapAccess::next_value(&mut __map)?,
                                            )),
                                        );
                                }
                            }
                        }
                        let __field0: PathBuf = _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        )?;
                        _serde::__private::Ok(IncludeTask {
                            file: __field0,
                            tasks: _serde::__private::Default::default(),
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<IncludeTask>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for IncludeTask {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_map(
                    __serializer,
                    _serde::__private::None,
                )?;
                _serde::Serialize::serialize(
                    &&self.file,
                    _serde::__private::ser::FlatMapSerializer(&mut __serde_state),
                )?;
                _serde::ser::SerializeMap::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    impl IncludeTask {
        ///Create an instance of [`IncludeTask`] using the builder syntax
        #[inline(always)]
        #[allow(clippy::inline_always, clippy::use_self, clippy::missing_const_for_fn)]
        #[allow(deprecated)]
        pub fn builder() -> IncludeTaskBuilder {
            IncludeTaskBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: ::core::default::Default::default(),
            }
        }
    }
    #[allow(unnameable_types, unreachable_pub, clippy::redundant_pub_crate)]
    /**Tools for manipulating the type state of [`IncludeTaskBuilder`].

See the [detailed guide](https://bon-rs.com/guide/typestate-api) that describes how all the pieces here fit together.*/
    #[allow(deprecated)]
    mod include_task_builder {
        #[doc(inline)]
        pub use ::bon::__::{IsSet, IsUnset};
        use ::bon::__::{Set, Unset};
        mod sealed {
            pub struct Sealed;
        }
        /**Builder's type state specifies if members are set or not (unset).

You can use the associated types of this trait to control the state of individual members with the [`IsSet`] and [`IsUnset`] traits. You can change the state of the members with the `Set*` structs available in this module.*/
        pub trait State: ::core::marker::Sized {
            /**Type state of the member `file`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type File;
            /**Type state of the member `tasks`.

It can implement either [`IsSet`] or [`IsUnset`]*/
            type Tasks;
            #[doc(hidden)]
            const SEALED: sealed::Sealed;
        }
        /**Marker trait that indicates that all required members are set.

In this state, you can finish building by calling the method [`IncludeTaskBuilder::build()`](super::IncludeTaskBuilder::build())*/
        pub trait IsComplete: State {
            #[doc(hidden)]
            const SEALED: sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> IsComplete for S
        where
            S::File: IsSet,
            S::Tasks: IsSet,
        {
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[deprecated = "this should not be used directly; it is an implementation detail; use the Set* type aliases to control the state of members instead"]
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        mod members {
            pub struct file(());
            pub struct tasks(());
        }
        /// Represents a [`State`] that has [`IsUnset`] implemented for all members.
        ///
        /// This is the initial state of the builder before any setters are called.
        pub struct Empty(());
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::File`].

The state for all other members is left the same as in the input state.*/
        pub struct SetFile<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        /**Represents a [`State`] that has [`IsSet`] implemented for [`State::Tasks`].

The state for all other members is left the same as in the input state.*/
        pub struct SetTasks<S: State = Empty>(::core::marker::PhantomData<fn() -> S>);
        #[doc(hidden)]
        impl State for Empty {
            type File = Unset<members::file>;
            type Tasks = Unset<members::tasks>;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetFile<S> {
            type File = Set<members::file>;
            type Tasks = S::Tasks;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
        #[doc(hidden)]
        impl<S: State> State for SetTasks<S> {
            type File = S::File;
            type Tasks = Set<members::tasks>;
            const SEALED: sealed::Sealed = sealed::Sealed;
        }
    }
    #[must_use = "the builder does nothing until you call `build()` on it to finish building"]
    ///Use builder syntax to set the inputs and finish with [`build()`](Self::build()).
    #[allow(unused_parens)]
    #[allow(clippy::struct_field_names, clippy::type_complexity)]
    #[allow(deprecated)]
    pub struct IncludeTaskBuilder<
        S: include_task_builder::State = include_task_builder::Empty,
    > {
        #[doc(hidden)]
        #[deprecated = "this field should not be used directly; it's an implementation detail, and if you access it directly, you may break some internal unsafe invariants; if you found yourself needing it, then you are probably doing something wrong; feel free to open an issue/discussion in our GitHub repository (https://github.com/elastio/bon) or ask for help in our Discord server (https://bon-rs.com/discord)"]
        __unsafe_private_phantom: ::core::marker::PhantomData<
            (fn() -> S, fn() -> ::core::marker::PhantomData<IncludeTask>),
        >,
        #[doc(hidden)]
        #[deprecated = "this field should not be used directly; it's an implementation detail, and if you access it directly, you may break some internal unsafe invariants; if you found yourself needing it, then you are probably doing something wrong; feel free to open an issue/discussion in our GitHub repository (https://github.com/elastio/bon) or ask for help in our Discord server (https://bon-rs.com/discord)"]
        __unsafe_private_named: (
            ::core::option::Option<PathBuf>,
            ::core::option::Option<Vec<Box<Task>>>,
        ),
    }
    #[allow(unused_parens)]
    #[automatically_derived]
    #[allow(deprecated)]
    impl<S: include_task_builder::State> IncludeTaskBuilder<S> {
        /// Finish building and return the requested object
        #[inline(always)]
        #[allow(
            clippy::inline_always,
            clippy::future_not_send,
            clippy::missing_const_for_fn,
        )]
        #[must_use = "building a struct without using it is likely a bug"]
        pub fn build(self) -> IncludeTask
        where
            S: include_task_builder::IsComplete,
        {
            let file: PathBuf = unsafe {
                ::core::option::Option::unwrap_unchecked(self.__unsafe_private_named.0)
            };
            let tasks: Vec<Box<Task>> = unsafe {
                ::core::option::Option::unwrap_unchecked(self.__unsafe_private_named.1)
            };
            IncludeTask { file, tasks }
        }
        /**_**Required.**_

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn file(
            mut self,
            value: PathBuf,
        ) -> IncludeTaskBuilder<include_task_builder::SetFile<S>>
        where
            S::File: include_task_builder::IsUnset,
        {
            self.__unsafe_private_named.0 = ::core::option::Option::Some(value);
            IncludeTaskBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
        /**_**Required.**_

*/
        #[allow(
            clippy::inline_always,
            clippy::impl_trait_in_params,
            clippy::missing_const_for_fn,
        )]
        #[inline(always)]
        pub fn tasks(
            mut self,
            value: Vec<Box<Task>>,
        ) -> IncludeTaskBuilder<include_task_builder::SetTasks<S>>
        where
            S::Tasks: include_task_builder::IsUnset,
        {
            self.__unsafe_private_named.1 = ::core::option::Option::Some(value);
            IncludeTaskBuilder {
                __unsafe_private_phantom: ::core::marker::PhantomData,
                __unsafe_private_named: self.__unsafe_private_named,
            }
        }
    }
}
#[command(version, about, long_about = None)]
struct Cli {
    cwd: PathBuf,
    relpath: PathBuf,
}
#[automatically_derived]
#[allow(unused_qualifications, clippy::redundant_locals)]
impl clap::Parser for Cli {}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::CommandFactory for Cli {
    fn command<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("ansible-playbook-editor");
        <Self as clap::Args>::augment_args(__clap_app)
    }
    fn command_for_update<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("ansible-playbook-editor");
        <Self as clap::Args>::augment_args_for_update(__clap_app)
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::FromArgMatches for Cli {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        let v = Cli {
            cwd: __clap_arg_matches
                .remove_one::<PathBuf>("cwd")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: cwd",
                ))?,
            relpath: __clap_arg_matches
                .remove_one::<PathBuf>("relpath")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: relpath",
                ))?,
        };
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if __clap_arg_matches.contains_id("cwd") {
            #[allow(non_snake_case)]
            let cwd = &mut self.cwd;
            *cwd = __clap_arg_matches
                .remove_one::<PathBuf>("cwd")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: cwd",
                ))?;
        }
        if __clap_arg_matches.contains_id("relpath") {
            #[allow(non_snake_case)]
            let relpath = &mut self.relpath;
            *relpath = __clap_arg_matches
                .remove_one::<PathBuf>("relpath")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: relpath",
                ))?;
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::Args for Cli {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("Cli"))
    }
    fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("Cli")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 2usize] = [
                                clap::Id::from("cwd"),
                                clap::Id::from("relpath"),
                            ];
                            members
                        }),
                );
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("cwd")
                        .value_name("CWD")
                        .required(true && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::impl_prelude::*;
                            let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                PathBuf,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg;
                    let arg = arg;
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("relpath")
                        .value_name("RELPATH")
                        .required(true && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::impl_prelude::*;
                            let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                PathBuf,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg;
                    let arg = arg;
                    arg
                });
            __clap_app.version("0.1.0").long_about(None)
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("Cli")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 2usize] = [
                                clap::Id::from("cwd"),
                                clap::Id::from("relpath"),
                            ];
                            members
                        }),
                );
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("cwd")
                        .value_name("CWD")
                        .required(true && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::impl_prelude::*;
                            let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                PathBuf,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg;
                    let arg = arg.required(false);
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("relpath")
                        .value_name("RELPATH")
                        .required(true && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::impl_prelude::*;
                            let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                PathBuf,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg;
                    let arg = arg.required(false);
                    arg
                });
            __clap_app.version("0.1.0").long_about(None)
        }
    }
}
fn main() {
    let args = Cli::parse();
    let path = args.cwd.join(args.relpath);
    std::env::set_current_dir(args.cwd).unwrap();
    let file = std::fs::read_to_string(path).unwrap();
    let playbook: file::Playbook = serde_yaml::from_str(&file).unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", playbook));
    };
    {
        ::std::io::_print(
            format_args!("{0}\n", serde_json::to_string_pretty(&playbook).unwrap()),
        );
    };
}
