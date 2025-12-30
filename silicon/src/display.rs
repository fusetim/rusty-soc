use embedded_graphics::{
    Pixel,
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget, PixelColor },
    primitives::Rectangle,
};

/// A wrapper around a DrawTarget that maps binary colors to specified on/off colors.
///
/// Type Parameters:
/// * `T` - The underlying DrawTarget type.
///
/// Fields:
/// * `on_color` - The color to use for "on" pixels.
/// * `off_color` - The color to use for "off" pixels.
/// * `target` - The underlying DrawTarget instance.
pub struct BinWrapDrawTarget<'a, T: DrawTarget> {
    on_color: T::Color,
    off_color: T::Color,
    target: &'a mut T,
}

impl<'a, T: DrawTarget> BinWrapDrawTarget<'a, T> {
    /// Create a new BinWrapDrawTarget.
    ///
    /// # Arguments
    ///
    /// * `on_color` - The color to use for "on" pixels.
    /// * `off_color` - The color to use for "off" pixels.
    /// * `target` - The underlying DrawTarget instance.
    ///
    /// # Returns
    ///
    /// A new BinWrapDrawTarget instance.
    #[inline]
    pub fn new(on_color: T::Color, off_color: T::Color, target: &'a mut T) -> Self {
        Self {
            on_color,
            off_color,
            target,
        }
    }

    /// Consume the wrapper and return the underlying DrawTarget.
    ///
    /// # Returns
    ///
    /// The underlying DrawTarget instance.
    #[inline]
    pub fn dispose(self) -> &'a mut T {
        self.target
    }
}

impl<'a, T: DrawTarget> Dimensions for BinWrapDrawTarget<'a, T> {
    #[inline(always)]
    fn bounding_box(&self) -> Rectangle {
        self.target.bounding_box()
    }
}

impl<'a, T: DrawTarget> DrawTarget for BinWrapDrawTarget<'a, T> {
    type Color = BinaryColor;
    type Error = T::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let mapped_pixels = pixels
            .into_iter()
            .map(|p| map_pixel(p, self.on_color, self.off_color));
        self.target.draw_iter(mapped_pixels)
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let mapped_colors = colors
            .into_iter()
            .map(|c| map_color(c, self.on_color, self.off_color));
        self.target.fill_contiguous(area, mapped_colors)
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let mapped_color = map_color(color, self.on_color, self.off_color);
        self.target.fill_solid(area, mapped_color)
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        let mapped_color = map_color(color, self.on_color, self.off_color);
        self.target.clear(mapped_color)
    }
}

/// Map a pixel from BinaryColor to the target color.
#[inline(always)]
fn map_pixel<C: PixelColor>(input: Pixel<BinaryColor>, on_color: C, off_color: C) -> Pixel<C> {
    let Pixel(coord, color) = input;
    Pixel(coord, map_color(color, on_color, off_color))
}

/// Map a color based on the wrapper configuration.
#[inline(always)]
fn map_color<C: PixelColor>(color: BinaryColor, on_color: C, off_color: C) -> C {
    match color {
        BinaryColor::On => on_color,
        BinaryColor::Off => off_color,
    }
}
