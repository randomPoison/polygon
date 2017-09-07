use image::DynamicImage;

/// Represents texture data that has been sent to the GPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GpuTexture(usize);
derive_Counter!(GpuTexture);

/// Represents a texture loaded into memory and ready to be sent to the GPU.
///
/// `Texture2d` defines a backend-agnostic in-memory representation of texture data that can be
/// used by any of the rendering backends to send texture data to the GPU. It encapsulates all
/// relevant information about the texture, including the raw bytes of the texture and information
/// describing the in-memory layout of that data. It also provides functionality for safely
/// loading textures from common formats (NOTE: Only bitmap is supported currently).
#[derive(Debug)]
pub struct Texture2d {
    width: usize,
    height: usize,
    format: DataFormat,
    data: TextureData,
}

impl Texture2d {
    /// Loads a new `Texture` from a bitmap file.
    pub fn from_bitmap(image: DynamicImage) -> Texture2d {
        // TODO: We should directly support image types more directly. For simplicity, we'll
        // start by converting everything to RGBA, but we probably don't need to do that.
        let image = image.to_rgba();

        Texture2d {
            width: image.width() as usize,
            height: image.height() as usize,
            format: DataFormat::Rgba,
            data: TextureData::u8(image.into_raw()),
        }
    }

    /// Returns the width of the texture.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the texture.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Gets the data format for the texture.
    pub fn format(&self) -> DataFormat {
        self.format
    }

    /// Gets the data for the texture.
    pub fn data(&self) -> &TextureData {
        &self.data
    }
}

/// An enum representing the supported data formats for a texture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataFormat {
    Rgb,
    Rgba,
    Bgr,
    Bgra,
}

/// An enum representing the possible data types for a texture.
///
/// `TextureData` also owns the texture raw data buffer in order to maintain type safety.
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq)]
pub enum TextureData {
    f32(Vec<f32>),
    u8(Vec<u8>),
}
