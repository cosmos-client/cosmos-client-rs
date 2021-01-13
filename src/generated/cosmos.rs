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
//! Generated file from `cosmos_proto/cosmos.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_20_0;

/// Extension fields
pub mod exts {

    pub const interface_type: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 93001, phantom: ::std::marker::PhantomData };

    pub const implements_interface: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 93002, phantom: ::std::marker::PhantomData };

    pub const accepts_interface: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 93001, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19cosmos_proto/cosmos.proto\x12\x0ccosmos_proto\x1a\x20google/protob\
    uf/descriptor.proto:H\n\x0einterface_type\x18\xc9\xd6\x05\x20\x01(\t\x12\
    \x1f.google.protobuf.MessageOptionsR\rinterfaceType:T\n\x14implements_in\
    terface\x18\xca\xd6\x05\x20\x01(\t\x12\x1f.google.protobuf.MessageOption\
    sR\x13implementsInterface:L\n\x11accepts_interface\x18\xc9\xd6\x05\x20\
    \x01(\t\x12\x1d.google.protobuf.FieldOptionsR\x10acceptsInterfaceB'Z%git\
    hub.com/regen-network/cosmos-protoJ\xaa\x02\n\x06\x12\x04\0\0\x0f\x01\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\x14\n\t\n\
    \x02\x03\0\x12\x03\x03\x07)\n\x08\n\x01\x08\x12\x03\x05\0<\n\t\n\x02\x08\
    \x0b\x12\x03\x05\0<\n\t\n\x01\x07\x12\x04\x07\0\x0b\x01\n\t\n\x02\x07\0\
    \x12\x03\x08\x04\"\n\n\n\x03\x07\0\x02\x12\x03\x07\x07%\n\x0b\n\x03\x07\
    \0\x04\x12\x04\x08\x04\x07'\n\n\n\x03\x07\0\x05\x12\x03\x08\x04\n\n\n\n\
    \x03\x07\0\x01\x12\x03\x08\x0b\x19\n\n\n\x03\x07\0\x03\x12\x03\x08\x1c!\
    \n\t\n\x02\x07\x01\x12\x03\n\x04(\n\n\n\x03\x07\x01\x02\x12\x03\x07\x07%\
    \n\x0b\n\x03\x07\x01\x04\x12\x04\n\x04\x08\"\n\n\n\x03\x07\x01\x05\x12\
    \x03\n\x04\n\n\n\n\x03\x07\x01\x01\x12\x03\n\x0b\x1f\n\n\n\x03\x07\x01\
    \x03\x12\x03\n\"'\n\t\n\x01\x07\x12\x04\r\0\x0f\x01\n\t\n\x02\x07\x02\
    \x12\x03\x0e\x04%\n\n\n\x03\x07\x02\x02\x12\x03\r\x07#\n\x0b\n\x03\x07\
    \x02\x04\x12\x04\x0e\x04\r%\n\n\n\x03\x07\x02\x05\x12\x03\x0e\x04\n\n\n\
    \n\x03\x07\x02\x01\x12\x03\x0e\x0b\x1c\n\n\n\x03\x07\x02\x03\x12\x03\x0e\
    \x1f$b\x06proto3\
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
