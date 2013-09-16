// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/* automatically generated by rust-bindgen */

#[allow(non_uppercase_statics)];

use std::libc::*;

pub type AzFontOptions = *c_void;
pub type AzFloat = c_float;

pub type enum_AzSurfaceType = c_uint;
pub static AZ_SURFACE_DATA: u32 = 0_u32;
pub static AZ_SURFACE_D2D1_BITMAP: u32 = 1_u32;
pub static AZ_SURFACE_D2D1_DRAWTARGET: u32 = 2_u32;
pub static AZ_SURFACE_CAIRO: u32 = 3_u32;
pub static AZ_SURFACE_CAIRO_IMAGE: u32 = 4_u32;
pub static AZ_SURFACE_COREGRAPHICS_IMAGE: u32 = 5_u32;
pub static AZ_SURFACE_COREGRAPHICS_CGCONTEXT: u32 = 6_u32;
pub static AZ_SURFACE_SKIA: u32 = 7_u32;
pub static AZ_SURFACE_DUAL_DT: u32 = 8_u32;

pub type enum_AzSurfaceFormat = c_uint;
pub static AZ_FORMAT_B8G8R8A8: u32 = 0_u32;
pub static AZ_FORMAT_B8G8R8X8: u32 = 1_u32;
pub static AZ_FORMAT_R5G6B5: u32 = 2_u32;
pub static AZ_FORMAT_A8: u32 = 3_u32;

pub type AzSurfaceFormat = enum_AzSurfaceFormat;

pub type enum_AzBackendType = c_uint;
pub static AZ_BACKEND_NONE: u32 = 0_u32;
pub static AZ_BACKEND_DIRECT2D: u32 = 1_u32;
pub static AZ_BACKEND_COREGRAPHICS: u32 = 2_u32;
pub static AZ_BACKEND_COREGRAPHICS_ACCELERATED: u32 = 3_u32;
pub static AZ_BACKEND_CAIRO: u32 = 4_u32;
pub static AZ_BACKEND_SKIA: u32 = 5_u32;
pub static AZ_BACKEND_RECORDING: u32 = 6_u32;

pub type AzBackendType = enum_AzBackendType;

pub type enum_AzFontType = c_uint;
pub static AZ_FONT_DWRITE: u32 = 0_u32;
pub static AZ_FONT_GDI: u32 = 1_u32;
pub static AZ_FONT_MAC: u32 = 2_u32;
pub static AZ_FONT_SKIA: u32 = 3_u32;
pub static AZ_FONT_CAIRO: u32 = 4_u32;
pub static AZ_FONT_COREGRAPHICS: u32 = 5_u32;

pub type enum_AzNativeSurfaceType = c_uint;
pub static AZ_NATIVE_SURFACE_D3D10_TEXTURE: u32 = 0_u32;
pub static AZ_NATIVE_SURFACE_CAIRO_SURFACE: u32 = 1_u32;
pub static AZ_NATIVE_SURFACE_CGCONTEXT: u32 = 2_u32;

pub type enum_AzNativeFontType = c_uint;
pub static AZ_NATIVE_FONT_DWRITE_FONT_FACE: u32 = 0_u32;
pub static AZ_NATIVE_FONT_GDI_FONT_FACE: u32 = 1_u32;
pub static AZ_NATIVE_FONT_MAC_FONT_FACE: u32 = 2_u32;
pub static AZ_NATIVE_FONT_SKIA_FONT_FACE: u32 = 3_u32;
pub static AZ_NATIVE_FONT_CAIRO_FONT_FACE: u32 = 4_u32;

pub type enum_AzFontStyle = c_uint;
pub static AZ_FONT_STYLE_NORMAL: u32 = 0_u32;
pub static AZ_FONT_STYLE_ITALIC: u32 = 1_u32;
pub static AZ_FONT_STYLE_BOLD: u32 = 2_u32;
pub static AZ_FONT_STYLE_BOLD_ITALIC: u32 = 3_u32;

pub type enum_AzCompositionOp = c_uint;
pub static AZ_OP_OVER: u32 = 0_u32;
pub static AZ_OP_ADD: u32 = 1_u32;
pub static AZ_OP_ATOP: u32 = 2_u32;
pub static AZ_OP_OUT: u32 = 3_u32;
pub static AZ_OP_IN: u32 = 4_u32;
pub static AZ_OP_SOURCE: u32 = 5_u32;
pub static AZ_OP_DEST_IN: u32 = 6_u32;
pub static AZ_OP_DEST_OUT: u32 = 7_u32;
pub static AZ_OP_DEST_OVER: u32 = 8_u32;
pub static AZ_OP_DEST_ATOP: u32 = 9_u32;
pub static AZ_OP_XOR: u32 = 10_u32;
pub static AZ_OP_COUNT: u32 = 11_u32;

pub type enum_AzExtendMode = c_uint;
pub static AZ_EXTEND_CLAMP: u32 = 0_u32;
pub static AZ_EXTEND_REPEAT: u32 = 1_u32;
pub static AZ_EXTEND_REFLECT: u32 = 2_u32;

pub type enum_AzFillRule = c_uint;
pub static AZ_FILL_WINDING: u32 = 0_u32;
pub static AZ_FILL_EVEN_ODD: u32 = 1_u32;

pub type enum_AzAntialiasMode = c_uint;
pub static AZ_AA_NONE: u32 = 0_u32;
pub static AZ_AA_GRAY: u32 = 1_u32;
pub static AZ_AA_SUBPIXEL: u32 = 2_u32;

pub type enum_AzSnapping = c_uint;
pub static AZ_SNAP_NONE: u32 = 0_u32;
pub static AZ_SNAP_ALIGNED: u32 = 1_u32;

pub type enum_AzFilter = c_uint;
pub static AZ_FILTER_LINEAR: u32 = 0_u32;
pub static AZ_FILTER_POINT: u32 = 1_u32;

pub type AzFilter = enum_AzFilter;

pub type enum_AzPatternType = c_uint;
pub static AZ_PATTERN_COLOR: u32 = 0_u32;
pub static AZ_PATTERN_SURFACE: u32 = 1_u32;
pub static AZ_PATTERN_LINEAR_GRADIENT: u32 = 2_u32;
pub static AZ_PATTERN_RADIAL_GRADIENT: u32 = 3_u32;

pub type enum_AzJoinStyle = c_uint;
pub static AZ_JOIN_BEVEL: u32 = 0_u32;
pub static AZ_JOIN_ROUND: u32 = 1_u32;
pub static AZ_JOIN_MITER: u32 = 2_u32;
pub static AZ_JOIN_MITER_OR_BEVEL: u32 = 3_u32;

pub type enum_AzCapStyle = c_uint;
pub static AZ_CAP_BUTT: u32 = 0_u32;
pub static AZ_CAP_ROUND: u32 = 1_u32;
pub static AZ_CAP_SQUARE: u32 = 2_u32;

pub type enum_AzSamplingBounds = c_uint;
pub static AZ_SAMPLING_UNBOUNDED: u32 = 0_u32;
pub static AZ_SAMPLING_BOUNDED: u32 = 1_u32;

pub type enum_AzSide = c_uint;
pub static AZ_eSideTop: u32 = 0_u32;
pub static AZ_eSideRight: u32 = 1_u32;
pub static AZ_eSideBottom: u32 = 2_u32;
pub static AZ_eSideLeft: u32 = 3_u32;

pub struct struct__AzColor {
    r: AzFloat,
    g: AzFloat,
    b: AzFloat,
    a: AzFloat,
}

pub type AzColor = struct__AzColor;

pub struct struct__AzGradientStop {
    offset: AzFloat,
    color: AzColor,
}

pub type AzGradientStop = struct__AzGradientStop;

pub struct struct__AzIntRect {
    x: int32_t,
    y: int32_t,
    width: int32_t,
    height: int32_t,
}

pub type AzIntRect = struct__AzIntRect;

pub struct struct__AzRect {
    x: AzFloat,
    y: AzFloat,
    width: AzFloat,
    height: AzFloat,
}

pub type AzRect = struct__AzRect;

pub struct struct__AzIntPoint {
    x: int32_t,
    y: int32_t,
}

pub type AzIntPoint = struct__AzIntPoint;

pub struct struct__AzPoint {
    x: AzFloat,
    y: AzFloat,
}

pub type AzPoint = struct__AzPoint;

pub struct struct__AzIntSize {
    width: int32_t,
    height: int32_t,
}

pub type AzIntSize = struct__AzIntSize;

pub struct struct__AzSize {
    width: AzFloat,
    height: AzFloat,
}

pub type AzSize = struct__AzSize;

pub struct struct__AzMatrix {
    _11: AzFloat,
    _12: AzFloat,
    _21: AzFloat,
    _22: AzFloat,
    _31: AzFloat,
    _32: AzFloat,
}

pub type AzMatrix = struct__AzMatrix;

pub struct struct__AzDrawOptions {
    mAlpha: AzFloat,
    fields: uint16_t,
}

pub type AzDrawOptions = struct__AzDrawOptions;

pub struct struct__AzStrokeOptions {
    mLineWidth: AzFloat,
    mMiterLimit: AzFloat,
    mDashPattern: *AzFloat,
    mDashLength: size_t,
    mDashOffset: AzFloat,
    fields: uint8_t,
}

pub type AzStrokeOptions = struct__AzStrokeOptions;

pub struct struct__AzDrawSurfaceOptions {
    fields: uint32_t,
}

pub type AzDrawSurfaceOptions = struct__AzDrawSurfaceOptions;

pub struct struct__AzGlyph {
    mIndex: uint32_t,
    mPosition: AzPoint,
}

pub type AzGlyph = struct__AzGlyph;

pub struct struct__AzGlyphBuffer {
    mGlyphs: *AzGlyph,
    mNumGlyphs: uint32_t,
}

pub type AzGlyphBuffer = struct__AzGlyphBuffer;

pub struct struct__AzNativeFont {
    mType: enum_AzNativeFontType,
    mFont: *c_void,
}

pub type AzNativeFont = struct__AzNativeFont;

pub type AzGradientStopsRef = *c_void;

pub type AzSkiaSharedGLContextRef = *c_void;

pub type AzSkiaGrContextRef = *c_void;

pub type AzDrawTargetRef = *c_void;

pub type AzPatternRef = *c_void;

pub type AzColorPatternRef = *c_void;

pub type AzScaledFontRef = *c_void;

pub type AzGlyphRenderingOptionsRef = *c_void;

pub type AzSourceSurfaceRef = *c_void;

pub type AzDataSourceSurfaceRef = *c_void;

pub type AzDrawSurfaceOptionsRef = *AzDrawSurfaceOptions;

pub type AzGLContext = *c_void;

#[link_args="-lazure"]
extern {

pub fn AzSanityCheck(/* FIXME: variadic function */);

pub fn AzCreateColorPattern(aColor: *AzColor) -> AzColorPatternRef;

pub fn AzReleaseColorPattern(aColorPattern: AzColorPatternRef);

pub fn AzCreateSkiaSharedGLContext(aGLContext: AzGLContext, extra: *c_void, aSize: *AzIntSize) -> AzSkiaSharedGLContextRef;

pub fn AzRetainSkiaSharedGLContext(aGLContext: AzSkiaSharedGLContextRef);

pub fn AzReleaseSkiaSharedGLContext(aGLContext: AzSkiaSharedGLContextRef);

pub fn AzSkiaSharedGLContextGetFBOID(aGLContext: AzSkiaSharedGLContextRef) -> c_uint;

pub fn AzSkiaSharedGLContextGetTextureID(aGLContext: AzSkiaSharedGLContextRef) -> c_uint;

pub fn AzSkiaSharedGLContextStealTextureID(aGLContext: AzSkiaSharedGLContextRef) -> c_uint;

pub fn AzSkiaSharedGLContextGetGrContext(aGLContext: AzSkiaSharedGLContextRef) -> AzSkiaGrContextRef;

pub fn AzSkiaSharedGLContextMakeCurrent(aGLContext: AzSkiaSharedGLContextRef);

pub fn AzSkiaSharedGLContextFlush(aGLContext: AzSkiaSharedGLContextRef);

pub fn AzCreateDrawTarget(aBackend: AzBackendType, aSize: *AzIntSize, aFormat: AzSurfaceFormat) -> AzDrawTargetRef;

pub fn AzCreateDrawTargetForData(aBackend: AzBackendType, aData: *c_uchar, aSize: *AzIntSize, aStride: i32, aFormat: AzSurfaceFormat) -> AzDrawTargetRef;

pub fn AzCreateSkiaDrawTargetForFBO(aGLContext: AzSkiaSharedGLContextRef, aSize: *AzIntSize, aFormat: AzSurfaceFormat) -> AzDrawTargetRef;

pub fn AzRetainDrawTarget(aTarget: AzDrawTargetRef);

pub fn AzReleaseDrawTarget(aTarget: AzDrawTargetRef);

pub fn AzDrawTargetGetSize(aDrawTarget: AzDrawTargetRef) -> AzIntSize;

pub fn AzDrawTargetFlush(aDrawTarget: AzDrawTargetRef);

pub fn AzDrawTargetClearRect(aDrawTarget: AzDrawTargetRef, aRect: *AzRect);

pub fn AzDrawTargetFillRect(aDrawTarget: AzDrawTargetRef, aRect: *AzRect, aPattern: AzPatternRef);

pub fn AzDrawTargetStrokeRect(aDrawTarget: AzDrawTargetRef, aRect: *AzRect, aPattern: AzPatternRef, aStrokeOptions: *AzStrokeOptions, aDrawOptions: *AzDrawOptions);

pub fn AzDrawTargetStrokeLine(aDrawTarget: AzDrawTargetRef, aStart: *AzPoint, aEnd: *AzPoint, aPattern: AzPatternRef, aStrokeOptions: *AzStrokeOptions, aDrawOptions: *AzDrawOptions);

pub fn AzDrawTargetFillGlyphs(aDrawTarget: AzDrawTargetRef, aFont: AzScaledFontRef, aGlyphBuffer: *AzGlyphBuffer, aPattern: AzPatternRef, aOptions: *AzDrawOptions, aRenderingOptions: AzGlyphRenderingOptionsRef);

pub fn AzDrawTargetDrawSurface(aDrawTarget: AzDrawTargetRef, aSurface: AzSourceSurfaceRef, aDest: *AzRect, aSource: *AzRect, aSurfOptions: AzDrawSurfaceOptionsRef, aOptions: *AzDrawOptions);

pub fn AzDrawTargetGetSnapshot(aDrawTarget: AzDrawTargetRef) -> AzSourceSurfaceRef;

pub fn AzDrawTargetCreateSourceSurfaceFromData(aDrawTarget: AzDrawTargetRef, aData: *u8, aSize: *AzIntSize, aStride: i32, aFormat: AzSurfaceFormat) -> AzSourceSurfaceRef;

pub fn AzReleaseSourceSurface(aSurface: AzSourceSurfaceRef);

pub fn AzSourceSurfaceGetSize(aSurface: AzSourceSurfaceRef) -> AzIntSize;

pub fn AzSourceSurfaceGetFormat(aSurface: AzSourceSurfaceRef) -> AzSurfaceFormat;

pub fn AzSourceSurfaceGetDataSurface(aSurface: AzSourceSurfaceRef) -> AzDataSourceSurfaceRef;

pub fn AzDataSourceSurfaceGetData(aSurface: AzDataSourceSurfaceRef) -> *u8;

pub fn AzDataSourceSurfaceGetStride(aSurface: AzDataSourceSurfaceRef) -> i32;

pub fn AzCreateScaledFontForNativeFont(aNativeFont: *AzNativeFont, aSize: AzFloat) -> AzScaledFontRef;

pub fn AzReleaseScaledFont(aFont: AzScaledFontRef);

pub fn AzDrawTargetSetTransform(aDrawTarget: AzDrawTargetRef, aTransform: *AzMatrix);

pub fn AzCreateFontOptions(aName: *c_char, aStyle: enum_AzFontStyle) -> *AzFontOptions;

pub fn AzDestroyFontOptions(aOptions: *AzFontOptions);

pub fn AzSkiaGetCurrentGLContext() -> AzGLContext;

}
