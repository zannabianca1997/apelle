use std::marker::PhantomData;

use serde::{
    Deserialize, Serialize,
    de::{self, IntoDeserializer, Unexpected, value::*},
    forward_to_deserialize_any,
};

/// Deserialize false as special value
/// false -> None
/// _ -> Some(T::deserialize(_))
#[derive(Debug)]
pub enum OrFalse<T> {
    Some(T),
    False,
}
impl<T> OrFalse<T> {
    pub fn into_option(self) -> Option<T> {
        match self {
            OrFalse::Some(t) => Some(t),
            OrFalse::False => None,
        }
    }
}
impl<T> Serialize for OrFalse<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            OrFalse::Some(t) => t.serialize(serializer),
            OrFalse::False => serializer.serialize_str("none"),
        }
    }
}

impl<'de, T> Deserialize<'de> for OrFalse<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(OrFalseVisitor(std::marker::PhantomData))
    }
}
struct OrFalseVisitor<T>(std::marker::PhantomData<T>);

impl<'de, T> serde::de::Visitor<'de> for OrFalseVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = OrFalse<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("false or a value")
    }

    fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>,
    {
        Ok(OrFalse::Some(T::deserialize(MapAccessDeserializer::new(
            map,
        ))?))
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(if v {
            OrFalse::Some(T::deserialize(BoolDeserializer::new(true))?)
        } else {
            OrFalse::False
        })
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(I8Deserializer::new(v))?))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(I16Deserializer::new(v))?))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(I32Deserializer::new(v))?))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(I64Deserializer::new(v))?))
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(I128Deserializer::new(v))?))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(U8Deserializer::new(v))?))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(U16Deserializer::new(v))?))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(U32Deserializer::new(v))?))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(U64Deserializer::new(v))?))
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(U128Deserializer::new(v))?))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(F32Deserializer::new(v))?))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(F64Deserializer::new(v))?))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(CharDeserializer::new(v))?))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(StrDeserializer::new(v))?))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(
            BorrowedStrDeserializer::new(v),
        )?))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(StringDeserializer::new(v))?))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(BytesDeserializer::new(v))?))
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(
            BorrowedBytesDeserializer::new(v),
        )?))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(ByteBufDeserializer::new(v))?))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(NoneDeserializer::new())?))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(OrFalse::Some(T::deserialize(SomeDeserializer::new(
            deserializer,
        ))?))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OrFalse::Some(T::deserialize(UnitDeserializer::new())?))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(OrFalse::Some(T::deserialize(
            NewtypeStructDeserializer::new(deserializer),
        )?))
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        Ok(OrFalse::Some(T::deserialize(SeqAccessDeserializer::new(
            seq,
        ))?))
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        Ok(OrFalse::Some(T::deserialize(EnumAccessDeserializer::new(
            data,
        ))?))
    }
}

pub struct ByteBufDeserializer<E> {
    value: Vec<u8>,
    marker: PhantomData<E>,
}

impl<E> Clone for ByteBufDeserializer<E> {
    fn clone(&self) -> Self {
        ByteBufDeserializer {
            value: self.value.clone(),
            marker: PhantomData,
        }
    }
}

impl<E> ByteBufDeserializer<E> {
    #[allow(missing_docs)]
    pub fn new(value: Vec<u8>) -> Self {
        ByteBufDeserializer {
            value,
            marker: PhantomData,
        }
    }
}

impl<'de, E> de::Deserializer<'de> for ByteBufDeserializer<E>
where
    E: de::Error,
{
    type Error = E;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_byte_buf(self.value)
    }

    fn deserialize_enum<V>(
        self,
        name: &str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let _ = name;
        let _ = variants;
        visitor.visit_enum(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}
impl<'de, E> de::EnumAccess<'de> for ByteBufDeserializer<E>
where
    E: de::Error,
{
    type Error = E;
    type Variant = UnitOnly<E>;

    fn variant_seed<T>(self, seed: T) -> Result<(T::Value, Self::Variant), Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self).map(unit_only)
    }
}

impl<'de, E> IntoDeserializer<'de, E> for ByteBufDeserializer<E>
where
    E: de::Error,
{
    type Deserializer = Self;

    fn into_deserializer(self) -> Self {
        self
    }
}

pub struct NoneDeserializer<E> {
    marker: PhantomData<E>,
}

impl<E> Clone for NoneDeserializer<E> {
    fn clone(&self) -> Self {
        NoneDeserializer {
            marker: PhantomData,
        }
    }
}

impl<E> NoneDeserializer<E> {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        NoneDeserializer {
            marker: PhantomData,
        }
    }
}

impl<'de, E> de::Deserializer<'de> for NoneDeserializer<E>
where
    E: de::Error,
{
    type Error = E;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_none()
    }

    fn deserialize_enum<V>(
        self,
        name: &str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let _ = name;
        let _ = variants;
        visitor.visit_enum(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}
impl<'de, E> de::EnumAccess<'de> for NoneDeserializer<E>
where
    E: de::Error,
{
    type Error = E;
    type Variant = UnitOnly<E>;

    fn variant_seed<T>(self, seed: T) -> Result<(T::Value, Self::Variant), Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self).map(unit_only)
    }
}

impl<'de, E> IntoDeserializer<'de, E> for NoneDeserializer<E>
where
    E: de::Error,
{
    type Deserializer = Self;

    fn into_deserializer(self) -> Self {
        self
    }
}

pub struct SomeDeserializer<E, D> {
    deserializer: D,
    marker: PhantomData<E>,
}

impl<E, D> Clone for SomeDeserializer<E, D>
where
    D: Clone,
{
    fn clone(&self) -> Self {
        SomeDeserializer {
            deserializer: self.deserializer.clone(),
            marker: PhantomData,
        }
    }
}

impl<E, D> SomeDeserializer<E, D> {
    #[allow(missing_docs)]
    pub fn new(deserializer: D) -> Self {
        SomeDeserializer {
            deserializer,
            marker: PhantomData,
        }
    }
}

impl<'de, E, D> de::Deserializer<'de> for SomeDeserializer<E, D>
where
    E: de::Error,
    D: de::Deserializer<'de, Error = E>,
{
    type Error = E;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_some(self.deserializer)
    }

    fn deserialize_enum<V>(
        self,
        name: &str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let _ = name;
        let _ = variants;
        visitor.visit_enum(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}
impl<'de, E, D> de::EnumAccess<'de> for SomeDeserializer<E, D>
where
    E: de::Error,
    D: de::Deserializer<'de, Error = E>,
{
    type Error = E;
    type Variant = UnitOnly<E>;

    fn variant_seed<T>(self, seed: T) -> Result<(T::Value, Self::Variant), Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self).map(unit_only)
    }
}

impl<'de, E, D> IntoDeserializer<'de, E> for SomeDeserializer<E, D>
where
    E: de::Error,
    D: de::Deserializer<'de, Error = E>,
{
    type Deserializer = Self;

    fn into_deserializer(self) -> Self {
        self
    }
}

pub struct NewtypeStructDeserializer<E, D> {
    deserializer: D,
    marker: PhantomData<E>,
}

impl<E, D> Clone for NewtypeStructDeserializer<E, D>
where
    D: Clone,
{
    fn clone(&self) -> Self {
        NewtypeStructDeserializer {
            deserializer: self.deserializer.clone(),
            marker: PhantomData,
        }
    }
}

impl<E, D> NewtypeStructDeserializer<E, D> {
    #[allow(missing_docs)]
    pub fn new(deserializer: D) -> Self {
        NewtypeStructDeserializer {
            deserializer,
            marker: PhantomData,
        }
    }
}

impl<'de, E, D> de::Deserializer<'de> for NewtypeStructDeserializer<E, D>
where
    E: de::Error,
    D: de::Deserializer<'de, Error = E>,
{
    type Error = E;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self.deserializer)
    }

    fn deserialize_enum<V>(
        self,
        name: &str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let _ = name;
        let _ = variants;
        visitor.visit_enum(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}
impl<'de, E, D> de::EnumAccess<'de> for NewtypeStructDeserializer<E, D>
where
    E: de::Error,
    D: de::Deserializer<'de, Error = E>,
{
    type Error = E;
    type Variant = UnitOnly<E>;

    fn variant_seed<T>(self, seed: T) -> Result<(T::Value, Self::Variant), Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self).map(unit_only)
    }
}

impl<'de, E, D> IntoDeserializer<'de, E> for NewtypeStructDeserializer<E, D>
where
    E: de::Error,
    D: de::Deserializer<'de, Error = E>,
{
    type Deserializer = Self;

    fn into_deserializer(self) -> Self {
        self
    }
}

pub struct UnitOnly<E> {
    marker: PhantomData<E>,
}

pub fn unit_only<T, E>(t: T) -> (T, UnitOnly<E>) {
    (
        t,
        UnitOnly {
            marker: PhantomData,
        },
    )
}

impl<'de, E> de::VariantAccess<'de> for UnitOnly<E>
where
    E: de::Error,
{
    type Error = E;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        Err(de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"newtype variant",
        ))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"tuple variant",
        ))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"struct variant",
        ))
    }
}
