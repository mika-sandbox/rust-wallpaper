use std::ptr::null_mut;

use objc::runtime::{Class, Object, BOOL};
use objc::*;

#[link(name = "AppKit", kind = "framework")]
extern "system" {}

pub fn apply_change(path: &str) -> Result<(), ()> {
  let cls = Class::get("NSString").unwrap();
  let path: *mut Object = unsafe { msg_send![cls, stringWithUTF8String: path] };

  let cls = Class::get("NSURL").unwrap();
  let path: *mut Object = unsafe { msg_send![cls, fileURLWithPath: path] };

  let cls = Class::get("NSScreen").unwrap();
  let screen: *mut Object = unsafe { msg_send![cls, mainScreen] };

  let cls = Class::get("NSWorkspace").unwrap();
  let workspace: *mut Object = unsafe { msg_send![cls, sharedWorkspace] };

  let options: *mut Object = unsafe { msg_send![workspace, desktopImageOptionsForScreen: screen] };

  let mut error = null_mut::<Object>();

  let r: BOOL = unsafe {
    msg_send![workspace, setDesktopImageURL: path forScreen: screen options: options error: &mut error]
  };

  match r as BOOL {
    1 => Ok(()),
    _ => Err(()),
  }
}
