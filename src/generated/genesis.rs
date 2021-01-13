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
//! Generated file from `ibc/core/types/v1/genesis.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_20_0;

#[derive(PartialEq,Clone,Default)]
pub struct GenesisState {
    // message fields
    pub client_genesis: ::protobuf::SingularPtrField<super::genesis::GenesisState>,
    pub connection_genesis: ::protobuf::SingularPtrField<super::genesis::GenesisState>,
    pub channel_genesis: ::protobuf::SingularPtrField<super::genesis::GenesisState>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GenesisState {
    fn default() -> &'a GenesisState {
        <GenesisState as ::protobuf::Message>::default_instance()
    }
}

impl GenesisState {
    pub fn new() -> GenesisState {
        ::std::default::Default::default()
    }

    // .ibc.core.client.v1.GenesisState client_genesis = 1;


    pub fn get_client_genesis(&self) -> &super::genesis::GenesisState {
        self.client_genesis.as_ref().unwrap_or_else(|| <super::genesis::GenesisState as ::protobuf::Message>::default_instance())
    }
    pub fn clear_client_genesis(&mut self) {
        self.client_genesis.clear();
    }

    pub fn has_client_genesis(&self) -> bool {
        self.client_genesis.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_genesis(&mut self, v: super::genesis::GenesisState) {
        self.client_genesis = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_genesis(&mut self) -> &mut super::genesis::GenesisState {
        if self.client_genesis.is_none() {
            self.client_genesis.set_default();
        }
        self.client_genesis.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_genesis(&mut self) -> super::genesis::GenesisState {
        self.client_genesis.take().unwrap_or_else(|| super::genesis::GenesisState::new())
    }

    // .ibc.core.connection.v1.GenesisState connection_genesis = 2;


    pub fn get_connection_genesis(&self) -> &super::genesis::GenesisState {
        self.connection_genesis.as_ref().unwrap_or_else(|| <super::genesis::GenesisState as ::protobuf::Message>::default_instance())
    }
    pub fn clear_connection_genesis(&mut self) {
        self.connection_genesis.clear();
    }

    pub fn has_connection_genesis(&self) -> bool {
        self.connection_genesis.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connection_genesis(&mut self, v: super::genesis::GenesisState) {
        self.connection_genesis = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_connection_genesis(&mut self) -> &mut super::genesis::GenesisState {
        if self.connection_genesis.is_none() {
            self.connection_genesis.set_default();
        }
        self.connection_genesis.as_mut().unwrap()
    }

    // Take field
    pub fn take_connection_genesis(&mut self) -> super::genesis::GenesisState {
        self.connection_genesis.take().unwrap_or_else(|| super::genesis::GenesisState::new())
    }

    // .ibc.core.channel.v1.GenesisState channel_genesis = 3;


    pub fn get_channel_genesis(&self) -> &super::genesis::GenesisState {
        self.channel_genesis.as_ref().unwrap_or_else(|| <super::genesis::GenesisState as ::protobuf::Message>::default_instance())
    }
    pub fn clear_channel_genesis(&mut self) {
        self.channel_genesis.clear();
    }

    pub fn has_channel_genesis(&self) -> bool {
        self.channel_genesis.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_genesis(&mut self, v: super::genesis::GenesisState) {
        self.channel_genesis = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel_genesis(&mut self) -> &mut super::genesis::GenesisState {
        if self.channel_genesis.is_none() {
            self.channel_genesis.set_default();
        }
        self.channel_genesis.as_mut().unwrap()
    }

    // Take field
    pub fn take_channel_genesis(&mut self) -> super::genesis::GenesisState {
        self.channel_genesis.take().unwrap_or_else(|| super::genesis::GenesisState::new())
    }
}

impl ::protobuf::Message for GenesisState {
    fn is_initialized(&self) -> bool {
        for v in &self.client_genesis {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.connection_genesis {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.channel_genesis {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.client_genesis)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.connection_genesis)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.channel_genesis)?;
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
        if let Some(ref v) = self.client_genesis.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.connection_genesis.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.channel_genesis.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.client_genesis.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.connection_genesis.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.channel_genesis.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> GenesisState {
        GenesisState::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::genesis::GenesisState>>(
                "client_genesis",
                |m: &GenesisState| { &m.client_genesis },
                |m: &mut GenesisState| { &mut m.client_genesis },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::genesis::GenesisState>>(
                "connection_genesis",
                |m: &GenesisState| { &m.connection_genesis },
                |m: &mut GenesisState| { &mut m.connection_genesis },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::genesis::GenesisState>>(
                "channel_genesis",
                |m: &GenesisState| { &m.channel_genesis },
                |m: &mut GenesisState| { &mut m.channel_genesis },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<GenesisState>(
                "GenesisState",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GenesisState {
        static instance: ::protobuf::rt::LazyV2<GenesisState> = ::protobuf::rt::LazyV2::INIT;
        instance.get(GenesisState::new)
    }
}

impl ::protobuf::Clear for GenesisState {
    fn clear(&mut self) {
        self.client_genesis.clear();
        self.connection_genesis.clear();
        self.channel_genesis.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GenesisState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GenesisState {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fibc/core/types/v1/genesis.proto\x12\x11ibc.core.types.v1\x1a\x14go\
    goproto/gogo.proto\x1a\x20ibc/core/client/v1/genesis.proto\x1a$ibc/core/\
    connection/v1/genesis.proto\x1a!ibc/core/channel/v1/genesis.proto\"\xda\
    \x02\n\x0cGenesisState\x12f\n\x0eclient_genesis\x18\x01\x20\x01(\x0b2\
    \x20.ibc.core.client.v1.GenesisStateR\rclientGenesisB\x1d\xf2\xde\x1f\
    \x15yaml:\"client_genesis\"\xc8\xde\x1f\0\x12v\n\x12connection_genesis\
    \x18\x02\x20\x01(\x0b2$.ibc.core.connection.v1.GenesisStateR\x11connecti\
    onGenesisB!\xf2\xde\x1f\x19yaml:\"connection_genesis\"\xc8\xde\x1f\0\x12\
    j\n\x0fchannel_genesis\x18\x03\x20\x01(\x0b2!.ibc.core.channel.v1.Genesi\
    sStateR\x0echannelGenesisB\x1e\xf2\xde\x1f\x16yaml:\"channel_genesis\"\
    \xc8\xde\x1f\0B/Z-github.com/cosmos/cosmos-sdk/x/ibc/core/typesJ\xfc\x04\
    \n\x06\x12\x04\0\0\x15\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\
    \x02\x12\x03\x01\x08\x19\n\x08\n\x01\x08\x12\x03\x03\0D\n\t\n\x02\x08\
    \x0b\x12\x03\x03\0D\n\t\n\x02\x03\0\x12\x03\x05\x07\x1d\n\t\n\x02\x03\
    \x01\x12\x03\x06\x07)\n\t\n\x02\x03\x02\x12\x03\x07\x07-\n\t\n\x02\x03\
    \x03\x12\x03\x08\x07*\nB\n\x02\x04\0\x12\x04\x0b\0\x15\x01\x1a6\x20Genes\
    isState\x20defines\x20the\x20ibc\x20module's\x20genesis\x20state.\n\n\n\
    \n\x03\x04\0\x01\x12\x03\x0b\x08\x14\n.\n\x04\x04\0\x02\0\x12\x04\r\x02\
    \x0eW\x1a\x20\x20ICS002\x20-\x20Clients\x20genesis\x20state\n\n\r\n\x05\
    \x04\0\x02\0\x04\x12\x04\r\x02\x0b\x16\n\x0c\n\x05\x04\0\x02\0\x06\x12\
    \x03\r\x02!\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\r\"0\n\x0c\n\x05\x04\0\
    \x02\0\x03\x12\x03\r34\n\x0c\n\x05\x04\0\x02\0\x08\x12\x03\x0e\x06V\n\
    \x0f\n\x08\x04\0\x02\0\x08\xe9\xfb\x03\x12\x03\x0e\x07#\n\x0f\n\x08\x04\
    \0\x02\0\x08\xee\xfb\x03\x12\x03\x0e%U\n2\n\x04\x04\0\x02\x01\x12\x04\
    \x10\x02\x11[\x1a$\x20ICS003\x20-\x20Connections\x20genesis\x20state\n\n\
    \r\n\x05\x04\0\x02\x01\x04\x12\x04\x10\x02\x0eW\n\x0c\n\x05\x04\0\x02\
    \x01\x06\x12\x03\x10\x02%\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x10&8\n\
    \x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x10;<\n\x0c\n\x05\x04\0\x02\x01\x08\
    \x12\x03\x11\x06Z\n\x0f\n\x08\x04\0\x02\x01\x08\xe9\xfb\x03\x12\x03\x11\
    \x07#\n\x0f\n\x08\x04\0\x02\x01\x08\xee\xfb\x03\x12\x03\x11%Y\n.\n\x04\
    \x04\0\x02\x02\x12\x04\x13\x02\x14X\x1a\x20\x20ICS004\x20-\x20Channel\
    \x20genesis\x20state\n\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x13\x02\x11[\
    \n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03\x13\x02\"\n\x0c\n\x05\x04\0\x02\
    \x02\x01\x12\x03\x13#2\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x1356\n\x0c\
    \n\x05\x04\0\x02\x02\x08\x12\x03\x14\x06W\n\x0f\n\x08\x04\0\x02\x02\x08\
    \xe9\xfb\x03\x12\x03\x14\x07#\n\x0f\n\x08\x04\0\x02\x02\x08\xee\xfb\x03\
    \x12\x03\x14%Vb\x06proto3\
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
