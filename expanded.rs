#![feature(prelude_import)]
#![no_std]
#![feature(custom_attribute)]
#![feature(proc_macro_hygiene, decl_macro)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket_codegen;
#[macro_use]
extern crate serde;
extern crate dotenv;
extern crate rocket;
pub mod database {
    pub mod library {
        use crate::database::media::*;
        use crate::schema::library;
        use diesel::prelude::*;
        use rocket_contrib::json::Json;
        #[table_name = "library"]
        pub struct Library {
            pub id: i32,
            pub name: String,
            pub location: String,
            pub media_type: String,
        }
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_queryable_for_library() {
            extern crate std;
            use diesel;
            use diesel::Queryable;
            impl<__DB: diesel::backend::Backend, __ST> Queryable<__ST, __DB> for Library
            where
                (i32, String, String, String): Queryable<__ST, __DB>,
            {
                type Row = <(i32, String, String, String) as Queryable<__ST, __DB>>::Row;
                fn build(row: Self::Row) -> Self {
                    let row: (i32, String, String, String) = Queryable::build(row);
                    Self {
                        id: (row.0.into()),
                        name: (row.1.into()),
                        location: (row.2.into()),
                        media_type: (row.3.into()),
                    }
                }
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_Library: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Library {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Library",
                        false as usize + 1 + 1 + 1 + 1,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "media_type",
                        &self.media_type,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_Library: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Library {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                2u64 => _serde::export::Ok(__Field::__field2),
                                3u64 => _serde::export::Ok(__Field::__field3),
                                _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 4",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::export::Ok(__Field::__field0),
                                "name" => _serde::export::Ok(__Field::__field1),
                                "location" => _serde::export::Ok(__Field::__field2),
                                "media_type" => _serde::export::Ok(__Field::__field3),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::export::Ok(__Field::__field0),
                                b"name" => _serde::export::Ok(__Field::__field1),
                                b"location" => _serde::export::Ok(__Field::__field2),
                                b"media_type" => _serde::export::Ok(__Field::__field3),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<Library>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Library;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "struct Library")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<i32>(
                                &mut __seq,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Library with 4 elements",
                                    ));
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Library with 4 elements",
                                    ));
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Library with 4 elements",
                                    ));
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Library with 4 elements",
                                    ));
                                }
                            };
                            _serde::export::Ok(Library {
                                id: __field0,
                                name: __field1,
                                location: __field2,
                                media_type: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::export::Option<i32> = _serde::export::None;
                            let mut __field1: _serde::export::Option<String> = _serde::export::None;
                            let mut __field2: _serde::export::Option<String> = _serde::export::None;
                            let mut __field3: _serde::export::Option<String> = _serde::export::None;
                            while let _serde::export::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::export::Option::is_some(&__field0) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "id",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<i32>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::export::Option::is_some(&__field1) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "name",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::export::Option::is_some(&__field2) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::export::Option::is_some(&__field3) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "media_type",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::export::Some(__field0) => __field0,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("id") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::export::Some(__field1) => __field1,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("name") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::export::Some(__field2) => __field2,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("location") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::export::Some(__field3) => __field3,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("media_type") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::export::Ok(Library {
                                id: __field0,
                                name: __field1,
                                location: __field2,
                                media_type: __field3,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["id", "name", "location", "media_type"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Library",
                        FIELDS,
                        __Visitor {
                            marker: _serde::export::PhantomData::<Library>,
                            lifetime: _serde::export::PhantomData,
                        },
                    )
                }
            }
        };
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_identifiable_for_library() {
            extern crate std;
            use diesel;
            use diesel::associations::{HasTable, Identifiable};
            impl HasTable for Library {
                type Table = library::table;
                fn table() -> Self::Table {
                    library::table
                }
            }
            impl<'ident> Identifiable for &'ident Library {
                type Id = (&'ident i32);
                fn id(self) -> Self::Id {
                    (&self.id)
                }
            }
        }
        #[table_name = "library"]
        pub struct InsertableLibrary {
            pub name: String,
            pub location: String,
            pub media_type: String,
        }
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_insertable_for_insertablelibrary() {
            extern crate std;
            use diesel;
            use diesel::insertable::Insertable;
            use diesel::prelude::*;
            use diesel::query_builder::UndecoratedInsertRecord;
            impl<'insert> Insertable<library::table> for InsertableLibrary {
                type Values = <(
                    std::option::Option<diesel::dsl::Eq<library::name, String>>,
                    std::option::Option<diesel::dsl::Eq<library::location, String>>,
                    std::option::Option<diesel::dsl::Eq<library::media_type, String>>,
                ) as Insertable<library::table>>::Values;
                fn values(self) -> Self::Values {
                    (
                        std::option::Option::Some(library::name.eq(self.name)),
                        std::option::Option::Some(library::location.eq(self.location)),
                        std::option::Option::Some(library::media_type.eq(self.media_type)),
                    )
                        .values()
                }
            }
            impl<'insert> Insertable<library::table> for &'insert InsertableLibrary {
                type Values = <(
                    std::option::Option<diesel::dsl::Eq<library::name, &'insert String>>,
                    std::option::Option<diesel::dsl::Eq<library::location, &'insert String>>,
                    std::option::Option<diesel::dsl::Eq<library::media_type, &'insert String>>,
                ) as Insertable<library::table>>::Values;
                fn values(self) -> Self::Values {
                    (
                        std::option::Option::Some(library::name.eq(&self.name)),
                        std::option::Option::Some(library::location.eq(&self.location)),
                        std::option::Option::Some(library::media_type.eq(&self.media_type)),
                    )
                        .values()
                }
            }
            impl<'insert> UndecoratedInsertRecord<library::table> for InsertableLibrary {}
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_InsertableLibrary: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for InsertableLibrary {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "InsertableLibrary",
                        false as usize + 1 + 1 + 1,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "media_type",
                        &self.media_type,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_InsertableLibrary: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for InsertableLibrary {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                2u64 => _serde::export::Ok(__Field::__field2),
                                _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 3",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "name" => _serde::export::Ok(__Field::__field0),
                                "location" => _serde::export::Ok(__Field::__field1),
                                "media_type" => _serde::export::Ok(__Field::__field2),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"name" => _serde::export::Ok(__Field::__field0),
                                b"location" => _serde::export::Ok(__Field::__field1),
                                b"media_type" => _serde::export::Ok(__Field::__field2),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<InsertableLibrary>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = InsertableLibrary;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(
                                __formatter,
                                "struct InsertableLibrary",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct InsertableLibrary with 3 elements",
                                    ));
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct InsertableLibrary with 3 elements",
                                    ));
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct InsertableLibrary with 3 elements",
                                    ));
                                }
                            };
                            _serde::export::Ok(InsertableLibrary {
                                name: __field0,
                                location: __field1,
                                media_type: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::export::Option<String> = _serde::export::None;
                            let mut __field1: _serde::export::Option<String> = _serde::export::None;
                            let mut __field2: _serde::export::Option<String> = _serde::export::None;
                            while let _serde::export::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::export::Option::is_some(&__field0) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "name",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::export::Option::is_some(&__field1) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::export::Option::is_some(&__field2) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "media_type",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::export::Some(__field0) => __field0,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("name") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::export::Some(__field1) => __field1,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("location") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::export::Some(__field2) => __field2,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("media_type") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::export::Ok(InsertableLibrary {
                                name: __field0,
                                location: __field1,
                                media_type: __field2,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["name", "location", "media_type"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "InsertableLibrary",
                        FIELDS,
                        __Visitor {
                            marker: _serde::export::PhantomData::<InsertableLibrary>,
                            lifetime: _serde::export::PhantomData,
                        },
                    )
                }
            }
        };
        impl Library {
            pub fn get_all(conn: &diesel::SqliteConnection) -> Json<Vec<Library>> {
                use crate::schema::library::dsl::*;
                library
                    .load::<Library>(conn)
                    .map(|x| Json(x))
                    .expect("Error querying all libraries")
            }
            pub fn get(
                conn: &diesel::SqliteConnection,
                lib_id: i32,
            ) -> Result<Json<Vec<Media>>, diesel::result::Error> {
                use crate::schema::library::dsl::*;
                let result = library.filter(id.eq(lib_id)).first::<Library>(conn)?;
                Media::get_all(conn, lib_id, result)
            }
            pub fn new(
                conn: &diesel::SqliteConnection,
                data: Json<InsertableLibrary>,
            ) -> Result<usize, diesel::result::Error> {
                use crate::schema::library;
                let result = diesel::insert_into(library::table)
                    .values(&*data)
                    .execute(conn)?;
                Ok(result)
            }
            pub fn delete(
                conn: &diesel::SqliteConnection,
                id_to_del: i32,
            ) -> Result<usize, diesel::result::Error> {
                use crate::schema::library::dsl::*;
                let result = diesel::delete(library.filter(id.eq(id_to_del))).execute(conn)?;
                Ok(result)
            }
        }
    }
    pub mod media {
        use crate::database::library::Library;
        use crate::schema::media;
        use diesel::prelude::*;
        use rocket_contrib::json::Json;
        #[belongs_to(Library, foreign_key = "library_id")]
        #[table_name = "media"]
        pub struct Media {
            pub id: i32,
            pub library_id: i32,
            pub name: String,
            pub description: Option<String>,
            pub rating: Option<i32>,
            pub year: Option<i32>,
            pub added: Option<String>,
            pub poster_path: Option<String>,
            pub media_type: Option<String>,
        }
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_identifiable_for_media() {
            extern crate std;
            use diesel;
            use diesel::associations::{HasTable, Identifiable};
            impl HasTable for Media {
                type Table = media::table;
                fn table() -> Self::Table {
                    media::table
                }
            }
            impl<'ident> Identifiable for &'ident Media {
                type Id = (&'ident i32);
                fn id(self) -> Self::Id {
                    (&self.id)
                }
            }
        }
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_queryable_for_media() {
            extern crate std;
            use diesel;
            use diesel::Queryable;
            impl<__DB: diesel::backend::Backend, __ST> Queryable<__ST, __DB> for Media
            where
                (
                    i32,
                    i32,
                    String,
                    Option<String>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                ): Queryable<__ST, __DB>,
            {
                type Row = <(
                    i32,
                    i32,
                    String,
                    Option<String>,
                    Option<i32>,
                    Option<i32>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                ) as Queryable<__ST, __DB>>::Row;
                fn build(row: Self::Row) -> Self {
                    let row: (
                        i32,
                        i32,
                        String,
                        Option<String>,
                        Option<i32>,
                        Option<i32>,
                        Option<String>,
                        Option<String>,
                        Option<String>,
                    ) = Queryable::build(row);
                    Self {
                        id: (row.0.into()),
                        library_id: (row.1.into()),
                        name: (row.2.into()),
                        description: (row.3.into()),
                        rating: (row.4.into()),
                        year: (row.5.into()),
                        added: (row.6.into()),
                        poster_path: (row.7.into()),
                        media_type: (row.8.into()),
                    }
                }
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_Media: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Media {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Media",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "library_id",
                        &self.library_id,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "description",
                        &self.description,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "rating",
                        &self.rating,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "year",
                        &self.year,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "added",
                        &self.added,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "poster_path",
                        &self.poster_path,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "media_type",
                        &self.media_type,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_Media: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Media {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
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
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                2u64 => _serde::export::Ok(__Field::__field2),
                                3u64 => _serde::export::Ok(__Field::__field3),
                                4u64 => _serde::export::Ok(__Field::__field4),
                                5u64 => _serde::export::Ok(__Field::__field5),
                                6u64 => _serde::export::Ok(__Field::__field6),
                                7u64 => _serde::export::Ok(__Field::__field7),
                                8u64 => _serde::export::Ok(__Field::__field8),
                                _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 9",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::export::Ok(__Field::__field0),
                                "library_id" => _serde::export::Ok(__Field::__field1),
                                "name" => _serde::export::Ok(__Field::__field2),
                                "description" => _serde::export::Ok(__Field::__field3),
                                "rating" => _serde::export::Ok(__Field::__field4),
                                "year" => _serde::export::Ok(__Field::__field5),
                                "added" => _serde::export::Ok(__Field::__field6),
                                "poster_path" => _serde::export::Ok(__Field::__field7),
                                "media_type" => _serde::export::Ok(__Field::__field8),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::export::Ok(__Field::__field0),
                                b"library_id" => _serde::export::Ok(__Field::__field1),
                                b"name" => _serde::export::Ok(__Field::__field2),
                                b"description" => _serde::export::Ok(__Field::__field3),
                                b"rating" => _serde::export::Ok(__Field::__field4),
                                b"year" => _serde::export::Ok(__Field::__field5),
                                b"added" => _serde::export::Ok(__Field::__field6),
                                b"poster_path" => _serde::export::Ok(__Field::__field7),
                                b"media_type" => _serde::export::Ok(__Field::__field8),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<Media>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Media;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "struct Media")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<i32>(
                                &mut __seq,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Media with 9 elements",
                                    ));
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<i32>(
                                &mut __seq,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Media with 9 elements",
                                    ));
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Media with 9 elements",
                                    ));
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Media with 9 elements",
                                    ));
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Option<i32>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct Media with 9 elements",
                                    ));
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<
                                Option<i32>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct Media with 9 elements",
                                    ));
                                }
                            };
                            let __field6 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct Media with 9 elements",
                                    ));
                                }
                            };
                            let __field7 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        7usize,
                                        &"struct Media with 9 elements",
                                    ));
                                }
                            };
                            let __field8 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        8usize,
                                        &"struct Media with 9 elements",
                                    ));
                                }
                            };
                            _serde::export::Ok(Media {
                                id: __field0,
                                library_id: __field1,
                                name: __field2,
                                description: __field3,
                                rating: __field4,
                                year: __field5,
                                added: __field6,
                                poster_path: __field7,
                                media_type: __field8,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::export::Option<i32> = _serde::export::None;
                            let mut __field1: _serde::export::Option<i32> = _serde::export::None;
                            let mut __field2: _serde::export::Option<String> = _serde::export::None;
                            let mut __field3: _serde::export::Option<Option<String>> =
                                _serde::export::None;
                            let mut __field4: _serde::export::Option<Option<i32>> =
                                _serde::export::None;
                            let mut __field5: _serde::export::Option<Option<i32>> =
                                _serde::export::None;
                            let mut __field6: _serde::export::Option<Option<String>> =
                                _serde::export::None;
                            let mut __field7: _serde::export::Option<Option<String>> =
                                _serde::export::None;
                            let mut __field8: _serde::export::Option<Option<String>> =
                                _serde::export::None;
                            while let _serde::export::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::export::Option::is_some(&__field0) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "id",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<i32>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::export::Option::is_some(&__field1) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "library_id",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<i32>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::export::Option::is_some(&__field2) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "name",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::export::Option::is_some(&__field3) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "description",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::export::Option::is_some(&__field4) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "rating",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<i32>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::export::Option::is_some(&__field5) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "year",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<i32>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::export::Option::is_some(&__field6) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "added",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::export::Option::is_some(&__field7) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "poster_path",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::export::Option::is_some(&__field8) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "media_type",
                                                ),
                                            );
                                        }
                                        __field8 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::export::Some(__field0) => __field0,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("id") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::export::Some(__field1) => __field1,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("library_id") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::export::Some(__field2) => __field2,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("name") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::export::Some(__field3) => __field3,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("description") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::export::Some(__field4) => __field4,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("rating") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::export::Some(__field5) => __field5,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("year") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::export::Some(__field6) => __field6,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("added") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::export::Some(__field7) => __field7,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("poster_path") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::export::Some(__field8) => __field8,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("media_type") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::export::Ok(Media {
                                id: __field0,
                                library_id: __field1,
                                name: __field2,
                                description: __field3,
                                rating: __field4,
                                year: __field5,
                                added: __field6,
                                poster_path: __field7,
                                media_type: __field8,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "library_id",
                        "name",
                        "description",
                        "rating",
                        "year",
                        "added",
                        "poster_path",
                        "media_type",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Media",
                        FIELDS,
                        __Visitor {
                            marker: _serde::export::PhantomData::<Media>,
                            lifetime: _serde::export::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for Media {
            #[inline]
            fn eq(&self, other: &Media) -> bool {
                match *other {
                    Media {
                        id: ref __self_1_0,
                        library_id: ref __self_1_1,
                        name: ref __self_1_2,
                        description: ref __self_1_3,
                        rating: ref __self_1_4,
                        year: ref __self_1_5,
                        added: ref __self_1_6,
                        poster_path: ref __self_1_7,
                        media_type: ref __self_1_8,
                    } => match *self {
                        Media {
                            id: ref __self_0_0,
                            library_id: ref __self_0_1,
                            name: ref __self_0_2,
                            description: ref __self_0_3,
                            rating: ref __self_0_4,
                            year: ref __self_0_5,
                            added: ref __self_0_6,
                            poster_path: ref __self_0_7,
                            media_type: ref __self_0_8,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                                && (*__self_0_3) == (*__self_1_3)
                                && (*__self_0_4) == (*__self_1_4)
                                && (*__self_0_5) == (*__self_1_5)
                                && (*__self_0_6) == (*__self_1_6)
                                && (*__self_0_7) == (*__self_1_7)
                                && (*__self_0_8) == (*__self_1_8)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &Media) -> bool {
                match *other {
                    Media {
                        id: ref __self_1_0,
                        library_id: ref __self_1_1,
                        name: ref __self_1_2,
                        description: ref __self_1_3,
                        rating: ref __self_1_4,
                        year: ref __self_1_5,
                        added: ref __self_1_6,
                        poster_path: ref __self_1_7,
                        media_type: ref __self_1_8,
                    } => match *self {
                        Media {
                            id: ref __self_0_0,
                            library_id: ref __self_0_1,
                            name: ref __self_0_2,
                            description: ref __self_0_3,
                            rating: ref __self_0_4,
                            year: ref __self_0_5,
                            added: ref __self_0_6,
                            poster_path: ref __self_0_7,
                            media_type: ref __self_0_8,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                                || (*__self_0_3) != (*__self_1_3)
                                || (*__self_0_4) != (*__self_1_4)
                                || (*__self_0_5) != (*__self_1_5)
                                || (*__self_0_6) != (*__self_1_6)
                                || (*__self_0_7) != (*__self_1_7)
                                || (*__self_0_8) != (*__self_1_8)
                        }
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for Media {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    Media {
                        id: ref __self_0_0,
                        library_id: ref __self_0_1,
                        name: ref __self_0_2,
                        description: ref __self_0_3,
                        rating: ref __self_0_4,
                        year: ref __self_0_5,
                        added: ref __self_0_6,
                        poster_path: ref __self_0_7,
                        media_type: ref __self_0_8,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("Media");
                        let _ = debug_trait_builder.field("id", &&(*__self_0_0));
                        let _ = debug_trait_builder.field("library_id", &&(*__self_0_1));
                        let _ = debug_trait_builder.field("name", &&(*__self_0_2));
                        let _ = debug_trait_builder.field("description", &&(*__self_0_3));
                        let _ = debug_trait_builder.field("rating", &&(*__self_0_4));
                        let _ = debug_trait_builder.field("year", &&(*__self_0_5));
                        let _ = debug_trait_builder.field("added", &&(*__self_0_6));
                        let _ = debug_trait_builder.field("poster_path", &&(*__self_0_7));
                        let _ = debug_trait_builder.field("media_type", &&(*__self_0_8));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_associations_for_media() {
            extern crate std;
            use diesel;
            impl<__FK> diesel::associations::BelongsTo<Library> for Media
            where
                __FK: std::hash::Hash + std::cmp::Eq,
                for<'__a> &'__a i32: std::convert::Into<::std::option::Option<&'__a __FK>>,
                for<'__a> &'__a Library: diesel::associations::Identifiable<Id = &'__a __FK>,
            {
                type ForeignKey = __FK;
                type ForeignKeyColumn = media::library_id;
                fn foreign_key(&self) -> std::option::Option<&Self::ForeignKey> {
                    std::convert::Into::into(&self.library_id)
                }
                fn foreign_key_column() -> Self::ForeignKeyColumn {
                    media::library_id
                }
            }
        }
        #[table_name = "media"]
        pub struct InsertableMedia {
            pub library_id: i32,
            pub name: String,
            pub description: Option<String>,
            pub rating: Option<i32>,
            pub year: Option<i32>,
            pub added: String,
            pub poster_path: Option<String>,
            pub media_type: Option<String>,
        }
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_insertable_for_insertablemedia() {
            extern crate std;
            use diesel;
            use diesel::insertable::Insertable;
            use diesel::prelude::*;
            use diesel::query_builder::UndecoratedInsertRecord;
            impl<'insert> Insertable<media::table> for InsertableMedia {
                type Values = <(
                    std::option::Option<diesel::dsl::Eq<media::library_id, i32>>,
                    std::option::Option<diesel::dsl::Eq<media::name, String>>,
                    std::option::Option<diesel::dsl::Eq<media::description, String>>,
                    std::option::Option<diesel::dsl::Eq<media::rating, i32>>,
                    std::option::Option<diesel::dsl::Eq<media::year, i32>>,
                    std::option::Option<diesel::dsl::Eq<media::added, String>>,
                    std::option::Option<diesel::dsl::Eq<media::poster_path, String>>,
                    std::option::Option<diesel::dsl::Eq<media::media_type, String>>,
                ) as Insertable<media::table>>::Values;
                fn values(self) -> Self::Values {
                    (
                        std::option::Option::Some(media::library_id.eq(self.library_id)),
                        std::option::Option::Some(media::name.eq(self.name)),
                        self.description.map(|x| media::description.eq(x)),
                        self.rating.map(|x| media::rating.eq(x)),
                        self.year.map(|x| media::year.eq(x)),
                        std::option::Option::Some(media::added.eq(self.added)),
                        self.poster_path.map(|x| media::poster_path.eq(x)),
                        self.media_type.map(|x| media::media_type.eq(x)),
                    )
                        .values()
                }
            }
            impl<'insert> Insertable<media::table> for &'insert InsertableMedia {
                type Values = <(
                    std::option::Option<diesel::dsl::Eq<media::library_id, &'insert i32>>,
                    std::option::Option<diesel::dsl::Eq<media::name, &'insert String>>,
                    std::option::Option<diesel::dsl::Eq<media::description, &'insert String>>,
                    std::option::Option<diesel::dsl::Eq<media::rating, &'insert i32>>,
                    std::option::Option<diesel::dsl::Eq<media::year, &'insert i32>>,
                    std::option::Option<diesel::dsl::Eq<media::added, &'insert String>>,
                    std::option::Option<diesel::dsl::Eq<media::poster_path, &'insert String>>,
                    std::option::Option<diesel::dsl::Eq<media::media_type, &'insert String>>,
                ) as Insertable<media::table>>::Values;
                fn values(self) -> Self::Values {
                    (
                        std::option::Option::Some(media::library_id.eq(&self.library_id)),
                        std::option::Option::Some(media::name.eq(&self.name)),
                        self.description.as_ref().map(|x| media::description.eq(x)),
                        self.rating.as_ref().map(|x| media::rating.eq(x)),
                        self.year.as_ref().map(|x| media::year.eq(x)),
                        std::option::Option::Some(media::added.eq(&self.added)),
                        self.poster_path.as_ref().map(|x| media::poster_path.eq(x)),
                        self.media_type.as_ref().map(|x| media::media_type.eq(x)),
                    )
                        .values()
                }
            }
            impl<'insert> UndecoratedInsertRecord<media::table> for InsertableMedia {}
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_InsertableMedia: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for InsertableMedia {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "InsertableMedia",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "library_id",
                        &self.library_id,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "description",
                        &self.description,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "rating",
                        &self.rating,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "year",
                        &self.year,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "added",
                        &self.added,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "poster_path",
                        &self.poster_path,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "media_type",
                        &self.media_type,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_InsertableMedia: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for InsertableMedia {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                2u64 => _serde::export::Ok(__Field::__field2),
                                3u64 => _serde::export::Ok(__Field::__field3),
                                4u64 => _serde::export::Ok(__Field::__field4),
                                5u64 => _serde::export::Ok(__Field::__field5),
                                6u64 => _serde::export::Ok(__Field::__field6),
                                7u64 => _serde::export::Ok(__Field::__field7),
                                _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 8",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "library_id" => _serde::export::Ok(__Field::__field0),
                                "name" => _serde::export::Ok(__Field::__field1),
                                "description" => _serde::export::Ok(__Field::__field2),
                                "rating" => _serde::export::Ok(__Field::__field3),
                                "year" => _serde::export::Ok(__Field::__field4),
                                "added" => _serde::export::Ok(__Field::__field5),
                                "poster_path" => _serde::export::Ok(__Field::__field6),
                                "media_type" => _serde::export::Ok(__Field::__field7),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"library_id" => _serde::export::Ok(__Field::__field0),
                                b"name" => _serde::export::Ok(__Field::__field1),
                                b"description" => _serde::export::Ok(__Field::__field2),
                                b"rating" => _serde::export::Ok(__Field::__field3),
                                b"year" => _serde::export::Ok(__Field::__field4),
                                b"added" => _serde::export::Ok(__Field::__field5),
                                b"poster_path" => _serde::export::Ok(__Field::__field6),
                                b"media_type" => _serde::export::Ok(__Field::__field7),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<InsertableMedia>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = InsertableMedia;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(
                                __formatter,
                                "struct InsertableMedia",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<i32>(
                                &mut __seq,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct InsertableMedia with 8 elements",
                                    ));
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct InsertableMedia with 8 elements",
                                    ));
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct InsertableMedia with 8 elements",
                                    ));
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<i32>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct InsertableMedia with 8 elements",
                                    ));
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Option<i32>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct InsertableMedia with 8 elements",
                                    ));
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct InsertableMedia with 8 elements",
                                    ));
                                }
                            };
                            let __field6 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct InsertableMedia with 8 elements",
                                    ));
                                }
                            };
                            let __field7 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        7usize,
                                        &"struct InsertableMedia with 8 elements",
                                    ));
                                }
                            };
                            _serde::export::Ok(InsertableMedia {
                                library_id: __field0,
                                name: __field1,
                                description: __field2,
                                rating: __field3,
                                year: __field4,
                                added: __field5,
                                poster_path: __field6,
                                media_type: __field7,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::export::Option<i32> = _serde::export::None;
                            let mut __field1: _serde::export::Option<String> = _serde::export::None;
                            let mut __field2: _serde::export::Option<Option<String>> =
                                _serde::export::None;
                            let mut __field3: _serde::export::Option<Option<i32>> =
                                _serde::export::None;
                            let mut __field4: _serde::export::Option<Option<i32>> =
                                _serde::export::None;
                            let mut __field5: _serde::export::Option<String> = _serde::export::None;
                            let mut __field6: _serde::export::Option<Option<String>> =
                                _serde::export::None;
                            let mut __field7: _serde::export::Option<Option<String>> =
                                _serde::export::None;
                            while let _serde::export::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::export::Option::is_some(&__field0) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "library_id",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<i32>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::export::Option::is_some(&__field1) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "name",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::export::Option::is_some(&__field2) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "description",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::export::Option::is_some(&__field3) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "rating",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<i32>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::export::Option::is_some(&__field4) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "year",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<i32>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::export::Option::is_some(&__field5) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "added",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::export::Option::is_some(&__field6) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "poster_path",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::export::Option::is_some(&__field7) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "media_type",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::export::Some(__field0) => __field0,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("library_id") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::export::Some(__field1) => __field1,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("name") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::export::Some(__field2) => __field2,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("description") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::export::Some(__field3) => __field3,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("rating") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::export::Some(__field4) => __field4,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("year") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::export::Some(__field5) => __field5,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("added") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::export::Some(__field6) => __field6,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("poster_path") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::export::Some(__field7) => __field7,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("media_type") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::export::Ok(InsertableMedia {
                                library_id: __field0,
                                name: __field1,
                                description: __field2,
                                rating: __field3,
                                year: __field4,
                                added: __field5,
                                poster_path: __field6,
                                media_type: __field7,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "library_id",
                        "name",
                        "description",
                        "rating",
                        "year",
                        "added",
                        "poster_path",
                        "media_type",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "InsertableMedia",
                        FIELDS,
                        __Visitor {
                            marker: _serde::export::PhantomData::<InsertableMedia>,
                            lifetime: _serde::export::PhantomData,
                        },
                    )
                }
            }
        };
        #[table_name = "media"]
        pub struct UpdateMedia {
            pub name: Option<String>,
            pub description: Option<Option<String>>,
            pub rating: Option<Option<i32>>,
            pub year: Option<Option<i32>>,
            pub added: Option<Option<String>>,
            pub poster_path: Option<Option<String>>,
            pub media_type: Option<Option<String>>,
        }
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_as_changeset_for_updatemedia() {
            extern crate std;
            use diesel;
            use diesel::prelude::*;
            use diesel::query_builder::AsChangeset;
            impl<'update> AsChangeset for &'update UpdateMedia {
                type Target = media::table;
                type Changeset = <(
                    std::option::Option<diesel::dsl::Eq<media::name, &'update String>>,
                    std::option::Option<
                        diesel::dsl::Eq<media::description, &'update Option<String>>,
                    >,
                    std::option::Option<diesel::dsl::Eq<media::rating, &'update Option<i32>>>,
                    std::option::Option<diesel::dsl::Eq<media::year, &'update Option<i32>>>,
                    std::option::Option<diesel::dsl::Eq<media::added, &'update Option<String>>>,
                    std::option::Option<
                        diesel::dsl::Eq<media::poster_path, &'update Option<String>>,
                    >,
                    std::option::Option<
                        diesel::dsl::Eq<media::media_type, &'update Option<String>>,
                    >,
                ) as AsChangeset>::Changeset;
                fn as_changeset(self) -> Self::Changeset {
                    (
                        self.name.as_ref().map(|x| media::name.eq(x)),
                        self.description.as_ref().map(|x| media::description.eq(x)),
                        self.rating.as_ref().map(|x| media::rating.eq(x)),
                        self.year.as_ref().map(|x| media::year.eq(x)),
                        self.added.as_ref().map(|x| media::added.eq(x)),
                        self.poster_path.as_ref().map(|x| media::poster_path.eq(x)),
                        self.media_type.as_ref().map(|x| media::media_type.eq(x)),
                    )
                        .as_changeset()
                }
            }
            impl<'update> AsChangeset for UpdateMedia {
                type Target = media::table;
                type Changeset = <(
                    std::option::Option<diesel::dsl::Eq<media::name, String>>,
                    std::option::Option<diesel::dsl::Eq<media::description, Option<String>>>,
                    std::option::Option<diesel::dsl::Eq<media::rating, Option<i32>>>,
                    std::option::Option<diesel::dsl::Eq<media::year, Option<i32>>>,
                    std::option::Option<diesel::dsl::Eq<media::added, Option<String>>>,
                    std::option::Option<diesel::dsl::Eq<media::poster_path, Option<String>>>,
                    std::option::Option<diesel::dsl::Eq<media::media_type, Option<String>>>,
                ) as AsChangeset>::Changeset;
                fn as_changeset(self) -> Self::Changeset {
                    (
                        self.name.map(|x| media::name.eq(x)),
                        self.description.map(|x| media::description.eq(x)),
                        self.rating.map(|x| media::rating.eq(x)),
                        self.year.map(|x| media::year.eq(x)),
                        self.added.map(|x| media::added.eq(x)),
                        self.poster_path.map(|x| media::poster_path.eq(x)),
                        self.media_type.map(|x| media::media_type.eq(x)),
                    )
                        .as_changeset()
                }
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_UpdateMedia: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for UpdateMedia {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                2u64 => _serde::export::Ok(__Field::__field2),
                                3u64 => _serde::export::Ok(__Field::__field3),
                                4u64 => _serde::export::Ok(__Field::__field4),
                                5u64 => _serde::export::Ok(__Field::__field5),
                                6u64 => _serde::export::Ok(__Field::__field6),
                                _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 7",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "name" => _serde::export::Ok(__Field::__field0),
                                "description" => _serde::export::Ok(__Field::__field1),
                                "rating" => _serde::export::Ok(__Field::__field2),
                                "year" => _serde::export::Ok(__Field::__field3),
                                "added" => _serde::export::Ok(__Field::__field4),
                                "poster_path" => _serde::export::Ok(__Field::__field5),
                                "media_type" => _serde::export::Ok(__Field::__field6),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"name" => _serde::export::Ok(__Field::__field0),
                                b"description" => _serde::export::Ok(__Field::__field1),
                                b"rating" => _serde::export::Ok(__Field::__field2),
                                b"year" => _serde::export::Ok(__Field::__field3),
                                b"added" => _serde::export::Ok(__Field::__field4),
                                b"poster_path" => _serde::export::Ok(__Field::__field5),
                                b"media_type" => _serde::export::Ok(__Field::__field6),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<UpdateMedia>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = UpdateMedia;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "struct UpdateMedia")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct UpdateMedia with 7 elements",
                                    ));
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<Option<String>>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UpdateMedia with 7 elements",
                                    ));
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<Option<i32>>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct UpdateMedia with 7 elements",
                                    ));
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<Option<i32>>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct UpdateMedia with 7 elements",
                                    ));
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Option<Option<String>>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct UpdateMedia with 7 elements",
                                    ));
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<
                                Option<Option<String>>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct UpdateMedia with 7 elements",
                                    ));
                                }
                            };
                            let __field6 = match match _serde::de::SeqAccess::next_element::<
                                Option<Option<String>>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct UpdateMedia with 7 elements",
                                    ));
                                }
                            };
                            _serde::export::Ok(UpdateMedia {
                                name: __field0,
                                description: __field1,
                                rating: __field2,
                                year: __field3,
                                added: __field4,
                                poster_path: __field5,
                                media_type: __field6,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::export::Option<Option<String>> =
                                _serde::export::None;
                            let mut __field1: _serde::export::Option<Option<Option<String>>> =
                                _serde::export::None;
                            let mut __field2: _serde::export::Option<Option<Option<i32>>> =
                                _serde::export::None;
                            let mut __field3: _serde::export::Option<Option<Option<i32>>> =
                                _serde::export::None;
                            let mut __field4: _serde::export::Option<Option<Option<String>>> =
                                _serde::export::None;
                            let mut __field5: _serde::export::Option<Option<Option<String>>> =
                                _serde::export::None;
                            let mut __field6: _serde::export::Option<Option<Option<String>>> =
                                _serde::export::None;
                            while let _serde::export::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::export::Option::is_some(&__field0) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "name",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::export::Option::is_some(&__field1) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "description",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Option<String>>,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::export::Option::is_some(&__field2) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "rating",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Option<i32>>,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::export::Option::is_some(&__field3) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "year",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Option<i32>>,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::export::Option::is_some(&__field4) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "added",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Option<String>>,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::export::Option::is_some(&__field5) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "poster_path",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Option<String>>,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::export::Option::is_some(&__field6) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "media_type",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Option<String>>,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::export::Some(__field0) => __field0,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("name") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::export::Some(__field1) => __field1,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("description") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::export::Some(__field2) => __field2,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("rating") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::export::Some(__field3) => __field3,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("year") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::export::Some(__field4) => __field4,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("added") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::export::Some(__field5) => __field5,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("poster_path") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::export::Some(__field6) => __field6,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("media_type") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::export::Ok(UpdateMedia {
                                name: __field0,
                                description: __field1,
                                rating: __field2,
                                year: __field3,
                                added: __field4,
                                poster_path: __field5,
                                media_type: __field6,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "name",
                        "description",
                        "rating",
                        "year",
                        "added",
                        "poster_path",
                        "media_type",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "UpdateMedia",
                        FIELDS,
                        __Visitor {
                            marker: _serde::export::PhantomData::<UpdateMedia>,
                            lifetime: _serde::export::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for UpdateMedia {
            #[inline]
            fn eq(&self, other: &UpdateMedia) -> bool {
                match *other {
                    UpdateMedia {
                        name: ref __self_1_0,
                        description: ref __self_1_1,
                        rating: ref __self_1_2,
                        year: ref __self_1_3,
                        added: ref __self_1_4,
                        poster_path: ref __self_1_5,
                        media_type: ref __self_1_6,
                    } => match *self {
                        UpdateMedia {
                            name: ref __self_0_0,
                            description: ref __self_0_1,
                            rating: ref __self_0_2,
                            year: ref __self_0_3,
                            added: ref __self_0_4,
                            poster_path: ref __self_0_5,
                            media_type: ref __self_0_6,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                                && (*__self_0_3) == (*__self_1_3)
                                && (*__self_0_4) == (*__self_1_4)
                                && (*__self_0_5) == (*__self_1_5)
                                && (*__self_0_6) == (*__self_1_6)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &UpdateMedia) -> bool {
                match *other {
                    UpdateMedia {
                        name: ref __self_1_0,
                        description: ref __self_1_1,
                        rating: ref __self_1_2,
                        year: ref __self_1_3,
                        added: ref __self_1_4,
                        poster_path: ref __self_1_5,
                        media_type: ref __self_1_6,
                    } => match *self {
                        UpdateMedia {
                            name: ref __self_0_0,
                            description: ref __self_0_1,
                            rating: ref __self_0_2,
                            year: ref __self_0_3,
                            added: ref __self_0_4,
                            poster_path: ref __self_0_5,
                            media_type: ref __self_0_6,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                                || (*__self_0_3) != (*__self_1_3)
                                || (*__self_0_4) != (*__self_1_4)
                                || (*__self_0_5) != (*__self_1_5)
                                || (*__self_0_6) != (*__self_1_6)
                        }
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for UpdateMedia {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    UpdateMedia {
                        name: ref __self_0_0,
                        description: ref __self_0_1,
                        rating: ref __self_0_2,
                        year: ref __self_0_3,
                        added: ref __self_0_4,
                        poster_path: ref __self_0_5,
                        media_type: ref __self_0_6,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("UpdateMedia");
                        let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                        let _ = debug_trait_builder.field("description", &&(*__self_0_1));
                        let _ = debug_trait_builder.field("rating", &&(*__self_0_2));
                        let _ = debug_trait_builder.field("year", &&(*__self_0_3));
                        let _ = debug_trait_builder.field("added", &&(*__self_0_4));
                        let _ = debug_trait_builder.field("poster_path", &&(*__self_0_5));
                        let _ = debug_trait_builder.field("media_type", &&(*__self_0_6));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl Media {
            pub fn get_all(
                conn: &diesel::SqliteConnection,
                lib_id: i32,
                library: Library,
            ) -> Result<Json<Vec<Media>>, diesel::result::Error> {
                use crate::schema::media;
                use crate::schema::media::dsl::*;
                let result = Media::belonging_to(&library)
                    .load::<Media>(conn)
                    .map(|x| Json(x))?;
                Ok(result)
            }
            pub fn get(
                conn: &diesel::SqliteConnection,
                req_id: i32,
            ) -> Result<Json<Media>, diesel::result::Error> {
                use crate::schema::media::dsl::*;
                let result = media.filter(id.eq(req_id)).first::<Media>(conn)?;
                Ok(Json(result))
            }
            pub fn new(
                conn: &diesel::SqliteConnection,
                data: Json<InsertableMedia>,
            ) -> Result<usize, diesel::result::Error> {
                use crate::schema::library::dsl::*;
                use crate::schema::media;
                library
                    .filter(id.eq(data.library_id))
                    .first::<Library>(conn)?;
                let result = diesel::insert_into(media::table)
                    .values(&*data)
                    .execute(conn)?;
                Ok(result)
            }
            pub fn delete(
                conn: &diesel::SqliteConnection,
                id_to_del: i32,
            ) -> Result<usize, diesel::result::Error> {
                use crate::schema::media::dsl::*;
                let result = diesel::delete(media.filter(id.eq(id_to_del))).execute(conn)?;
                Ok(result)
            }
            pub fn update(
                conn: &diesel::SqliteConnection,
                id: i32,
                data: Json<UpdateMedia>,
            ) -> Result<usize, diesel::result::Error> {
                use crate::schema::media::dsl::*;
                use diesel::update;
                let entry = media.filter(id.eq(id));
                diesel::update(entry).set(&*data).execute(conn)
            }
        }
    }
}
pub mod routes {
    pub mod library {
        use crate::core::DbConnection;
        use crate::database::library::{InsertableLibrary, Library};
        use crate::database::media::Media;
        use rocket::http::Status;
        use rocket_contrib::json::Json;
        pub fn library_get(conn: DbConnection) -> Json<Vec<Library>> {
            Library::get_all(&conn)
        }
        /// Rocket code generated wrapping route function.
        pub fn rocket_route_fn_library_get<'_b>(
            __req: &'_b ::rocket::Request,
            __data: ::rocket::Data,
        ) -> ::rocket::handler::Outcome<'_b> {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_conn: DbConnection =
                match <DbConnection as ::rocket::request::FromRequest>::from_request(__req) {
                    ::rocket::Outcome::Success(__v) => __v,
                    ::rocket::Outcome::Forward(_) => return ::rocket::Outcome::Forward(__data),
                    ::rocket::Outcome::Failure((__c, _)) => return ::rocket::Outcome::Failure(__c),
                };
            let ___responder = library_get(__rocket_param_conn);
            ::rocket::handler::Outcome::from(__req, ___responder)
        }
        /// Rocket code generated static route info.
        #[allow(non_upper_case_globals)]
        pub static static_rocket_route_info_for_library_get: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                name: "library_get",
                method: ::rocket::http::Method::Get,
                path: "/",
                handler: rocket_route_fn_library_get,
                format: None,
                rank: None,
            };
        pub fn library_post(
            conn: DbConnection,
            new_library: Json<InsertableLibrary>,
        ) -> Result<Status, Status> {
            match Library::new(&conn, new_library) {
                Ok(_) => Ok(Status::Ok),
                Err(_) => Err(Status::NotImplemented),
            }
        }
        /// Rocket code generated wrapping route function.
        pub fn rocket_route_fn_library_post<'_b>(
            __req: &'_b ::rocket::Request,
            __data: ::rocket::Data,
        ) -> ::rocket::handler::Outcome<'_b> {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_conn: DbConnection =
                match <DbConnection as ::rocket::request::FromRequest>::from_request(__req) {
                    ::rocket::Outcome::Success(__v) => __v,
                    ::rocket::Outcome::Forward(_) => return ::rocket::Outcome::Forward(__data),
                    ::rocket::Outcome::Failure((__c, _)) => return ::rocket::Outcome::Failure(__c),
                };
            let __transform =
                <Json<InsertableLibrary> as ::rocket::data::FromData>::transform(__req, __data);
            # [ allow ( unreachable_patterns , unreachable_code ) ] let __outcome = match __transform { :: rocket :: data :: Transform :: Owned ( :: rocket :: Outcome :: Success ( __v ) ) => { :: rocket :: data :: Transform :: Owned ( :: rocket :: Outcome :: Success ( __v ) ) } :: rocket :: data :: Transform :: Borrowed ( :: rocket :: Outcome :: Success ( ref __v ) ) => { :: rocket :: data :: Transform :: Borrowed ( :: rocket :: Outcome :: Success ( :: std :: borrow :: Borrow :: borrow ( __v ) ) ) } :: rocket :: data :: Transform :: Borrowed ( __o ) => :: rocket :: data :: Transform :: Borrowed ( __o . map ( | _ | { { { { :: std :: rt :: begin_panic_fmt ( & :: std :: fmt :: Arguments :: new_v1 ( & [ "internal error: entered unreachable code: " ] , & match ( & "Borrowed(Success(..)) case handled in previous block" , ) { ( arg0 , ) => [ :: std :: fmt :: ArgumentV1 :: new ( arg0 , :: std :: fmt :: Display :: fmt ) ] , } ) , & ( "src/routes/library.rs" , 15u32 , 5u32 ) ) } } } } ) ) , :: rocket :: data :: Transform :: Owned ( __o ) => :: rocket :: data :: Transform :: Owned ( __o ) , } ;
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_new_library: Json<InsertableLibrary> =
                match <Json<InsertableLibrary> as ::rocket::data::FromData>::from_data(
                    __req, __outcome,
                ) {
                    ::rocket::Outcome::Success(__d) => __d,
                    ::rocket::Outcome::Forward(__d) => return ::rocket::Outcome::Forward(__d),
                    ::rocket::Outcome::Failure((__c, _)) => return ::rocket::Outcome::Failure(__c),
                };
            let ___responder = library_post(__rocket_param_conn, __rocket_param_new_library);
            ::rocket::handler::Outcome::from(__req, ___responder)
        }
        /// Rocket code generated static route info.
        #[allow(non_upper_case_globals)]
        pub static static_rocket_route_info_for_library_post: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                name: "library_post",
                method: ::rocket::http::Method::Post,
                path: "/",
                handler: rocket_route_fn_library_post,
                format: Some(::rocket::http::MediaType {
                    source: ::rocket::http::private::Source::None,
                    top: ::rocket::http::private::Indexed::Concrete(::std::borrow::Cow::Borrowed(
                        "application",
                    )),
                    sub: ::rocket::http::private::Indexed::Concrete(::std::borrow::Cow::Borrowed(
                        "json",
                    )),
                    params: ::rocket::http::private::MediaParams::Static(&[]),
                }),
                rank: None,
            };
        pub fn library_delete(conn: DbConnection, id: i32) -> Result<Status, Status> {
            match Library::delete(&conn, id) {
                Ok(_) => Ok(Status::Ok),
                Err(_) => Err(Status::NotFound),
            }
        }
        /// Rocket code generated wrapping route function.
        pub fn rocket_route_fn_library_delete<'_b>(
            __req: &'_b ::rocket::Request,
            __data: ::rocket::Data,
        ) -> ::rocket::handler::Outcome<'_b> {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_conn: DbConnection =
                match <DbConnection as ::rocket::request::FromRequest>::from_request(__req) {
                    ::rocket::Outcome::Success(__v) => __v,
                    ::rocket::Outcome::Forward(_) => return ::rocket::Outcome::Forward(__data),
                    ::rocket::Outcome::Failure((__c, _)) => return ::rocket::Outcome::Failure(__c),
                };
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_id: i32 = match __req.raw_segment_str(0usize) {
                Some(__s) => match <i32 as ::rocket::request::FromParam>::from_param(__s) {
                    Ok(__v) => __v,
                    Err(__error) => {
                        return {
                            ::rocket::logger::warn_(&::alloc::fmt::format(
                                ::std::fmt::Arguments::new_v1(
                                    &["Failed to parse \'", "\': "],
                                    &match (&"__rocket_param_id", &__error) {
                                        (arg0, arg1) => [
                                            ::std::fmt::ArgumentV1::new(
                                                arg0,
                                                ::std::fmt::Display::fmt,
                                            ),
                                            ::std::fmt::ArgumentV1::new(
                                                arg1,
                                                ::std::fmt::Debug::fmt,
                                            ),
                                        ],
                                    },
                                ),
                            ));
                            ::rocket::Outcome::Forward(__data)
                        }
                    }
                },
                None => {
                    return {
                        ::rocket::logger::error(
                            "Internal invariant error: expected dynamic parameter not found.",
                        );
                        ::rocket::logger::error(
                            "Please report this error to the Rocket issue tracker.",
                        );
                        ::rocket::Outcome::Forward(__data)
                    }
                }
            };
            let ___responder = library_delete(__rocket_param_conn, __rocket_param_id);
            ::rocket::handler::Outcome::from(__req, ___responder)
        }
        /// Rocket code generated static route info.
        #[allow(non_upper_case_globals)]
        pub static static_rocket_route_info_for_library_delete: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                name: "library_delete",
                method: ::rocket::http::Method::Delete,
                path: "/<id>",
                handler: rocket_route_fn_library_delete,
                format: None,
                rank: None,
            };
        pub fn get_all_library(conn: DbConnection, id: i32) -> Result<Json<Vec<Media>>, Status> {
            match Library::get(&conn, id) {
                Ok(data) => Ok(data),
                Err(_) => Err(Status::NotFound),
            }
        }
        /// Rocket code generated wrapping route function.
        pub fn rocket_route_fn_get_all_library<'_b>(
            __req: &'_b ::rocket::Request,
            __data: ::rocket::Data,
        ) -> ::rocket::handler::Outcome<'_b> {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_conn: DbConnection =
                match <DbConnection as ::rocket::request::FromRequest>::from_request(__req) {
                    ::rocket::Outcome::Success(__v) => __v,
                    ::rocket::Outcome::Forward(_) => return ::rocket::Outcome::Forward(__data),
                    ::rocket::Outcome::Failure((__c, _)) => return ::rocket::Outcome::Failure(__c),
                };
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_id: i32 = match __req.raw_segment_str(0usize) {
                Some(__s) => match <i32 as ::rocket::request::FromParam>::from_param(__s) {
                    Ok(__v) => __v,
                    Err(__error) => {
                        return {
                            ::rocket::logger::warn_(&::alloc::fmt::format(
                                ::std::fmt::Arguments::new_v1(
                                    &["Failed to parse \'", "\': "],
                                    &match (&"__rocket_param_id", &__error) {
                                        (arg0, arg1) => [
                                            ::std::fmt::ArgumentV1::new(
                                                arg0,
                                                ::std::fmt::Display::fmt,
                                            ),
                                            ::std::fmt::ArgumentV1::new(
                                                arg1,
                                                ::std::fmt::Debug::fmt,
                                            ),
                                        ],
                                    },
                                ),
                            ));
                            ::rocket::Outcome::Forward(__data)
                        }
                    }
                },
                None => {
                    return {
                        ::rocket::logger::error(
                            "Internal invariant error: expected dynamic parameter not found.",
                        );
                        ::rocket::logger::error(
                            "Please report this error to the Rocket issue tracker.",
                        );
                        ::rocket::Outcome::Forward(__data)
                    }
                }
            };
            let ___responder = get_all_library(__rocket_param_conn, __rocket_param_id);
            ::rocket::handler::Outcome::from(__req, ___responder)
        }
        /// Rocket code generated static route info.
        #[allow(non_upper_case_globals)]
        pub static static_rocket_route_info_for_get_all_library: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                name: "get_all_library",
                method: ::rocket::http::Method::Get,
                path: "/<id>",
                handler: rocket_route_fn_get_all_library,
                format: None,
                rank: None,
            };
    }
    pub mod media {
        use crate::core::DbConnection;
        use crate::database::library::{InsertableLibrary, Library};
        use crate::database::media::{InsertableMedia, Media, UpdateMedia};
        use rocket::http::Status;
        use rocket_contrib::json::Json;
        pub fn get_media_by_id(conn: DbConnection, id: i32) -> Result<Json<Media>, Status> {
            match Media::get(&conn, id) {
                Ok(data) => Ok(data),
                Err(_) => Err(Status::NotFound),
            }
        }
        /// Rocket code generated wrapping route function.
        pub fn rocket_route_fn_get_media_by_id<'_b>(
            __req: &'_b ::rocket::Request,
            __data: ::rocket::Data,
        ) -> ::rocket::handler::Outcome<'_b> {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_conn: DbConnection =
                match <DbConnection as ::rocket::request::FromRequest>::from_request(__req) {
                    ::rocket::Outcome::Success(__v) => __v,
                    ::rocket::Outcome::Forward(_) => return ::rocket::Outcome::Forward(__data),
                    ::rocket::Outcome::Failure((__c, _)) => return ::rocket::Outcome::Failure(__c),
                };
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_id: i32 = match __req.raw_segment_str(0usize) {
                Some(__s) => match <i32 as ::rocket::request::FromParam>::from_param(__s) {
                    Ok(__v) => __v,
                    Err(__error) => {
                        return {
                            ::rocket::logger::warn_(&::alloc::fmt::format(
                                ::std::fmt::Arguments::new_v1(
                                    &["Failed to parse \'", "\': "],
                                    &match (&"__rocket_param_id", &__error) {
                                        (arg0, arg1) => [
                                            ::std::fmt::ArgumentV1::new(
                                                arg0,
                                                ::std::fmt::Display::fmt,
                                            ),
                                            ::std::fmt::ArgumentV1::new(
                                                arg1,
                                                ::std::fmt::Debug::fmt,
                                            ),
                                        ],
                                    },
                                ),
                            ));
                            ::rocket::Outcome::Forward(__data)
                        }
                    }
                },
                None => {
                    return {
                        ::rocket::logger::error(
                            "Internal invariant error: expected dynamic parameter not found.",
                        );
                        ::rocket::logger::error(
                            "Please report this error to the Rocket issue tracker.",
                        );
                        ::rocket::Outcome::Forward(__data)
                    }
                }
            };
            let ___responder = get_media_by_id(__rocket_param_conn, __rocket_param_id);
            ::rocket::handler::Outcome::from(__req, ___responder)
        }
        /// Rocket code generated static route info.
        #[allow(non_upper_case_globals)]
        pub static static_rocket_route_info_for_get_media_by_id: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                name: "get_media_by_id",
                method: ::rocket::http::Method::Get,
                path: "/<id>",
                handler: rocket_route_fn_get_media_by_id,
                format: None,
                rank: None,
            };
        pub fn insert_media_by_lib_id(
            conn: DbConnection,
            data: Json<InsertableMedia>,
        ) -> Result<Status, Status> {
            match Media::new(&conn, data) {
                Ok(_) => Ok(Status::Ok),
                Err(_) => Err(Status::NotFound),
            }
        }
        /// Rocket code generated wrapping route function.
        pub fn rocket_route_fn_insert_media_by_lib_id<'_b>(
            __req: &'_b ::rocket::Request,
            __data: ::rocket::Data,
        ) -> ::rocket::handler::Outcome<'_b> {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_conn: DbConnection =
                match <DbConnection as ::rocket::request::FromRequest>::from_request(__req) {
                    ::rocket::Outcome::Success(__v) => __v,
                    ::rocket::Outcome::Forward(_) => return ::rocket::Outcome::Forward(__data),
                    ::rocket::Outcome::Failure((__c, _)) => return ::rocket::Outcome::Failure(__c),
                };
            let __transform =
                <Json<InsertableMedia> as ::rocket::data::FromData>::transform(__req, __data);
            # [ allow ( unreachable_patterns , unreachable_code ) ] let __outcome = match __transform { :: rocket :: data :: Transform :: Owned ( :: rocket :: Outcome :: Success ( __v ) ) => { :: rocket :: data :: Transform :: Owned ( :: rocket :: Outcome :: Success ( __v ) ) } :: rocket :: data :: Transform :: Borrowed ( :: rocket :: Outcome :: Success ( ref __v ) ) => { :: rocket :: data :: Transform :: Borrowed ( :: rocket :: Outcome :: Success ( :: std :: borrow :: Borrow :: borrow ( __v ) ) ) } :: rocket :: data :: Transform :: Borrowed ( __o ) => :: rocket :: data :: Transform :: Borrowed ( __o . map ( | _ | { { { { :: std :: rt :: begin_panic_fmt ( & :: std :: fmt :: Arguments :: new_v1 ( & [ "internal error: entered unreachable code: " ] , & match ( & "Borrowed(Success(..)) case handled in previous block" , ) { ( arg0 , ) => [ :: std :: fmt :: ArgumentV1 :: new ( arg0 , :: std :: fmt :: Display :: fmt ) ] , } ) , & ( "src/routes/media.rs" , 18u32 , 5u32 ) ) } } } } ) ) , :: rocket :: data :: Transform :: Owned ( __o ) => :: rocket :: data :: Transform :: Owned ( __o ) , } ;
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_data: Json<InsertableMedia> =
                match <Json<InsertableMedia> as ::rocket::data::FromData>::from_data(
                    __req, __outcome,
                ) {
                    ::rocket::Outcome::Success(__d) => __d,
                    ::rocket::Outcome::Forward(__d) => return ::rocket::Outcome::Forward(__d),
                    ::rocket::Outcome::Failure((__c, _)) => return ::rocket::Outcome::Failure(__c),
                };
            let ___responder = insert_media_by_lib_id(__rocket_param_conn, __rocket_param_data);
            ::rocket::handler::Outcome::from(__req, ___responder)
        }
        /// Rocket code generated static route info.
        #[allow(non_upper_case_globals)]
        pub static static_rocket_route_info_for_insert_media_by_lib_id: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                name: "insert_media_by_lib_id",
                method: ::rocket::http::Method::Post,
                path: "/",
                handler: rocket_route_fn_insert_media_by_lib_id,
                format: Some(::rocket::http::MediaType {
                    source: ::rocket::http::private::Source::None,
                    top: ::rocket::http::private::Indexed::Concrete(::std::borrow::Cow::Borrowed(
                        "application",
                    )),
                    sub: ::rocket::http::private::Indexed::Concrete(::std::borrow::Cow::Borrowed(
                        "json",
                    )),
                    params: ::rocket::http::private::MediaParams::Static(&[]),
                }),
                rank: None,
            };
        pub fn update_media_by_id(
            conn: DbConnection,
            id: i32,
            data: Json<UpdateMedia>,
        ) -> Result<Status, Status> {
            match Media::update(&conn, id, data) {
                Ok(_) => Ok(Status::Ok),
                Err(_) => Err(Status::NotFound),
            }
        }
        /// Rocket code generated wrapping route function.
        pub fn rocket_route_fn_update_media_by_id<'_b>(
            __req: &'_b ::rocket::Request,
            __data: ::rocket::Data,
        ) -> ::rocket::handler::Outcome<'_b> {
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_conn: DbConnection =
                match <DbConnection as ::rocket::request::FromRequest>::from_request(__req) {
                    ::rocket::Outcome::Success(__v) => __v,
                    ::rocket::Outcome::Forward(_) => return ::rocket::Outcome::Forward(__data),
                    ::rocket::Outcome::Failure((__c, _)) => return ::rocket::Outcome::Failure(__c),
                };
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_id: i32 = match __req.raw_segment_str(0usize) {
                Some(__s) => match <i32 as ::rocket::request::FromParam>::from_param(__s) {
                    Ok(__v) => __v,
                    Err(__error) => {
                        return {
                            ::rocket::logger::warn_(&::alloc::fmt::format(
                                ::std::fmt::Arguments::new_v1(
                                    &["Failed to parse \'", "\': "],
                                    &match (&"__rocket_param_id", &__error) {
                                        (arg0, arg1) => [
                                            ::std::fmt::ArgumentV1::new(
                                                arg0,
                                                ::std::fmt::Display::fmt,
                                            ),
                                            ::std::fmt::ArgumentV1::new(
                                                arg1,
                                                ::std::fmt::Debug::fmt,
                                            ),
                                        ],
                                    },
                                ),
                            ));
                            ::rocket::Outcome::Forward(__data)
                        }
                    }
                },
                None => {
                    return {
                        ::rocket::logger::error(
                            "Internal invariant error: expected dynamic parameter not found.",
                        );
                        ::rocket::logger::error(
                            "Please report this error to the Rocket issue tracker.",
                        );
                        ::rocket::Outcome::Forward(__data)
                    }
                }
            };
            let __transform =
                <Json<UpdateMedia> as ::rocket::data::FromData>::transform(__req, __data);
            # [ allow ( unreachable_patterns , unreachable_code ) ] let __outcome = match __transform { :: rocket :: data :: Transform :: Owned ( :: rocket :: Outcome :: Success ( __v ) ) => { :: rocket :: data :: Transform :: Owned ( :: rocket :: Outcome :: Success ( __v ) ) } :: rocket :: data :: Transform :: Borrowed ( :: rocket :: Outcome :: Success ( ref __v ) ) => { :: rocket :: data :: Transform :: Borrowed ( :: rocket :: Outcome :: Success ( :: std :: borrow :: Borrow :: borrow ( __v ) ) ) } :: rocket :: data :: Transform :: Borrowed ( __o ) => :: rocket :: data :: Transform :: Borrowed ( __o . map ( | _ | { { { { :: std :: rt :: begin_panic_fmt ( & :: std :: fmt :: Arguments :: new_v1 ( & [ "internal error: entered unreachable code: " ] , & match ( & "Borrowed(Success(..)) case handled in previous block" , ) { ( arg0 , ) => [ :: std :: fmt :: ArgumentV1 :: new ( arg0 , :: std :: fmt :: Display :: fmt ) ] , } ) , & ( "src/routes/media.rs" , 30u32 , 5u32 ) ) } } } } ) ) , :: rocket :: data :: Transform :: Owned ( __o ) => :: rocket :: data :: Transform :: Owned ( __o ) , } ;
            #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
            let __rocket_param_data: Json<UpdateMedia> =
                match <Json<UpdateMedia> as ::rocket::data::FromData>::from_data(__req, __outcome) {
                    ::rocket::Outcome::Success(__d) => __d,
                    ::rocket::Outcome::Forward(__d) => return ::rocket::Outcome::Forward(__d),
                    ::rocket::Outcome::Failure((__c, _)) => return ::rocket::Outcome::Failure(__c),
                };
            let ___responder =
                update_media_by_id(__rocket_param_conn, __rocket_param_id, __rocket_param_data);
            ::rocket::handler::Outcome::from(__req, ___responder)
        }
        /// Rocket code generated static route info.
        #[allow(non_upper_case_globals)]
        pub static static_rocket_route_info_for_update_media_by_id: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                name: "update_media_by_id",
                method: ::rocket::http::Method::Patch,
                path: "/<id>",
                handler: rocket_route_fn_update_media_by_id,
                format: Some(::rocket::http::MediaType {
                    source: ::rocket::http::private::Source::None,
                    top: ::rocket::http::private::Indexed::Concrete(::std::borrow::Cow::Borrowed(
                        "application",
                    )),
                    sub: ::rocket::http::private::Indexed::Concrete(::std::borrow::Cow::Borrowed(
                        "json",
                    )),
                    params: ::rocket::http::private::MediaParams::Static(&[]),
                }),
                rank: None,
            };
    }
}
pub mod schema {
    pub mod episode {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::episode_;
            pub use super::columns::id;
            pub use super::columns::seasonid;
            pub use super::table as episode;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, seasonid, episode_) = (id, seasonid, episode_);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Integer, Integer, Integer);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("episode")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (id, seasonid, episode_);
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (id, seasonid, episode_)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct seasonid;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for seasonid {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        seasonid => {
                            let mut debug_trait_builder = f.debug_tuple("seasonid");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for seasonid {
                #[inline]
                fn clone(&self) -> seasonid {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for seasonid {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_seasonid() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for seasonid {
                    type QueryId = seasonid;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for seasonid {
                #[inline]
                fn default() -> seasonid {
                    seasonid
                }
            }
            impl ::diesel::expression::Expression for seasonid {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for seasonid
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("seasonid")
                }
            }
            impl SelectableExpression<table> for seasonid {}
            impl<QS> AppearsOnTable<QS> for seasonid where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for seasonid
            where
                seasonid: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for seasonid
            where
                seasonid: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for seasonid where
                seasonid: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for seasonid where
                seasonid: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for seasonid {}
            impl ::diesel::query_source::Column for seasonid {
                type Table = table;
                const NAME: &'static str = "seasonid";
            }
            impl<T> ::diesel::EqAll<T> for seasonid
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<seasonid, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for seasonid where Rhs : :: diesel :: expression :: AsExpression < < < seasonid as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for seasonid where Rhs : :: diesel :: expression :: AsExpression < < < seasonid as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for seasonid where Rhs : :: diesel :: expression :: AsExpression < < < seasonid as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for seasonid where Rhs : :: diesel :: expression :: AsExpression < < < seasonid as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[column_name = "episode"]
            #[rustc_copy_clone_marker]
            pub struct episode_;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for episode_ {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        episode_ => {
                            let mut debug_trait_builder = f.debug_tuple("episode_");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for episode_ {
                #[inline]
                fn clone(&self) -> episode_ {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for episode_ {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_episode_() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for episode_ {
                    type QueryId = episode_;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for episode_ {
                #[inline]
                fn default() -> episode_ {
                    episode_
                }
            }
            impl ::diesel::expression::Expression for episode_ {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for episode_
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("episode_")
                }
            }
            impl SelectableExpression<table> for episode_ {}
            impl<QS> AppearsOnTable<QS> for episode_ where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for episode_
            where
                episode_: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for episode_
            where
                episode_: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for episode_ where
                episode_: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for episode_ where
                episode_: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for episode_ {}
            impl ::diesel::query_source::Column for episode_ {
                type Table = table;
                const NAME: &'static str = "episode_";
            }
            impl<T> ::diesel::EqAll<T> for episode_
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<episode_, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for episode_ where Rhs : :: diesel :: expression :: AsExpression < < < episode_ as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for episode_ where Rhs : :: diesel :: expression :: AsExpression < < < episode_ as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for episode_ where Rhs : :: diesel :: expression :: AsExpression < < < episode_ as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for episode_ where Rhs : :: diesel :: expression :: AsExpression < < < episode_ as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
        }
    }
    pub mod library {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::id;
            pub use super::columns::location;
            pub use super::columns::media_type;
            pub use super::columns::name;
            pub use super::table as library;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, name, location, media_type) = (id, name, location, media_type);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Integer, Text, Text, Text);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("library")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (id, name, location, media_type);
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (id, name, location, media_type)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct name;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        name => {
                            let mut debug_trait_builder = f.debug_tuple("name");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for name {
                #[inline]
                fn clone(&self) -> name {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for name {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_name() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for name {
                    type QueryId = name;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for name {
                #[inline]
                fn default() -> name {
                    name
                }
            }
            impl ::diesel::expression::Expression for name {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for name
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("name")
                }
            }
            impl SelectableExpression<table> for name {}
            impl<QS> AppearsOnTable<QS> for name where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for name
            where
                name: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for name
            where
                name: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for name where
                name: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for name where
                name: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for name {}
            impl ::diesel::query_source::Column for name {
                type Table = table;
                const NAME: &'static str = "name";
            }
            impl<T> ::diesel::EqAll<T> for name
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<name, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct location;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for location {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        location => {
                            let mut debug_trait_builder = f.debug_tuple("location");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for location {
                #[inline]
                fn clone(&self) -> location {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for location {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_location() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for location {
                    type QueryId = location;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for location {
                #[inline]
                fn default() -> location {
                    location
                }
            }
            impl ::diesel::expression::Expression for location {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for location
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("location")
                }
            }
            impl SelectableExpression<table> for location {}
            impl<QS> AppearsOnTable<QS> for location where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for location
            where
                location: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for location
            where
                location: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for location where
                location: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for location where
                location: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for location {}
            impl ::diesel::query_source::Column for location {
                type Table = table;
                const NAME: &'static str = "location";
            }
            impl<T> ::diesel::EqAll<T> for location
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<location, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct media_type;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for media_type {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        media_type => {
                            let mut debug_trait_builder = f.debug_tuple("media_type");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for media_type {
                #[inline]
                fn clone(&self) -> media_type {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for media_type {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_media_type() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for media_type {
                    type QueryId = media_type;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for media_type {
                #[inline]
                fn default() -> media_type {
                    media_type
                }
            }
            impl ::diesel::expression::Expression for media_type {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for media_type
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("media_type")
                }
            }
            impl SelectableExpression<table> for media_type {}
            impl<QS> AppearsOnTable<QS> for media_type where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for media_type
            where
                media_type: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for media_type
            where
                media_type: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for media_type where
                media_type: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for media_type where
                media_type: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for media_type {}
            impl ::diesel::query_source::Column for media_type {
                type Table = table;
                const NAME: &'static str = "media_type";
            }
            impl<T> ::diesel::EqAll<T> for media_type
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<media_type, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod media {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::added;
            pub use super::columns::description;
            pub use super::columns::id;
            pub use super::columns::library_id;
            pub use super::columns::media_type;
            pub use super::columns::name;
            pub use super::columns::poster_path;
            pub use super::columns::rating;
            pub use super::columns::year;
            pub use super::table as media;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (
            id,
            library_id,
            name,
            description,
            rating,
            year,
            added,
            poster_path,
            media_type,
        ) = (
            id,
            library_id,
            name,
            description,
            rating,
            year,
            added,
            poster_path,
            media_type,
        );
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (
            Integer,
            Integer,
            Text,
            Nullable<Text>,
            Nullable<Integer>,
            Nullable<Integer>,
            Nullable<Text>,
            Nullable<Text>,
            Nullable<Text>,
        );
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("media")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (
                id,
                library_id,
                name,
                description,
                rating,
                year,
                added,
                poster_path,
                media_type,
            );
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (
                    id,
                    library_id,
                    name,
                    description,
                    rating,
                    year,
                    added,
                    poster_path,
                    media_type,
                )
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct library_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for library_id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        library_id => {
                            let mut debug_trait_builder = f.debug_tuple("library_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for library_id {
                #[inline]
                fn clone(&self) -> library_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for library_id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_library_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for library_id {
                    type QueryId = library_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for library_id {
                #[inline]
                fn default() -> library_id {
                    library_id
                }
            }
            impl ::diesel::expression::Expression for library_id {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for library_id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("library_id")
                }
            }
            impl SelectableExpression<table> for library_id {}
            impl<QS> AppearsOnTable<QS> for library_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for library_id
            where
                library_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for library_id
            where
                library_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for library_id where
                library_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for library_id where
                library_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for library_id {}
            impl ::diesel::query_source::Column for library_id {
                type Table = table;
                const NAME: &'static str = "library_id";
            }
            impl<T> ::diesel::EqAll<T> for library_id
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<library_id, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for library_id where Rhs : :: diesel :: expression :: AsExpression < < < library_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for library_id where Rhs : :: diesel :: expression :: AsExpression < < < library_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for library_id where Rhs : :: diesel :: expression :: AsExpression < < < library_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for library_id where Rhs : :: diesel :: expression :: AsExpression < < < library_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct name;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        name => {
                            let mut debug_trait_builder = f.debug_tuple("name");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for name {
                #[inline]
                fn clone(&self) -> name {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for name {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_name() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for name {
                    type QueryId = name;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for name {
                #[inline]
                fn default() -> name {
                    name
                }
            }
            impl ::diesel::expression::Expression for name {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for name
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("name")
                }
            }
            impl SelectableExpression<table> for name {}
            impl<QS> AppearsOnTable<QS> for name where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for name
            where
                name: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for name
            where
                name: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for name where
                name: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for name where
                name: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for name {}
            impl ::diesel::query_source::Column for name {
                type Table = table;
                const NAME: &'static str = "name";
            }
            impl<T> ::diesel::EqAll<T> for name
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<name, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct description;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for description {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        description => {
                            let mut debug_trait_builder = f.debug_tuple("description");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for description {
                #[inline]
                fn clone(&self) -> description {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for description {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_description() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for description {
                    type QueryId = description;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for description {
                #[inline]
                fn default() -> description {
                    description
                }
            }
            impl ::diesel::expression::Expression for description {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for description
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("description")
                }
            }
            impl SelectableExpression<table> for description {}
            impl<QS> AppearsOnTable<QS> for description where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for description
            where
                description: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for description
            where
                description: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for description where
                description: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for description where
                description: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for description {}
            impl ::diesel::query_source::Column for description {
                type Table = table;
                const NAME: &'static str = "description";
            }
            impl<T> ::diesel::EqAll<T> for description
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<description, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct rating;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for rating {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        rating => {
                            let mut debug_trait_builder = f.debug_tuple("rating");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for rating {
                #[inline]
                fn clone(&self) -> rating {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for rating {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_rating() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for rating {
                    type QueryId = rating;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for rating {
                #[inline]
                fn default() -> rating {
                    rating
                }
            }
            impl ::diesel::expression::Expression for rating {
                type SqlType = Nullable<Integer>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for rating
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("rating")
                }
            }
            impl SelectableExpression<table> for rating {}
            impl<QS> AppearsOnTable<QS> for rating where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for rating
            where
                rating: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for rating
            where
                rating: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for rating where
                rating: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for rating where
                rating: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for rating {}
            impl ::diesel::query_source::Column for rating {
                type Table = table;
                const NAME: &'static str = "rating";
            }
            impl<T> ::diesel::EqAll<T> for rating
            where
                T: ::diesel::expression::AsExpression<Nullable<Integer>>,
                ::diesel::dsl::Eq<rating, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for rating where Rhs : :: diesel :: expression :: AsExpression < < < rating as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for rating where Rhs : :: diesel :: expression :: AsExpression < < < rating as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for rating where Rhs : :: diesel :: expression :: AsExpression < < < rating as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for rating where Rhs : :: diesel :: expression :: AsExpression < < < rating as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct year;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for year {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        year => {
                            let mut debug_trait_builder = f.debug_tuple("year");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for year {
                #[inline]
                fn clone(&self) -> year {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for year {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_year() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for year {
                    type QueryId = year;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for year {
                #[inline]
                fn default() -> year {
                    year
                }
            }
            impl ::diesel::expression::Expression for year {
                type SqlType = Nullable<Integer>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for year
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("year")
                }
            }
            impl SelectableExpression<table> for year {}
            impl<QS> AppearsOnTable<QS> for year where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for year
            where
                year: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for year
            where
                year: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for year where
                year: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for year where
                year: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for year {}
            impl ::diesel::query_source::Column for year {
                type Table = table;
                const NAME: &'static str = "year";
            }
            impl<T> ::diesel::EqAll<T> for year
            where
                T: ::diesel::expression::AsExpression<Nullable<Integer>>,
                ::diesel::dsl::Eq<year, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for year
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<year as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for year
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<year as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for year
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<year as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for year
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<year as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct added;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for added {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        added => {
                            let mut debug_trait_builder = f.debug_tuple("added");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for added {
                #[inline]
                fn clone(&self) -> added {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for added {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_added() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for added {
                    type QueryId = added;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for added {
                #[inline]
                fn default() -> added {
                    added
                }
            }
            impl ::diesel::expression::Expression for added {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for added
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("added")
                }
            }
            impl SelectableExpression<table> for added {}
            impl<QS> AppearsOnTable<QS> for added where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for added
            where
                added: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for added
            where
                added: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for added where
                added: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for added where
                added: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for added {}
            impl ::diesel::query_source::Column for added {
                type Table = table;
                const NAME: &'static str = "added";
            }
            impl<T> ::diesel::EqAll<T> for added
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<added, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct poster_path;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for poster_path {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        poster_path => {
                            let mut debug_trait_builder = f.debug_tuple("poster_path");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for poster_path {
                #[inline]
                fn clone(&self) -> poster_path {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for poster_path {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_poster_path() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for poster_path {
                    type QueryId = poster_path;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for poster_path {
                #[inline]
                fn default() -> poster_path {
                    poster_path
                }
            }
            impl ::diesel::expression::Expression for poster_path {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for poster_path
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("poster_path")
                }
            }
            impl SelectableExpression<table> for poster_path {}
            impl<QS> AppearsOnTable<QS> for poster_path where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for poster_path
            where
                poster_path: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for poster_path
            where
                poster_path: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for poster_path where
                poster_path: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for poster_path where
                poster_path: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for poster_path {}
            impl ::diesel::query_source::Column for poster_path {
                type Table = table;
                const NAME: &'static str = "poster_path";
            }
            impl<T> ::diesel::EqAll<T> for poster_path
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<poster_path, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct media_type;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for media_type {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        media_type => {
                            let mut debug_trait_builder = f.debug_tuple("media_type");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for media_type {
                #[inline]
                fn clone(&self) -> media_type {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for media_type {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_media_type() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for media_type {
                    type QueryId = media_type;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for media_type {
                #[inline]
                fn default() -> media_type {
                    media_type
                }
            }
            impl ::diesel::expression::Expression for media_type {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for media_type
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("media_type")
                }
            }
            impl SelectableExpression<table> for media_type {}
            impl<QS> AppearsOnTable<QS> for media_type where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for media_type
            where
                media_type: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for media_type
            where
                media_type: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for media_type where
                media_type: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for media_type where
                media_type: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for media_type {}
            impl ::diesel::query_source::Column for media_type {
                type Table = table;
                const NAME: &'static str = "media_type";
            }
            impl<T> ::diesel::EqAll<T> for media_type
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<media_type, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod mediafile {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::audio;
            pub use super::columns::codec;
            pub use super::columns::duration;
            pub use super::columns::id;
            pub use super::columns::media_id;
            pub use super::columns::original_resolution;
            pub use super::columns::quality;
            pub use super::columns::target_file;
            pub use super::table as mediafile;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (
            id,
            media_id,
            target_file,
            quality,
            codec,
            audio,
            original_resolution,
            duration,
        ) = (
            id,
            media_id,
            target_file,
            quality,
            codec,
            audio,
            original_resolution,
            duration,
        );
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (
            Integer,
            Nullable<Integer>,
            Text,
            Nullable<Text>,
            Nullable<Text>,
            Nullable<Text>,
            Nullable<Text>,
            Nullable<Integer>,
        );
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("mediafile")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (
                id,
                media_id,
                target_file,
                quality,
                codec,
                audio,
                original_resolution,
                duration,
            );
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (
                    id,
                    media_id,
                    target_file,
                    quality,
                    codec,
                    audio,
                    original_resolution,
                    duration,
                )
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct media_id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for media_id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        media_id => {
                            let mut debug_trait_builder = f.debug_tuple("media_id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for media_id {
                #[inline]
                fn clone(&self) -> media_id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for media_id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_media_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for media_id {
                    type QueryId = media_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for media_id {
                #[inline]
                fn default() -> media_id {
                    media_id
                }
            }
            impl ::diesel::expression::Expression for media_id {
                type SqlType = Nullable<Integer>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for media_id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("media_id")
                }
            }
            impl SelectableExpression<table> for media_id {}
            impl<QS> AppearsOnTable<QS> for media_id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for media_id
            where
                media_id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for media_id
            where
                media_id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for media_id where
                media_id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for media_id where
                media_id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for media_id {}
            impl ::diesel::query_source::Column for media_id {
                type Table = table;
                const NAME: &'static str = "media_id";
            }
            impl<T> ::diesel::EqAll<T> for media_id
            where
                T: ::diesel::expression::AsExpression<Nullable<Integer>>,
                ::diesel::dsl::Eq<media_id, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for media_id where Rhs : :: diesel :: expression :: AsExpression < < < media_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for media_id where Rhs : :: diesel :: expression :: AsExpression < < < media_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for media_id where Rhs : :: diesel :: expression :: AsExpression < < < media_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for media_id where Rhs : :: diesel :: expression :: AsExpression < < < media_id as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct target_file;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for target_file {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        target_file => {
                            let mut debug_trait_builder = f.debug_tuple("target_file");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for target_file {
                #[inline]
                fn clone(&self) -> target_file {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for target_file {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_target_file() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for target_file {
                    type QueryId = target_file;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for target_file {
                #[inline]
                fn default() -> target_file {
                    target_file
                }
            }
            impl ::diesel::expression::Expression for target_file {
                type SqlType = Text;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for target_file
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("target_file")
                }
            }
            impl SelectableExpression<table> for target_file {}
            impl<QS> AppearsOnTable<QS> for target_file where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for target_file
            where
                target_file: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for target_file
            where
                target_file: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for target_file where
                target_file: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for target_file where
                target_file: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for target_file {}
            impl ::diesel::query_source::Column for target_file {
                type Table = table;
                const NAME: &'static str = "target_file";
            }
            impl<T> ::diesel::EqAll<T> for target_file
            where
                T: ::diesel::expression::AsExpression<Text>,
                ::diesel::dsl::Eq<target_file, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct quality;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for quality {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        quality => {
                            let mut debug_trait_builder = f.debug_tuple("quality");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for quality {
                #[inline]
                fn clone(&self) -> quality {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for quality {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_quality() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for quality {
                    type QueryId = quality;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for quality {
                #[inline]
                fn default() -> quality {
                    quality
                }
            }
            impl ::diesel::expression::Expression for quality {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for quality
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("quality")
                }
            }
            impl SelectableExpression<table> for quality {}
            impl<QS> AppearsOnTable<QS> for quality where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for quality
            where
                quality: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for quality
            where
                quality: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for quality where
                quality: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for quality where
                quality: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for quality {}
            impl ::diesel::query_source::Column for quality {
                type Table = table;
                const NAME: &'static str = "quality";
            }
            impl<T> ::diesel::EqAll<T> for quality
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<quality, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct codec;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for codec {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        codec => {
                            let mut debug_trait_builder = f.debug_tuple("codec");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for codec {
                #[inline]
                fn clone(&self) -> codec {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for codec {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_codec() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for codec {
                    type QueryId = codec;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for codec {
                #[inline]
                fn default() -> codec {
                    codec
                }
            }
            impl ::diesel::expression::Expression for codec {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for codec
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("codec")
                }
            }
            impl SelectableExpression<table> for codec {}
            impl<QS> AppearsOnTable<QS> for codec where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for codec
            where
                codec: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for codec
            where
                codec: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for codec where
                codec: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for codec where
                codec: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for codec {}
            impl ::diesel::query_source::Column for codec {
                type Table = table;
                const NAME: &'static str = "codec";
            }
            impl<T> ::diesel::EqAll<T> for codec
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<codec, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct audio;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for audio {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        audio => {
                            let mut debug_trait_builder = f.debug_tuple("audio");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for audio {
                #[inline]
                fn clone(&self) -> audio {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for audio {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_audio() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for audio {
                    type QueryId = audio;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for audio {
                #[inline]
                fn default() -> audio {
                    audio
                }
            }
            impl ::diesel::expression::Expression for audio {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for audio
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("audio")
                }
            }
            impl SelectableExpression<table> for audio {}
            impl<QS> AppearsOnTable<QS> for audio where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for audio
            where
                audio: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for audio
            where
                audio: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for audio where
                audio: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for audio where
                audio: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for audio {}
            impl ::diesel::query_source::Column for audio {
                type Table = table;
                const NAME: &'static str = "audio";
            }
            impl<T> ::diesel::EqAll<T> for audio
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<audio, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct original_resolution;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for original_resolution {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        original_resolution => {
                            let mut debug_trait_builder = f.debug_tuple("original_resolution");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for original_resolution {
                #[inline]
                fn clone(&self) -> original_resolution {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for original_resolution {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_original_resolution() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for original_resolution {
                    type QueryId = original_resolution;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for original_resolution {
                #[inline]
                fn default() -> original_resolution {
                    original_resolution
                }
            }
            impl ::diesel::expression::Expression for original_resolution {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for original_resolution
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("original_resolution")
                }
            }
            impl SelectableExpression<table> for original_resolution {}
            impl<QS> AppearsOnTable<QS> for original_resolution where
                QS: AppearsInFromClause<table, Count = Once>
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for original_resolution
            where
                original_resolution: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for original_resolution
            where
                original_resolution: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for original_resolution where
                original_resolution: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for original_resolution where
                original_resolution:
                    SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for original_resolution {}
            impl ::diesel::query_source::Column for original_resolution {
                type Table = table;
                const NAME: &'static str = "original_resolution";
            }
            impl<T> ::diesel::EqAll<T> for original_resolution
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<original_resolution, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct duration;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for duration {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        duration => {
                            let mut debug_trait_builder = f.debug_tuple("duration");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for duration {
                #[inline]
                fn clone(&self) -> duration {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for duration {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_duration() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for duration {
                    type QueryId = duration;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for duration {
                #[inline]
                fn default() -> duration {
                    duration
                }
            }
            impl ::diesel::expression::Expression for duration {
                type SqlType = Nullable<Integer>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for duration
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("duration")
                }
            }
            impl SelectableExpression<table> for duration {}
            impl<QS> AppearsOnTable<QS> for duration where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for duration
            where
                duration: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for duration
            where
                duration: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for duration where
                duration: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for duration where
                duration: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for duration {}
            impl ::diesel::query_source::Column for duration {
                type Table = table;
                const NAME: &'static str = "duration";
            }
            impl<T> ::diesel::EqAll<T> for duration
            where
                T: ::diesel::expression::AsExpression<Nullable<Integer>>,
                ::diesel::dsl::Eq<duration, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for duration where Rhs : :: diesel :: expression :: AsExpression < < < duration as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for duration where Rhs : :: diesel :: expression :: AsExpression < < < duration as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for duration where Rhs : :: diesel :: expression :: AsExpression < < < duration as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for duration where Rhs : :: diesel :: expression :: AsExpression < < < duration as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
        }
    }
    pub mod movie {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::id;
            pub use super::table as movie;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id,) = (id,);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Integer,);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("movie")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (id,);
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (id,)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod season {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::added;
            pub use super::columns::id;
            pub use super::columns::poster;
            pub use super::columns::season_number;
            pub use super::columns::tvshowid;
            pub use super::table as season;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, season_number, tvshowid, added, poster) =
            (id, season_number, tvshowid, added, poster);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Integer, Integer, Integer, Nullable<Text>, Nullable<Text>);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("season")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (id, season_number, tvshowid, added, poster);
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (id, season_number, tvshowid, added, poster)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct season_number;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for season_number {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        season_number => {
                            let mut debug_trait_builder = f.debug_tuple("season_number");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for season_number {
                #[inline]
                fn clone(&self) -> season_number {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for season_number {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_season_number() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for season_number {
                    type QueryId = season_number;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for season_number {
                #[inline]
                fn default() -> season_number {
                    season_number
                }
            }
            impl ::diesel::expression::Expression for season_number {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for season_number
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("season_number")
                }
            }
            impl SelectableExpression<table> for season_number {}
            impl<QS> AppearsOnTable<QS> for season_number where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for season_number
            where
                season_number: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for season_number
            where
                season_number: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for season_number where
                season_number: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for season_number where
                season_number: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for season_number {}
            impl ::diesel::query_source::Column for season_number {
                type Table = table;
                const NAME: &'static str = "season_number";
            }
            impl<T> ::diesel::EqAll<T> for season_number
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<season_number, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for season_number where Rhs : :: diesel :: expression :: AsExpression < < < season_number as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for season_number where Rhs : :: diesel :: expression :: AsExpression < < < season_number as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for season_number where Rhs : :: diesel :: expression :: AsExpression < < < season_number as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for season_number where Rhs : :: diesel :: expression :: AsExpression < < < season_number as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct tvshowid;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for tvshowid {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        tvshowid => {
                            let mut debug_trait_builder = f.debug_tuple("tvshowid");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for tvshowid {
                #[inline]
                fn clone(&self) -> tvshowid {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for tvshowid {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_tvshowid() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for tvshowid {
                    type QueryId = tvshowid;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for tvshowid {
                #[inline]
                fn default() -> tvshowid {
                    tvshowid
                }
            }
            impl ::diesel::expression::Expression for tvshowid {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for tvshowid
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("tvshowid")
                }
            }
            impl SelectableExpression<table> for tvshowid {}
            impl<QS> AppearsOnTable<QS> for tvshowid where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for tvshowid
            where
                tvshowid: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for tvshowid
            where
                tvshowid: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for tvshowid where
                tvshowid: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for tvshowid where
                tvshowid: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for tvshowid {}
            impl ::diesel::query_source::Column for tvshowid {
                type Table = table;
                const NAME: &'static str = "tvshowid";
            }
            impl<T> ::diesel::EqAll<T> for tvshowid
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<tvshowid, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl < Rhs > :: std :: ops :: Add < Rhs > for tvshowid where Rhs : :: diesel :: expression :: AsExpression < < < tvshowid as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Add > :: Rhs > { type Output = :: diesel :: expression :: ops :: Add < Self , Rhs :: Expression > ; fn add ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Add :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Sub < Rhs > for tvshowid where Rhs : :: diesel :: expression :: AsExpression < < < tvshowid as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Sub > :: Rhs > { type Output = :: diesel :: expression :: ops :: Sub < Self , Rhs :: Expression > ; fn sub ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Sub :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Div < Rhs > for tvshowid where Rhs : :: diesel :: expression :: AsExpression < < < tvshowid as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Div > :: Rhs > { type Output = :: diesel :: expression :: ops :: Div < Self , Rhs :: Expression > ; fn div ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Div :: new ( self , rhs . as_expression ( ) ) } }
            impl < Rhs > :: std :: ops :: Mul < Rhs > for tvshowid where Rhs : :: diesel :: expression :: AsExpression < < < tvshowid as :: diesel :: Expression > :: SqlType as :: diesel :: sql_types :: ops :: Mul > :: Rhs > { type Output = :: diesel :: expression :: ops :: Mul < Self , Rhs :: Expression > ; fn mul ( self , rhs : Rhs ) -> Self :: Output { :: diesel :: expression :: ops :: Mul :: new ( self , rhs . as_expression ( ) ) } }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct added;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for added {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        added => {
                            let mut debug_trait_builder = f.debug_tuple("added");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for added {
                #[inline]
                fn clone(&self) -> added {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for added {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_added() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for added {
                    type QueryId = added;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for added {
                #[inline]
                fn default() -> added {
                    added
                }
            }
            impl ::diesel::expression::Expression for added {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for added
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("added")
                }
            }
            impl SelectableExpression<table> for added {}
            impl<QS> AppearsOnTable<QS> for added where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for added
            where
                added: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for added
            where
                added: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for added where
                added: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for added where
                added: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for added {}
            impl ::diesel::query_source::Column for added {
                type Table = table;
                const NAME: &'static str = "added";
            }
            impl<T> ::diesel::EqAll<T> for added
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<added, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct poster;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for poster {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        poster => {
                            let mut debug_trait_builder = f.debug_tuple("poster");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for poster {
                #[inline]
                fn clone(&self) -> poster {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for poster {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_poster() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for poster {
                    type QueryId = poster;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for poster {
                #[inline]
                fn default() -> poster {
                    poster
                }
            }
            impl ::diesel::expression::Expression for poster {
                type SqlType = Nullable<Text>;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for poster
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("poster")
                }
            }
            impl SelectableExpression<table> for poster {}
            impl<QS> AppearsOnTable<QS> for poster where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for poster
            where
                poster: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for poster
            where
                poster: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for poster where
                poster: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for poster where
                poster: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for poster {}
            impl ::diesel::query_source::Column for poster {
                type Table = table;
                const NAME: &'static str = "poster";
            }
            impl<T> ::diesel::EqAll<T> for poster
            where
                T: ::diesel::expression::AsExpression<Nullable<Text>>,
                ::diesel::dsl::Eq<poster, T>:
                    ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod streamable_media {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::id;
            pub use super::table as streamable_media;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id,) = (id,);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Integer,);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("streamable_media")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (id,);
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (id,)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
        }
    }
    pub mod tv_show {
        #![allow(dead_code)]
        pub use self::columns::*;
        use diesel::associations::HasTable;
        use diesel::insertable::Insertable;
        use diesel::query_builder::nodes::Identifier;
        use diesel::query_builder::*;
        use diesel::query_source::joins::{Join, JoinOn};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::sql_types::*;
        use diesel::{JoinTo, QuerySource, Table};
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::id;
            pub use super::table as tv_show;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id,) = (id,);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        #[rustc_copy_clone_marker]
        pub struct table;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::fmt::Debug for table {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    table => {
                        let mut debug_trait_builder = f.debug_tuple("table");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::std::marker::Copy for table {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_table() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Integer,);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
        impl QuerySource for table {
            type FromClause = Identifier<'static>;
            type DefaultSelection = <Self as Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                Identifier("tv_show")
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                Self::all_columns()
            }
        }
        impl AsQuery for table {
            type SqlType = SqlType;
            type Query = SelectStatement<Self>;
            fn as_query(self) -> Self::Query {
                SelectStatement::simple(self)
            }
        }
        impl Table for table {
            type PrimaryKey = (id);
            type AllColumns = (id,);
            fn primary_key(&self) -> Self::PrimaryKey {
                (id)
            }
            fn all_columns() -> Self::AllColumns {
                (id,)
            }
        }
        impl HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl IntoUpdateTarget for table {
            type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
            fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
                self.as_query().into_update_target()
            }
        }
        impl AppearsInFromClause<table> for table {
            type Count = Once;
        }
        impl AppearsInFromClause<table> for () {
            type Count = Never;
        }
        impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
        where
            Join<Left, Right, Kind>: JoinTo<table>,
        {
            type FromClause = Join<Left, Right, Kind>;
            type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
            fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = Join::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<Join, On> JoinTo<JoinOn<Join, On>> for table
        where
            JoinOn<Join, On>: JoinTo<table>,
        {
            type FromClause = JoinOn<Join, On>;
            type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
            fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = JoinOn::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
        where
            SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
        {
            type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
            type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = SelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
        where
            BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
        {
            type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
            type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
            fn join_target(
                rhs: BoxedSelectStatement<'a, QS, ST, DB>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, on_clause) = BoxedSelectStatement::join_target(table);
                (rhs, on_clause)
            }
        }
        impl<T> Insertable<T> for table
        where
            <table as AsQuery>::Query: Insertable<T>,
        {
            type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                self.as_query().values()
            }
        }
        impl<'a, T> Insertable<T> for &'a table
        where
            table: Insertable<T>,
        {
            type Values = <table as Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use diesel::backend::Backend;
            use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
            use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
            use diesel::query_source::{AppearsInFromClause, Never, Once};
            use diesel::result::QueryResult;
            use diesel::sql_types::*;
            use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            #[rustc_copy_clone_marker]
            pub struct star;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for star {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        star => {
                            let mut debug_trait_builder = f.debug_tuple("star");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for star {}
            impl Expression for star {
                type SqlType = ();
            }
            impl<DB: Backend> QueryFragment<DB> for star
            where
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".*");
                    Ok(())
                }
            }
            impl SelectableExpression<table> for star {}
            impl AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            #[rustc_copy_clone_marker]
            pub struct id;
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::fmt::Debug for id {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        id => {
                            let mut debug_trait_builder = f.debug_tuple("id");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    {
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::marker::Copy for id {}
            #[allow(non_snake_case, unused_extern_crates, unused_imports)]
            fn _impl_query_id_for_id() {
                extern crate std;
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            #[allow(non_camel_case_types, dead_code)]
            impl ::std::default::Default for id {
                #[inline]
                fn default() -> id {
                    id
                }
            }
            impl ::diesel::expression::Expression for id {
                type SqlType = Integer;
            }
            impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
            where
                DB: ::diesel::backend::Backend,
                <table as QuerySource>::FromClause: QueryFragment<DB>,
            {
                fn walk_ast(
                    &self,
                    mut out: ::diesel::query_builder::AstPass<DB>,
                ) -> ::diesel::result::QueryResult<()> {
                    table.from_clause().walk_ast(out.reborrow())?;
                    out.push_sql(".");
                    out.push_identifier("id")
                }
            }
            impl SelectableExpression<table> for id {}
            impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
            impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
            where
                id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
                Left: AppearsInFromClause<table, Count = Once>,
                Right: AppearsInFromClause<table, Count = Never>,
            {
            }
            impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
            where
                id: AppearsOnTable<Join<Left, Right, Inner>>,
                Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
            {
            }
            impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
                id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
            {
            }
            impl<From> SelectableExpression<SelectStatement<From>> for id where
                id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
            {
            }
            impl ::diesel::expression::NonAggregate for id {}
            impl ::diesel::query_source::Column for id {
                type Table = table;
                const NAME: &'static str = "id";
            }
            impl<T> ::diesel::EqAll<T> for id
            where
                T: ::diesel::expression::AsExpression<Integer>,
                ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
            {
                type Output = ::diesel::dsl::Eq<Self, T>;
                fn eq_all(self, rhs: T) -> Self::Output {
                    ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
                fn add(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Add::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
                fn div(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Div::new(self, rhs.as_expression())
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: ::diesel::expression::AsExpression<
                    <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, rhs: Rhs) -> Self::Output {
                    ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
                }
            }
        }
    }
    impl ::diesel::JoinTo<streamable_media::table> for episode::table {
        type FromClause = streamable_media::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<episode::id>,
            ::diesel::expression::nullable::Nullable<
                <streamable_media::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: streamable_media::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                episode::id.nullable().eq(
                    <streamable_media::table as ::diesel::query_source::Table>::primary_key(
                        &streamable_media::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<episode::table> for streamable_media::table {
        type FromClause = episode::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<episode::id>,
            ::diesel::expression::nullable::Nullable<
                <streamable_media::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: episode::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                episode::id.nullable().eq(
                    <streamable_media::table as ::diesel::query_source::Table>::primary_key(
                        &streamable_media::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<library::table> for media::table {
        type FromClause = library::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<media::library_id>,
            ::diesel::expression::nullable::Nullable<
                <library::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: library::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                media::library_id.nullable().eq(
                    <library::table as ::diesel::query_source::Table>::primary_key(&library::table)
                        .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<media::table> for library::table {
        type FromClause = media::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<media::library_id>,
            ::diesel::expression::nullable::Nullable<
                <library::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: media::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                media::library_id.nullable().eq(
                    <library::table as ::diesel::query_source::Table>::primary_key(&library::table)
                        .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<streamable_media::table> for mediafile::table {
        type FromClause = streamable_media::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<mediafile::media_id>,
            ::diesel::expression::nullable::Nullable<
                <streamable_media::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: streamable_media::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                mediafile::media_id.nullable().eq(
                    <streamable_media::table as ::diesel::query_source::Table>::primary_key(
                        &streamable_media::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<mediafile::table> for streamable_media::table {
        type FromClause = mediafile::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<mediafile::media_id>,
            ::diesel::expression::nullable::Nullable<
                <streamable_media::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: mediafile::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                mediafile::media_id.nullable().eq(
                    <streamable_media::table as ::diesel::query_source::Table>::primary_key(
                        &streamable_media::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<streamable_media::table> for movie::table {
        type FromClause = streamable_media::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<movie::id>,
            ::diesel::expression::nullable::Nullable<
                <streamable_media::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: streamable_media::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                movie::id.nullable().eq(
                    <streamable_media::table as ::diesel::query_source::Table>::primary_key(
                        &streamable_media::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<movie::table> for streamable_media::table {
        type FromClause = movie::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<movie::id>,
            ::diesel::expression::nullable::Nullable<
                <streamable_media::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: movie::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                movie::id.nullable().eq(
                    <streamable_media::table as ::diesel::query_source::Table>::primary_key(
                        &streamable_media::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<tv_show::table> for season::table {
        type FromClause = tv_show::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<season::tvshowid>,
            ::diesel::expression::nullable::Nullable<
                <tv_show::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: tv_show::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                season::tvshowid.nullable().eq(
                    <tv_show::table as ::diesel::query_source::Table>::primary_key(&tv_show::table)
                        .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<season::table> for tv_show::table {
        type FromClause = season::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<season::tvshowid>,
            ::diesel::expression::nullable::Nullable<
                <tv_show::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: season::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                season::tvshowid.nullable().eq(
                    <tv_show::table as ::diesel::query_source::Table>::primary_key(&tv_show::table)
                        .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<media::table> for streamable_media::table {
        type FromClause = media::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<streamable_media::id>,
            ::diesel::expression::nullable::Nullable<
                <media::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: media::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                streamable_media::id.nullable().eq(
                    <media::table as ::diesel::query_source::Table>::primary_key(&media::table)
                        .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<streamable_media::table> for media::table {
        type FromClause = streamable_media::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<streamable_media::id>,
            ::diesel::expression::nullable::Nullable<
                <media::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: streamable_media::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                streamable_media::id.nullable().eq(
                    <media::table as ::diesel::query_source::Table>::primary_key(&media::table)
                        .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<media::table> for tv_show::table {
        type FromClause = media::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<tv_show::id>,
            ::diesel::expression::nullable::Nullable<
                <media::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: media::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                tv_show::id.nullable().eq(
                    <media::table as ::diesel::query_source::Table>::primary_key(&media::table)
                        .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<tv_show::table> for media::table {
        type FromClause = tv_show::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::expression::nullable::Nullable<tv_show::id>,
            ::diesel::expression::nullable::Nullable<
                <media::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: tv_show::table) -> (Self::FromClause, Self::OnClause) {
            use diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                tv_show::id.nullable().eq(
                    <media::table as ::diesel::query_source::Table>::primary_key(&media::table)
                        .nullable(),
                ),
            )
        }
    }
    impl ::diesel::query_source::AppearsInFromClause<episode::table> for library::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<library::table> for episode::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<episode::table> for media::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<media::table> for episode::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<episode::table> for mediafile::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<mediafile::table> for episode::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<episode::table> for movie::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<movie::table> for episode::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<episode::table> for season::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<season::table> for episode::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<episode::table> for streamable_media::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<streamable_media::table> for episode::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<episode::table> for tv_show::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<tv_show::table> for episode::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<library::table> for media::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<media::table> for library::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<library::table> for mediafile::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<mediafile::table> for library::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<library::table> for movie::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<movie::table> for library::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<library::table> for season::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<season::table> for library::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<library::table> for streamable_media::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<streamable_media::table> for library::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<library::table> for tv_show::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<tv_show::table> for library::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<media::table> for mediafile::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<mediafile::table> for media::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<media::table> for movie::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<movie::table> for media::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<media::table> for season::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<season::table> for media::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<media::table> for streamable_media::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<streamable_media::table> for media::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<media::table> for tv_show::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<tv_show::table> for media::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<mediafile::table> for movie::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<movie::table> for mediafile::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<mediafile::table> for season::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<season::table> for mediafile::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<mediafile::table> for streamable_media::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<streamable_media::table> for mediafile::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<mediafile::table> for tv_show::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<tv_show::table> for mediafile::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<movie::table> for season::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<season::table> for movie::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<movie::table> for streamable_media::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<streamable_media::table> for movie::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<movie::table> for tv_show::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<tv_show::table> for movie::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<season::table> for streamable_media::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<streamable_media::table> for season::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<season::table> for tv_show::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<tv_show::table> for season::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<streamable_media::table> for tv_show::table {
        type Count = ::diesel::query_source::Never;
    }
    impl ::diesel::query_source::AppearsInFromClause<tv_show::table> for streamable_media::table {
        type Count = ::diesel::query_source::Never;
    }
}
#[macro_use]
pub mod macros {}
pub mod tests {
    mod library_tests {
        use crate::core::rocket;
        use rocket::http::ContentType;
        use rocket::http::Status;
        use rocket::local::Client;
        use serde_json::Result;
    }
}
pub mod core {
    use crate::database::media;
    #[allow(unused_imports)]
    use crate::routes;
    use diesel::prelude::*;
    use rocket::fairing::AdHoc;
    use rocket::Request;
    use rocket::Rocket;
    use rocket_contrib::json::JsonValue;
    #[allow(dead_code)]
    mod embedded_migrations {
        struct _Dummy;
        extern crate diesel;
        extern crate diesel_migrations;
        use self::diesel::connection::SimpleConnection;
        use self::diesel_migrations::*;
        use std::io;
        const ALL_MIGRATIONS : & [ & Migration ] = & [ & EmbeddedMigration { version : "20190307120346" , up_sql : "-- Library table\nCREATE TABLE library (\n    id INTEGER NOT NULL,\n    name VARCHAR NOT NULL,\n    location VARCHAR NOT NULL,\n    media_type VARCHAR(50) NOT NULL,\n    PRIMARY KEY (id)\n);\n\n-- Media table\n-- This table contains the template for\n-- the movie and tv shows tables minus containing\n-- the paths because movies are streamable while\n-- tv shows generally arent\n-- The Episodes table will also inherit from here\nCREATE TABLE media (\n    id INTEGER NOT NULL,\n    -- library_id contains the id of the library\n    -- where the media was located in and attributes\n    -- to, this will be a foreign key to the table `library`\n    library_id INTEGER NOT NULL,\n\n    name VARCHAR(80) NOT NULL,\n    description TEXT,\n    rating INTEGER,\n    year INTEGER,\n    added TEXT,\n    poster_path TEXT,\n\n    -- media_type defines what kind of media this entry is\n    -- it can be anything but we currently\n    -- only support `movie` and `tv`\n    media_type VARCHAR(50),\n\n    PRIMARY KEY (id),\n    FOREIGN KEY(library_id) REFERENCES library (id)\n);\n\n-- Streamble Media Table\n-- This table contains the template for\n-- Media that has a file attached to it\n-- ie it can be streamed.\n-- Currently only movies and episodes inherit from this\n--\n-- Tables that reference a foreign key are: `movie` and `episode`\nCREATE TABLE streamable_media (\n    id INTEGER NOT NULL,\n    PRIMARY KEY (id),\n\n    -- We reference media here, by creating a many to many\n    -- relationship between media and streamable_media\n    FOREIGN KEY(id) REFERENCES media (id)\n);\n\nCREATE TABLE movie (\n    id INTEGER NOT NULL,\n    PRIMARY KEY (id),\n    FOREIGN KEY(id) REFERENCES streamable_media (id)\n);\n\nCREATE TABLE tv_show (\n\tid INTEGER NOT NULL,\n\tPRIMARY KEY (id),\n\tFOREIGN KEY(id) REFERENCES media (id)\n);\n\nCREATE TABLE season (\n    id INTEGER NOT NULL,\n\tseason_number INTEGER NOT NULL,\n\ttvshowid INTEGER NOT NULL,\n\tadded TEXT,\n\tposter TEXT,\n\tPRIMARY KEY (id),\n\tFOREIGN KEY(tvshowid) REFERENCES tv_show (id)\n);\n\nCREATE TABLE episode (\n\tid INTEGER NOT NULL,\n\tseasonid INTEGER NOT NULL,\n\tepisode INTEGER NOT NULL,\n\tPRIMARY KEY (id),\n\tFOREIGN KEY(id) REFERENCES streamable_media (id),\n\tFOREIGN KEY(seasonid) REFERENCES seasons (id)\n);\n\nCREATE TABLE mediafile (\n\tid INTEGER NOT NULL,\n\tmedia_id INTEGER,\n\ttarget_file TEXT NOT NULL,\n\tquality VARCHAR(10),\n\tcodec VARCHAR(10),\n\taudio VARCHAR(10),\n\toriginal_resolution VARCHAR(10),\n\tduration INTEGER,\n\tPRIMARY KEY (id),\n\tFOREIGN KEY(media_id) REFERENCES streamable_media (id)\n);" , } ] ;
        struct EmbeddedMigration {
            version: &'static str,
            up_sql: &'static str,
        }
        impl Migration for EmbeddedMigration {
            fn version(&self) -> &str {
                self.version
            }
            fn run(&self, conn: &SimpleConnection) -> Result<(), RunMigrationsError> {
                conn.batch_execute(self.up_sql).map_err(Into::into)
            }
            fn revert(&self, _conn: &SimpleConnection) -> Result<(), RunMigrationsError> {
                {
                    {
                        ::std::rt::begin_panic(
                            "internal error: entered unreachable code",
                            &("src/core.rs", 11u32, 1u32),
                        )
                    }
                }
            }
        }
        pub fn run<C: MigrationConnection>(conn: &C) -> Result<(), RunMigrationsError> {
            run_with_output(conn, &mut io::sink())
        }
        pub fn run_with_output<C: MigrationConnection>(
            conn: &C,
            out: &mut io::Write,
        ) -> Result<(), RunMigrationsError> {
            run_migrations(conn, ALL_MIGRATIONS.iter().map(|v| *v), out)
        }
    }
    /// The request guard type.
    pub struct DbConnection(
        pub  ::rocket_contrib::databases::r2d2::PooledConnection<
            <SqliteConnection as ::rocket_contrib::databases::Poolable>::Manager,
        >,
    );
    /// The pool type.
    pub struct DbConnectionPool(
        ::rocket_contrib::databases::r2d2::Pool<
            <SqliteConnection as ::rocket_contrib::databases::Poolable>::Manager,
        >,
    );
    impl DbConnection {
        /// Returns a fairing that initializes the associated database
        /// connection pool.
        pub fn fairing() -> impl ::rocket::fairing::Fairing {
            use rocket_contrib::databases::Poolable;
            ::rocket::fairing::AdHoc::on_attach("\'openflix\' Database Pool", |rocket| {
                let pool =
                    ::rocket_contrib::databases::database_config("openflix", rocket.config())
                        .map(SqliteConnection::pool);
                match pool {
                    Ok(Ok(p)) => Ok(rocket.manage(DbConnectionPool(p))),
                    Err(config_error) => {
                        ::rocket::logger::error(&::alloc::fmt::format(
                            ::std::fmt::Arguments::new_v1(
                                &["Database configuration failure: \'", "\'"],
                                &match (&"openflix",) {
                                    (arg0,) => [::std::fmt::ArgumentV1::new(
                                        arg0,
                                        ::std::fmt::Display::fmt,
                                    )],
                                },
                            ),
                        ));
                        ::rocket::logger::error_(&::alloc::fmt::format(
                            ::std::fmt::Arguments::new_v1(
                                &[""],
                                &match (&config_error,) {
                                    (arg0,) => [::std::fmt::ArgumentV1::new(
                                        arg0,
                                        ::std::fmt::Display::fmt,
                                    )],
                                },
                            ),
                        ));
                        Err(rocket)
                    }
                    Ok(Err(pool_error)) => {
                        ::rocket::logger::error(&::alloc::fmt::format(
                            ::std::fmt::Arguments::new_v1(
                                &["Failed to initialize pool for \'", "\'"],
                                &match (&"openflix",) {
                                    (arg0,) => [::std::fmt::ArgumentV1::new(
                                        arg0,
                                        ::std::fmt::Display::fmt,
                                    )],
                                },
                            ),
                        ));
                        ::rocket::logger::error_(&::alloc::fmt::format(
                            ::std::fmt::Arguments::new_v1(
                                &[""],
                                &match (&pool_error,) {
                                    (arg0,) => {
                                        [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt)]
                                    }
                                },
                            ),
                        ));
                        Err(rocket)
                    }
                }
            })
        }
        /// Retrieves a connection of type `Self` from the `rocket`
        /// instance. Returns `Some` as long as `Self::fairing()` has been
        /// attached and there is at least one connection in the pool.
        pub fn get_one(rocket: &::rocket::Rocket) -> Option<Self> {
            rocket
                .state::<DbConnectionPool>()
                .and_then(|pool| pool.0.get().ok())
                .map(DbConnection)
        }
    }
    impl ::std::ops::Deref for DbConnection {
        type Target = SqliteConnection;
        #[inline(always)]
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<'a, 'r> ::rocket::request::FromRequest<'a, 'r> for DbConnection {
        type Error = ();
        fn from_request(
            request: &'a ::rocket::request::Request<'r>,
        ) -> ::rocket::request::Outcome<Self, ()> {
            use rocket::{http::Status, Outcome};
            let pool = request.guard::<::rocket::State<DbConnectionPool>>()?;
            match pool.0.get() {
                Ok(conn) => Outcome::Success(DbConnection(conn)),
                Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
            }
        }
    }
    fn service_not_available(_req: &Request) -> JsonValue {
        ::rocket_contrib::json::JsonValue(::serde_json::Value::Object({
            let mut object = ::serde_json::Map::new();
            let _ = object.insert(("type").into(), ::serde_json::to_value(&503).unwrap());
            let _ = object.insert(
                ("error").into(),
                ::serde_json::to_value(&"Database is down").unwrap(),
            );
            object
        }))
    }
    /// Rocket code generated wrapping catch function.
    fn rocket_catch_fn_service_not_available<'_b>(
        __req: &'_b ::rocket::Request,
    ) -> ::rocket::response::Result<'_b> {
        let __response = {
            let __catcher: fn(&::rocket::Request) -> _ = service_not_available;
            ::rocket::response::Responder::respond_to(__catcher(__req), __req)?
        };
        ::rocket::response::Response::build()
            .status(rocket::http::Status {
                code: 503u16,
                reason: "Service Unavailable",
            })
            .merge(__response)
            .ok()
    }
    /// Rocket code generated static catcher info.
    #[allow(non_upper_case_globals)]
    static static_rocket_catch_info_for_service_not_available: ::rocket::StaticCatchInfo =
        ::rocket::StaticCatchInfo {
            code: 503u16,
            handler: rocket_catch_fn_service_not_available,
        };
    fn service_not_found(_req: &Request) -> JsonValue {
        ::rocket_contrib::json::JsonValue(::serde_json::Value::Object({
            let mut object = ::serde_json::Map::new();
            let _ = object.insert(("type").into(), ::serde_json::to_value(&404).unwrap());
            let _ = object.insert(
                ("error").into(),
                ::serde_json::to_value(&"Endpoint not found").unwrap(),
            );
            object
        }))
    }
    /// Rocket code generated wrapping catch function.
    fn rocket_catch_fn_service_not_found<'_b>(
        __req: &'_b ::rocket::Request,
    ) -> ::rocket::response::Result<'_b> {
        let __response = {
            let __catcher: fn(&::rocket::Request) -> _ = service_not_found;
            ::rocket::response::Responder::respond_to(__catcher(__req), __req)?
        };
        ::rocket::response::Response::build()
            .status(rocket::http::Status {
                code: 404u16,
                reason: "Not Found",
            })
            .merge(__response)
            .ok()
    }
    /// Rocket code generated static catcher info.
    #[allow(non_upper_case_globals)]
    static static_rocket_catch_info_for_service_not_found: ::rocket::StaticCatchInfo =
        ::rocket::StaticCatchInfo {
            code: 404u16,
            handler: rocket_catch_fn_service_not_found,
        };
    fn unprocessable_entity() -> JsonValue {
        ::rocket_contrib::json::JsonValue(::serde_json::Value::Object({
            let mut object = ::serde_json::Map::new();
            let _ = object.insert(("type").into(), ::serde_json::to_value(&422).unwrap());
            let _ = object.insert(
                ("error").into(),
                ::serde_json::to_value(&"Invalid json supplied").unwrap(),
            );
            object
        }))
    }
    /// Rocket code generated wrapping catch function.
    fn rocket_catch_fn_unprocessable_entity<'_b>(
        __req: &'_b ::rocket::Request,
    ) -> ::rocket::response::Result<'_b> {
        let __response = {
            let __catcher: fn() -> _ = unprocessable_entity;
            ::rocket::response::Responder::respond_to(__catcher(), __req)?
        };
        ::rocket::response::Response::build()
            .status(rocket::http::Status {
                code: 422u16,
                reason: "Unprocessable Entity",
            })
            .merge(__response)
            .ok()
    }
    /// Rocket code generated static catcher info.
    #[allow(non_upper_case_globals)]
    static static_rocket_catch_info_for_unprocessable_entity: ::rocket::StaticCatchInfo =
        ::rocket::StaticCatchInfo {
            code: 422u16,
            handler: rocket_catch_fn_unprocessable_entity,
        };
    fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
        let conn = DbConnection::get_one(&rocket).expect("Database Connection Failed");
        match embedded_migrations::run(&*conn) {
            Ok(()) => Ok(rocket),
            Err(e) => {
                {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::std::fmt::Arguments::new_v1(
                                &["Failed to run database migrations: "],
                                &match (&e,) {
                                    (arg0,) => {
                                        [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt)]
                                    }
                                },
                            ),
                            lvl,
                            &(
                                "OpenFlixServerRust::core",
                                "OpenFlixServerRust::core",
                                "src/core.rs",
                                45u32,
                            ),
                        );
                    }
                };
                Err(rocket)
            }
        }
    }
    pub fn rocket(db: SqliteConnection) -> Rocket {
        rocket::ignite()
            .attach(db::fairing())
            .attach(AdHoc::on_attach(
                "Running Database Migrations",
                run_db_migrations,
            ))
            .register({
                let __vector: Vec<::rocket::Catcher> = <[_]>::into_vec(box [
                    ::rocket::Catcher::from(&static_rocket_catch_info_for_service_not_found),
                    ::rocket::Catcher::from(&static_rocket_catch_info_for_service_not_available),
                    ::rocket::Catcher::from(&static_rocket_catch_info_for_unprocessable_entity),
                ]);
                __vector
            })
            .mount("/api/v1/library", {
                let __vector: Vec<::rocket::Route> = <[_]>::into_vec(box [
                    ::rocket::Route::from(
                        &routes::library::static_rocket_route_info_for_library_get,
                    ),
                    ::rocket::Route::from(
                        &routes::library::static_rocket_route_info_for_library_post,
                    ),
                    ::rocket::Route::from(
                        &routes::library::static_rocket_route_info_for_library_delete,
                    ),
                    ::rocket::Route::from(
                        &routes::library::static_rocket_route_info_for_get_all_library,
                    ),
                ]);
                __vector
            })
            .mount("/api/v1/media", {
                let __vector: Vec<::rocket::Route> = <[_]>::into_vec(box [
                    ::rocket::Route::from(
                        &routes::media::static_rocket_route_info_for_get_media_by_id,
                    ),
                    ::rocket::Route::from(
                        &routes::media::static_rocket_route_info_for_insert_media_by_lib_id,
                    ),
                    ::rocket::Route::from(
                        &routes::media::static_rocket_route_info_for_update_media_by_id,
                    ),
                ]);
                __vector
            })
    }
}
fn main() {
    let db = core::DbConnection;
    core::rocket(db).launch();
}
