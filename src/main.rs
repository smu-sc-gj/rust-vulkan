
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // has to be unsafe, this is wrapping a C++ api. 


    let entry = ash::Entry::new()?; // Returns a result. 

    /* The above is shorthand for this ... 
       1. If the results OK, unwrap it. 
       2. If not return the error

        This is why our main returns an error and error message.    
    

    let entry = match ash::Entry::new() {
        Ok(t) => t, 
        Err(e) => return Err(e.into()),
    };

   */

    // Stuff for app info struct
    let enginename = std::ffi::CString::new("UnknownGameEngine").unwrap();
    let appname = std::ffi::CString::new("The Black Window").unwrap();

    // application info.
    /* 
    let app_info = vk::ApplicationInfo {
        p_application_name: appname.as_ptr(),
        p_engine_name: enginename.as_ptr(),
        engine_version: vk::make_version(0, 42, 0),
        application_version: vk::make_version(0, 0, 1), 
        api_version: vk::make_version(1, 0, 106),
        ..Default::default()
    };*/

    let app_info = vk::ApplicationInfo::builder()
    .application_name(&appname)
    .application_version(vk::make_version(0, 0, 1))
    .engine_name(&enginename)
    .engine_version(vk::make_version(0, 42, 0))
    .api_version(vk::make_version(1, 0, 106));

    // create an array for layer names. 
    let layer_names: Vec<std::ffi::CString> = vec![std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];

    // map to a vector. 
    let layer_name_pointers: Vec<*const i8> = layer_names
        .iter()
        .map(|layer_name| layer_name.as_ptr())
        .collect();

    // vector of extenion name pointers
    let extension_name_pointers: Vec<*const i8> =
        vec![ash::extensions::ext::DebugUtils::name().as_ptr()];

    let instance_create_info = vk::InstanceCreateInfo {
        p_application_info: &app_info,
        pp_enabled_layer_names: layer_name_pointers.as_ptr(),
        enabled_layer_count: layer_name_pointers.len() as u32, 
        pp_enabled_extension_names: extension_name_pointers.as_ptr(),
        enabled_extension_count: extension_name_pointers.len() as u32,
        ..Default::default()
    };
    dbg!(&instance_create_info);

    // create instance with some customisation - from above.  
    let instance = unsafe { entry.create_instance(&instance_create_info, None)? };

    // create the extension and tag onto the entry point / instance. 
    let debug_utils = ash::extensions::ext::DebugUtils::new(&entry, &instance);

    // setup the extension
    let debugcreateinfo = vk::DebugUtilsMessengerCreateInfoEXT {
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
        | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
        | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
        | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL 
        | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
        | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        pfn_user_callback: Some(vulkan_debug_utils_callback), 
        ..Default::default()
    };

    // Create the messenger based on the structure above. 
    let utils_messenger = unsafe { debug_utils.create_debug_utils_messenger(&debugcreateinfo, None)? };

    // clean up. 
    unsafe { 
        debug_utils.destroy_debug_utils_messenger(utils_messenger, None);
        instance.destroy_instance(None) 
    };
    Ok(())
}

/*////////////////////////// Debug Layer Start////////////////////////// */

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);
    let severity = format!("{:?}", message_severity).to_lowercase();
    let ty = format!("{:?}", message_type).to_lowercase();
    println!("[Debug][{}][{}] {:?}", severity, ty, message);
    vk::FALSE
}



/*////////////////////////// Debug Layer End /////////////////////////// */