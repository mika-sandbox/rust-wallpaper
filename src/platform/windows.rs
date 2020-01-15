use std::ptr::null_mut;

use com::{
    com_interface, interfaces::iunknown::IUnknown, runtime::ApartmentThreadedRuntime as Runtime,
    ComInterface, InterfacePtr, InterfaceRc,
};

use winapi::{
    ctypes::c_void,
    shared::{
        guiddef::{IID, REFCLSID, REFIID},
        minwindef::LPVOID,
        winerror::HRESULT,
        wtypesbase::CLSCTX_LOCAL_SERVER,
    },
    um::{
        combaseapi::CoCreateInstance,
        unknwnbase::LPUNKNOWN,
        winnt::{LPCWSTR, LPWSTR},
    },
};

// C2CF3110-460E-4fc1-B9D0-8A1C0C9CC4BD
const CLSID_DESKTOP_WALLPAPER: IID = IID {
    Data1: 0xC2CF3110,
    Data2: 0x460E,
    Data3: 0x4FC1,
    Data4: [0xB9, 0xD0, 0x8A, 0x1C, 0x0C, 0x9C, 0xC4, 0xBD],
};

// Extension Methods for ApartmentThreadedRuntime
// All of implements are based on Original ApartmentThreadedRuntime
trait RuntimeExt {
    fn create_instance_ext<T: ComInterface + ?Sized>(
        &self,
        clsid: &IID,
        cls_ctx: u32,
    ) -> Result<InterfaceRc<T>, HRESULT>;

    unsafe fn create_raw_instance_ext<T: ComInterface + ?Sized>(
        &self,
        clsid: &IID,
        cls_ctx: u32,
        outer: LPUNKNOWN,
    ) -> Result<InterfacePtr<T>, HRESULT>;
}

impl RuntimeExt for Runtime {
    fn create_instance_ext<T: ComInterface + ?Sized>(
        &self,
        clsid: &IID,
        cls_ctx: u32,
    ) -> Result<InterfaceRc<T>, HRESULT> {
        unsafe {
            Ok(InterfaceRc::new(self.create_raw_instance_ext::<T>(
                clsid,
                cls_ctx,
                null_mut(),
            )?))
        }
    }

    unsafe fn create_raw_instance_ext<T: ComInterface + ?Sized>(
        &self,
        clsid: &IID,
        cls_ctx: u32,
        outer: LPUNKNOWN,
    ) -> Result<InterfacePtr<T>, HRESULT> {
        let mut instance = null_mut::<c_void>();
        let hr = CoCreateInstance(
            clsid as REFCLSID,
            outer,
            cls_ctx,
            &T::IID as REFIID,
            &mut instance as *mut LPVOID,
        );

        if hr != 0 {
            return Err(hr);
        }

        Ok(InterfacePtr::new(instance))
    }
}

#[com_interface("B92B56A9-8B55-4E14-9A89-0199BBB6F93B")]
pub trait IDesktopWallpaper: IUnknown {
    // define IDesktopWallpaper COM Object
    // see um/ShObjIdl_core.h

    fn set_wallpaper(&self, monitor_id: LPCWSTR, wallpaper: LPCWSTR) -> HRESULT;
    fn get_wallpaper(&self, monitor_id: LPCWSTR, wallpaper: LPWSTR) -> HRESULT;
    fn get_monitor_device_path_at(&self, monitor_index: u32, monitor_id: LPWSTR) -> HRESULT;
    fn get_monitor_device_path_count(&self);
    fn get_monitor_rect(&self);
    fn set_background_color(&self);
    fn get_background_color(&self);
    fn set_position(&self);
    fn get_position(&self);
    fn set_slideshow(&self);
    fn get_slideshow(&self);
    fn set_slideshow_options(&self);
    fn get_slideshow_options(&self);
    fn advance_slideshow(&self);
    fn get_status(&self);
    fn enable(&self);
}

pub fn apply_change(path: &str) -> Result<(), ()> {
    let runtime = Runtime::new().expect("Failed to initialize COM Library");
    let wallpaper = runtime
        .create_instance_ext::<dyn IDesktopWallpaper>(&CLSID_DESKTOP_WALLPAPER, CLSCTX_LOCAL_SERVER)
        .unwrap_or_else(|hr| panic!("Failed to get DesktopWallpaper object: HRESULT={:x}", hr));

    wallpaper.set_wallpaper(null_mut(), to_wstr(path).as_ptr());

    Ok(())
}

fn to_wstr(string: &str) -> Vec<u16> {
    string.encode_utf16().chain(Some(0)).collect()
}
