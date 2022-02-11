#![cfg(windows)]

mod ats_plugin;
use ats_plugin::*;

const ARRAY_LENGTH: usize = 256;

use std::cell::RefCell;

thread_local! {
    static POWER: RefCell<i32> = RefCell::new(0);
    static BRAKE: RefCell<i32> = RefCell::new(0);
    static REVERSER: RefCell<i32> = RefCell::new(0);
}

use winapi::shared::minwindef;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(_dll_module: HINSTANCE, call_reason: DWORD, _reserved: LPVOID) -> BOOL {
    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_THREAD_ATTACH: DWORD = 2;
    const DLL_THREAD_DETACH: DWORD = 3;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => (),
        DLL_THREAD_ATTACH => (),
        DLL_THREAD_DETACH => (),
        DLL_PROCESS_DETACH => (),
        _ => (),
    }

    minwindef::TRUE
}

// Called when this plug-in is loaded
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Load() {}

// Called when this plug_in is unloaded
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Dispose() {}

// Returns the version numbers of ATS plug-in
#[no_mangle]
pub extern "system" fn GetPluginVersion() -> i32 {
    ATS_VERSION
}

// Called when the train is loaded
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn SetVehicleSpec(_vehicle_spec: AtsVehicleSpec) {}

// Called when the game is started
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Initialize(_brake: i32) {}

// Called every frame
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Elapse(
    _vehicle_state: AtsVehicleState,
    p_panel: *mut i32,
    p_sound: *mut i32,
) -> AtsHandles {
    let _panel = unsafe { std::slice::from_raw_parts_mut(p_panel, ARRAY_LENGTH) };
    let _sound = unsafe { std::slice::from_raw_parts_mut(p_sound, ARRAY_LENGTH) };

    AtsHandles {
        brake: BRAKE.with(|value| *value.borrow()),
        power: POWER.with(|value| *value.borrow()),
        reverser: REVERSER.with(|value| *value.borrow()),
        constant_speed: ATS_CONSTANTSPEED_CONTINUE,
    }
}

// Called when the power is changed
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn SetPower(_notch: i32) {
    POWER.with(|value| {
        *value.borrow_mut() = _notch;
    });
}

// Called when the brake is changed
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn SetBrake(_notch: i32) {
    BRAKE.with(|value| {
        *value.borrow_mut() = _notch;
    });
}

// Called when the reverser is changed
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn SetReverser(_pos: i32) {
    REVERSER.with(|value| {
        *value.borrow_mut() = _pos;
    });
}

// Called when any ATS key is pressed
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn KeyDown(_ats_key_code: i32) {}

// Called when any ATS key is released
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn KeyUp(_ats_key_code: i32) {}

// Called when the horn is used
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn HornBlow(_horn_type: i32) {}

// Called when the door is opened
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DoorOpen() {}

// Called when the door is closed
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DoorClose() {}

// Called when current signal is changed
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn SetSignal(_signal: i32) {}

// Called when the beacon data is received
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn SetBeaconData(_beacon_data: AtsBeaconData) {}
