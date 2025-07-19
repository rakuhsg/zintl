use paste::paste;

macro_rules! define_unit {
    ($unit_name:ident, $type_name:ident, $zero_value:expr, $one_value:expr) => {
        #[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $unit_name($type_name);

        impl $unit_name {
            const ZERO: $unit_name = $unit_name($zero_value);

            pub fn new(value: $type_name) -> Self {
                $unit_name(value)
            }

            #[inline]
            pub fn zero() -> Self {
                $unit_name($zero_value)
            }

            #[inline]
            pub fn is_zero(self) -> bool {
                self.0 == Self::ZERO.0
            }

            #[inline]
            pub fn value(self) -> $type_name {
                self.0
            }

            #[inline]
            pub fn checked_div(self, other: Self) -> Option<Self> {
                if other == Self::ZERO {
                    None
                } else {
                    Some($unit_name(self.0 / other.0))
                }
            }

            #[inline]
            pub fn checked_div_value(self, other: $type_name) -> Option<Self> {
                if other == $zero_value {
                    None
                } else {
                    Some($unit_name(self.0 / other))
                }
            }

            pub fn checked_rem(self, other: Self) -> Option<Self> {
                if other == Self::ZERO {
                    None
                } else {
                    Some($unit_name(self.0 % other.0))
                }
            }

            pub fn checked_rem_value(self, other: $type_name) -> Option<Self> {
                if other == $zero_value {
                    None
                } else {
                    Some($unit_name(self.0 % other))
                }
            }

            pub fn max(self, other: Self) -> Self {
                $unit_name(self.0.max(other.0))
            }

            pub fn min(self, other: Self) -> Self {
                $unit_name(self.0.min(other.0))
            }
        }

        impl From<$type_name> for $unit_name {
            #[inline]
            fn from(value: $type_name) -> Self {
                $unit_name(value)
            }
        }

        impl From<$unit_name> for $type_name {
            #[inline]
            fn from(value: $unit_name) -> Self {
                value.0
            }
        }

        impl std::ops::Add for $unit_name {
            type Output = Self;

            #[inline]
            fn add(self, other: Self) -> Self::Output {
                $unit_name(self.0 + other.0)
            }
        }

        impl std::ops::AddAssign for $unit_name {
            #[inline]
            fn add_assign(&mut self, other: Self) {
                self.0 += other.0;
            }
        }

        impl std::ops::Sub for $unit_name {
            type Output = Self;

            #[inline]
            fn sub(self, other: Self) -> Self::Output {
                $unit_name(self.0 - other.0)
            }
        }

        impl std::ops::SubAssign for $unit_name {
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.0 -= other.0;
            }
        }

        impl std::ops::Mul for $unit_name {
            type Output = Self;

            #[inline]
            fn mul(self, other: Self) -> Self::Output {
                $unit_name(self.0 * other.0)
            }
        }

        impl std::ops::MulAssign for $unit_name {
            #[inline]
            fn mul_assign(&mut self, other: Self) {
                self.0 *= other.0;
            }
        }

        impl std::ops::Add<$type_name> for $unit_name {
            type Output = Self;

            #[inline]
            fn add(self, other: $type_name) -> Self::Output {
                $unit_name(self.0 + other)
            }
        }

        impl std::ops::AddAssign<$type_name> for $unit_name {
            #[inline]
            fn add_assign(&mut self, other: $type_name) {
                self.0 += other;
            }
        }

        impl std::ops::Sub<$type_name> for $unit_name {
            type Output = Self;

            #[inline]
            fn sub(self, other: $type_name) -> Self::Output {
                $unit_name(self.0 - other)
            }
        }

        impl std::ops::SubAssign<$type_name> for $unit_name {
            #[inline]
            fn sub_assign(&mut self, other: $type_name) {
                self.0 -= other;
            }
        }

        impl std::ops::Mul<$type_name> for $unit_name {
            type Output = Self;

            #[inline]
            fn mul(self, other: $type_name) -> Self::Output {
                $unit_name(self.0 * other)
            }
        }

        impl std::ops::MulAssign<$type_name> for $unit_name {
            #[inline]
            fn mul_assign(&mut self, other: $type_name) {
                self.0 *= other;
            }
        }

        impl std::ops::Add<$unit_name> for $type_name {
            type Output = $unit_name;

            #[inline]
            fn add(self, other: $unit_name) -> Self::Output {
                $unit_name(self + other.0)
            }
        }

        impl std::ops::AddAssign<$unit_name> for $type_name {
            #[inline]
            fn add_assign(&mut self, other: $unit_name) {
                *self += other.0;
            }
        }

        impl std::ops::Sub<$unit_name> for $type_name {
            type Output = $unit_name;

            #[inline]
            fn sub(self, other: $unit_name) -> Self::Output {
                $unit_name(self - other.0)
            }
        }

        impl std::ops::SubAssign<$unit_name> for $type_name {
            #[inline]
            fn sub_assign(&mut self, other: $unit_name) {
                *self -= other.0;
            }
        }

        impl std::ops::Mul<$unit_name> for $type_name {
            type Output = $unit_name;

            #[inline]
            fn mul(self, other: $unit_name) -> Self::Output {
                $unit_name(self * other.0)
            }
        }

        impl std::ops::MulAssign<$unit_name> for $type_name {
            #[inline]
            fn mul_assign(&mut self, other: $unit_name) {
                *self *= other.0;
            }
        }

        paste! {
            #[derive(Debug, Clone, Copy, Default, PartialEq)]
            pub struct [<$unit_name Point>] {
                pub x: $unit_name,
                pub y: $unit_name,
            }

            impl [<$unit_name Point>] {
                #[inline]
                pub fn new(x: $unit_name, y: $unit_name) -> Self {
                    [<$unit_name Point>] { x, y }
                }

                #[inline]
                pub fn zero() -> Self {
                    [<$unit_name Point>] {
                        x: $unit_name::zero(),
                        y: $unit_name::zero(),
                    }
                }

                #[inline]
                pub fn distance(self, other: Self) -> f32 {
                    let dx = (self.x.0 as f32 - other.x.0 as f32).powi(2);
                    let dy = (self.y.0 as f32 - other.y.0 as f32).powi(2);
                    (dx + dy).sqrt()
                }

                #[inline]
                fn x_value(&self) -> $type_name {
                    self.x.0
                }

                #[inline]
                fn y_value(&self) -> $type_name {
                    self.y.0
                }

                #[inline]
                fn checked_div(self, other: Self) -> Option<Self> {
                    if other.x == $unit_name::ZERO || other.y == $unit_name::ZERO {
                        None
                    } else {
                        Some([<$unit_name Point>] {
                            x: self.x.checked_div(other.x)?,
                            y: self.y.checked_div(other.y)?,
                        })
                    }
                }
            }

            impl std::ops::Add for [<$unit_name Point>] {
                type Output = Self;

                #[inline]
                fn add(self, other: Self) -> Self::Output {
                    [<$unit_name Point>] {
                        x: self.x + other.x,
                        y: self.y + other.y,
                    }
                }
            }

            impl std::ops::Sub for [<$unit_name Point>] {
                type Output = Self;

                #[inline]
                fn sub(self, other: Self) -> Self::Output {
                    [<$unit_name Point>] {
                        x: self.x - other.x,
                        y: self.y - other.y,
                    }
                }
            }

            impl std::ops::Mul for [<$unit_name Point>] {
                type Output = Self;

                #[inline]
                fn mul(self, other: Self) -> Self::Output {
                    [<$unit_name Point>] {
                        x: self.x * other.x,
                        y: self.y * other.y,
                    }
                }
            }


            impl From<[$unit_name; 2]> for [<$unit_name Point>] {
                #[inline]
                fn from(arr: [$unit_name; 2]) -> Self {
                    [<$unit_name Point>] {
                        x: arr[0],
                        y: arr[1],
                    }
                }
            }

            impl From<[<$unit_name Point>]> for [$unit_name; 2] {
                #[inline]
                fn from(point: [<$unit_name Point>]) -> Self {
                    [point.x, point.y]
                }
            }

            impl From<($unit_name, $unit_name)> for [<$unit_name Point>] {
                #[inline]
                fn from(tuple: ($unit_name, $unit_name)) -> Self {
                    [<$unit_name Point>] {
                        x: tuple.0,
                        y: tuple.1,
                    }
                }
            }

            impl From<[<$unit_name Point>]> for ($unit_name, $unit_name) {
                #[inline]
                fn from(point: [<$unit_name Point>]) -> Self {
                    (point.x, point.y)
                }
            }

            impl From<[$type_name; 2]> for [<$unit_name Point>] {
                #[inline]
                fn from(arr: [$type_name; 2]) -> Self {
                    [<$unit_name Point>] {
                        x: $unit_name(arr[0]),
                        y: $unit_name(arr[1]),
                    }
                }
            }

            impl From<[<$unit_name Point>]> for [$type_name; 2] {
                #[inline]
                fn from(point: [<$unit_name Point>]) -> Self {
                    [point.x.0, point.y.0]
                }
            }

            impl From<($type_name, $type_name)> for [<$unit_name Point>] {
                #[inline]
                fn from(tuple: ($type_name, $type_name)) -> Self {
                    [<$unit_name Point>] {
                        x: $unit_name(tuple.0),
                        y: $unit_name(tuple.1),
                    }
                }
            }

            impl From<[<$unit_name Point>]> for ($type_name, $type_name) {
                #[inline]
                fn from(point: [<$unit_name Point>]) -> Self {
                    (point.x.0, point.y.0)
                }
            }

            #[derive(Debug, Clone, Copy, Default, PartialEq)]
            pub struct [<$unit_name Rect>] {
                pub min: [<$unit_name Point>],
                pub max: [<$unit_name Point>],
            }

            impl [<$unit_name Rect>] {
                const TWO: $type_name = $one_value + $one_value;

                #[inline]
                pub fn new(min: [<$unit_name Point>], max: [<$unit_name Point>]) -> Self {
                    [<$unit_name Rect>] { min, max }
                }

                #[inline]
                pub fn with_size(
                    min: [<$unit_name Point>],
                    size: [<$unit_name Size>],
                ) -> Self {
                    [<$unit_name Rect>] {
                        min,
                        max: [<$unit_name Point>] {
                            x: min.x + size.width,
                            y: min.y + size.height,
                        },
                    }
                }

                #[inline]
                pub fn zero() -> Self {
                    Self {
                        min: [<$unit_name Point>]::zero(),
                        max: [<$unit_name Point>]::zero(),
                    }
                }

                #[inline]
                pub fn width(&self) -> $unit_name {
                    self.max.x - self.min.x
                }

                #[inline]
                pub fn height(&self) -> $unit_name {
                    self.max.y - self.min.y
                }

                #[inline]
                pub fn checked_div(self, other: Self) -> Option<Self> {
                    if other.min.x_value() == $zero_value || other.min.y_value() == $zero_value ||
                       other.max.x_value() == $zero_value || other.max.y_value() == $zero_value {
                        None
                    } else {
                        Some([<$unit_name Rect>] {
                            min: self.min.checked_div(other.min)?,
                            max: self.max.checked_div(other.max)?,
                        })
                    }
                }

                #[inline]
                pub fn center(&self) -> [<$unit_name Point>] {
                    [<$unit_name Point>] {
                        x: $unit_name((self.min.x_value() + self.max.x_value()) / Self::TWO),
                        y: $unit_name((self.min.y_value() + self.max.y_value()) / Self::TWO),
                    }
                }
            }

            #[derive(Debug, Clone, Copy, Default, PartialEq)]
            pub struct [<$unit_name Size>] {
                pub width: $unit_name,
                pub height: $unit_name,
            }

            impl [<$unit_name Size>] {
                #[inline]
                pub fn new(width: $unit_name, height: $unit_name) -> Self {
                    [<$unit_name Size>] { width, height }
                }

                #[inline]
                pub fn zero() -> Self {
                    [<$unit_name Size>] {
                        width: $unit_name::zero(),
                        height: $unit_name::zero(),
                    }
                }

                #[inline]
                pub fn is_zero(&self) -> bool {
                    self.width.is_zero() && self.height.is_zero()
                }

                #[inline]
                pub fn area(&self) -> $type_name {
                    self.width.0 * self.height.0
                }
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    /// A device pixels width. (non-logical, in physical pixels)
    pub device_width: PhysicalPixels,
    /// A device pixels height. (non-logical, in physical pixels)
    pub device_height: PhysicalPixels,
    pub scale_factor: ScaleFactor,
    pub rect: PhysicalPixelsRect,
}

impl Viewport {
    pub fn new(
        device_width: PhysicalPixels,
        device_height: PhysicalPixels,
        scale_factor: ScaleFactor,
    ) -> Self {
        let rect = PhysicalPixelsRect::with_size(
            PhysicalPixelsPoint::new(0.into(), 0.into()),
            PhysicalPixelsSize::new(device_width, device_height),
        );
        Viewport {
            device_width,
            device_height,
            scale_factor,
            rect,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScaleFactor {
    dpi: f32,
    dpr: f32,
}

impl ScaleFactor {
    pub fn new(dpi: f32, dpr: f32) -> Self {
        assert!(dpi > 0.0, "DPI must be greater than 0");
        assert!(dpr > 0.0, "DPR must be greater than 0");
        ScaleFactor { dpi, dpr }
    }

    #[inline]
    pub fn dpi(&self) -> f32 {
        self.dpi
    }

    #[inline]
    pub fn dpr(&self) -> f32 {
        self.dpr
    }
}

define_unit!(PhysicalPixels, u32, 0, 1);
define_unit!(PhysicalPixelsF, f32, 0.0, 1.0);
define_unit!(LogicalPixels, f32, 0.0, 1.0);

impl From<PhysicalPixels> for PhysicalPixelsF {
    #[inline]
    fn from(value: PhysicalPixels) -> Self {
        PhysicalPixelsF::new(value.0 as f32)
    }
}

impl From<PhysicalPixelsF> for PhysicalPixels {
    #[inline]
    fn from(value: PhysicalPixelsF) -> Self {
        PhysicalPixels::new(value.0.round() as u32)
    }
}

impl From<PhysicalPixelsPoint> for PhysicalPixelsFPoint {
    #[inline]
    fn from(point: PhysicalPixelsPoint) -> Self {
        PhysicalPixelsFPoint {
            x: point.x.into(),
            y: point.y.into(),
        }
    }
}

impl From<PhysicalPixelsFPoint> for PhysicalPixelsPoint {
    #[inline]
    fn from(point: PhysicalPixelsFPoint) -> Self {
        PhysicalPixelsPoint {
            x: point.x.into(),
            y: point.y.into(),
        }
    }
}

impl From<PhysicalPixelsSize> for PhysicalPixelsFSize {
    #[inline]
    fn from(size: PhysicalPixelsSize) -> Self {
        PhysicalPixelsFSize {
            width: size.width.into(),
            height: size.height.into(),
        }
    }
}

impl From<PhysicalPixelsFSize> for PhysicalPixelsSize {
    #[inline]
    fn from(size: PhysicalPixelsFSize) -> Self {
        PhysicalPixelsSize {
            width: size.width.into(),
            height: size.height.into(),
        }
    }
}

pub trait InPhysicalScale<PhysicalUnit> {
    fn in_physical_scale(&self, scale_factor: &ScaleFactor) -> PhysicalUnit;
}

pub trait InLogicalScale<LogicalUnit> {
    fn in_logical_scale(&self, scale_factor: &ScaleFactor) -> LogicalUnit;
}

macro_rules! impl_in_physical_scale {
    ($from:ty, $to:ty, $base_type:ty) => {
        impl InPhysicalScale<$to> for $from {
            #[inline]
            fn in_physical_scale(&self, scale_factor: &ScaleFactor) -> $to {
                <$to>::new((self.0 * scale_factor.dpr()).round() as $base_type)
            }
        }
        paste! {
            impl InPhysicalScale<[<$to Point>]> for [<$from Point>] {
                #[inline]
                fn in_physical_scale(&self, scale_factor: &ScaleFactor) -> [<$to Point>] {
                    [<$to Point>] {
                        x: self.x.in_physical_scale(scale_factor),
                        y: self.y.in_physical_scale(scale_factor),
                    }
                }
            }

            impl InPhysicalScale<[<$to Rect>]> for [<$from Rect>] {
                #[inline]
                fn in_physical_scale(&self, scale_factor: &ScaleFactor) -> [<$to Rect>] {
                    [<$to Rect>] {
                        min: self.min.in_physical_scale(scale_factor),
                        max: self.max.in_physical_scale(scale_factor),
                    }
                }
            }

            impl InPhysicalScale<[<$to Size>]> for [<$from Size>] {
                #[inline]
                fn in_physical_scale(&self, scale_factor: &ScaleFactor) -> [<$to Size>] {
                    [<$to Size>] {
                        width: self.width.in_physical_scale(scale_factor),
                        height: self.height.in_physical_scale(scale_factor),
                    }
                }
            }
        }
    };
}

macro_rules! impl_in_logical_scale {
    ($from:ty, $to:ty, $base_type:ty) => {
        impl InLogicalScale<$to> for $from {
            #[inline]
            fn in_logical_scale(&self, scale_factor: &ScaleFactor) -> $to {
                <$to>::new((self.0 as f32 / scale_factor.dpr()).round() as $base_type)
            }
        }

        paste! {
            impl InLogicalScale<[<$to Point>]> for [<$from Point>] {
                #[inline]
                fn in_logical_scale(&self, scale_factor: &ScaleFactor) -> [<$to Point>] {
                    [<$to Point>] {
                        x: self.x.in_logical_scale(scale_factor),
                        y: self.y.in_logical_scale(scale_factor),
                    }
                }
            }

            impl InLogicalScale<[<$to Rect>]> for [<$from Rect>] {
                #[inline]
                fn in_logical_scale(&self, scale_factor: &ScaleFactor) -> [<$to Rect>] {
                    [<$to Rect>] {
                        min: self.min.in_logical_scale(scale_factor),
                        max: self.max.in_logical_scale(scale_factor),
                    }
                }
            }

            impl InLogicalScale<[<$to Size>]> for [<$from Size>] {
                #[inline]
                fn in_logical_scale(&self, scale_factor: &ScaleFactor) -> [<$to Size>] {
                    [<$to Size>] {
                        width: self.width.in_logical_scale(scale_factor),
                        height: self.height.in_logical_scale(scale_factor),
                    }
                }
            }
        }
    };
}

impl_in_physical_scale!(LogicalPixels, PhysicalPixels, u32);
impl_in_physical_scale!(LogicalPixels, PhysicalPixelsF, f32);
impl_in_logical_scale!(PhysicalPixels, LogicalPixels, f32);
impl_in_logical_scale!(PhysicalPixelsF, LogicalPixels, f32);

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Alignment {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
    CenterLeft,
    CenterRight,
    CenterTop,
    CenterBottom,
}

impl Alignment {
    pub fn align_size(
        &self,
        bounds: LogicalPixelsRect,
        size: LogicalPixelsSize,
    ) -> LogicalPixelsRect {
        let mut x = bounds.min.x;
        let mut y = bounds.min.y;

        match self {
            Alignment::TopLeft => {}
            Alignment::TopRight => x += bounds.width() - size.width,
            Alignment::BottomLeft => y += bounds.height() - size.height,
            Alignment::BottomRight => {
                x += bounds.width() - size.width;
                y += bounds.height() - size.height;
            }
            Alignment::Center => {
                // SAFETY: Division by non-zero value
                x += (bounds.width() - size.width)
                    .checked_div_value(2.0)
                    .unwrap();
                // SAFETY: Division by non-zero value
                y += (bounds.height() - size.height)
                    .checked_div_value(2.0)
                    .unwrap();
            }
            Alignment::CenterLeft => {
                // SAFETY: Division by non-zero value
                y += (bounds.height() - size.height)
                    .checked_div_value(2.0)
                    .unwrap();
            }
            Alignment::CenterRight => {
                // SAFETY: Division by non-zero value
                x += bounds.width() - size.width;
                // SAFETY: Division by non-zero value
                y += (bounds.height() - size.height)
                    .checked_div_value(2.0)
                    .unwrap();
            }
            Alignment::CenterTop => {
                // SAFETY: Division by non-zero value
                x += (bounds.width() - size.width)
                    .checked_div_value(2.0)
                    .unwrap();
            }
            Alignment::CenterBottom => {
                // SAFETY: Division by non-zero value
                x += (bounds.width() - size.width)
                    .checked_div_value(2.0)
                    .unwrap();
                // SAFETY: Division by non-zero value
                y += bounds.height() - size.height;
            }
        }

        LogicalPixelsRect::with_size(LogicalPixelsPoint::new(x, y), size)
    }
}

/// A Normalized Texture Point.
/// This is a point in the range [0.0, 1.0] for both x and y coordinates.
/// Do not use this to logically scale pixels.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TexturePoint {
    x: f32,
    y: f32,
}

impl TexturePoint {
    /// Safety: If the x and y values are not in the range [0.0, 1.0], this returns `None`.
    #[inline]
    pub fn new(x: f32, y: f32) -> Option<Self> {
        // TODO: Validate that x and y are in the range [0.0, 1.0].
        Some(TexturePoint { x, y })
    }

    #[inline]
    pub fn from_physical_point(
        point: PhysicalPixelsPoint,
        texture_size: PhysicalPixelsSize,
    ) -> Option<Self> {
        // Convert the point to PhysicalPixelsFPoint to avoid rounding issues.
        let point: PhysicalPixelsFPoint = point.into();
        let x = point
            .x
            .checked_div_value(texture_size.width.value() as f32)?;
        let y = point
            .y
            .checked_div_value(texture_size.height.value() as f32)?;
        TexturePoint::new(x.into(), y.into())
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.x
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.y
    }
}

/// A Normalized Texture Rect.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TextureBounds {
    pub min: TexturePoint,
    pub max: TexturePoint,
}

impl TextureBounds {
    pub fn new(min: TexturePoint, max: TexturePoint) -> Self {
        TextureBounds { min, max }
    }

    pub fn from_phisical_rect(
        rect: PhysicalPixelsRect,
        texture_size: PhysicalPixelsSize,
    ) -> Option<Self> {
        let min = TexturePoint::from_physical_point(rect.min, texture_size)?;
        let max = TexturePoint::from_physical_point(rect.max, texture_size)?;
        Some(TextureBounds { min, max })
    }
}
