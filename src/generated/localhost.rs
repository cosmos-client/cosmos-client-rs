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
//! Generated file from `ibc/lightclients/localhost/v1/localhost.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_20_0;

#[derive(PartialEq,Clone,Default)]
pub struct ClientState {
    // message fields
    pub chain_id: ::std::string::String,
    pub height: ::protobuf::SingularPtrField<super::client::Height>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ClientState {
    fn default() -> &'a ClientState {
        <ClientState as ::protobuf::Message>::default_instance()
    }
}

impl ClientState {
    pub fn new() -> ClientState {
        ::std::default::Default::default()
    }

    // string chain_id = 1;


    pub fn get_chain_id(&self) -> &str {
        &self.chain_id
    }
    pub fn clear_chain_id(&mut self) {
        self.chain_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_chain_id(&mut self, v: ::std::string::String) {
        self.chain_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chain_id(&mut self) -> &mut ::std::string::String {
        &mut self.chain_id
    }

    // Take field
    pub fn take_chain_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.chain_id, ::std::string::String::new())
    }

    // .ibc.core.client.v1.Height height = 2;


    pub fn get_height(&self) -> &super::client::Height {
        self.height.as_ref().unwrap_or_else(|| <super::client::Height as ::protobuf::Message>::default_instance())
    }
    pub fn clear_height(&mut self) {
        self.height.clear();
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: super::client::Height) {
        self.height = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_height(&mut self) -> &mut super::client::Height {
        if self.height.is_none() {
            self.height.set_default();
        }
        self.height.as_mut().unwrap()
    }

    // Take field
    pub fn take_height(&mut self) -> super::client::Height {
        self.height.take().unwrap_or_else(|| super::client::Height::new())
    }
}

impl ::protobuf::Message for ClientState {
    fn is_initialized(&self) -> bool {
        for v in &self.height {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.chain_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.height)?;
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
        if !self.chain_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.chain_id);
        }
        if let Some(ref v) = self.height.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.chain_id.is_empty() {
            os.write_string(1, &self.chain_id)?;
        }
        if let Some(ref v) = self.height.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn new() -> ClientState {
        ClientState::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "chain_id",
                |m: &ClientState| { &m.chain_id },
                |m: &mut ClientState| { &mut m.chain_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::client::Height>>(
                "height",
                |m: &ClientState| { &m.height },
                |m: &mut ClientState| { &mut m.height },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ClientState>(
                "ClientState",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ClientState {
        static instance: ::protobuf::rt::LazyV2<ClientState> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ClientState::new)
    }
}

impl ::protobuf::Clear for ClientState {
    fn clear(&mut self) {
        self.chain_id.clear();
        self.height.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientState {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n-ibc/lightclients/localhost/v1/localhost.proto\x12\x1dibc.lightclients\
    .localhost.v1\x1a\x14gogoproto/gogo.proto\x1a\x1fibc/core/client/v1/clie\
    nt.proto\"}\n\x0bClientState\x12.\n\x08chain_id\x18\x01\x20\x01(\tR\x07c\
    hainIdB\x13\xf2\xde\x1f\x0fyaml:\"chain_id\"\x128\n\x06height\x18\x02\
    \x20\x01(\x0b2\x1a.ibc.core.client.v1.HeightR\x06heightB\x04\xc8\xde\x1f\
    \0:\x04\x88\xa0\x1f\0BEZCgithub.com/cosmos/cosmos-sdk/x/ibc/light-client\
    s/09-localhost/typesJ\xeb\x03\n\x06\x12\x04\0\0\x10\x01\n\x08\n\x01\x0c\
    \x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08%\n\t\n\x02\x03\0\x12\
    \x03\x03\x07\x1d\n\t\n\x02\x03\x01\x12\x03\x04\x07(\n\x08\n\x01\x08\x12\
    \x03\x06\0Z\n\t\n\x02\x08\x0b\x12\x03\x06\0Z\n\x83\x01\n\x02\x04\0\x12\
    \x04\n\0\x10\x01\x1aw\x20ClientState\x20defines\x20a\x20loopback\x20(loc\
    alhost)\x20client.\x20It\x20requires\x20(read-only)\n\x20access\x20to\
    \x20keys\x20outside\x20the\x20client\x20prefix.\n\n\n\n\x03\x04\0\x01\
    \x12\x03\n\x08\x13\n\n\n\x03\x04\0\x07\x12\x03\x0b\x02-\n\r\n\x06\x04\0\
    \x07\x81\xf4\x03\x12\x03\x0b\x02-\n\x1c\n\x04\x04\0\x02\0\x12\x03\r\x02C\
    \x1a\x0f\x20self\x20chain\x20ID\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\r\
    \x02\x0b-\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\r\x02\x08\n\x0c\n\x05\x04\
    \0\x02\0\x01\x12\x03\r\t\x11\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\r\x14\
    \x15\n\x0c\n\x05\x04\0\x02\0\x08\x12\x03\r\x16B\n\x0f\n\x08\x04\0\x02\0\
    \x08\xee\xfb\x03\x12\x03\r\x17A\n'\n\x04\x04\0\x02\x01\x12\x03\x0f\x02F\
    \x1a\x1a\x20self\x20latest\x20block\x20height\n\n\r\n\x05\x04\0\x02\x01\
    \x04\x12\x04\x0f\x02\rC\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\x0f\x02\
    \x1b\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0f\x1c\"\n\x0c\n\x05\x04\0\
    \x02\x01\x03\x12\x03\x0f%&\n\x0c\n\x05\x04\0\x02\x01\x08\x12\x03\x0f'E\n\
    \x0f\n\x08\x04\0\x02\x01\x08\xe9\xfb\x03\x12\x03\x0f(Db\x06proto3\
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
