
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
    let app_info = vk::ApplicationInfo {
        p_application_name: appname.as_ptr(),
        p_engine_name: enginename.as_ptr(),
        engine_version: vk::make_version(0, 42, 0),
        application_version: vk::make_version(0, 0, 1), 
        api_version: vk::make_version(1, 0, 106),
        ..Default::default()
    };

    let instance_create_info = vk::InstanceCreateInfo {
        p_application_info: &app_info,
        ..Default::default()
    };
    dbg!(&instance_create_info);

    // create instance with some customisation - from above.  
    let instance = unsafe { entry.create_instance(&instance_create_info, None)? };


    unsafe { instance.destroy_instance(None) };
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