use std::env;
use std::os::raw::c_char;
use once_cell::sync::Lazy;

// Picks the correct extension depending on OS
fn get_extension(system: &str) -> &str {
    if system == "windows" {
        ".dll"
    } else {
        ".so"
    }
}

// Load library for CAS BACnet Stack once
fn load_library() -> libloading::Library {
    unsafe {
        return libloading::Library::new("./bin/CASBACnetStack_x64_Debug".to_owned() + get_extension(env::consts::OS)).unwrap();
    }
}

static lib: Lazy<libloading::Library> = Lazy::new(|| {
    load_library()
});

// Versioning
pub fn get_api_major_version() -> Result<u32, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn() -> u32> = lib.get(b"BACnetStack_GetAPIMajorVersion")?;
        Ok(func())
    }
}
pub fn get_api_minor_version() -> Result<u32, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn() -> u32> = lib.get(b"BACnetStack_GetAPIMinorVersion")?;
        Ok(func())
    }
}
pub fn get_api_patch_version() -> Result<u32, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn() -> u32> = lib.get(b"BACnetStack_GetAPIPatchVersion")?;
        Ok(func())
    }
}
pub fn get_api_build_version() -> Result<u32, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn() -> u32> = lib.get(b"BACnetStack_GetAPIBuildVersion")?;
        Ok(func())
    }
}

// Main Loop
pub fn bacnet_loop() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn()> = lib.get(b"BACnetStack_Loop")?;
        Ok(func())
    }
}
pub fn bacnet_tick() -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn() -> bool> = lib.get(b"BACnetStack_Tick")?;
        Ok(func())
    }
}

// Device Setup Functions
pub fn add_device(device_instance: u32) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32) -> bool> = lib.get(b"BACnetStack_AddDevice")?;
        Ok(func(device_instance))
    }
}
pub fn add_object(device_instance: u32, object_type: u16, object_instance: u32) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u16, u32) -> bool> = lib.get(b"BACnetStack_AddObject")?;
        Ok(func(device_instance, object_type, object_instance))
    }
}
pub fn add_network_port_object(a: u32, b: u16, c: u8, d: u8, e: u32) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u16, u8, u8, u32) -> bool> = lib.get(b"BACnetStack_AddNetworkPortObject")?;
        Ok(func(a, b, c, d, e))
    }
}
pub fn add_trend_log_object(a: u32, b: u32, c: u16, d: u32, e: u32, f: u32, g: bool, h: u32) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u32, u16, u32, u32, u32, bool, u32) -> bool> = lib.get(b"BACnetStack_AddTrendLogObject")?;
        Ok(func(a, b, c, d, e, f, g, h))
    }
}
pub fn add_trend_log_multiple_object(a: u32, b: u32, c: u32) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u32, u32) -> bool> = lib.get(b"BACnetStack_AddTrendLogMultipleObject")?;
        Ok(func(a, b, c))
    }
}
pub fn add_notification_class_object(a: u32, b: u32, c: u8, d: u8, e: u8, f: bool, g: bool, h: bool) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u32, u8, u8, u8, bool, bool, bool) -> bool> = lib.get(b"BACnetStack_AddNotificationClassObject")?;
        Ok(func(a, b, c, d, e, f, g, h))
    }
}

// Property Setup Functions
pub fn set_property_enabled(device_instance: u32, object_type: u16, object_instance: u32, property_identifier: u32, enabled: bool) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u16, u32, u32, bool) -> bool> = lib.get(b"BACnetStack_SetPropertyEnabled")?;
        Ok(func(device_instance, object_type, object_instance, property_identifier, enabled))
    }
}
pub fn set_property_by_object_type_enabled(device_instance: u32, object_type: u16, property_identifier: u32, enabled: bool) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u16, u32, bool) -> bool> = lib.get(b"BACnetStack_SetPropertyByObjectTypeEnabled")?;
        Ok(func(device_instance, object_type, property_identifier, enabled))
    }
}
pub fn set_proprietary_property(a: u32, b: u16, c: u32, d: u32, e: bool, f: bool, g: u32, h: bool, i: bool, j: bool) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u16, u32, u32, bool, bool, u32, bool, bool, bool) -> bool> = lib.get(b"BACnetStack_SetProprietaryProperty")?;
        Ok(func(a, b, c, d, e, f, g, h, i, j))
    }
}
pub fn set_property_writable(a: u32, b: u16, c: u32, d: u32, e: bool) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u16, u32, u32, bool) -> bool> = lib.get(b"BACnetStack_SetPropertyWritable")?;
        Ok(func(a, b, c, d, e))
    }
}
pub fn set_property_by_object_type_writable(a: u32, b: u16, c: u32, d: bool) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u16, u32, bool) -> bool> = lib.get(b"BACnetStack_SetPropertyByObjectTypeWritable")?;
        Ok(func(a, b, c, d))
    }
}
pub fn set_property_subscribable(a: u32, b: u16, c: u32, d: u32, e: bool) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u16, u32, u32, bool) -> bool> = lib.get(b"BACnetStack_SetPropertySubscribable")?;
        Ok(func(a, b, c, d, e))
    }
}
pub fn set_property_by_object_type_subscribable(a: u32, b: u16, c: u32, d: bool) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u16, u32, bool) -> bool> = lib.get(b"BACnetStack_SetPropertyByObjectTypeSubscribable")?;
        Ok(func(a, b, c, d))
    }
}
pub fn set_object_type_creatable(a: u32, b: u16, c: bool) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u16, bool) -> bool> = lib.get(b"BACnetStack_SetObjectTypeCreatable")?;
        Ok(func(a, b, c))
    }
}
pub fn set_object_type_supported(a: u32, b: u16, c: bool) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u16, bool) -> bool> = lib.get(b"BACnetStack_SetObjectTypeSupported")?;
        Ok(func(a, b, c))
    }
}
pub fn set_service_enabled(device_instance: u32, service: u32, enabled: bool) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u32, bool) -> bool> = lib.get(b"BACnetStack_SetServiceEnabled")?;
        Ok(func(device_instance, service, enabled))
    }
}
pub fn set_max_active_cov_subscriptions(a: u32, b: u32) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u32) -> bool> = lib.get(b"BACnetStack_SetMaxActiveCOVSubscriptions")?;
        Ok(func(a, b))
    }
}
pub fn set_cov_settings(a: u32, b: u32, c: u32) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u32, u32) -> bool> = lib.get(b"BACnetStack_SetCOVSettings")?;
        Ok(func(a, b, c))
    }
}
pub fn remove_device(a: u32) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32)> = lib.get(b"BACnetStack_RemoveDevice")?;
        Ok(func(a))
    }
}
pub fn remove_object(a: u32, b: u16, c: u32) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u16, u32) -> bool> = lib.get(b"BACnetStack_RemoveObject")?;
        Ok(func(a, b, c))
    }
}


// Alarm and Event Setup Functions
pub fn enable_alarms_and_events_for_object(a: u32, b: u16, c: u32, d: u32, e: u8, f: bool, g: bool, h: bool, i: bool) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u16, u32, u32, u8, bool, bool, bool, bool) -> bool> = lib.get(b"BACnetStack_EnableAlarmsAndEventsForObject")?;
        Ok(func(a, b, c, d, e, f, g, h, i))
    }
}

// Data Notification Functions
pub fn value_updated(a: u32, b: u16, c: u32, d: u32) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u16, u32, u32)> = lib.get(b"BACnetStack_ValueUpdated")?;
        Ok(func(a, b, c, d))
    }
}

// Callback Registration Functions
// Send and Receive Message Functions
pub fn register_callback_receive_message(callback: fn(*mut u8, u16, *mut u8, u8, *mut u8, *mut u8) -> u16) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(*mut u8, u16, *mut u8, u8, *mut u8, *mut u8) -> u16)> = lib.get(b"BACnetStack_RegisterCallbackReceiveMessage")?;
        Ok(func(callback))
    }
}
pub fn register_callback_send_message(callback: fn(*const u8, u16, *const u8, u8, u8, bool) -> u16) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(*const u8, u16, *const u8, u8, u8, bool) -> u16)> = lib.get(b"BACnetStack_RegisterCallbackSendMessage")?;
        Ok(func(callback))
    }
}

// System Functions
pub fn register_callback_get_system_time(callback: fn() -> u64) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn() -> u64)> = lib.get(b"BACnetStack_RegisterCallbackGetSystemTime")?;
        Ok(func(callback))
    }
}
pub fn register_callback_set_system_time(callback: fn(u32, u8, u8, u8, u8, u8, u8, u8, u8) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u8, u8, u8, u8, u8, u8, u8, u8) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackSetSystemTime")?;
        Ok(func(callback))
    }
}

// Get Data Functions
pub fn register_callback_get_property_bitstring(callback: fn(u32, u16, u32, u32, *mut bool, *mut u32, u32, bool, u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, *mut bool, *mut u32, u32, bool, u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackGetPropertyBitString")?;
        Ok(func(callback))
    }
}
pub fn register_callback_get_property_bool(callback: fn(u32, u16, u32, u32, *mut bool, bool, u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, *mut bool, bool, u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackGetPropertyBool")?;
        Ok(func(callback))
    }
}
pub fn register_callback_get_property_character_string(callback: fn(u32, u16, u32, u32, *mut c_char, *mut u32, u32, *mut u8, bool, u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, *mut c_char, *mut u32, u32, *mut u8, bool, u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackGetPropertyCharacterString")?;
        Ok(func(callback))
    }
}
pub fn register_callback_get_property_date(callback: fn(u32, u16, u32, u32, *mut u8, *mut u8, *mut u8, *mut u8, bool, u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, *mut u8, *mut u8, *mut u8, *mut u8, bool, u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackGetPropertyDate")?;
        Ok(func(callback))
    }
}
pub fn register_callback_get_property_double(callback: fn(u32, u16, u32, u32, *mut f64, bool, u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, *mut f64, bool, u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackGetPropertyDouble")?;
        Ok(func(callback))
    }
}
pub fn register_callback_get_property_enumerated(callback: fn(u32, u16, u32, u32, *mut u32, bool, u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, *mut u32, bool, u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackGetPropertyEnumerated")?;
        Ok(func(callback))
    }
}
pub fn register_callback_get_property_octet_string(callback: fn(u32, u16, u32, u32, *mut u32, bool, u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, *mut u32, bool, u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackGetPropertyOctetString")?;
        Ok(func(callback))
    }
}
pub fn register_callback_get_property_real(callback: fn(u32, u16, u32, u32, *mut f32, bool, u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, *mut f32, bool, u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackGetPropertyReal")?;
        Ok(func(callback))
    }
}
pub fn register_callback_get_property_signed_integer(callback: fn(u32, u16, u32, u32, *mut i32, bool, u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, *mut i32, bool, u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackGetPropertySignedInteger")?;
        Ok(func(callback))
    }
}
pub fn register_callback_get_property_time(callback: fn(u32, u16, u32, u32, *mut u8, *mut u8, *mut u8, *mut u8, bool, u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, *mut u8, *mut u8, *mut u8, *mut u8, bool, u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackGetPropertyTime")?;
        Ok(func(callback))
    }
}
pub fn register_callback_get_property_unsigned_integer(callback: fn(u32, u16, u32, u32, *mut u32, bool, u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, *mut u32, bool, u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackGetPropertyUnsignedInteger")?;
        Ok(func(callback))
    }
}

// Set Data Functions
pub fn register_callback_set_property_bitstring(callback: fn(u32, u16, u32, u32, *const bool, u32, bool, u32, u8, *mut u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, *const bool, u32, bool, u32, u8, *mut u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackSetPropertyBitString")?;
        Ok(func(callback))
    }
}
pub fn register_callback_set_property_bool(callback: fn(u32, u16, u32, u32, bool, bool, u32, u8, *mut u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, bool, bool, u32, u8, *mut u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackSetPropertyBool")?;
        Ok(func(callback))
    }
}
pub fn register_callback_set_property_character_string(callback: fn(u32, u16, u32, u32, *const c_char, u32, u8, bool, u32, u8, *mut u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, *const c_char, u32, u8, bool, u32, u8, *mut u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackSetPropertyCharacterString")?;
        Ok(func(callback))
    }
}
pub fn register_callback_set_property_date(callback: fn(u32, u16, u32, u32, u8, u8, u8, u8, bool, u32, u8, *mut u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, u8, u8, u8, u8, bool, u32, u8, *mut u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackSetPropertyDate")?;
        Ok(func(callback))
    }
}
pub fn register_callback_set_property_double(callback: fn(u32, u16, u32, u32, f64, bool, u32, u8, *mut u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, f64, bool, u32, u8, *mut u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackSetPropertyDouble")?;
        Ok(func(callback))
    }
}
pub fn register_callback_set_property_enumerated(callback: fn(u32, u16, u32, u32, u32, bool, u32, u8, *mut u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, u32, bool, u32, u8, *mut u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackSetPropertyEnumerated")?;
        Ok(func(callback))
    }
}
pub fn register_callback_set_property_null(callback: fn(u32, u16, u32, u32, bool, u32, u8, *mut u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, bool, u32, u8, *mut u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackSetPropertyNull")?;
        Ok(func(callback))
    }
}
pub fn register_callback_set_property_octet_string(callback: fn(u32, u16, u32, u32, *const u8, u32, bool, u32, u8, *mut u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, *const u8, u32, bool, u32, u8, *mut u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackSetPropertyOctetString")?;
        Ok(func(callback))
    }
}
pub fn register_callback_set_property_real(callback: fn(u32, u16, u32, u32, f32, bool, u32, u8, *mut u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, f32, bool, u32, u8, *mut u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackSetPropertyReal")?;
        Ok(func(callback))
    }
}
pub fn register_callback_set_property_signed_integer(callback: fn(u32, u16, u32, u32, i32, bool, u32, u8, *mut u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, i32, bool, u32, u8, *mut u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackSetPropertySignedInteger")?;
        Ok(func(callback))
    }
}
pub fn register_callback_set_property_time(callback: fn(u32, u16, u32, u32, u8, u8, u8, u8, bool, u32, u8, *mut u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, u8, u8, u8, u8, bool, u32, u8, *mut u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackSetPropertyTime")?;
        Ok(func(callback))
    }
}
pub fn register_callback_set_property_unsigned_integer(callback: fn(u32, u16, u32, u32, u32, bool, u32, u8, *mut u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32, u32, u32, bool, u32, u8, *mut u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackSetPropertyUnsignedInteger")?;
        Ok(func(callback))
    }
}

// Object Creation Functions
pub fn register_callback_create_object(callback: fn(u32, u16, u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackCreateObject")?;
        Ok(func(callback))
    }
}
pub fn register_callback_delete_object(callback: fn(u32, u16, u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u16, u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackDeleteObject")?;
        Ok(func(callback))
    }
}

// Remote Device Management Functions
pub fn register_callback_reinitialize_device(callback: fn(u32, u32, *const c_char, u32, *mut u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u32, *const c_char, u32, *mut u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackCreateObject")?;
        Ok(func(callback))
    }
}
pub fn register_callback_device_communication_control(callback: fn(u32, u8, *const c_char, u8, bool, u16, *mut u32) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, u8, *const c_char, u8, bool, u16, *mut u32) -> bool)> = lib.get(b"BACnetStack_RegisterCallbackDeleteObject")?;
        Ok(func(callback))
    }
}

// Debug Message Functions
pub fn register_callback_log_debug_message(callback: fn(*const c_char, u16, u8)) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(*const c_char, u16, u8))> = lib.get(b"BACnetStack_RegisterCallbackLogDebugMessage")?;
        Ok(func(callback))
    }
}

// Client Functions
// Client Hooks
pub fn register_hook_text_message(callback: fn(u32, bool, u32, *const c_char, u32, u8, *const c_char, u32, *const u8, u8, u8, u16, *const u8, u8, *mut u16, *mut u16) -> bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(fn(u32, bool, u32, *const c_char, u32, u8, *const c_char, u32, *const u8, u8, u8, u16, *const u8, u8, *mut u16, *mut u16) -> bool)> = lib.get(b"BACnetStack_RegisterHookTextMessage")?;
        Ok(func(callback))
    }
}

// Object Access Services
pub fn build_read_property(object_type: u16, object_instance: u32, property_identifier: u32, use_array_index: bool, property_array_index: u32) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u16, u32, u32, bool, u32) -> bool> = lib.get(b"BACnetStack_BuildReadProperty")?;
        Ok(func(object_type, object_instance, property_identifier, use_array_index, property_array_index))
    }
}
pub fn clear_read_property() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn()> = lib.get(b"BACnetStack_ClearReadProperty")?;
        Ok(func())
    }
}
pub fn send_read_property(sent_invoke_id: *mut u8, connection_string: *const u8, connection_string_length: u8, network_type: u8, destination_network: u16, destination_address: *const u8, destination_address_length: u8) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(*mut u8, *const u8, u8, u8, u16, *const u8, u8) -> bool> = lib.get(b"BACnetStack_SendReadProperty")?;
        Ok(func(sent_invoke_id, connection_string, connection_string_length, network_type, destination_network, destination_address, destination_address_length))
    }
}

// Remote Device Management Services
pub fn send_i_am(device_instance: u32, connection_string: *const u8, connection_string_length: u8, network_type: u8, broadcast: bool, destination_network: u16, destination_address: *const u8, destination_address_length: u8) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, *const u8, u8, u8, bool, u16, *const u8, u8) -> bool> = lib.get(b"BACnetStack_SendIAm")?;
        Ok(func(device_instance, connection_string, connection_string_length, network_type, broadcast, destination_network, destination_address, destination_address_length))
    }
}
pub fn send_who_is(connection_string: *const u8, connection_string_length: u8, network_type: u8, broadcast: bool, destination_network: u16, destination_address: *const u8, destination_address_length: u8) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(*const u8, u8, u8, bool, u16, *const u8, u8) -> bool> = lib.get(b"BACnetStack_SendWhoIs")?;
        Ok(func(connection_string, connection_string_length, network_type, broadcast, destination_network, destination_address, destination_address_length))
    }
}
pub fn send_who_is_with_limits(device_instance_range_low_limit: u32, device_instance_range_high_limit: u32, connection_string: *const u8, connection_string_length: u8, network_type: u8, broadcast: bool, destination_network: u16, destination_address: *const u8, destination_address_length: u8) -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(u32, u32, *const u8, u8, u8, bool, u16, *const u8, u8) -> bool> = lib.get(b"BACnetStack_SendWhoIsWithLimits")?;
        Ok(func(device_instance_range_low_limit, device_instance_range_high_limit, connection_string, connection_string_length, network_type, broadcast, destination_network, destination_address, destination_address_length))
    }
}