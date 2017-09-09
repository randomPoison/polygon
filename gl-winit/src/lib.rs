extern crate gl_util;
extern crate gl_raw;
extern crate winit;

use gl_util::context::Context;

pub trait CreateContext {
    fn create_context(&self) -> Result<Context, CreationError>;
}

#[derive(Debug, Clone)]
pub struct CreationError;

#[cfg(target_os = "windows")]
mod windows {
    extern crate gdi32;
    extern crate kernel32;
    extern crate user32;
    extern crate winapi;

    use {CreationError, CreateContext};
    use gl_util::context::Context;
    use gl_raw;
    use self::winapi::*;
    use std::mem;
    use winit::os::windows::WindowExt;

    impl CreateContext for ::winit::Window {
        fn create_context(&self) -> Result<Context, CreationError> {
            let window_handle = self.get_hwnd();
            println!("hwnd: {:?}", window_handle);
            assert!(!window_handle.is_null(), "Window handle was null???");

            // Get the device context for the window, returning an error if it's null.
            let device_context = unsafe { user32::GetDC(window_handle as *mut _) };
            println!("dc: {:?}", device_context);
            if device_context.is_null() {
                return Err(CreationError);
            }

            // Set pixel format before creating context.
            let pfd = PIXELFORMATDESCRIPTOR {
                nSize: mem::size_of::<PIXELFORMATDESCRIPTOR>() as WORD,
                nVersion: 1,
                dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
                iPixelType: PFD_TYPE_RGBA,
                cColorBits: 32,
                cRedBits: 0,
                cRedShift: 0,
                cGreenBits: 0,
                cGreenShift: 0,
                cBlueBits: 0,
                cBlueShift: 0,
                cAlphaBits: 0,
                cAlphaShift: 0,
                cAccumBits: 0,
                cAccumRedBits: 0,
                cAccumGreenBits: 0,
                cAccumBlueBits: 0,
                cAccumAlphaBits: 0,
                cDepthBits: 24,
                cStencilBits: 8,
                cAuxBuffers: 0,
                iLayerType: PFD_MAIN_PLANE,
                bReserved: 0,
                dwLayerMask: 0,
                dwVisibleMask: 0,
                dwDamageMask: 0
            };

            let raw_context = unsafe {
                let pixel_format = gdi32::ChoosePixelFormat(device_context, &pfd);
                if pixel_format == 0 {
                    let error_code = kernel32::GetLastError();
                    println!("WARNING: Unable to find appropriate pixel format, OpenGL rendering might not work, last error: 0x{:x}", error_code);
                }

                let result = gdi32::SetPixelFormat(device_context, pixel_format, &pfd);
                if result == 0 {
                    let error_code = kernel32::GetLastError();
                    println!("WARNING: Failed to set pixel format, OpenGL rendering might not work, last error: 0x{:x}", error_code);
                }

                // Create the raw context once the pixel format has been set.
                gl_raw::create_context(device_context).ok_or(CreationError)?
            };

            // Create an OpenGL context from the raw context.
            Context::from_raw_context(raw_context).map_err(|error| {
                println!("error: {:?}", error);
                CreationError
            })
        }
    }
}
