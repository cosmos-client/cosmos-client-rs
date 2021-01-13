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
//! Generated file from `ibc/core/commitment/v1/commitment.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_20_0;

#[derive(PartialEq,Clone,Default)]
pub struct MerkleRoot {
    // message fields
    pub hash: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MerkleRoot {
    fn default() -> &'a MerkleRoot {
        <MerkleRoot as ::protobuf::Message>::default_instance()
    }
}

impl MerkleRoot {
    pub fn new() -> MerkleRoot {
        ::std::default::Default::default()
    }

    // bytes hash = 1;


    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }
    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for MerkleRoot {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
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
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.hash.is_empty() {
            os.write_bytes(1, &self.hash)?;
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

    fn new() -> MerkleRoot {
        MerkleRoot::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "hash",
                |m: &MerkleRoot| { &m.hash },
                |m: &mut MerkleRoot| { &mut m.hash },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MerkleRoot>(
                "MerkleRoot",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MerkleRoot {
        static instance: ::protobuf::rt::LazyV2<MerkleRoot> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MerkleRoot::new)
    }
}

impl ::protobuf::Clear for MerkleRoot {
    fn clear(&mut self) {
        self.hash.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MerkleRoot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MerkleRoot {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MerklePrefix {
    // message fields
    pub key_prefix: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MerklePrefix {
    fn default() -> &'a MerklePrefix {
        <MerklePrefix as ::protobuf::Message>::default_instance()
    }
}

impl MerklePrefix {
    pub fn new() -> MerklePrefix {
        ::std::default::Default::default()
    }

    // bytes key_prefix = 1;


    pub fn get_key_prefix(&self) -> &[u8] {
        &self.key_prefix
    }
    pub fn clear_key_prefix(&mut self) {
        self.key_prefix.clear();
    }

    // Param is passed by value, moved
    pub fn set_key_prefix(&mut self, v: ::std::vec::Vec<u8>) {
        self.key_prefix = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key_prefix(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key_prefix
    }

    // Take field
    pub fn take_key_prefix(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key_prefix, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for MerklePrefix {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key_prefix)?;
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
        if !self.key_prefix.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key_prefix);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.key_prefix.is_empty() {
            os.write_bytes(1, &self.key_prefix)?;
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

    fn new() -> MerklePrefix {
        MerklePrefix::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "key_prefix",
                |m: &MerklePrefix| { &m.key_prefix },
                |m: &mut MerklePrefix| { &mut m.key_prefix },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MerklePrefix>(
                "MerklePrefix",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MerklePrefix {
        static instance: ::protobuf::rt::LazyV2<MerklePrefix> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MerklePrefix::new)
    }
}

impl ::protobuf::Clear for MerklePrefix {
    fn clear(&mut self) {
        self.key_prefix.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MerklePrefix {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MerklePrefix {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MerklePath {
    // message fields
    pub key_path: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MerklePath {
    fn default() -> &'a MerklePath {
        <MerklePath as ::protobuf::Message>::default_instance()
    }
}

impl MerklePath {
    pub fn new() -> MerklePath {
        ::std::default::Default::default()
    }

    // repeated string key_path = 1;


    pub fn get_key_path(&self) -> &[::std::string::String] {
        &self.key_path
    }
    pub fn clear_key_path(&mut self) {
        self.key_path.clear();
    }

    // Param is passed by value, moved
    pub fn set_key_path(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.key_path = v;
    }

    // Mutable pointer to the field.
    pub fn mut_key_path(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.key_path
    }

    // Take field
    pub fn take_key_path(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.key_path, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for MerklePath {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.key_path)?;
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
        for value in &self.key_path {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.key_path {
            os.write_string(1, &v)?;
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

    fn new() -> MerklePath {
        MerklePath::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "key_path",
                |m: &MerklePath| { &m.key_path },
                |m: &mut MerklePath| { &mut m.key_path },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MerklePath>(
                "MerklePath",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MerklePath {
        static instance: ::protobuf::rt::LazyV2<MerklePath> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MerklePath::new)
    }
}

impl ::protobuf::Clear for MerklePath {
    fn clear(&mut self) {
        self.key_path.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MerklePath {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MerklePath {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MerkleProof {
    // message fields
    pub proofs: ::protobuf::RepeatedField<super::proofs::CommitmentProof>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MerkleProof {
    fn default() -> &'a MerkleProof {
        <MerkleProof as ::protobuf::Message>::default_instance()
    }
}

impl MerkleProof {
    pub fn new() -> MerkleProof {
        ::std::default::Default::default()
    }

    // repeated .ics23.CommitmentProof proofs = 1;


    pub fn get_proofs(&self) -> &[super::proofs::CommitmentProof] {
        &self.proofs
    }
    pub fn clear_proofs(&mut self) {
        self.proofs.clear();
    }

    // Param is passed by value, moved
    pub fn set_proofs(&mut self, v: ::protobuf::RepeatedField<super::proofs::CommitmentProof>) {
        self.proofs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_proofs(&mut self) -> &mut ::protobuf::RepeatedField<super::proofs::CommitmentProof> {
        &mut self.proofs
    }

    // Take field
    pub fn take_proofs(&mut self) -> ::protobuf::RepeatedField<super::proofs::CommitmentProof> {
        ::std::mem::replace(&mut self.proofs, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for MerkleProof {
    fn is_initialized(&self) -> bool {
        for v in &self.proofs {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.proofs)?;
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
        for value in &self.proofs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.proofs {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn new() -> MerkleProof {
        MerkleProof::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::proofs::CommitmentProof>>(
                "proofs",
                |m: &MerkleProof| { &m.proofs },
                |m: &mut MerkleProof| { &mut m.proofs },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MerkleProof>(
                "MerkleProof",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MerkleProof {
        static instance: ::protobuf::rt::LazyV2<MerkleProof> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MerkleProof::new)
    }
}

impl ::protobuf::Clear for MerkleProof {
    fn clear(&mut self) {
        self.proofs.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MerkleProof {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MerkleProof {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'ibc/core/commitment/v1/commitment.proto\x12\x16ibc.core.commitment.v1\
    \x1a\x14gogoproto/gogo.proto\x1a\x13confio/proofs.proto\"&\n\nMerkleRoot\
    \x12\x12\n\x04hash\x18\x01\x20\x01(\x0cR\x04hash:\x04\x88\xa0\x1f\0\"D\n\
    \x0cMerklePrefix\x124\n\nkey_prefix\x18\x01\x20\x01(\x0cR\tkeyPrefixB\
    \x15\xf2\xde\x1f\x11yaml:\"key_prefix\"\"B\n\nMerklePath\x12.\n\x08key_p\
    ath\x18\x01\x20\x03(\tR\x07keyPathB\x13\xf2\xde\x1f\x0fyaml:\"key_path\"\
    :\x04\x98\xa0\x1f\0\"=\n\x0bMerkleProof\x12.\n\x06proofs\x18\x01\x20\x03\
    (\x0b2\x16.ics23.CommitmentProofR\x06proofsB=Z;github.com/cosmos/cosmos-\
    sdk/x/ibc/core/23-commitment/typesJ\x91\n\n\x06\x12\x04\0\0'\x01\n\x08\n\
    \x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\x1e\n\x08\n\x01\
    \x08\x12\x03\x03\0R\n\t\n\x02\x08\x0b\x12\x03\x03\0R\n\t\n\x02\x03\0\x12\
    \x03\x05\x07\x1d\n\t\n\x02\x03\x01\x12\x03\x06\x07\x1c\nx\n\x02\x04\0\
    \x12\x04\n\0\x0e\x01\x1al\x20MerkleRoot\x20defines\x20a\x20merkle\x20roo\
    t\x20hash.\n\x20In\x20the\x20Cosmos\x20SDK,\x20the\x20AppHash\x20of\x20a\
    \x20block\x20header\x20becomes\x20the\x20root.\n\n\n\n\x03\x04\0\x01\x12\
    \x03\n\x08\x12\n\n\n\x03\x04\0\x07\x12\x03\x0b\x02-\n\r\n\x06\x04\0\x07\
    \x81\xf4\x03\x12\x03\x0b\x02-\n\x0b\n\x04\x04\0\x02\0\x12\x03\r\x02\x11\
    \n\r\n\x05\x04\0\x02\0\x04\x12\x04\r\x02\x0b-\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\r\x02\x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\r\x08\x0c\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\r\x0f\x10\n\xac\x01\n\x02\x04\x01\x12\
    \x04\x13\0\x15\x01\x1a\x9f\x01\x20MerklePrefix\x20is\x20merkle\x20path\
    \x20prefixed\x20to\x20the\x20key.\n\x20The\x20constructed\x20key\x20from\
    \x20the\x20Path\x20and\x20the\x20key\x20will\x20be\x20append(Path.KeyPat\
    h,\n\x20append(Path.KeyPrefix,\x20key...))\n\n\n\n\x03\x04\x01\x01\x12\
    \x03\x13\x08\x14\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x14\x02F\n\r\n\x05\
    \x04\x01\x02\0\x04\x12\x04\x14\x02\x13\x16\n\x0c\n\x05\x04\x01\x02\0\x05\
    \x12\x03\x14\x02\x07\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x14\x08\x12\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x14\x15\x16\n\x0c\n\x05\x04\x01\x02\
    \0\x08\x12\x03\x14\x17E\n\x0f\n\x08\x04\x01\x02\0\x08\xee\xfb\x03\x12\
    \x03\x14\x18D\n\xc1\x01\n\x02\x04\x02\x12\x04\x1a\0\x1e\x01\x1a\xb4\x01\
    \x20MerklePath\x20is\x20the\x20path\x20used\x20to\x20verify\x20commitmen\
    t\x20proofs,\x20which\x20can\x20be\x20an\n\x20arbitrary\x20structured\
    \x20object\x20(defined\x20by\x20a\x20commitment\x20type).\n\x20MerklePat\
    h\x20is\x20represented\x20from\x20root-to-leaf\n\n\n\n\x03\x04\x02\x01\
    \x12\x03\x1a\x08\x12\n\n\n\x03\x04\x02\x07\x12\x03\x1b\x02.\n\r\n\x06\
    \x04\x02\x07\x83\xf4\x03\x12\x03\x1b\x02.\n\x0b\n\x04\x04\x02\x02\0\x12\
    \x03\x1d\x02L\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03\x1d\x02\n\n\x0c\n\
    \x05\x04\x02\x02\0\x05\x12\x03\x1d\x0b\x11\n\x0c\n\x05\x04\x02\x02\0\x01\
    \x12\x03\x1d\x12\x1a\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x1d\x1d\x1e\n\
    \x0c\n\x05\x04\x02\x02\0\x08\x12\x03\x1d\x1fK\n\x0f\n\x08\x04\x02\x02\0\
    \x08\xee\xfb\x03\x12\x03\x1d\x20J\n\xa0\x02\n\x02\x04\x03\x12\x04%\0'\
    \x01\x1a\x93\x02\x20MerkleProof\x20is\x20a\x20wrapper\x20type\x20over\
    \x20a\x20chain\x20of\x20CommitmentProofs.\n\x20It\x20demonstrates\x20mem\
    bership\x20or\x20non-membership\x20for\x20an\x20element\x20or\x20set\x20\
    of\n\x20elements,\x20verifiable\x20in\x20conjunction\x20with\x20a\x20kno\
    wn\x20commitment\x20root.\x20Proofs\n\x20should\x20be\x20succinct.\n\x20\
    MerkleProofs\x20are\x20ordered\x20from\x20leaf-to-root\n\n\n\n\x03\x04\
    \x03\x01\x12\x03%\x08\x13\n\x0b\n\x04\x04\x03\x02\0\x12\x03&\x02,\n\x0c\
    \n\x05\x04\x03\x02\0\x04\x12\x03&\x02\n\n\x0c\n\x05\x04\x03\x02\0\x06\
    \x12\x03&\x0b\x20\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03&!'\n\x0c\n\x05\
    \x04\x03\x02\0\x03\x12\x03&*+b\x06proto3\
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
