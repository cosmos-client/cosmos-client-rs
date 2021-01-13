// This file is generated by rust-protobuf 2.20.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `cosmos/mint/v1beta1/mint.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_20_0;

#[derive(PartialEq,Clone,Default)]
pub struct Minter {
    // message fields
    pub inflation: ::std::string::String,
    pub annual_provisions: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Minter {
    fn default() -> &'a Minter {
        <Minter as ::protobuf::Message>::default_instance()
    }
}

impl Minter {
    pub fn new() -> Minter {
        ::std::default::Default::default()
    }

    // string inflation = 1;


    pub fn get_inflation(&self) -> &str {
        &self.inflation
    }
    pub fn clear_inflation(&mut self) {
        self.inflation.clear();
    }

    // Param is passed by value, moved
    pub fn set_inflation(&mut self, v: ::std::string::String) {
        self.inflation = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inflation(&mut self) -> &mut ::std::string::String {
        &mut self.inflation
    }

    // Take field
    pub fn take_inflation(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.inflation, ::std::string::String::new())
    }

    // string annual_provisions = 2;


    pub fn get_annual_provisions(&self) -> &str {
        &self.annual_provisions
    }
    pub fn clear_annual_provisions(&mut self) {
        self.annual_provisions.clear();
    }

    // Param is passed by value, moved
    pub fn set_annual_provisions(&mut self, v: ::std::string::String) {
        self.annual_provisions = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_annual_provisions(&mut self) -> &mut ::std::string::String {
        &mut self.annual_provisions
    }

    // Take field
    pub fn take_annual_provisions(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.annual_provisions, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Minter {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.inflation)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.annual_provisions)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.inflation.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.inflation);
        }
        if !self.annual_provisions.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.annual_provisions);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.inflation.is_empty() {
            os.write_string(1, &self.inflation)?;
        }
        if !self.annual_provisions.is_empty() {
            os.write_string(2, &self.annual_provisions)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Minter {
        Minter::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "inflation",
                |m: &Minter| { &m.inflation },
                |m: &mut Minter| { &mut m.inflation },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "annual_provisions",
                |m: &Minter| { &m.annual_provisions },
                |m: &mut Minter| { &mut m.annual_provisions },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Minter>(
                "Minter",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Minter {
        static instance: ::protobuf::rt::LazyV2<Minter> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Minter::new)
    }
}

impl ::protobuf::Clear for Minter {
    fn clear(&mut self) {
        self.inflation.clear();
        self.annual_provisions.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Minter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Minter {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Params {
    // message fields
    pub mint_denom: ::std::string::String,
    pub inflation_rate_change: ::std::string::String,
    pub inflation_max: ::std::string::String,
    pub inflation_min: ::std::string::String,
    pub goal_bonded: ::std::string::String,
    pub blocks_per_year: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Params {
    fn default() -> &'a Params {
        <Params as ::protobuf::Message>::default_instance()
    }
}

impl Params {
    pub fn new() -> Params {
        ::std::default::Default::default()
    }

    // string mint_denom = 1;


    pub fn get_mint_denom(&self) -> &str {
        &self.mint_denom
    }
    pub fn clear_mint_denom(&mut self) {
        self.mint_denom.clear();
    }

    // Param is passed by value, moved
    pub fn set_mint_denom(&mut self, v: ::std::string::String) {
        self.mint_denom = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mint_denom(&mut self) -> &mut ::std::string::String {
        &mut self.mint_denom
    }

    // Take field
    pub fn take_mint_denom(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.mint_denom, ::std::string::String::new())
    }

    // string inflation_rate_change = 2;


    pub fn get_inflation_rate_change(&self) -> &str {
        &self.inflation_rate_change
    }
    pub fn clear_inflation_rate_change(&mut self) {
        self.inflation_rate_change.clear();
    }

    // Param is passed by value, moved
    pub fn set_inflation_rate_change(&mut self, v: ::std::string::String) {
        self.inflation_rate_change = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inflation_rate_change(&mut self) -> &mut ::std::string::String {
        &mut self.inflation_rate_change
    }

    // Take field
    pub fn take_inflation_rate_change(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.inflation_rate_change, ::std::string::String::new())
    }

    // string inflation_max = 3;


    pub fn get_inflation_max(&self) -> &str {
        &self.inflation_max
    }
    pub fn clear_inflation_max(&mut self) {
        self.inflation_max.clear();
    }

    // Param is passed by value, moved
    pub fn set_inflation_max(&mut self, v: ::std::string::String) {
        self.inflation_max = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inflation_max(&mut self) -> &mut ::std::string::String {
        &mut self.inflation_max
    }

    // Take field
    pub fn take_inflation_max(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.inflation_max, ::std::string::String::new())
    }

    // string inflation_min = 4;


    pub fn get_inflation_min(&self) -> &str {
        &self.inflation_min
    }
    pub fn clear_inflation_min(&mut self) {
        self.inflation_min.clear();
    }

    // Param is passed by value, moved
    pub fn set_inflation_min(&mut self, v: ::std::string::String) {
        self.inflation_min = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inflation_min(&mut self) -> &mut ::std::string::String {
        &mut self.inflation_min
    }

    // Take field
    pub fn take_inflation_min(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.inflation_min, ::std::string::String::new())
    }

    // string goal_bonded = 5;


    pub fn get_goal_bonded(&self) -> &str {
        &self.goal_bonded
    }
    pub fn clear_goal_bonded(&mut self) {
        self.goal_bonded.clear();
    }

    // Param is passed by value, moved
    pub fn set_goal_bonded(&mut self, v: ::std::string::String) {
        self.goal_bonded = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_goal_bonded(&mut self) -> &mut ::std::string::String {
        &mut self.goal_bonded
    }

    // Take field
    pub fn take_goal_bonded(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.goal_bonded, ::std::string::String::new())
    }

    // uint64 blocks_per_year = 6;


    pub fn get_blocks_per_year(&self) -> u64 {
        self.blocks_per_year
    }
    pub fn clear_blocks_per_year(&mut self) {
        self.blocks_per_year = 0;
    }

    // Param is passed by value, moved
    pub fn set_blocks_per_year(&mut self, v: u64) {
        self.blocks_per_year = v;
    }
}

impl ::protobuf::Message for Params {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.mint_denom)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.inflation_rate_change)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.inflation_max)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.inflation_min)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.goal_bonded)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.blocks_per_year = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.mint_denom.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.mint_denom);
        }
        if !self.inflation_rate_change.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.inflation_rate_change);
        }
        if !self.inflation_max.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.inflation_max);
        }
        if !self.inflation_min.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.inflation_min);
        }
        if !self.goal_bonded.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.goal_bonded);
        }
        if self.blocks_per_year != 0 {
            my_size += ::protobuf::rt::value_size(6, self.blocks_per_year, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.mint_denom.is_empty() {
            os.write_string(1, &self.mint_denom)?;
        }
        if !self.inflation_rate_change.is_empty() {
            os.write_string(2, &self.inflation_rate_change)?;
        }
        if !self.inflation_max.is_empty() {
            os.write_string(3, &self.inflation_max)?;
        }
        if !self.inflation_min.is_empty() {
            os.write_string(4, &self.inflation_min)?;
        }
        if !self.goal_bonded.is_empty() {
            os.write_string(5, &self.goal_bonded)?;
        }
        if self.blocks_per_year != 0 {
            os.write_uint64(6, self.blocks_per_year)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Params {
        Params::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "mint_denom",
                |m: &Params| { &m.mint_denom },
                |m: &mut Params| { &mut m.mint_denom },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "inflation_rate_change",
                |m: &Params| { &m.inflation_rate_change },
                |m: &mut Params| { &mut m.inflation_rate_change },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "inflation_max",
                |m: &Params| { &m.inflation_max },
                |m: &mut Params| { &mut m.inflation_max },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "inflation_min",
                |m: &Params| { &m.inflation_min },
                |m: &mut Params| { &mut m.inflation_min },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "goal_bonded",
                |m: &Params| { &m.goal_bonded },
                |m: &mut Params| { &mut m.goal_bonded },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "blocks_per_year",
                |m: &Params| { &m.blocks_per_year },
                |m: &mut Params| { &mut m.blocks_per_year },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Params>(
                "Params",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Params {
        static instance: ::protobuf::rt::LazyV2<Params> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Params::new)
    }
}

impl ::protobuf::Clear for Params {
    fn clear(&mut self) {
        self.mint_denom.clear();
        self.inflation_rate_change.clear();
        self.inflation_max.clear();
        self.inflation_min.clear();
        self.goal_bonded.clear();
        self.blocks_per_year = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Params {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Params {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1ecosmos/mint/v1beta1/mint.proto\x12\x13cosmos.mint.v1beta1\x1a\x14g\
    ogoproto/gogo.proto\"\xcf\x01\n\x06Minter\x12L\n\tinflation\x18\x01\x20\
    \x01(\tR\tinflationB.\xc8\xde\x1f\0\xda\xde\x1f&github.com/cosmos/cosmos\
    -sdk/types.Dec\x12w\n\x11annual_provisions\x18\x02\x20\x01(\tR\x10annual\
    ProvisionsBJ\xda\xde\x1f&github.com/cosmos/cosmos-sdk/types.Dec\xf2\xde\
    \x1f\x18yaml:\"annual_provisions\"\xc8\xde\x1f\0\"\xb7\x04\n\x06Params\
    \x12\x1d\n\nmint_denom\x18\x01\x20\x01(\tR\tmintDenom\x12\x82\x01\n\x15i\
    nflation_rate_change\x18\x02\x20\x01(\tR\x13inflationRateChangeBN\xf2\
    \xde\x1f\x1cyaml:\"inflation_rate_change\"\xda\xde\x1f&github.com/cosmos\
    /cosmos-sdk/types.Dec\xc8\xde\x1f\0\x12k\n\rinflation_max\x18\x03\x20\
    \x01(\tR\x0cinflationMaxBF\xda\xde\x1f&github.com/cosmos/cosmos-sdk/type\
    s.Dec\xf2\xde\x1f\x14yaml:\"inflation_max\"\xc8\xde\x1f\0\x12k\n\rinflat\
    ion_min\x18\x04\x20\x01(\tR\x0cinflationMinBF\xc8\xde\x1f\0\xda\xde\x1f&\
    github.com/cosmos/cosmos-sdk/types.Dec\xf2\xde\x1f\x14yaml:\"inflation_m\
    in\"\x12e\n\x0bgoal_bonded\x18\x05\x20\x01(\tR\ngoalBondedBD\xf2\xde\x1f\
    \x12yaml:\"goal_bonded\"\xda\xde\x1f&github.com/cosmos/cosmos-sdk/types.\
    Dec\xc8\xde\x1f\0\x12B\n\x0fblocks_per_year\x18\x06\x20\x01(\x04R\rblock\
    sPerYearB\x1a\xf2\xde\x1f\x16yaml:\"blocks_per_year\":\x04\x98\xa0\x1f\0\
    B+Z)github.com/cosmos/cosmos-sdk/x/mint/typesJ\xa8\x0b\n\x06\x12\x04\0\0\
    4\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\
    \x1b\n\x08\n\x01\x08\x12\x03\x03\0@\n\t\n\x02\x08\x0b\x12\x03\x03\0@\n\t\
    \n\x02\x03\0\x12\x03\x05\x07\x1d\n2\n\x02\x04\0\x12\x04\x08\0\x12\x01\
    \x1a&\x20Minter\x20represents\x20the\x20minting\x20state.\n\n\n\n\x03\
    \x04\0\x01\x12\x03\x08\x08\x0e\n-\n\x04\x04\0\x02\0\x12\x04\n\x02\x0bh\
    \x1a\x1f\x20current\x20annual\x20inflation\x20rate\n\n\r\n\x05\x04\0\x02\
    \0\x04\x12\x04\n\x02\x08\x10\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\n\x02\
    \x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\n\t\x12\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\n\x15\x16\n\x0c\n\x05\x04\0\x02\0\x08\x12\x03\x0b\x06g\n\
    \x0f\n\x08\x04\0\x02\0\x08\xeb\xfb\x03\x12\x03\x0b\x07H\n\x0f\n\x08\x04\
    \0\x02\0\x08\xe9\xfb\x03\x12\x03\x0bJf\n2\n\x04\x04\0\x02\x01\x12\x04\r\
    \x02\x11\x04\x1a$\x20current\x20annual\x20expected\x20provisions\n\n\r\n\
    \x05\x04\0\x02\x01\x04\x12\x04\r\x02\x0bh\n\x0c\n\x05\x04\0\x02\x01\x05\
    \x12\x03\r\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\r\t\x1a\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\r\x1d\x1e\n\r\n\x05\x04\0\x02\x01\x08\x12\
    \x04\r\x1f\x11\x03\n\x0f\n\x08\x04\0\x02\x01\x08\xee\xfb\x03\x12\x03\x0e\
    \x049\n\x0f\n\x08\x04\0\x02\x01\x08\xeb\xfb\x03\x12\x03\x0f\x04E\n\x0f\n\
    \x08\x04\0\x02\x01\x08\xe9\xfb\x03\x12\x03\x10\x04\"\n:\n\x02\x04\x01\
    \x12\x04\x15\04\x01\x1a.\x20Params\x20holds\x20parameters\x20for\x20the\
    \x20mint\x20module.\n\n\n\n\x03\x04\x01\x01\x12\x03\x15\x08\x0e\n\n\n\
    \x03\x04\x01\x07\x12\x03\x16\x02.\n\r\n\x06\x04\x01\x07\x83\xf4\x03\x12\
    \x03\x16\x02.\n#\n\x04\x04\x01\x02\0\x12\x03\x19\x02\x18\x1a\x16\x20type\
    \x20of\x20coin\x20to\x20mint\n\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x19\
    \x02\x16.\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x19\x02\x08\n\x0c\n\x05\
    \x04\x01\x02\0\x01\x12\x03\x19\t\x13\n\x0c\n\x05\x04\x01\x02\0\x03\x12\
    \x03\x19\x16\x17\n7\n\x04\x04\x01\x02\x01\x12\x04\x1b\x02\x1f\x04\x1a)\
    \x20maximum\x20annual\x20change\x20in\x20inflation\x20rate\n\n\r\n\x05\
    \x04\x01\x02\x01\x04\x12\x04\x1b\x02\x19\x18\n\x0c\n\x05\x04\x01\x02\x01\
    \x05\x12\x03\x1b\x02\x08\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x1b\t\
    \x1e\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x1b!\"\n\r\n\x05\x04\x01\
    \x02\x01\x08\x12\x04\x1b#\x1f\x03\n\x0f\n\x08\x04\x01\x02\x01\x08\xee\
    \xfb\x03\x12\x03\x1c\x04=\n\x0f\n\x08\x04\x01\x02\x01\x08\xeb\xfb\x03\
    \x12\x03\x1d\x04E\n\x0f\n\x08\x04\x01\x02\x01\x08\xe9\xfb\x03\x12\x03\
    \x1e\x04\"\n&\n\x04\x04\x01\x02\x02\x12\x04!\x02%\x04\x1a\x18\x20maximum\
    \x20inflation\x20rate\n\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04!\x02\x1f\
    \x04\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03!\x02\x08\n\x0c\n\x05\x04\
    \x01\x02\x02\x01\x12\x03!\t\x16\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03!\
    \x19\x1a\n\r\n\x05\x04\x01\x02\x02\x08\x12\x04!\x1b%\x03\n\x0f\n\x08\x04\
    \x01\x02\x02\x08\xee\xfb\x03\x12\x03\"\x045\n\x0f\n\x08\x04\x01\x02\x02\
    \x08\xeb\xfb\x03\x12\x03#\x04E\n\x0f\n\x08\x04\x01\x02\x02\x08\xe9\xfb\
    \x03\x12\x03$\x04\"\n&\n\x04\x04\x01\x02\x03\x12\x04'\x02+\x04\x1a\x18\
    \x20minimum\x20inflation\x20rate\n\n\r\n\x05\x04\x01\x02\x03\x04\x12\x04\
    '\x02%\x04\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03'\x02\x08\n\x0c\n\x05\
    \x04\x01\x02\x03\x01\x12\x03'\t\x16\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\
    \x03'\x19\x1a\n\r\n\x05\x04\x01\x02\x03\x08\x12\x04'\x1b+\x03\n\x0f\n\
    \x08\x04\x01\x02\x03\x08\xee\xfb\x03\x12\x03(\x045\n\x0f\n\x08\x04\x01\
    \x02\x03\x08\xeb\xfb\x03\x12\x03)\x04E\n\x0f\n\x08\x04\x01\x02\x03\x08\
    \xe9\xfb\x03\x12\x03*\x04\"\n,\n\x04\x04\x01\x02\x04\x12\x04-\x021\x04\
    \x1a\x1e\x20goal\x20of\x20percent\x20bonded\x20atoms\n\n\r\n\x05\x04\x01\
    \x02\x04\x04\x12\x04-\x02+\x04\n\x0c\n\x05\x04\x01\x02\x04\x05\x12\x03-\
    \x02\x08\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x03-\t\x14\n\x0c\n\x05\x04\
    \x01\x02\x04\x03\x12\x03-\x17\x18\n\r\n\x05\x04\x01\x02\x04\x08\x12\x04-\
    \x191\x03\n\x0f\n\x08\x04\x01\x02\x04\x08\xee\xfb\x03\x12\x03.\x043\n\
    \x0f\n\x08\x04\x01\x02\x04\x08\xeb\xfb\x03\x12\x03/\x04E\n\x0f\n\x08\x04\
    \x01\x02\x04\x08\xe9\xfb\x03\x12\x030\x04\"\n'\n\x04\x04\x01\x02\x05\x12\
    \x033\x02Q\x1a\x1a\x20expected\x20blocks\x20per\x20year\n\n\r\n\x05\x04\
    \x01\x02\x05\x04\x12\x043\x021\x04\n\x0c\n\x05\x04\x01\x02\x05\x05\x12\
    \x033\x02\x08\n\x0c\n\x05\x04\x01\x02\x05\x01\x12\x033\t\x18\n\x0c\n\x05\
    \x04\x01\x02\x05\x03\x12\x033\x1b\x1c\n\x0c\n\x05\x04\x01\x02\x05\x08\
    \x12\x033\x1dP\n\x0f\n\x08\x04\x01\x02\x05\x08\xee\xfb\x03\x12\x033\x1eO\
    b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
