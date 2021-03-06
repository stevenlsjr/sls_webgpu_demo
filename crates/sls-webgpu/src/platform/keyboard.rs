use bitflags::bitflags;

/// Replica of
/// SDL2's keycode type
///
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Keycode {
  Backspace,
  Tab,
  Return,
  Escape,
  Space,
  Exclaim,
  Quotedbl,
  Hash,
  Dollar,
  Percent,
  Ampersand,
  Quote,
  LeftParen,
  RightParen,
  Asterisk,
  Plus,
  Comma,
  Minus,
  Period,
  Slash,
  Num0,
  Num1,
  Num2,
  Num3,
  Num4,
  Num5,
  Num6,
  Num7,
  Num8,
  Num9,
  Colon,
  Semicolon,
  Less,
  Equals,
  Greater,
  Question,
  At,
  LeftBracket,
  Backslash,
  RightBracket,
  Caret,
  Underscore,
  Backquote,
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
  Delete,
  CapsLock,
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
  PrintScreen,
  ScrollLock,
  Pause,
  Insert,
  Home,
  PageUp,
  End,
  PageDown,
  Right,
  Left,
  Down,
  Up,
  NumLockClear,
  KpDivide,
  KpMultiply,
  KpMinus,
  KpPlus,
  KpEnter,
  Kp1,
  Kp2,
  Kp3,
  Kp4,
  Kp5,
  Kp6,
  Kp7,
  Kp8,
  Kp9,
  Kp0,
  KpPeriod,
  Application,
  Power,
  KpEquals,
  F13,
  F14,
  F15,
  F16,
  F17,
  F18,
  F19,
  F20,
  F21,
  F22,
  F23,
  F24,
  Execute,
  Help,
  Menu,
  Select,
  Stop,
  Again,
  Undo,
  Cut,
  Copy,
  Paste,
  Find,
  Mute,
  VolumeUp,
  VolumeDown,
  KpComma,
  KpEqualsAS400,
  AltErase,
  Sysreq,
  Cancel,
  Clear,
  Prior,
  Return2,
  Separator,
  Out,
  Oper,
  ClearAgain,
  CrSel,
  ExSel,
  Kp00,
  Kp000,
  ThousandsSeparator,
  DecimalSeparator,
  CurrencyUnit,
  CurrencySubUnit,
  KpLeftParen,
  KpRightParen,
  KpLeftBrace,
  KpRightBrace,
  KpTab,
  KpBackspace,
  KpA,
  KpB,
  KpC,
  KpD,
  KpE,
  KpF,
  KpXor,
  KpPower,
  KpPercent,
  KpLess,
  KpGreater,
  KpAmpersand,
  KpDblAmpersand,
  KpVerticalBar,
  KpDblVerticalBar,
  KpColon,
  KpHash,
  KpSpace,
  KpAt,
  KpExclam,
  KpMemStore,
  KpMemRecall,
  KpMemClear,
  KpMemAdd,
  KpMemSubtract,
  KpMemMultiply,
  KpMemDivide,
  KpPlusMinus,
  KpClear,
  KpClearEntry,
  KpBinary,
  KpOctal,
  KpDecimal,
  KpHexadecimal,
  LCtrl,
  LShift,
  LAlt,
  LGui,
  RCtrl,
  RShift,
  RAlt,
  RGui,
  Mode,
  AudioNext,
  AudioPrev,
  AudioStop,
  AudioPlay,
  AudioMute,
  MediaSelect,
  Www,
  Mail,
  Calculator,
  Computer,
  AcSearch,
  AcHome,
  AcBack,
  AcForward,
  AcStop,
  AcRefresh,
  AcBookmarks,
  BrightnessDown,
  BrightnessUp,
  DisplaySwitch,
  KbdIllumToggle,
  KbdIllumDown,
  KbdIllumUp,
  Eject,
  Sleep,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Scancode {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
  Num1,
  Num2,
  Num3,
  Num4,
  Num5,
  Num6,
  Num7,
  Num8,
  Num9,
  Num0,
  Return,
  Escape,
  Backspace,
  Tab,
  Space,
  Minus,
  Equals,
  LeftBracket,
  RightBracket,
  Backslash,
  NonUsHash,
  Semicolon,
  Apostrophe,
  Grave,
  Comma,
  Period,
  Slash,
  CapsLock,
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
  PrintScreen,
  ScrollLock,
  Pause,
  Insert,
  Home,
  PageUp,
  Delete,
  End,
  PageDown,
  Right,
  Left,
  Down,
  Up,
  NumLockClear,
  KpDivide,
  KpMultiply,
  KpMinus,
  KpPlus,
  KpEnter,
  Kp1,
  Kp2,
  Kp3,
  Kp4,
  Kp5,
  Kp6,
  Kp7,
  Kp8,
  Kp9,
  Kp0,
  KpPeriod,
  NonUsBackslash,
  Application,
  Power,
  KpEquals,
  F13,
  F14,
  F15,
  F16,
  F17,
  F18,
  F19,
  F20,
  F21,
  F22,
  F23,
  F24,
  Execute,
  Help,
  Menu,
  Select,
  Stop,
  Again,
  Undo,
  Cut,
  Copy,
  Paste,
  Find,
  Mute,
  VolumeUp,
  VolumeDown,
  KpComma,
  KpEqualsAS400,
  International1,
  International2,
  International3,
  International4,
  International5,
  International6,
  International7,
  International8,
  International9,
  Lang1,
  Lang2,
  Lang3,
  Lang4,
  Lang5,
  Lang6,
  Lang7,
  Lang8,
  Lang9,
  AltErase,
  SysReq,
  Cancel,
  Clear,
  Prior,
  Return2,
  Separator,
  Out,
  Oper,
  ClearAgain,
  CrSel,
  ExSel,
  Kp00,
  Kp000,
  ThousandsSeparator,
  DecimalSeparator,
  CurrencyUnit,
  CurrencySubUnit,
  KpLeftParen,
  KpRightParen,
  KpLeftBrace,
  KpRightBrace,
  KpTab,
  KpBackspace,
  KpA,
  KpB,
  KpC,
  KpD,
  KpE,
  KpF,
  KpXor,
  KpPower,
  KpPercent,
  KpLess,
  KpGreater,
  KpAmpersand,
  KpDblAmpersand,
  KpVerticalBar,
  KpDblVerticalBar,
  KpColon,
  KpHash,
  KpSpace,
  KpAt,
  KpExclam,
  KpMemStore,
  KpMemRecall,
  KpMemClear,
  KpMemAdd,
  KpMemSubtract,
  KpMemMultiply,
  KpMemDivide,
  KpPlusMinus,
  KpClear,
  KpClearEntry,
  KpBinary,
  KpOctal,
  KpDecimal,
  KpHexadecimal,
  LCtrl,
  LShift,
  LAlt,
  LGui,
  RCtrl,
  RShift,
  RAlt,
  RGui,
  Mode,
  AudioNext,
  AudioPrev,
  AudioStop,
  AudioPlay,
  AudioMute,
  MediaSelect,
  Www,
  Mail,
  Calculator,
  Computer,
  AcSearch,
  AcHome,
  AcBack,
  AcForward,
  AcStop,
  AcRefresh,
  AcBookmarks,
  BrightnessDown,
  BrightnessUp,
  DisplaySwitch,
  KbdIllumToggle,
  KbdIllumDown,
  KbdIllumUp,
  Eject,
  Sleep,
  App1,
  App2,
  Num,
}
bitflags! {
  /// Key modifier mask.
  /// Copied from SDL2
  #[derive()]
  pub struct KeyMod: u16 {
    const NOMOD = 0x0000;
    const LSHIFTMOD = 0x0001;
    const RSHIFTMOD = 0x0002;
    const LCTRLMOD = 0x0040;
    const RCTRLMOD = 0x0080;
    const LALTMOD = 0x0100;
    const RALTMOD = 0x0200;
    const LGUIMOD = 0x0400;
    const RGUIMOD = 0x0800;
    const NUMMOD = 0x1000;
    const CAPSMOD = 0x2000;
    const MODEMOD = 0x4000;
    const RESERVEDMOD = 0x8000;
  }
}

#[cfg(feature = "sdl2_backend")]
mod sdl_backend {
  use super::*;
  use sdl2::keyboard::{Keycode as SdlKeycode, Mod as SdlMod, Scancode as SdlScancode};

  impl From<SdlMod> for KeyMod {
    fn from(sdl_mod: SdlMod) -> Self {
      unsafe { Self::from_bits_unchecked(sdl_mod.bits()) }
    }
  }

  impl Into<SdlMod> for KeyMod {
    fn into(self) -> SdlMod {
      unsafe { SdlMod::from_bits_unchecked(self.bits) }
    }
  }

  impl From<SdlKeycode> for Keycode {
    fn from(code: SdlKeycode) -> Self {
      match code {
        SdlKeycode::Backspace => Keycode::Backspace,
        SdlKeycode::Tab => Keycode::Tab,
        SdlKeycode::Return => Keycode::Return,
        SdlKeycode::Escape => Keycode::Escape,
        SdlKeycode::Space => Keycode::Space,
        SdlKeycode::Exclaim => Keycode::Exclaim,
        SdlKeycode::Quotedbl => Keycode::Quotedbl,
        SdlKeycode::Hash => Keycode::Hash,
        SdlKeycode::Dollar => Keycode::Dollar,
        SdlKeycode::Percent => Keycode::Percent,
        SdlKeycode::Ampersand => Keycode::Ampersand,
        SdlKeycode::Quote => Keycode::Quote,
        SdlKeycode::LeftParen => Keycode::LeftParen,
        SdlKeycode::RightParen => Keycode::RightParen,
        SdlKeycode::Asterisk => Keycode::Asterisk,
        SdlKeycode::Plus => Keycode::Plus,
        SdlKeycode::Comma => Keycode::Comma,
        SdlKeycode::Minus => Keycode::Minus,
        SdlKeycode::Period => Keycode::Period,
        SdlKeycode::Slash => Keycode::Slash,
        SdlKeycode::Num0 => Keycode::Num0,
        SdlKeycode::Num1 => Keycode::Num1,
        SdlKeycode::Num2 => Keycode::Num2,
        SdlKeycode::Num3 => Keycode::Num3,
        SdlKeycode::Num4 => Keycode::Num4,
        SdlKeycode::Num5 => Keycode::Num5,
        SdlKeycode::Num6 => Keycode::Num6,
        SdlKeycode::Num7 => Keycode::Num7,
        SdlKeycode::Num8 => Keycode::Num8,
        SdlKeycode::Num9 => Keycode::Num9,
        SdlKeycode::Colon => Keycode::Colon,
        SdlKeycode::Semicolon => Keycode::Semicolon,
        SdlKeycode::Less => Keycode::Less,
        SdlKeycode::Equals => Keycode::Equals,
        SdlKeycode::Greater => Keycode::Greater,
        SdlKeycode::Question => Keycode::Question,
        SdlKeycode::At => Keycode::At,
        SdlKeycode::LeftBracket => Keycode::LeftBracket,
        SdlKeycode::Backslash => Keycode::Backslash,
        SdlKeycode::RightBracket => Keycode::RightBracket,
        SdlKeycode::Caret => Keycode::Caret,
        SdlKeycode::Underscore => Keycode::Underscore,
        SdlKeycode::Backquote => Keycode::Backquote,
        SdlKeycode::A => Keycode::A,
        SdlKeycode::B => Keycode::B,
        SdlKeycode::C => Keycode::C,
        SdlKeycode::D => Keycode::D,
        SdlKeycode::E => Keycode::E,
        SdlKeycode::F => Keycode::F,
        SdlKeycode::G => Keycode::G,
        SdlKeycode::H => Keycode::H,
        SdlKeycode::I => Keycode::I,
        SdlKeycode::J => Keycode::J,
        SdlKeycode::K => Keycode::K,
        SdlKeycode::L => Keycode::L,
        SdlKeycode::M => Keycode::M,
        SdlKeycode::N => Keycode::N,
        SdlKeycode::O => Keycode::O,
        SdlKeycode::P => Keycode::P,
        SdlKeycode::Q => Keycode::Q,
        SdlKeycode::R => Keycode::R,
        SdlKeycode::S => Keycode::S,
        SdlKeycode::T => Keycode::T,
        SdlKeycode::U => Keycode::U,
        SdlKeycode::V => Keycode::V,
        SdlKeycode::W => Keycode::W,
        SdlKeycode::X => Keycode::X,
        SdlKeycode::Y => Keycode::Y,
        SdlKeycode::Z => Keycode::Z,
        SdlKeycode::Delete => Keycode::Delete,
        SdlKeycode::CapsLock => Keycode::CapsLock,
        SdlKeycode::F1 => Keycode::F1,
        SdlKeycode::F2 => Keycode::F2,
        SdlKeycode::F3 => Keycode::F3,
        SdlKeycode::F4 => Keycode::F4,
        SdlKeycode::F5 => Keycode::F5,
        SdlKeycode::F6 => Keycode::F6,
        SdlKeycode::F7 => Keycode::F7,
        SdlKeycode::F8 => Keycode::F8,
        SdlKeycode::F9 => Keycode::F9,
        SdlKeycode::F10 => Keycode::F10,
        SdlKeycode::F11 => Keycode::F11,
        SdlKeycode::F12 => Keycode::F12,
        SdlKeycode::PrintScreen => Keycode::PrintScreen,
        SdlKeycode::ScrollLock => Keycode::ScrollLock,
        SdlKeycode::Pause => Keycode::Pause,
        SdlKeycode::Insert => Keycode::Insert,
        SdlKeycode::Home => Keycode::Home,
        SdlKeycode::PageUp => Keycode::PageUp,
        SdlKeycode::End => Keycode::End,
        SdlKeycode::PageDown => Keycode::PageDown,
        SdlKeycode::Right => Keycode::Right,
        SdlKeycode::Left => Keycode::Left,
        SdlKeycode::Down => Keycode::Down,
        SdlKeycode::Up => Keycode::Up,
        SdlKeycode::NumLockClear => Keycode::NumLockClear,
        SdlKeycode::KpDivide => Keycode::KpDivide,
        SdlKeycode::KpMultiply => Keycode::KpMultiply,
        SdlKeycode::KpMinus => Keycode::KpMinus,
        SdlKeycode::KpPlus => Keycode::KpPlus,
        SdlKeycode::KpEnter => Keycode::KpEnter,
        SdlKeycode::Kp1 => Keycode::Kp1,
        SdlKeycode::Kp2 => Keycode::Kp2,
        SdlKeycode::Kp3 => Keycode::Kp3,
        SdlKeycode::Kp4 => Keycode::Kp4,
        SdlKeycode::Kp5 => Keycode::Kp5,
        SdlKeycode::Kp6 => Keycode::Kp6,
        SdlKeycode::Kp7 => Keycode::Kp7,
        SdlKeycode::Kp8 => Keycode::Kp8,
        SdlKeycode::Kp9 => Keycode::Kp9,
        SdlKeycode::Kp0 => Keycode::Kp0,
        SdlKeycode::KpPeriod => Keycode::KpPeriod,
        SdlKeycode::Application => Keycode::Application,
        SdlKeycode::Power => Keycode::Power,
        SdlKeycode::KpEquals => Keycode::KpEquals,
        SdlKeycode::F13 => Keycode::F13,
        SdlKeycode::F14 => Keycode::F14,
        SdlKeycode::F15 => Keycode::F15,
        SdlKeycode::F16 => Keycode::F16,
        SdlKeycode::F17 => Keycode::F17,
        SdlKeycode::F18 => Keycode::F18,
        SdlKeycode::F19 => Keycode::F19,
        SdlKeycode::F20 => Keycode::F20,
        SdlKeycode::F21 => Keycode::F21,
        SdlKeycode::F22 => Keycode::F22,
        SdlKeycode::F23 => Keycode::F23,
        SdlKeycode::F24 => Keycode::F24,
        SdlKeycode::Execute => Keycode::Execute,
        SdlKeycode::Help => Keycode::Help,
        SdlKeycode::Menu => Keycode::Menu,
        SdlKeycode::Select => Keycode::Select,
        SdlKeycode::Stop => Keycode::Stop,
        SdlKeycode::Again => Keycode::Again,
        SdlKeycode::Undo => Keycode::Undo,
        SdlKeycode::Cut => Keycode::Cut,
        SdlKeycode::Copy => Keycode::Copy,
        SdlKeycode::Paste => Keycode::Paste,
        SdlKeycode::Find => Keycode::Find,
        SdlKeycode::Mute => Keycode::Mute,
        SdlKeycode::VolumeUp => Keycode::VolumeUp,
        SdlKeycode::VolumeDown => Keycode::VolumeDown,
        SdlKeycode::KpComma => Keycode::KpComma,
        SdlKeycode::KpEqualsAS400 => Keycode::KpEqualsAS400,
        SdlKeycode::AltErase => Keycode::AltErase,
        SdlKeycode::Sysreq => Keycode::Sysreq,
        SdlKeycode::Cancel => Keycode::Cancel,
        SdlKeycode::Clear => Keycode::Clear,
        SdlKeycode::Prior => Keycode::Prior,
        SdlKeycode::Return2 => Keycode::Return2,
        SdlKeycode::Separator => Keycode::Separator,
        SdlKeycode::Out => Keycode::Out,
        SdlKeycode::Oper => Keycode::Oper,
        SdlKeycode::ClearAgain => Keycode::ClearAgain,
        SdlKeycode::CrSel => Keycode::CrSel,
        SdlKeycode::ExSel => Keycode::ExSel,
        SdlKeycode::Kp00 => Keycode::Kp00,
        SdlKeycode::Kp000 => Keycode::Kp000,
        SdlKeycode::ThousandsSeparator => Keycode::ThousandsSeparator,
        SdlKeycode::DecimalSeparator => Keycode::DecimalSeparator,
        SdlKeycode::CurrencyUnit => Keycode::CurrencyUnit,
        SdlKeycode::CurrencySubUnit => Keycode::CurrencySubUnit,
        SdlKeycode::KpLeftParen => Keycode::KpLeftParen,
        SdlKeycode::KpRightParen => Keycode::KpRightParen,
        SdlKeycode::KpLeftBrace => Keycode::KpLeftBrace,
        SdlKeycode::KpRightBrace => Keycode::KpRightBrace,
        SdlKeycode::KpTab => Keycode::KpTab,
        SdlKeycode::KpBackspace => Keycode::KpBackspace,
        SdlKeycode::KpA => Keycode::KpA,
        SdlKeycode::KpB => Keycode::KpB,
        SdlKeycode::KpC => Keycode::KpC,
        SdlKeycode::KpD => Keycode::KpD,
        SdlKeycode::KpE => Keycode::KpE,
        SdlKeycode::KpF => Keycode::KpF,
        SdlKeycode::KpXor => Keycode::KpXor,
        SdlKeycode::KpPower => Keycode::KpPower,
        SdlKeycode::KpPercent => Keycode::KpPercent,
        SdlKeycode::KpLess => Keycode::KpLess,
        SdlKeycode::KpGreater => Keycode::KpGreater,
        SdlKeycode::KpAmpersand => Keycode::KpAmpersand,
        SdlKeycode::KpDblAmpersand => Keycode::KpDblAmpersand,
        SdlKeycode::KpVerticalBar => Keycode::KpVerticalBar,
        SdlKeycode::KpDblVerticalBar => Keycode::KpDblVerticalBar,
        SdlKeycode::KpColon => Keycode::KpColon,
        SdlKeycode::KpHash => Keycode::KpHash,
        SdlKeycode::KpSpace => Keycode::KpSpace,
        SdlKeycode::KpAt => Keycode::KpAt,
        SdlKeycode::KpExclam => Keycode::KpExclam,
        SdlKeycode::KpMemStore => Keycode::KpMemStore,
        SdlKeycode::KpMemRecall => Keycode::KpMemRecall,
        SdlKeycode::KpMemClear => Keycode::KpMemClear,
        SdlKeycode::KpMemAdd => Keycode::KpMemAdd,
        SdlKeycode::KpMemSubtract => Keycode::KpMemSubtract,
        SdlKeycode::KpMemMultiply => Keycode::KpMemMultiply,
        SdlKeycode::KpMemDivide => Keycode::KpMemDivide,
        SdlKeycode::KpPlusMinus => Keycode::KpPlusMinus,
        SdlKeycode::KpClear => Keycode::KpClear,
        SdlKeycode::KpClearEntry => Keycode::KpClearEntry,
        SdlKeycode::KpBinary => Keycode::KpBinary,
        SdlKeycode::KpOctal => Keycode::KpOctal,
        SdlKeycode::KpDecimal => Keycode::KpDecimal,
        SdlKeycode::KpHexadecimal => Keycode::KpHexadecimal,
        SdlKeycode::LCtrl => Keycode::LCtrl,
        SdlKeycode::LShift => Keycode::LShift,
        SdlKeycode::LAlt => Keycode::LAlt,
        SdlKeycode::LGui => Keycode::LGui,
        SdlKeycode::RCtrl => Keycode::RCtrl,
        SdlKeycode::RShift => Keycode::RShift,
        SdlKeycode::RAlt => Keycode::RAlt,
        SdlKeycode::RGui => Keycode::RGui,
        SdlKeycode::Mode => Keycode::Mode,
        SdlKeycode::AudioNext => Keycode::AudioNext,
        SdlKeycode::AudioPrev => Keycode::AudioPrev,
        SdlKeycode::AudioStop => Keycode::AudioStop,
        SdlKeycode::AudioPlay => Keycode::AudioPlay,
        SdlKeycode::AudioMute => Keycode::AudioMute,
        SdlKeycode::MediaSelect => Keycode::MediaSelect,
        SdlKeycode::Www => Keycode::Www,
        SdlKeycode::Mail => Keycode::Mail,
        SdlKeycode::Calculator => Keycode::Calculator,
        SdlKeycode::Computer => Keycode::Computer,
        SdlKeycode::AcSearch => Keycode::AcSearch,
        SdlKeycode::AcHome => Keycode::AcHome,
        SdlKeycode::AcBack => Keycode::AcBack,
        SdlKeycode::AcForward => Keycode::AcForward,
        SdlKeycode::AcStop => Keycode::AcStop,
        SdlKeycode::AcRefresh => Keycode::AcRefresh,
        SdlKeycode::AcBookmarks => Keycode::AcBookmarks,
        SdlKeycode::BrightnessDown => Keycode::BrightnessDown,
        SdlKeycode::BrightnessUp => Keycode::BrightnessUp,
        SdlKeycode::DisplaySwitch => Keycode::DisplaySwitch,
        SdlKeycode::KbdIllumToggle => Keycode::KbdIllumToggle,
        SdlKeycode::KbdIllumDown => Keycode::KbdIllumDown,
        SdlKeycode::KbdIllumUp => Keycode::KbdIllumUp,
        SdlKeycode::Eject => Keycode::Eject,
        SdlKeycode::Sleep => Keycode::Sleep,
      }
    }
  }

  impl From<SdlScancode> for Scancode {
    fn from(code: SdlScancode) -> Self {
      match code {
        SdlScancode::A => Scancode::A,
        SdlScancode::B => Scancode::B,
        SdlScancode::C => Scancode::C,
        SdlScancode::D => Scancode::D,
        SdlScancode::E => Scancode::E,
        SdlScancode::F => Scancode::F,
        SdlScancode::G => Scancode::G,
        SdlScancode::H => Scancode::H,
        SdlScancode::I => Scancode::I,
        SdlScancode::J => Scancode::J,
        SdlScancode::K => Scancode::K,
        SdlScancode::L => Scancode::L,
        SdlScancode::M => Scancode::M,
        SdlScancode::N => Scancode::N,
        SdlScancode::O => Scancode::O,
        SdlScancode::P => Scancode::P,
        SdlScancode::Q => Scancode::Q,
        SdlScancode::R => Scancode::R,
        SdlScancode::S => Scancode::S,
        SdlScancode::T => Scancode::T,
        SdlScancode::U => Scancode::U,
        SdlScancode::V => Scancode::V,
        SdlScancode::W => Scancode::W,
        SdlScancode::X => Scancode::X,
        SdlScancode::Y => Scancode::Y,
        SdlScancode::Z => Scancode::Z,
        SdlScancode::Num1 => Scancode::Num1,
        SdlScancode::Num2 => Scancode::Num2,
        SdlScancode::Num3 => Scancode::Num3,
        SdlScancode::Num4 => Scancode::Num4,
        SdlScancode::Num5 => Scancode::Num5,
        SdlScancode::Num6 => Scancode::Num6,
        SdlScancode::Num7 => Scancode::Num7,
        SdlScancode::Num8 => Scancode::Num8,
        SdlScancode::Num9 => Scancode::Num9,
        SdlScancode::Num0 => Scancode::Num0,
        SdlScancode::Return => Scancode::Return,
        SdlScancode::Escape => Scancode::Escape,
        SdlScancode::Backspace => Scancode::Backspace,
        SdlScancode::Tab => Scancode::Tab,
        SdlScancode::Space => Scancode::Space,
        SdlScancode::Minus => Scancode::Minus,
        SdlScancode::Equals => Scancode::Equals,
        SdlScancode::LeftBracket => Scancode::LeftBracket,
        SdlScancode::RightBracket => Scancode::RightBracket,
        SdlScancode::Backslash => Scancode::Backslash,
        SdlScancode::NonUsHash => Scancode::NonUsHash,
        SdlScancode::Semicolon => Scancode::Semicolon,
        SdlScancode::Apostrophe => Scancode::Apostrophe,
        SdlScancode::Grave => Scancode::Grave,
        SdlScancode::Comma => Scancode::Comma,
        SdlScancode::Period => Scancode::Period,
        SdlScancode::Slash => Scancode::Slash,
        SdlScancode::CapsLock => Scancode::CapsLock,
        SdlScancode::F1 => Scancode::F1,
        SdlScancode::F2 => Scancode::F2,
        SdlScancode::F3 => Scancode::F3,
        SdlScancode::F4 => Scancode::F4,
        SdlScancode::F5 => Scancode::F5,
        SdlScancode::F6 => Scancode::F6,
        SdlScancode::F7 => Scancode::F7,
        SdlScancode::F8 => Scancode::F8,
        SdlScancode::F9 => Scancode::F9,
        SdlScancode::F10 => Scancode::F10,
        SdlScancode::F11 => Scancode::F11,
        SdlScancode::F12 => Scancode::F12,
        SdlScancode::PrintScreen => Scancode::PrintScreen,
        SdlScancode::ScrollLock => Scancode::ScrollLock,
        SdlScancode::Pause => Scancode::Pause,
        SdlScancode::Insert => Scancode::Insert,
        SdlScancode::Home => Scancode::Home,
        SdlScancode::PageUp => Scancode::PageUp,
        SdlScancode::Delete => Scancode::Delete,
        SdlScancode::End => Scancode::End,
        SdlScancode::PageDown => Scancode::PageDown,
        SdlScancode::Right => Scancode::Right,
        SdlScancode::Left => Scancode::Left,
        SdlScancode::Down => Scancode::Down,
        SdlScancode::Up => Scancode::Up,
        SdlScancode::NumLockClear => Scancode::NumLockClear,
        SdlScancode::KpDivide => Scancode::KpDivide,
        SdlScancode::KpMultiply => Scancode::KpMultiply,
        SdlScancode::KpMinus => Scancode::KpMinus,
        SdlScancode::KpPlus => Scancode::KpPlus,
        SdlScancode::KpEnter => Scancode::KpEnter,
        SdlScancode::Kp1 => Scancode::Kp1,
        SdlScancode::Kp2 => Scancode::Kp2,
        SdlScancode::Kp3 => Scancode::Kp3,
        SdlScancode::Kp4 => Scancode::Kp4,
        SdlScancode::Kp5 => Scancode::Kp5,
        SdlScancode::Kp6 => Scancode::Kp6,
        SdlScancode::Kp7 => Scancode::Kp7,
        SdlScancode::Kp8 => Scancode::Kp8,
        SdlScancode::Kp9 => Scancode::Kp9,
        SdlScancode::Kp0 => Scancode::Kp0,
        SdlScancode::KpPeriod => Scancode::KpPeriod,
        SdlScancode::NonUsBackslash => Scancode::NonUsBackslash,
        SdlScancode::Application => Scancode::Application,
        SdlScancode::Power => Scancode::Power,
        SdlScancode::KpEquals => Scancode::KpEquals,
        SdlScancode::F13 => Scancode::F13,
        SdlScancode::F14 => Scancode::F14,
        SdlScancode::F15 => Scancode::F15,
        SdlScancode::F16 => Scancode::F16,
        SdlScancode::F17 => Scancode::F17,
        SdlScancode::F18 => Scancode::F18,
        SdlScancode::F19 => Scancode::F19,
        SdlScancode::F20 => Scancode::F20,
        SdlScancode::F21 => Scancode::F21,
        SdlScancode::F22 => Scancode::F22,
        SdlScancode::F23 => Scancode::F23,
        SdlScancode::F24 => Scancode::F24,
        SdlScancode::Execute => Scancode::Execute,
        SdlScancode::Help => Scancode::Help,
        SdlScancode::Menu => Scancode::Menu,
        SdlScancode::Select => Scancode::Select,
        SdlScancode::Stop => Scancode::Stop,
        SdlScancode::Again => Scancode::Again,
        SdlScancode::Undo => Scancode::Undo,
        SdlScancode::Cut => Scancode::Cut,
        SdlScancode::Copy => Scancode::Copy,
        SdlScancode::Paste => Scancode::Paste,
        SdlScancode::Find => Scancode::Find,
        SdlScancode::Mute => Scancode::Mute,
        SdlScancode::VolumeUp => Scancode::VolumeUp,
        SdlScancode::VolumeDown => Scancode::VolumeDown,
        SdlScancode::KpComma => Scancode::KpComma,
        SdlScancode::KpEqualsAS400 => Scancode::KpEqualsAS400,
        SdlScancode::International1 => Scancode::International1,
        SdlScancode::International2 => Scancode::International2,
        SdlScancode::International3 => Scancode::International3,
        SdlScancode::International4 => Scancode::International4,
        SdlScancode::International5 => Scancode::International5,
        SdlScancode::International6 => Scancode::International6,
        SdlScancode::International7 => Scancode::International7,
        SdlScancode::International8 => Scancode::International8,
        SdlScancode::International9 => Scancode::International9,
        SdlScancode::Lang1 => Scancode::Lang1,
        SdlScancode::Lang2 => Scancode::Lang2,
        SdlScancode::Lang3 => Scancode::Lang3,
        SdlScancode::Lang4 => Scancode::Lang4,
        SdlScancode::Lang5 => Scancode::Lang5,
        SdlScancode::Lang6 => Scancode::Lang6,
        SdlScancode::Lang7 => Scancode::Lang7,
        SdlScancode::Lang8 => Scancode::Lang8,
        SdlScancode::Lang9 => Scancode::Lang9,
        SdlScancode::AltErase => Scancode::AltErase,
        SdlScancode::SysReq => Scancode::SysReq,
        SdlScancode::Cancel => Scancode::Cancel,
        SdlScancode::Clear => Scancode::Clear,
        SdlScancode::Prior => Scancode::Prior,
        SdlScancode::Return2 => Scancode::Return2,
        SdlScancode::Separator => Scancode::Separator,
        SdlScancode::Out => Scancode::Out,
        SdlScancode::Oper => Scancode::Oper,
        SdlScancode::ClearAgain => Scancode::ClearAgain,
        SdlScancode::CrSel => Scancode::CrSel,
        SdlScancode::ExSel => Scancode::ExSel,
        SdlScancode::Kp00 => Scancode::Kp00,
        SdlScancode::Kp000 => Scancode::Kp000,
        SdlScancode::ThousandsSeparator => Scancode::ThousandsSeparator,
        SdlScancode::DecimalSeparator => Scancode::DecimalSeparator,
        SdlScancode::CurrencyUnit => Scancode::CurrencyUnit,
        SdlScancode::CurrencySubUnit => Scancode::CurrencySubUnit,
        SdlScancode::KpLeftParen => Scancode::KpLeftParen,
        SdlScancode::KpRightParen => Scancode::KpRightParen,
        SdlScancode::KpLeftBrace => Scancode::KpLeftBrace,
        SdlScancode::KpRightBrace => Scancode::KpRightBrace,
        SdlScancode::KpTab => Scancode::KpTab,
        SdlScancode::KpBackspace => Scancode::KpBackspace,
        SdlScancode::KpA => Scancode::KpA,
        SdlScancode::KpB => Scancode::KpB,
        SdlScancode::KpC => Scancode::KpC,
        SdlScancode::KpD => Scancode::KpD,
        SdlScancode::KpE => Scancode::KpE,
        SdlScancode::KpF => Scancode::KpF,
        SdlScancode::KpXor => Scancode::KpXor,
        SdlScancode::KpPower => Scancode::KpPower,
        SdlScancode::KpPercent => Scancode::KpPercent,
        SdlScancode::KpLess => Scancode::KpLess,
        SdlScancode::KpGreater => Scancode::KpGreater,
        SdlScancode::KpAmpersand => Scancode::KpAmpersand,
        SdlScancode::KpDblAmpersand => Scancode::KpDblAmpersand,
        SdlScancode::KpVerticalBar => Scancode::KpVerticalBar,
        SdlScancode::KpDblVerticalBar => Scancode::KpDblVerticalBar,
        SdlScancode::KpColon => Scancode::KpColon,
        SdlScancode::KpHash => Scancode::KpHash,
        SdlScancode::KpSpace => Scancode::KpSpace,
        SdlScancode::KpAt => Scancode::KpAt,
        SdlScancode::KpExclam => Scancode::KpExclam,
        SdlScancode::KpMemStore => Scancode::KpMemStore,
        SdlScancode::KpMemRecall => Scancode::KpMemRecall,
        SdlScancode::KpMemClear => Scancode::KpMemClear,
        SdlScancode::KpMemAdd => Scancode::KpMemAdd,
        SdlScancode::KpMemSubtract => Scancode::KpMemSubtract,
        SdlScancode::KpMemMultiply => Scancode::KpMemMultiply,
        SdlScancode::KpMemDivide => Scancode::KpMemDivide,
        SdlScancode::KpPlusMinus => Scancode::KpPlusMinus,
        SdlScancode::KpClear => Scancode::KpClear,
        SdlScancode::KpClearEntry => Scancode::KpClearEntry,
        SdlScancode::KpBinary => Scancode::KpBinary,
        SdlScancode::KpOctal => Scancode::KpOctal,
        SdlScancode::KpDecimal => Scancode::KpDecimal,
        SdlScancode::KpHexadecimal => Scancode::KpHexadecimal,
        SdlScancode::LCtrl => Scancode::LCtrl,
        SdlScancode::LShift => Scancode::LShift,
        SdlScancode::LAlt => Scancode::LAlt,
        SdlScancode::LGui => Scancode::LGui,
        SdlScancode::RCtrl => Scancode::RCtrl,
        SdlScancode::RShift => Scancode::RShift,
        SdlScancode::RAlt => Scancode::RAlt,
        SdlScancode::RGui => Scancode::RGui,
        SdlScancode::Mode => Scancode::Mode,
        SdlScancode::AudioNext => Scancode::AudioNext,
        SdlScancode::AudioPrev => Scancode::AudioPrev,
        SdlScancode::AudioStop => Scancode::AudioStop,
        SdlScancode::AudioPlay => Scancode::AudioPlay,
        SdlScancode::AudioMute => Scancode::AudioMute,
        SdlScancode::MediaSelect => Scancode::MediaSelect,
        SdlScancode::Www => Scancode::Www,
        SdlScancode::Mail => Scancode::Mail,
        SdlScancode::Calculator => Scancode::Calculator,
        SdlScancode::Computer => Scancode::Computer,
        SdlScancode::AcSearch => Scancode::AcSearch,
        SdlScancode::AcHome => Scancode::AcHome,
        SdlScancode::AcBack => Scancode::AcBack,
        SdlScancode::AcForward => Scancode::AcForward,
        SdlScancode::AcStop => Scancode::AcStop,
        SdlScancode::AcRefresh => Scancode::AcRefresh,
        SdlScancode::AcBookmarks => Scancode::AcBookmarks,
        SdlScancode::BrightnessDown => Scancode::BrightnessDown,
        SdlScancode::BrightnessUp => Scancode::BrightnessUp,
        SdlScancode::DisplaySwitch => Scancode::DisplaySwitch,
        SdlScancode::KbdIllumToggle => Scancode::KbdIllumToggle,
        SdlScancode::KbdIllumDown => Scancode::KbdIllumDown,
        SdlScancode::KbdIllumUp => Scancode::KbdIllumUp,
        SdlScancode::Eject => Scancode::Eject,
        SdlScancode::Sleep => Scancode::Sleep,
        SdlScancode::App1 => Scancode::App1,
        SdlScancode::App2 => Scancode::App2,
        SdlScancode::Num => Scancode::Num,
      }
    }
  }
}

#[cfg(feature = "sdl2_backend")]
pub use sdl_backend::*;
