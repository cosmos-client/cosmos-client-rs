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
//! Generated file from `cosmos/crypto/multisig/v1beta1/multisig.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_20_0;

#[derive(PartialEq,Clone,Default)]
pub struct MultiSignature {
    // message fields
    pub signatures: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MultiSignature {
    fn default() -> &'a MultiSignature {
        <MultiSignature as ::protobuf::Message>::default_instance()
    }
}

impl MultiSignature {
    pub fn new() -> MultiSignature {
        ::std::default::Default::default()
    }

    // repeated bytes signatures = 1;


    pub fn get_signatures(&self) -> &[::std::vec::Vec<u8>] {
        &self.signatures
    }
    pub fn clear_signatures(&mut self) {
        self.signatures.clear();
    }

    // Param is passed by value, moved
    pub fn set_signatures(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.signatures = v;
    }

    // Mutable pointer to the field.
    pub fn mut_signatures(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.signatures
    }

    // Take field
    pub fn take_signatures(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.signatures, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for MultiSignature {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.signatures)?;
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
        for value in &self.signatures {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.signatures {
            os.write_bytes(1, &v)?;
        };
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

    fn new() -> MultiSignature {
        MultiSignature::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "signatures",
                |m: &MultiSignature| { &m.signatures },
                |m: &mut MultiSignature| { &mut m.signatures },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MultiSignature>(
                "MultiSignature",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MultiSignature {
        static instance: ::protobuf::rt::LazyV2<MultiSignature> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MultiSignature::new)
    }
}

impl ::protobuf::Clear for MultiSignature {
    fn clear(&mut self) {
        self.signatures.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MultiSignature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MultiSignature {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CompactBitArray {
    // message fields
    pub extra_bits_stored: u32,
    pub elems: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CompactBitArray {
    fn default() -> &'a CompactBitArray {
        <CompactBitArray as ::protobuf::Message>::default_instance()
    }
}

impl CompactBitArray {
    pub fn new() -> CompactBitArray {
        ::std::default::Default::default()
    }

    // uint32 extra_bits_stored = 1;


    pub fn get_extra_bits_stored(&self) -> u32 {
        self.extra_bits_stored
    }
    pub fn clear_extra_bits_stored(&mut self) {
        self.extra_bits_stored = 0;
    }

    // Param is passed by value, moved
    pub fn set_extra_bits_stored(&mut self, v: u32) {
        self.extra_bits_stored = v;
    }

    // bytes elems = 2;


    pub fn get_elems(&self) -> &[u8] {
        &self.elems
    }
    pub fn clear_elems(&mut self) {
        self.elems.clear();
    }

    // Param is passed by value, moved
    pub fn set_elems(&mut self, v: ::std::vec::Vec<u8>) {
        self.elems = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_elems(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.elems
    }

    // Take field
    pub fn take_elems(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.elems, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for CompactBitArray {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.extra_bits_stored = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.elems)?;
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
        if self.extra_bits_stored != 0 {
            my_size += ::protobuf::rt::value_size(1, self.extra_bits_stored, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.elems.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.elems);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.extra_bits_stored != 0 {
            os.write_uint32(1, self.extra_bits_stored)?;
        }
        if !self.elems.is_empty() {
            os.write_bytes(2, &self.elems)?;
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

    fn new() -> CompactBitArray {
        CompactBitArray::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "extra_bits_stored",
                |m: &CompactBitArray| { &m.extra_bits_stored },
                |m: &mut CompactBitArray| { &mut m.extra_bits_stored },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "elems",
                |m: &CompactBitArray| { &m.elems },
                |m: &mut CompactBitArray| { &mut m.elems },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CompactBitArray>(
                "CompactBitArray",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CompactBitArray {
        static instance: ::protobuf::rt::LazyV2<CompactBitArray> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CompactBitArray::new)
    }
}

impl ::protobuf::Clear for CompactBitArray {
    fn clear(&mut self) {
        self.extra_bits_stored = 0;
        self.elems.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CompactBitArray {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CompactBitArray {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n-cosmos/crypto/multisig/v1beta1/multisig.proto\x12\x1ecosmos.crypto.mu\
    ltisig.v1beta1\x1a\x14gogoproto/gogo.proto\"6\n\x0eMultiSignature\x12\
    \x1e\n\nsignatures\x18\x01\x20\x03(\x0cR\nsignatures:\x04\xd0\xa1\x1f\
    \x01\"Y\n\x0fCompactBitArray\x12*\n\x11extra_bits_stored\x18\x01\x20\x01\
    (\rR\x0fextraBitsStored\x12\x14\n\x05elems\x18\x02\x20\x01(\x0cR\x05elem\
    s:\x04\x98\xa0\x1f\0B+Z)github.com/cosmos/cosmos-sdk/crypto/typesJ\x9c\
    \x06\n\x06\x12\x04\0\0\x18\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\
    \x01\x02\x12\x03\x01\x08&\n\t\n\x02\x03\0\x12\x03\x03\x07\x1d\n\x08\n\
    \x01\x08\x12\x03\x05\0@\n\t\n\x02\x08\x0b\x12\x03\x05\0@\n\xbc\x01\n\x02\
    \x04\0\x12\x04\n\0\r\x01\x1a\xaf\x01\x20MultiSignature\x20wraps\x20the\
    \x20signatures\x20from\x20a\x20multisig.LegacyAminoPubKey.\n\x20See\x20c\
    osmos.tx.v1betata1.ModeInfo.Multi\x20for\x20how\x20to\x20specify\x20whic\
    h\x20signers\n\x20signed\x20and\x20with\x20which\x20modes.\n\n\n\n\x03\
    \x04\0\x01\x12\x03\n\x08\x16\n\n\n\x03\x04\0\x07\x12\x03\x0b\x021\n\r\n\
    \x06\x04\0\x07\x9a\xf4\x03\x12\x03\x0b\x021\n\x0b\n\x04\x04\0\x02\0\x12\
    \x03\x0c\x02.\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x0c\x02\n\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\x0c\x0b\x10\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x0c\x11\x1b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0c,-\n\xff\x01\n\x02\
    \x04\x01\x12\x04\x13\0\x18\x01\x1a\xf2\x01\x20CompactBitArray\x20is\x20a\
    n\x20implementation\x20of\x20a\x20space\x20efficient\x20bit\x20array.\n\
    \x20This\x20is\x20used\x20to\x20ensure\x20that\x20the\x20encoded\x20data\
    \x20takes\x20up\x20a\x20minimal\x20amount\x20of\n\x20space\x20after\x20p\
    roto\x20encoding.\n\x20This\x20is\x20not\x20thread\x20safe,\x20and\x20is\
    \x20not\x20intended\x20for\x20concurrent\x20usage.\n\n\n\n\x03\x04\x01\
    \x01\x12\x03\x13\x08\x17\n\n\n\x03\x04\x01\x07\x12\x03\x14\x02.\n\r\n\
    \x06\x04\x01\x07\x83\xf4\x03\x12\x03\x14\x02.\n\x0b\n\x04\x04\x01\x02\0\
    \x12\x03\x16\x02\x1f\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x16\x02\x14.\n\
    \x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x16\x02\x08\n\x0c\n\x05\x04\x01\x02\
    \0\x01\x12\x03\x16\t\x1a\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x16\x1d\
    \x1e\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x17\x02\x1f\n\r\n\x05\x04\x01\
    \x02\x01\x04\x12\x04\x17\x02\x16\x1f\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\
    \x03\x17\x02\x07\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x17\t\x0e\n\x0c\
    \n\x05\x04\x01\x02\x01\x03\x12\x03\x17\x1d\x1eb\x06proto3\
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
