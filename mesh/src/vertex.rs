//! Built-in vertex formats.

use std::{borrow::Cow, fmt::Debug};

/// Vertex attribute type.
pub type Attribute = gfx_hal::pso::Element<gfx_hal::format::Format>;

/// Trait for vertex attributes to implement
pub trait AsAttribute: Debug + PartialEq + PartialOrd + Copy + Send + Sync + 'static {
    /// Name of the attribute
    const NAME: &'static str;

    /// Size of the attribute.
    const SIZE: u32;

    /// Attribute format.
    const FORMAT: gfx_hal::format::Format;
}

/// Type for position attribute of vertex.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Position(pub [f32; 3]);
impl<T> From<T> for Position
where
    T: Into<[f32; 3]>,
{
    fn from(from: T) -> Self {
        Position(from.into())
    }
}
impl AsAttribute for Position {
    const NAME: &'static str = "position";
    const SIZE: u32 = 12;
    const FORMAT: gfx_hal::format::Format = gfx_hal::format::Format::Rgb32Sfloat;
}

/// Type for color attribute of vertex
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color(pub [f32; 4]);
impl<T> From<T> for Color
where
    T: Into<[f32; 4]>,
{
    fn from(from: T) -> Self {
        Color(from.into())
    }
}
impl AsAttribute for Color {
    const NAME: &'static str = "color";
    const SIZE: u32 = 16;
    const FORMAT: gfx_hal::format::Format = gfx_hal::format::Format::Rgba32Sfloat;
}

/// Type for texture coord attribute of vertex
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Normal(pub [f32; 3]);
impl<T> From<T> for Normal
where
    T: Into<[f32; 3]>,
{
    fn from(from: T) -> Self {
        Normal(from.into())
    }
}

impl AsAttribute for Normal {
    const NAME: &'static str = "normal";
    const SIZE: u32 = 12;
    const FORMAT: gfx_hal::format::Format = gfx_hal::format::Format::Rgb32Sfloat;
}

/// Type for tangent attribute of vertex
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tangent(pub [f32; 3]);
impl<T> From<T> for Tangent
where
    T: Into<[f32; 3]>,
{
    fn from(from: T) -> Self {
        Tangent(from.into())
    }
}

impl AsAttribute for Tangent {
    const NAME: &'static str = "tangent";
    const SIZE: u32 = 12;
    const FORMAT: gfx_hal::format::Format = gfx_hal::format::Format::Rgb32Sfloat;
}

/// Type for texture coord attribute of vertex
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TexCoord(pub [f32; 2]);
impl<T> From<T> for TexCoord
where
    T: Into<[f32; 2]>,
{
    fn from(from: T) -> Self {
        TexCoord(from.into())
    }
}

impl AsAttribute for TexCoord {
    const NAME: &'static str = "tex_coord";
    const SIZE: u32 = 8;
    const FORMAT: gfx_hal::format::Format = gfx_hal::format::Format::Rg32Sfloat;
}

/// Vertex format contains information to initialize graphics pipeline
/// Attributes must be sorted by offset.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VertexFormat<'a> {
    /// Attributes for format.
    pub attributes: Cow<'a, [Attribute]>,

    /// Size of single vertex.
    pub stride: u32,
}

impl<'a> VertexFormat<'a> {
    /// Convert into gfx digestible type.
    pub fn gfx_vertex_input_desc(
        &self,
        rate: gfx_hal::pso::VertexInputRate,
    ) -> (
        Vec<gfx_hal::pso::Element<gfx_hal::format::Format>>,
        gfx_hal::pso::ElemStride,
        gfx_hal::pso::VertexInputRate,
    ) {
        (self.attributes.clone().into_owned(), self.stride, rate)
    }
}

/// Trait implemented by all valid vertex formats.
pub trait AsVertex: Debug + PartialEq + PartialOrd + Copy + Sized + Send + Sync + 'static {
    /// List of all attributes formats with name and offset.
    const VERTEX: VertexFormat<'static>;

    /// Returns attribute of vertex by type
    #[inline]
    fn attribute<F>() -> Attribute
    where
        F: AsAttribute,
        Self: WithAttribute<F>,
    {
        <Self as WithAttribute<F>>::ATTRIBUTE
    }
}

impl<T> AsVertex for T
where
    T: AsAttribute,
{
    const VERTEX: VertexFormat<'static> = VertexFormat {
        attributes: Cow::Borrowed(&[Attribute {
            format: T::FORMAT,
            offset: 0,
        }]),
        stride: T::SIZE,
    };
}

/// Trait implemented by all valid vertex formats for each field
pub trait WithAttribute<F: AsAttribute>: AsVertex {
    /// Individual format of the attribute for this vertex format
    const ATTRIBUTE: Attribute;
}

impl<T> WithAttribute<T> for T
where
    T: AsAttribute,
{
    const ATTRIBUTE: Attribute = Attribute {
        format: T::FORMAT,
        offset: 0,
    };
}

/// Vertex format with position and RGBA8 color attributes.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PosColor {
    /// Position of the vertex in 3D space.
    pub position: Position,
    /// RGBA color value of the vertex.
    pub color: Color,
}

impl AsVertex for PosColor {
    const VERTEX: VertexFormat<'static> = VertexFormat {
        attributes: Cow::Borrowed(&[
            <Self as WithAttribute<Position>>::ATTRIBUTE,
            <Self as WithAttribute<Color>>::ATTRIBUTE,
        ]),
        stride: Position::SIZE + Color::SIZE,
    };
}

impl WithAttribute<Position> for PosColor {
    const ATTRIBUTE: Attribute = Attribute {
        offset: 0,
        format: Position::FORMAT,
    };
}

impl WithAttribute<Color> for PosColor {
    const ATTRIBUTE: Attribute = Attribute {
        offset: Position::SIZE,
        format: Color::FORMAT,
    };
}

/// Vertex format with position and normal attributes.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PosNorm {
    /// Position of the vertex in 3D space.
    pub position: Position,
    /// Normal vector of the vertex.
    pub normal: Normal,
}

impl AsVertex for PosNorm {
    const VERTEX: VertexFormat<'static> = VertexFormat {
        attributes: Cow::Borrowed(&[
            <Self as WithAttribute<Position>>::ATTRIBUTE,
            <Self as WithAttribute<Normal>>::ATTRIBUTE,
        ]),
        stride: Position::SIZE + Normal::SIZE,
    };
}

impl WithAttribute<Position> for PosNorm {
    const ATTRIBUTE: Attribute = Attribute {
        offset: 0,
        format: Position::FORMAT,
    };
}

impl WithAttribute<Normal> for PosNorm {
    const ATTRIBUTE: Attribute = Attribute {
        offset: Position::SIZE,
        format: Normal::FORMAT,
    };
}

/// Vertex format with position, color and normal attributes.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PosColorNorm {
    /// Position of the vertex in 3D space.
    pub position: Position,
    /// RGBA color value of the vertex.
    pub color: Color,
    /// Normal vector of the vertex.
    pub normal: Normal,
}

impl AsVertex for PosColorNorm {
    const VERTEX: VertexFormat<'static> = VertexFormat {
        attributes: Cow::Borrowed(&[
            <Self as WithAttribute<Position>>::ATTRIBUTE,
            <Self as WithAttribute<Color>>::ATTRIBUTE,
            <Self as WithAttribute<Normal>>::ATTRIBUTE,
        ]),
        stride: Position::SIZE + Color::SIZE + Normal::SIZE,
    };
}

impl WithAttribute<Position> for PosColorNorm {
    const ATTRIBUTE: Attribute = Attribute {
        offset: 0,
        format: Position::FORMAT,
    };
}

impl WithAttribute<Color> for PosColorNorm {
    const ATTRIBUTE: Attribute = Attribute {
        offset: Position::SIZE,
        format: Color::FORMAT,
    };
}

impl WithAttribute<Normal> for PosColorNorm {
    const ATTRIBUTE: Attribute = Attribute {
        offset: Position::SIZE + Color::SIZE,
        format: Normal::FORMAT,
    };
}

/// Vertex format with position and UV texture coordinate attributes.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PosTex {
    /// Position of the vertex in 3D space.
    pub position: Position,
    /// UV texture coordinates used by the vertex.
    pub tex_coord: TexCoord,
}

impl AsVertex for PosTex {
    const VERTEX: VertexFormat<'static> = VertexFormat {
        attributes: Cow::Borrowed(&[
            <Self as WithAttribute<Position>>::ATTRIBUTE,
            <Self as WithAttribute<TexCoord>>::ATTRIBUTE,
        ]),
        stride: Position::SIZE + TexCoord::SIZE,
    };
}

impl WithAttribute<Position> for PosTex {
    const ATTRIBUTE: Attribute = Attribute {
        offset: 0,
        format: Position::FORMAT,
    };
}

impl WithAttribute<TexCoord> for PosTex {
    const ATTRIBUTE: Attribute = Attribute {
        offset: Position::SIZE,
        format: TexCoord::FORMAT,
    };
}

/// Vertex format with position, normal and UV texture coordinate attributes.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PosNormTex {
    /// Position of the vertex in 3D space.
    pub position: Position,
    /// Normal vector of the vertex.
    pub normal: Normal,
    /// UV texture coordinates used by the vertex.
    pub tex_coord: TexCoord,
}

impl AsVertex for PosNormTex {
    const VERTEX: VertexFormat<'static> = VertexFormat {
        attributes: Cow::Borrowed(&[
            <Self as WithAttribute<Position>>::ATTRIBUTE,
            <Self as WithAttribute<Normal>>::ATTRIBUTE,
            <Self as WithAttribute<TexCoord>>::ATTRIBUTE,
        ]),
        stride: Position::SIZE + Normal::SIZE + TexCoord::SIZE,
    };
}

impl WithAttribute<Position> for PosNormTex {
    const ATTRIBUTE: Attribute = Attribute {
        offset: 0,
        format: Position::FORMAT,
    };
}

impl WithAttribute<Normal> for PosNormTex {
    const ATTRIBUTE: Attribute = Attribute {
        offset: Position::SIZE,
        format: Normal::FORMAT,
    };
}

impl WithAttribute<TexCoord> for PosNormTex {
    const ATTRIBUTE: Attribute = Attribute {
        offset: Position::SIZE + Normal::SIZE,
        format: TexCoord::FORMAT,
    };
}

/// Vertex format with position, normal, tangent, and UV texture coordinate attributes.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PosNormTangTex {
    /// Position of the vertex in 3D space.
    pub position: Position,
    /// Normal vector of the vertex.
    pub normal: Normal,
    /// Tangent vector of the vertex.
    pub tangent: Tangent,
    /// UV texture coordinates used by the vertex.
    pub tex_coord: TexCoord,
}

impl AsVertex for PosNormTangTex {
    const VERTEX: VertexFormat<'static> = VertexFormat {
        attributes: Cow::Borrowed(&[
            <Self as WithAttribute<Position>>::ATTRIBUTE,
            <Self as WithAttribute<Normal>>::ATTRIBUTE,
            <Self as WithAttribute<Tangent>>::ATTRIBUTE,
            <Self as WithAttribute<TexCoord>>::ATTRIBUTE,
        ]),
        stride: Position::SIZE + Normal::SIZE + Tangent::SIZE + TexCoord::SIZE,
    };
}

impl WithAttribute<Position> for PosNormTangTex {
    const ATTRIBUTE: Attribute = Attribute {
        offset: 0,
        format: Position::FORMAT,
    };
}

impl WithAttribute<Normal> for PosNormTangTex {
    const ATTRIBUTE: Attribute = Attribute {
        offset: Position::SIZE,
        format: Normal::FORMAT,
    };
}

impl WithAttribute<Tangent> for PosNormTangTex {
    const ATTRIBUTE: Attribute = Attribute {
        offset: Position::SIZE + Normal::SIZE,
        format: Tangent::FORMAT,
    };
}

impl WithAttribute<TexCoord> for PosNormTangTex {
    const ATTRIBUTE: Attribute = Attribute {
        offset: Position::SIZE + Normal::SIZE + Tangent::SIZE,
        format: TexCoord::FORMAT,
    };
}

/// Full vertex transformation attribute.
/// Typically provided on per-instance basis.
/// It takes 4 attribute locations.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Transform(pub [[f32; 4]; 4]);
impl<T> From<T> for Transform
where
    T: Into<[[f32; 4]; 4]>,
{
    fn from(from: T) -> Self {
        Transform(from.into())
    }
}

/// It should be `AsAttribute` with multiple locations occupied.
/// But rust doesn't allow constructing static slices with generic size.
impl AsVertex for Transform {
    const VERTEX: VertexFormat<'static> = VertexFormat {
        attributes: Cow::Borrowed(&[
            Attribute {
                format: gfx_hal::format::Format::Rgba32Sfloat,
                offset: 0,
            },
            Attribute {
                format: gfx_hal::format::Format::Rgba32Sfloat,
                offset: 16,
            },
            Attribute {
                format: gfx_hal::format::Format::Rgba32Sfloat,
                offset: 32,
            },
            Attribute {
                format: gfx_hal::format::Format::Rgba32Sfloat,
                offset: 48,
            },
        ]),
        stride: 64,
    };
}

/// Allows to query specific `Attribute`s of `AsVertex`
pub trait Query<T>: AsVertex {
    /// Attributes from tuple `T`
    const QUERIED_ATTRIBUTES: &'static [(&'static str, Attribute)];
}

macro_rules! impl_query {
    ($($a:ident),*) => {
        impl<VF $(,$a)*> Query<($($a,)*)> for VF
            where VF: AsVertex,
            $(
                $a: AsAttribute,
                VF: WithAttribute<$a>,
            )*
        {
            const QUERIED_ATTRIBUTES: &'static [(&'static str, Attribute)] = &[
                $(
                    ($a::NAME, <VF as WithAttribute<$a>>::ATTRIBUTE),
                )*
            ];
        }

        impl_query!(@ $($a),*);
    };
    (@) => {};
    (@ $head:ident $(,$tail:ident)*) => {
        impl_query!($($tail),*);
    };
}

impl_query!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
