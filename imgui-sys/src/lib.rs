#![allow(non_upper_case_globals)]

use std::convert::From;
use std::os::raw::{c_char, c_float, c_int, c_uint, c_ushort, c_void};

pub use self::enums::*;
pub use self::flags::*;
pub use self::structs::*;

mod enums;
mod flags;
mod structs;

#[cfg(feature = "gfx")]
mod gfx_support;

#[cfg(feature = "glium")]
mod glium_support;

/// Vertex index
pub type ImDrawIdx = c_ushort;

/// ImGui context (opaque)
pub enum ImGuiContext {}

/// Unique ID used by widgets (typically hashed from a stack of string)
pub type ImGuiID = ImU32;

/// User data to identify a texture
pub type ImTextureID = *mut c_void;

/// 32-bit unsigned integer (typically used to store packed colors)
pub type ImU32 = c_uint;

/// Character for keyboard input/display
pub type ImWchar = c_ushort;

/// Draw callback for advanced use
pub type ImDrawCallback =
    Option<extern "C" fn(parent_list: *const ImDrawList, cmd: *const ImDrawCmd)>;

/// Input text callback for advanced use
pub type ImGuiInputTextCallback =
    Option<extern "C" fn(data: *mut ImGuiInputTextCallbackData) -> c_int>;

/// Size constraint callback for advanced use
pub type ImGuiSizeCallback = Option<extern "C" fn(data: *mut ImGuiSizeCallbackData)>;

/// A tuple of 2 floating-point values
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ImVec2 {
    pub x: c_float,
    pub y: c_float,
}

impl ImVec2 {
    pub fn new(x: f32, y: f32) -> ImVec2 {
        ImVec2 {
            x: x as c_float,
            y: y as c_float,
        }
    }
    pub fn zero() -> ImVec2 {
        ImVec2 {
            x: 0.0 as c_float,
            y: 0.0 as c_float,
        }
    }
}

impl From<[f32; 2]> for ImVec2 {
    fn from(array: [f32; 2]) -> ImVec2 {
        ImVec2::new(array[0], array[1])
    }
}

impl From<(f32, f32)> for ImVec2 {
    fn from((x, y): (f32, f32)) -> ImVec2 {
        ImVec2::new(x, y)
    }
}

impl Into<[f32; 2]> for ImVec2 {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl Into<(f32, f32)> for ImVec2 {
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

/// A tuple of 4 floating-point values
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ImVec4 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub w: c_float,
}

impl ImVec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> ImVec4 {
        ImVec4 {
            x: x as c_float,
            y: y as c_float,
            z: z as c_float,
            w: w as c_float,
        }
    }
    pub fn zero() -> ImVec4 {
        ImVec4 {
            x: 0.0 as c_float,
            y: 0.0 as c_float,
            z: 0.0 as c_float,
            w: 0.0 as c_float,
        }
    }
}

impl From<[f32; 4]> for ImVec4 {
    fn from(array: [f32; 4]) -> ImVec4 {
        ImVec4::new(array[0], array[1], array[2], array[3])
    }
}

impl From<(f32, f32, f32, f32)> for ImVec4 {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> ImVec4 {
        ImVec4::new(x, y, z, w)
    }
}

impl Into<[f32; 4]> for ImVec4 {
    fn into(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl Into<(f32, f32, f32, f32)> for ImVec4 {
    fn into(self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }
}

// Context creation and access
extern "C" {
    pub fn igCreateContext(shared_font_atlas: *mut ImFontAtlas) -> *mut ImGuiContext;
    pub fn igDestroyContext(ctx: *mut ImGuiContext);
    pub fn igGetCurrentContext() -> *mut ImGuiContext;
    pub fn igSetCurrentContext(ctx: *mut ImGuiContext);
    pub fn igDebugCheckVersionAndDataLayout(
        version_str: *const c_char,
        sz_io: usize,
        sz_style: usize,
        sz_vec2: usize,
        sz_vec4: usize,
        sz_drawvert: usize,
        sz_drawidx: usize,
    ) -> bool;
}

// Main
extern "C" {
    pub fn igGetIO() -> *mut ImGuiIO;
    pub fn igGetStyle() -> *mut ImGuiStyle;
    pub fn igNewFrame();
    pub fn igEndFrame();
    pub fn igRender();
    pub fn igGetDrawData() -> *mut ImDrawData;
}

// Demo, Debug, Information
extern "C" {
    pub fn igShowDemoWindow(p_open: *mut bool);
    pub fn igShowAboutWindow(p_open: *mut bool);
    pub fn igShowMetricsWindow(p_open: *mut bool);
    pub fn igShowStyleEditor(ref_: *mut ImGuiStyle);
    pub fn igShowStyleSelector(label: *const c_char) -> bool;
    pub fn igShowFontSelector(label: *const c_char);
    pub fn igShowUserGuide();
    pub fn igGetVersion() -> *const c_char;
}

// Styles
extern "C" {
    pub fn igStyleColorsDark(dst: *mut ImGuiStyle);
    pub fn igStyleColorsClassic(dst: *mut ImGuiStyle);
    pub fn igStyleColorsLight(dst: *mut ImGuiStyle);
}

// Windows
extern "C" {
    pub fn igBegin(name: *const c_char, p_open: *mut bool, flags: ImGuiWindowFlags) -> bool;
    pub fn igEnd();
    pub fn igBeginChild(
        str_id: *const c_char,
        size: ImVec2,
        border: bool,
        flags: ImGuiWindowFlags,
    ) -> bool;
    pub fn igBeginChildID(id: ImGuiID, size: ImVec2, border: bool, flags: ImGuiWindowFlags)
        -> bool;
    pub fn igEndChild();
}

// Windows Utilities
extern "C" {
    pub fn igIsWindowAppearing() -> bool;
    pub fn igIsWindowCollapsed() -> bool;
    pub fn igIsWindowFocused(flags: ImGuiFocusedFlags) -> bool;
    pub fn igIsWindowHovered(flags: ImGuiHoveredFlags) -> bool;
    pub fn igGetWindowDrawList() -> *mut ImDrawList;
    pub fn igGetWindowPos() -> ImVec2;
    pub fn igGetWindowPos_nonUDT(pOut: *mut ImVec2);
    pub fn igGetWindowPos_nonUDT2() -> ImVec2;
    pub fn igGetWindowSize() -> ImVec2;
    pub fn igGetWindowSize_nonUDT(pOut: *mut ImVec2);
    pub fn igGetWindowSize_nonUDT2() -> ImVec2;
    pub fn igGetWindowWidth() -> f32;
    pub fn igGetWindowHeight() -> f32;
    pub fn igSetNextWindowPos(pos: ImVec2, cond: ImGuiCond, pivot: ImVec2);
    pub fn igSetNextWindowSize(size: ImVec2, cond: ImGuiCond);
    pub fn igSetNextWindowSizeConstraints(
        size_min: ImVec2,
        size_max: ImVec2,
        custom_callback: ImGuiSizeCallback,
        custom_callback_data: *mut c_void,
    );
    pub fn igSetNextWindowContentSize(size: ImVec2);
    pub fn igSetNextWindowCollapsed(collapsed: bool, cond: ImGuiCond);
    pub fn igSetNextWindowFocus();
    pub fn igSetNextWindowBgAlpha(alpha: f32);
    pub fn igSetWindowPosVec2(pos: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowSizeVec2(size: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowCollapsedBool(collapsed: bool, cond: ImGuiCond);
    pub fn igSetWindowFocus();
    pub fn igSetWindowFontScale(scale: f32);
    pub fn igSetWindowPosStr(name: *const c_char, pos: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowSizeStr(name: *const c_char, size: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowCollapsedStr(name: *const c_char, collapsed: bool, cond: ImGuiCond);
    pub fn igSetWindowFocusStr(name: *const c_char);
    pub fn igGetContentRegionMax() -> ImVec2;
    pub fn igGetContentRegionMax_nonUDT(pOut: *mut ImVec2);
    pub fn igGetContentRegionMax_nonUDT2() -> ImVec2;
    pub fn igGetContentRegionAvail() -> ImVec2;
    pub fn igGetContentRegionAvail_nonUDT(pOut: *mut ImVec2);
    pub fn igGetContentRegionAvail_nonUDT2() -> ImVec2;
    pub fn igGetWindowContentRegionMin() -> ImVec2;
    pub fn igGetWindowContentRegionMin_nonUDT(pOut: *mut ImVec2);
    pub fn igGetWindowContentRegionMin_nonUDT2() -> ImVec2;
    pub fn igGetWindowContentRegionMax() -> ImVec2;
    pub fn igGetWindowContentRegionMax_nonUDT(pOut: *mut ImVec2);
    pub fn igGetWindowContentRegionMax_nonUDT2() -> ImVec2;
    pub fn igGetWindowContentRegionWidth() -> f32;
}

// Windows scrolling
extern "C" {
    pub fn igGetScrollX() -> f32;
    pub fn igGetScrollY() -> f32;
    pub fn igGetScrollMaxX() -> f32;
    pub fn igGetScrollMaxY() -> f32;
    pub fn igSetScrollX(scroll_x: f32);
    pub fn igSetScrollY(scroll_y: f32);
    pub fn igSetScrollHereY(center_y_ratio: f32);
    pub fn igSetScrollFromPosY(local_y: f32, center_y_ratio: f32);
}

// Parameter stacks (shared)
extern "C" {
    pub fn igPushFont(font: *mut ImFont);
    pub fn igPopFont();
    pub fn igPushStyleColorU32(idx: ImGuiCol, col: ImU32);
    pub fn igPushStyleColor(idx: ImGuiCol, col: ImVec4);
    pub fn igPopStyleColor(count: c_int);
    pub fn igPushStyleVarFloat(idx: ImGuiStyleVar, val: f32);
    pub fn igPushStyleVarVec2(idx: ImGuiStyleVar, val: ImVec2);
    pub fn igPopStyleVar(count: c_int);
    pub fn igGetStyleColorVec4(idx: ImGuiCol) -> *const ImVec4;
    pub fn igGetFont() -> *mut ImFont;
    pub fn igGetFontSize() -> f32;
    pub fn igGetFontTexUvWhitePixel() -> ImVec2;
    pub fn igGetFontTexUvWhitePixel_nonUDT(pOut: *mut ImVec2);
    pub fn igGetFontTexUvWhitePixel_nonUDT2() -> ImVec2;
    pub fn igGetColorU32(idx: ImGuiCol, alpha_mul: f32) -> ImU32;
    pub fn igGetColorU32Vec4(col: ImVec4) -> ImU32;
    pub fn igGetColorU32U32(col: ImU32) -> ImU32;
}

// Parameter stack (current window)
extern "C" {
    pub fn igPushItemWidth(item_width: f32);
    pub fn igPopItemWidth();
    pub fn igSetNextItemWidth(item_width: f32);
    pub fn igCalcItemWidth() -> f32;
    pub fn igPushTextWrapPos(wrap_local_pos_x: f32);
    pub fn igPopTextWrapPos();
    pub fn igPushAllowKeyboardFocus(allow_keyboard_focus: bool);
    pub fn igPopAllowKeyboardFocus();
    pub fn igPushButtonRepeat(repeat: bool);
    pub fn igPopButtonRepeat();
}

// Cursor / Layout
extern "C" {
    pub fn igSeparator();
    pub fn igSameLine(offset_from_start_x: f32, spacing: f32);
    pub fn igNewLine();
    pub fn igSpacing();
    pub fn igDummy(size: ImVec2);
    pub fn igIndent(indent_w: f32);
    pub fn igUnindent(indent_w: f32);
    pub fn igBeginGroup();
    pub fn igEndGroup();
    pub fn igGetCursorPos() -> ImVec2;
    pub fn igGetCursorPos_nonUDT(pOut: *mut ImVec2);
    pub fn igGetCursorPos_nonUDT2() -> ImVec2;
    pub fn igGetCursorPosX() -> f32;
    pub fn igGetCursorPosY() -> f32;
    pub fn igSetCursorPos(local_pos: ImVec2);
    pub fn igSetCursorPosX(local_x: f32);
    pub fn igSetCursorPosY(local_y: f32);
    pub fn igGetCursorStartPos() -> ImVec2;
    pub fn igGetCursorStartPos_nonUDT(pOut: *mut ImVec2);
    pub fn igGetCursorStartPos_nonUDT2() -> ImVec2;
    pub fn igGetCursorScreenPos() -> ImVec2;
    pub fn igGetCursorScreenPos_nonUDT(pOut: *mut ImVec2);
    pub fn igGetCursorScreenPos_nonUDT2() -> ImVec2;
    pub fn igSetCursorScreenPos(pos: ImVec2);
    pub fn igAlignTextToFramePadding();
    pub fn igGetTextLineHeight() -> f32;
    pub fn igGetTextLineHeightWithSpacing() -> f32;
    pub fn igGetFrameHeight() -> f32;
    pub fn igGetFrameHeightWithSpacing() -> f32;
}

// ID stack/scopes
extern "C" {
    pub fn igPushIDStr(str_id: *const c_char);
    pub fn igPushIDRange(str_id_begin: *const c_char, str_id_end: *const c_char);
    pub fn igPushIDPtr(ptr_id: *const c_void);
    pub fn igPushIDInt(int_id: c_int);
    pub fn igPopID();
    pub fn igGetIDStr(str_id: *const c_char) -> ImGuiID;
    pub fn igGetIDRange(str_id_begin: *const c_char, str_id_end: *const c_char) -> ImGuiID;
    pub fn igGetIDPtr(ptr_id: *const c_void) -> ImGuiID;
}

// Widgets: Text
extern "C" {
    pub fn igTextUnformatted(text: *const c_char, text_end: *const c_char);
    pub fn igText(fmt: *const c_char, ...);
    pub fn igTextColored(col: ImVec4, fmt: *const c_char, ...);
    pub fn igTextDisabled(fmt: *const c_char, ...);
    pub fn igTextWrapped(fmt: *const c_char, ...);
    pub fn igLabelText(label: *const c_char, fmt: *const c_char, ...);
    pub fn igBulletText(fmt: *const c_char, ...);
}

// Widgets: Main
extern "C" {
    pub fn igButton(label: *const c_char, size: ImVec2) -> bool;
    pub fn igSmallButton(label: *const c_char) -> bool;
    pub fn igInvisibleButton(str_id: *const c_char, size: ImVec2) -> bool;
    pub fn igArrowButton(str_id: *const c_char, dir: ImGuiDir) -> bool;
    pub fn igImage(
        user_texture_id: ImTextureID,
        size: ImVec2,
        uv0: ImVec2,
        uv1: ImVec2,
        tint_col: ImVec4,
        border_col: ImVec4,
    );
    pub fn igImageButton(
        user_texture_id: ImTextureID,
        size: ImVec2,
        uv0: ImVec2,
        uv1: ImVec2,
        frame_padding: c_int,
        bg_col: ImVec4,
        tint_col: ImVec4,
    ) -> bool;
    pub fn igCheckbox(label: *const c_char, v: *mut bool) -> bool;
    pub fn igCheckboxFlags(label: *const c_char, flags: *mut c_uint, flags_value: c_uint) -> bool;
    pub fn igRadioButtonBool(label: *const c_char, active: bool) -> bool;
    pub fn igRadioButtonIntPtr(label: *const c_char, v: *mut c_int, v_button: c_int) -> bool;
    pub fn igProgressBar(fraction: f32, size_arg: ImVec2, overlay: *const c_char);
    pub fn igBullet();
}

// Widgets: Combo Box
extern "C" {
    pub fn igBeginCombo(
        label: *const c_char,
        preview_value: *const c_char,
        flags: ImGuiComboFlags,
    ) -> bool;
    pub fn igEndCombo();
    pub fn igCombo(
        label: *const c_char,
        current_item: *mut c_int,
        items: *const *const c_char,
        items_count: c_int,
        popup_max_height_in_items: c_int,
    ) -> bool;
    pub fn igComboStr(
        label: *const c_char,
        current_item: *mut c_int,
        items_separated_by_zeros: *const c_char,
        popup_max_height_in_items: c_int,
    ) -> bool;
    pub fn igComboFnPtr(
        label: *const c_char,
        current_item: *mut c_int,
        items_getter: Option<
            unsafe extern "C" fn(
                data: *mut c_void,
                idx: c_int,
                out_text: *mut *const c_char,
            ) -> bool,
        >,
        data: *mut c_void,
        items_count: c_int,
        popup_max_height_in_items: c_int,
    ) -> bool;
}

// Widgets: Drags
extern "C" {
    pub fn igDragFloat(
        label: *const c_char,
        v: *mut f32,
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: *const c_char,
        power: f32,
    ) -> bool;
    pub fn igDragFloat2(
        label: *const c_char,
        v: *mut f32,
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: *const c_char,
        power: f32,
    ) -> bool;
    pub fn igDragFloat3(
        label: *const c_char,
        v: *mut f32,
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: *const c_char,
        power: f32,
    ) -> bool;
    pub fn igDragFloat4(
        label: *const c_char,
        v: *mut f32,
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: *const c_char,
        power: f32,
    ) -> bool;
    pub fn igDragFloatRange2(
        label: *const c_char,
        v_current_min: *mut f32,
        v_current_max: *mut f32,
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: *const c_char,
        format_max: *const c_char,
        power: f32,
    ) -> bool;
    pub fn igDragInt(
        label: *const c_char,
        v: *mut c_int,
        v_speed: f32,
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
    ) -> bool;
    pub fn igDragInt2(
        label: *const c_char,
        v: *mut c_int,
        v_speed: f32,
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
    ) -> bool;
    pub fn igDragInt3(
        label: *const c_char,
        v: *mut c_int,
        v_speed: f32,
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
    ) -> bool;
    pub fn igDragInt4(
        label: *const c_char,
        v: *mut c_int,
        v_speed: f32,
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
    ) -> bool;
    pub fn igDragIntRange2(
        label: *const c_char,
        v_current_min: *mut c_int,
        v_current_max: *mut c_int,
        v_speed: f32,
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
        format_max: *const c_char,
    ) -> bool;
    pub fn igDragScalar(
        label: *const c_char,
        data_type: ImGuiDataType,
        v: *mut c_void,
        v_speed: f32,
        v_min: *const c_void,
        v_max: *const c_void,
        format: *const c_char,
        power: f32,
    ) -> bool;
    pub fn igDragScalarN(
        label: *const c_char,
        data_type: ImGuiDataType,
        v: *mut c_void,
        components: c_int,
        v_speed: f32,
        v_min: *const c_void,
        v_max: *const c_void,
        format: *const c_char,
        power: f32,
    ) -> bool;
}

// Widgets: Sliders
extern "C" {
    pub fn igSliderFloat(
        label: *const c_char,
        v: *mut f32,
        v_min: f32,
        v_max: f32,
        format: *const c_char,
        power: f32,
    ) -> bool;
    pub fn igSliderFloat2(
        label: *const c_char,
        v: *mut f32,
        v_min: f32,
        v_max: f32,
        format: *const c_char,
        power: f32,
    ) -> bool;
    pub fn igSliderFloat3(
        label: *const c_char,
        v: *mut f32,
        v_min: f32,
        v_max: f32,
        format: *const c_char,
        power: f32,
    ) -> bool;
    pub fn igSliderFloat4(
        label: *const c_char,
        v: *mut f32,
        v_min: f32,
        v_max: f32,
        format: *const c_char,
        power: f32,
    ) -> bool;
    pub fn igSliderAngle(
        label: *const c_char,
        v_rad: *mut f32,
        v_degrees_min: f32,
        v_degrees_max: f32,
        format: *const c_char,
    ) -> bool;
    pub fn igSliderInt(
        label: *const c_char,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
    ) -> bool;
    pub fn igSliderInt2(
        label: *const c_char,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
    ) -> bool;
    pub fn igSliderInt3(
        label: *const c_char,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
    ) -> bool;
    pub fn igSliderInt4(
        label: *const c_char,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
    ) -> bool;
    pub fn igSliderScalar(
        label: *const c_char,
        data_type: ImGuiDataType,
        v: *mut c_void,
        v_min: *const c_void,
        v_max: *const c_void,
        format: *const c_char,
        power: f32,
    ) -> bool;
    pub fn igSliderScalarN(
        label: *const c_char,
        data_type: ImGuiDataType,
        v: *mut c_void,
        components: c_int,
        v_min: *const c_void,
        v_max: *const c_void,
        format: *const c_char,
        power: f32,
    ) -> bool;
    pub fn igVSliderFloat(
        label: *const c_char,
        size: ImVec2,
        v: *mut f32,
        v_min: f32,
        v_max: f32,
        format: *const c_char,
        power: f32,
    ) -> bool;
    pub fn igVSliderInt(
        label: *const c_char,
        size: ImVec2,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
    ) -> bool;
    pub fn igVSliderScalar(
        label: *const c_char,
        size: ImVec2,
        data_type: ImGuiDataType,
        v: *mut c_void,
        v_min: *const c_void,
        v_max: *const c_void,
        format: *const c_char,
        power: f32,
    ) -> bool;
}

// Widgets: Input with Keyboard
extern "C" {
    pub fn igInputText(
        label: *const c_char,
        buf: *mut c_char,
        buf_size: usize,
        flags: ImGuiInputTextFlags,
        callback: ImGuiInputTextCallback,
        user_data: *mut c_void,
    ) -> bool;
    pub fn igInputTextMultiline(
        label: *const c_char,
        buf: *mut c_char,
        buf_size: usize,
        size: ImVec2,
        flags: ImGuiInputTextFlags,
        callback: ImGuiInputTextCallback,
        user_data: *mut c_void,
    ) -> bool;
    pub fn igInputTextWithHint(
        label: *const c_char,
        hint: *const c_char,
        buf: *mut c_char,
        buf_size: usize,
        flags: ImGuiInputTextFlags,
        callback: ImGuiInputTextCallback,
        user_data: *mut c_void,
    ) -> bool;
    pub fn igInputFloat(
        label: *const c_char,
        v: *mut f32,
        step: f32,
        step_fast: f32,
        format: *const c_char,
        flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputFloat2(
        label: *const c_char,
        v: *mut f32,
        format: *const c_char,
        flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputFloat3(
        label: *const c_char,
        v: *mut f32,
        format: *const c_char,
        flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputFloat4(
        label: *const c_char,
        v: *mut f32,
        format: *const c_char,
        flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputInt(
        label: *const c_char,
        v: *mut c_int,
        step: c_int,
        step_fast: c_int,
        flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputInt2(label: *const c_char, v: *mut c_int, flags: ImGuiInputTextFlags) -> bool;
    pub fn igInputInt3(label: *const c_char, v: *mut c_int, flags: ImGuiInputTextFlags) -> bool;
    pub fn igInputInt4(label: *const c_char, v: *mut c_int, flags: ImGuiInputTextFlags) -> bool;
    pub fn igInputDouble(
        label: *const c_char,
        v: *mut f64,
        step: f64,
        step_fast: f64,
        format: *const c_char,
        flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputScalar(
        label: *const c_char,
        data_type: ImGuiDataType,
        v: *mut c_void,
        step: *const c_void,
        step_fast: *const c_void,
        format: *const c_char,
        flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputScalarN(
        label: *const c_char,
        data_type: ImGuiDataType,
        v: *mut c_void,
        components: c_int,
        step: *const c_void,
        step_fast: *const c_void,
        format: *const c_char,
        flags: ImGuiInputTextFlags,
    ) -> bool;
}

// Widgets: Color Editor/Picker
extern "C" {
    pub fn igColorEdit3(label: *const c_char, col: *mut f32, flags: ImGuiColorEditFlags) -> bool;
    pub fn igColorEdit4(label: *const c_char, col: *mut f32, flags: ImGuiColorEditFlags) -> bool;
    pub fn igColorPicker3(label: *const c_char, col: *mut f32, flags: ImGuiColorEditFlags) -> bool;
    pub fn igColorPicker4(
        label: *const c_char,
        col: *mut f32,
        flags: ImGuiColorEditFlags,
        ref_col: *const f32,
    ) -> bool;
    pub fn igColorButton(
        desc_id: *const c_char,
        col: ImVec4,
        flags: ImGuiColorEditFlags,
        size: ImVec2,
    ) -> bool;
    pub fn igSetColorEditOptions(flags: ImGuiColorEditFlags);
}

// Widgets: Trees
extern "C" {
    pub fn igTreeNodeStr(label: *const c_char) -> bool;
    pub fn igTreeNodeStrStr(str_id: *const c_char, fmt: *const c_char, ...) -> bool;
    pub fn igTreeNodePtr(ptr_id: *const c_void, fmt: *const c_char, ...) -> bool;
    pub fn igTreeNodeExStr(label: *const c_char, flags: ImGuiTreeNodeFlags) -> bool;
    pub fn igTreeNodeExStrStr(
        str_id: *const c_char,
        flags: ImGuiTreeNodeFlags,
        fmt: *const c_char,
        ...
    ) -> bool;
    pub fn igTreeNodeExPtr(
        ptr_id: *const c_void,
        flags: ImGuiTreeNodeFlags,
        fmt: *const c_char,
        ...
    ) -> bool;
    pub fn igTreePushStr(str_id: *const c_char);
    pub fn igTreePushPtr(ptr_id: *const c_void);
    pub fn igTreePop();
    pub fn igTreeAdvanceToLabelPos();
    pub fn igGetTreeNodeToLabelSpacing() -> f32;
    pub fn igCollapsingHeader(label: *const c_char, flags: ImGuiTreeNodeFlags) -> bool;
    pub fn igCollapsingHeaderBoolPtr(
        label: *const c_char,
        p_open: *mut bool,
        flags: ImGuiTreeNodeFlags,
    ) -> bool;
    pub fn igSetNextItemOpen(is_open: bool, cond: ImGuiCond);
}

// Widgets: Selectables
extern "C" {
    pub fn igSelectable(
        label: *const c_char,
        selected: bool,
        flags: ImGuiSelectableFlags,
        size: ImVec2,
    ) -> bool;
    pub fn igSelectableBoolPtr(
        label: *const c_char,
        p_selected: *mut bool,
        flags: ImGuiSelectableFlags,
        size: ImVec2,
    ) -> bool;
}

// Widgets: List Boxes
extern "C" {
    pub fn igListBoxStr_arr(
        label: *const c_char,
        current_item: *mut c_int,
        items: *const *const c_char,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
    pub fn igListBoxFnPtr(
        label: *const c_char,
        current_item: *mut c_int,
        items_getter: Option<
            unsafe extern "C" fn(
                data: *mut c_void,
                idx: c_int,
                out_text: *mut *const c_char,
            ) -> bool,
        >,
        data: *mut c_void,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
    pub fn igListBoxHeaderVec2(label: *const c_char, size: ImVec2) -> bool;
    pub fn igListBoxHeaderInt(
        label: *const c_char,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
    pub fn igListBoxFooter();
}

// Widgets: Data Plotting
extern "C" {
    pub fn igPlotLines(
        label: *const c_char,
        values: *const f32,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: *const c_char,
        scale_min: f32,
        scale_max: f32,
        graph_size: ImVec2,
        stride: c_int,
    );
    pub fn igPlotLinesFnPtr(
        label: *const c_char,
        values_getter: Option<unsafe extern "C" fn(data: *mut c_void, idx: c_int) -> f32>,
        data: *mut c_void,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: *const c_char,
        scale_min: f32,
        scale_max: f32,
        graph_size: ImVec2,
    );
    pub fn igPlotHistogramFloatPtr(
        label: *const c_char,
        values: *const f32,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: *const c_char,
        scale_min: f32,
        scale_max: f32,
        graph_size: ImVec2,
        stride: c_int,
    );
    pub fn igPlotHistogramFnPtr(
        label: *const c_char,
        values_getter: Option<unsafe extern "C" fn(data: *mut c_void, idx: c_int) -> f32>,
        data: *mut c_void,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: *const c_char,
        scale_min: f32,
        scale_max: f32,
        graph_size: ImVec2,
    );
}

// Widgets: Value() Helpers
extern "C" {
    pub fn igValueBool(prefix: *const c_char, b: bool);
    pub fn igValueInt(prefix: *const c_char, v: c_int);
    pub fn igValueUint(prefix: *const c_char, v: c_uint);
    pub fn igValueFloat(prefix: *const c_char, v: f32, float_format: *const c_char);
}

// Widgets: Menus
extern "C" {
    pub fn igBeginMainMenuBar() -> bool;
    pub fn igEndMainMenuBar();
    pub fn igBeginMenuBar() -> bool;
    pub fn igEndMenuBar();
    pub fn igBeginMenu(label: *const c_char, enabled: bool) -> bool;
    pub fn igEndMenu();
    pub fn igMenuItemBool(
        label: *const c_char,
        shortcut: *const c_char,
        selected: bool,
        enabled: bool,
    ) -> bool;
    pub fn igMenuItemBoolPtr(
        label: *const c_char,
        shortcut: *const c_char,
        p_selected: *mut bool,
        enabled: bool,
    ) -> bool;
}

// Tooltips
extern "C" {
    pub fn igBeginTooltip();
    pub fn igEndTooltip();
    pub fn igSetTooltip(fmt: *const c_char, ...);
}

// Popups
extern "C" {
    pub fn igOpenPopup(str_id: *const c_char);
    pub fn igBeginPopup(str_id: *const c_char, flags: ImGuiWindowFlags) -> bool;
    pub fn igBeginPopupContextItem(str_id: *const c_char, mouse_button: c_int) -> bool;
    pub fn igBeginPopupContextWindow(
        str_id: *const c_char,
        mouse_button: c_int,
        also_over_items: bool,
    ) -> bool;
    pub fn igBeginPopupContextVoid(str_id: *const c_char, mouse_button: c_int) -> bool;
    pub fn igBeginPopupModal(
        name: *const c_char,
        p_open: *mut bool,
        flags: ImGuiWindowFlags,
    ) -> bool;
    pub fn igEndPopup();
    pub fn igOpenPopupOnItemClick(str_id: *const c_char, mouse_button: c_int) -> bool;
    pub fn igIsPopupOpen(str_id: *const c_char) -> bool;
    pub fn igCloseCurrentPopup();
}

// Columns
extern "C" {
    pub fn igColumns(count: c_int, id: *const c_char, border: bool);
    pub fn igNextColumn();
    pub fn igGetColumnIndex() -> c_int;
    pub fn igGetColumnWidth(column_index: c_int) -> f32;
    pub fn igSetColumnWidth(column_index: c_int, width: f32);
    pub fn igGetColumnOffset(column_index: c_int) -> f32;
    pub fn igSetColumnOffset(column_index: c_int, offset_x: f32);
    pub fn igGetColumnsCount() -> c_int;
}

// Tab bar
extern "C" {
    pub fn igBeginTabBar(str_id: *const c_char, flags: ImGuiTabBarFlags) -> bool;
    pub fn igEndTabBar();
    pub fn igBeginTabItem(
        label: *const c_char,
        p_open: *mut bool,
        flags: ImGuiTabItemFlags,
    ) -> bool;
    pub fn igEndTabItem();
    pub fn igSetTabItemClosed(tab_or_docked_window_label: *const c_char);
}

// Logging/Capture
extern "C" {
    pub fn igLogToTTY(auto_open_depth: c_int);
    pub fn igLogToFile(auto_open_depth: c_int, filename: *const c_char);
    pub fn igLogToClipboard(auto_open_depth: c_int);
    pub fn igLogFinish();
    pub fn igLogButtons();
    pub fn igLogText(fmt: *const c_char, ...);
}

// Drag and Drop
extern "C" {
    /// Call when current ID is active.
    ///
    /// When this returns true you need to:
    ///
    /// 1. call [`igSetDragDropPayload`] exactly once,
    /// 2. you may render the payload visual/description,
    /// 3. pcall [`igEndDragDropSource`]
    pub fn igBeginDragDropSource(flags: ImGuiDragDropFlags) -> bool;
    /// Use 'cond' to choose to submit payload on drag start or every frame
    pub fn igSetDragDropPayload(
        type_: *const c_char,
        data: *const c_void,
        sz: usize,
        cond: ImGuiCond,
    ) -> bool;
    pub fn igEndDragDropSource();
    pub fn igBeginDragDropTarget() -> bool;
    pub fn igAcceptDragDropPayload(
        type_: *const c_char,
        flags: ImGuiDragDropFlags,
    ) -> *const ImGuiPayload;
    pub fn igEndDragDropTarget();
    pub fn igGetDragDropPayload() -> *const ImGuiPayload;
}

// Clipping
extern "C" {
    pub fn igPushClipRect(
        clip_rect_min: ImVec2,
        clip_rect_max: ImVec2,
        intersect_with_current_clip_rect: bool,
    );
    pub fn igPopClipRect();
}

// Focus
extern "C" {
    pub fn igSetItemDefaultFocus();
    pub fn igSetKeyboardFocusHere(offset: c_int);
}

// Utilities
extern "C" {
    pub fn igIsItemHovered(flags: ImGuiHoveredFlags) -> bool;
    pub fn igIsItemActive() -> bool;
    pub fn igIsItemFocused() -> bool;
    pub fn igIsItemClicked(mouse_button: c_int) -> bool;
    pub fn igIsItemVisible() -> bool;
    pub fn igIsItemEdited() -> bool;
    pub fn igIsItemActivated() -> bool;
    pub fn igIsItemDeactivated() -> bool;
    pub fn igIsItemDeactivatedAfterEdit() -> bool;
    pub fn igIsAnyItemHovered() -> bool;
    pub fn igIsAnyItemActive() -> bool;
    pub fn igIsAnyItemFocused() -> bool;
    pub fn igGetItemRectMin() -> ImVec2;
    pub fn igGetItemRectMin_nonUDT(pOut: *mut ImVec2);
    pub fn igGetItemRectMin_nonUDT2() -> ImVec2;
    pub fn igGetItemRectMax() -> ImVec2;
    pub fn igGetItemRectMax_nonUDT(pOut: *mut ImVec2);
    pub fn igGetItemRectMax_nonUDT2() -> ImVec2;
    pub fn igGetItemRectSize() -> ImVec2;
    pub fn igGetItemRectSize_nonUDT(pOut: *mut ImVec2);
    pub fn igGetItemRectSize_nonUDT2() -> ImVec2;
    pub fn igSetItemAllowOverlap();
    pub fn igIsRectVisible(size: ImVec2) -> bool;
    pub fn igIsRectVisibleVec2(rect_min: ImVec2, rect_max: ImVec2) -> bool;
    pub fn igGetTime() -> f64;
    pub fn igGetFrameCount() -> c_int;
    pub fn igGetBackgroundDrawList() -> *mut ImDrawList;
    pub fn igGetForegroundDrawList() -> *mut ImDrawList;
    pub fn igGetDrawListSharedData() -> *mut ImDrawListSharedData;
    pub fn igGetStyleColorName(idx: ImGuiCol) -> *const c_char;
    pub fn igSetStateStorage(storage: *mut ImGuiStorage);
    pub fn igGetStateStorage() -> *mut ImGuiStorage;
    pub fn igCalcTextSize(
        text: *const c_char,
        text_end: *const c_char,
        hide_text_after_double_hash: bool,
        wrap_width: f32,
    ) -> ImVec2;
    pub fn igCalcTextSize_nonUDT(
        pOut: *mut ImVec2,
        text: *const c_char,
        text_end: *const c_char,
        hide_text_after_double_hash: bool,
        wrap_width: f32,
    );
    pub fn igCalcTextSize_nonUDT2(
        text: *const c_char,
        text_end: *const c_char,
        hide_text_after_double_hash: bool,
        wrap_width: f32,
    ) -> ImVec2;
    pub fn igCalcListClipping(
        items_count: c_int,
        items_height: f32,
        out_items_display_start: *mut c_int,
        out_items_display_end: *mut c_int,
    );
    pub fn igBeginChildFrame(id: ImGuiID, size: ImVec2, flags: ImGuiWindowFlags) -> bool;
    pub fn igEndChildFrame();
    pub fn igColorConvertU32ToFloat4(in_: ImU32) -> ImVec4;
    pub fn igColorConvertU32ToFloat4_nonUDT(pOut: *mut ImVec4, in_: ImU32);
    pub fn igColorConvertU32ToFloat4_nonUDT2(in_: ImU32) -> ImVec4;
    pub fn igColorConvertFloat4ToU32(in_: ImVec4) -> ImU32;
    pub fn igGET_FLT_MAX() -> f32;
    pub fn igColorConvertRGBtoHSV(
        r: f32,
        g: f32,
        b: f32,
        out_h: *mut f32,
        out_s: *mut f32,
        out_v: *mut f32,
    );
    pub fn igColorConvertHSVtoRGB(
        h: f32,
        s: f32,
        v: f32,
        out_r: *mut f32,
        out_g: *mut f32,
        out_b: *mut f32,
    );
}

// Inputs
extern "C" {
    pub fn igGetKeyIndex(imgui_key: ImGuiKey) -> c_int;
    pub fn igIsKeyDown(user_key_index: c_int) -> bool;
    pub fn igIsKeyPressed(user_key_index: c_int, repeat: bool) -> bool;
    pub fn igIsKeyReleased(user_key_index: c_int) -> bool;
    pub fn igGetKeyPressedAmount(key_index: c_int, repeat_delay: f32, rate: f32) -> c_int;
    pub fn igIsMouseDown(button: c_int) -> bool;
    pub fn igIsAnyMouseDown() -> bool;
    pub fn igIsMouseClicked(button: c_int, repeat: bool) -> bool;
    pub fn igIsMouseDoubleClicked(button: c_int) -> bool;
    pub fn igIsMouseReleased(button: c_int) -> bool;
    pub fn igIsMouseDragging(button: c_int, lock_threshold: f32) -> bool;
    pub fn igIsMouseHoveringRect(r_min: ImVec2, r_max: ImVec2, clip: bool) -> bool;
    pub fn igIsMousePosValid(mouse_pos: *const ImVec2) -> bool;
    pub fn igGetMousePos() -> ImVec2;
    pub fn igGetMousePos_nonUDT(pOut: *mut ImVec2);
    pub fn igGetMousePos_nonUDT2() -> ImVec2;
    pub fn igGetMousePosOnOpeningCurrentPopup() -> ImVec2;
    pub fn igGetMousePosOnOpeningCurrentPopup_nonUDT(pOut: *mut ImVec2);
    pub fn igGetMousePosOnOpeningCurrentPopup_nonUDT2() -> ImVec2;
    pub fn igGetMouseDragDelta(button: c_int, lock_threshold: f32) -> ImVec2;
    pub fn igGetMouseDragDelta_nonUDT(pOut: *mut ImVec2, button: c_int, lock_threshold: f32);
    pub fn igGetMouseDragDelta_nonUDT2(button: c_int, lock_threshold: f32) -> ImVec2;
    pub fn igResetMouseDragDelta(button: c_int);
    pub fn igGetMouseCursor() -> ImGuiMouseCursor;
    pub fn igSetMouseCursor(type_: ImGuiMouseCursor);
    pub fn igCaptureKeyboardFromApp(want_capture_keyboard_value: bool);
    pub fn igCaptureMouseFromApp(want_capture_mouse_value: bool);
}

// Clipboard utilities
extern "C" {
    pub fn igGetClipboardText() -> *const c_char;
    pub fn igSetClipboardText(text: *const c_char);
}

// Settings/.Ini Utilities
extern "C" {
    pub fn igLoadIniSettingsFromDisk(ini_filename: *const c_char);
    pub fn igLoadIniSettingsFromMemory(ini_data: *const c_char, ini_size: usize);
    pub fn igSaveIniSettingsToDisk(ini_filename: *const c_char);
    pub fn igSaveIniSettingsToMemory(out_ini_size: *mut usize) -> *const c_char;
}

// Memory Utilities
extern "C" {
    pub fn igSetAllocatorFunctions(
        alloc_func: Option<unsafe extern "C" fn(sz: usize, user_data: *mut c_void) -> *mut c_void>,
        free_func: Option<unsafe extern "C" fn(ptr: *mut c_void, user_data: *mut c_void)>,
        user_data: *mut c_void,
    );
    pub fn igMemAlloc(size: usize) -> *mut c_void;
    pub fn igMemFree(ptr: *mut c_void);
}
