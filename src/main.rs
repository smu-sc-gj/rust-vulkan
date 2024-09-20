
use ash::version::DeviceV1_0;
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
    let app_info = vk::ApplicationInfo::builder()
    .application_name(&appname)
    .application_version(vk::make_version(0, 0, 1))
    .engine_name(&enginename)
    .engine_version(vk::make_version(0, 42, 0))
    .api_version(vk::make_version(1, 0, 106));

    // create an array for layer names. 
    let layer_names: Vec<std::ffi::CString> =
        vec![std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];

    // map to a vector. 
    let layer_name_pointers: Vec<*const i8> = layer_names
        .iter()
        .map(|layer_name| layer_name.as_ptr())
        .collect();

    // vector of extenion name pointers
    // Load linux extensions
    /* 
    let extension_name_pointers: Vec<*const i8> = vec![
        ash::extensions::ext::DebugUtils::name().as_ptr(),
        ash::extensions::khr::Surface::name().as_ptr(),
        ash::extensions::khr::XlibSurface::name().as_ptr(),
    ]; */

    //load windows extensions
    let extension_name_pointers: Vec<*const i8> = vec![
        ash::extensions::ext::DebugUtils::name().as_ptr(),
        ash::extensions::khr::Surface::name().as_ptr(),
        ash::extensions::khr::Win32Surface::name().as_ptr(),
    ];    
    
    // setup the extension - moved earlier, this is so we can 
    // pass this to the creation function (and get errors back).
    let mut debugcreateinfo = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        )
        .pfn_user_callback(Some(vulkan_debug_utils_callback));


    // create the instance info - us a builder.
    let instance_create_info = vk::InstanceCreateInfo::builder()
        .push_next(&mut debugcreateinfo)
        .application_info(&app_info)
        .enabled_layer_names(&layer_name_pointers)
        .enabled_extension_names(&extension_name_pointers);
   
    // doesn't work?
    //dbg!(&instance_create_info);

    // create instance with some customisation - from above.  
    let instance = unsafe { entry.create_instance(&instance_create_info, None)? };

        // Use winint to setup a window ...
        let eventloop = winit::event_loop::EventLoop::new();
        let window = winit::window::Window::new(&eventloop);
    
        // Linux window setup:
        /*
        use winit::platform::unix::WindowExtUnix;
        let x11_display = window.xlib_display().unwrap();
        let x11_window = window.xlib_window().unwrap();
        let x11_create_info = vk::XlibSurfaceCreateInfoKHR::builder()
            .window(x11_window)
            .dpy(x11_display as *mut vk::Display);
        let xlib_surface_loader = ash::extensions::khr::XlibSurface::new(&entry, &instance);
        let surface = unsafe { xlib_surface_loader.create_xlib_surface(&x11_create_info, None) }?;
        let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);
        */

    
    use winit::platform::windows::WindowExtWindows;
    //let w32_display = window.win32_display().unwrap();
    let w32_window = window.unwrap();
    let win32_create_info = vk::Win32SurfaceCreateInfoKHR::builder()
        .hinstance(w32_window.hinstance())
        .hwnd(w32_window.hwnd());
    let win32_surface_loader = ash::extensions::khr::Win32Surface::new(&entry, &instance);
    let surface = unsafe { win32_surface_loader.create_win32_surface(&win32_create_info, None) }?;
    let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);

    // create the extension and tag onto the entry point / instance. 
    let debug_utils = ash::extensions::ext::DebugUtils::new(&entry, &instance);

    // Create the messenger based on the structure above. 
    let utils_messenger = unsafe { debug_utils.create_debug_utils_messenger(&debugcreateinfo, None)? };

    // Physical Device setup. 
    // List devices!
    let phys_devs = unsafe { instance.enumerate_physical_devices()?};
    
    // print for debugging 
    /* 
    for p in phys_devs {
        let props = unsafe { instance.get_physical_device_properties(p)};
        dbg!(props);
    }
    */

    // Don't grab the first one, might be integrated. 
    /* Mine is: NVIDIA GeForce RTX 2060 (laptop)

    let (physical_devices, physical_device_properties) = {
        let mut chosen = None;
        for p in phys_devs {
            let properties = unsafe { instance.get_physical_device_properties(p) };

            let name = String::from( 
                unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) }
                    .to_str()
                    .unwrap(),
            );

            if name = "NVIDIA GeForce RTX 2060" {
                chosen = Some((p, properties));
            }
        }
    };

    */

    // Better idea, pick the last non-integrated gpu (i.e. discrete)
    // - I need a fall back, what if there is no disrete card?!
    let (physical_device, physical_device_properties) = {
        let mut chosen = None;
        for p in phys_devs {
            let properties = unsafe { instance.get_physical_device_properties(p) };
            if properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
                chosen = Some((p, properties));
            }
        }
   
        chosen.expect("Tried to unwrap chosen graphics card: No discrete graphics card found!")
    };

    // Setup queues - data transfer, graphics or computing
    let queue_family_properties = 
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
    dbg!(&queue_family_properties);

    let queue_family_indicies = {
        let mut found_graphics_q_index = None;
        let mut found_transfer_q_index = None;
        // let mut found_compute_q_index = None;

        for(index, queue_family) in queue_family_properties.iter().enumerate() {
            if queue_family.queue_count > 0 && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                found_graphics_q_index = Some( index as u32);
            }
            
            if queue_family.queue_count > 0 && queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER) {
                if found_transfer_q_index.is_none() 
                    || !queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                        found_transfer_q_index = Some(index as u32);
                    }
            }
        }

        // make a tuple
        (
            found_graphics_q_index.unwrap(),
            found_transfer_q_index.unwrap(),
        )
    };

    // Create a logical device as a primary interface to the physical device

    let priorities = [1.0f32];
    let queue_infos = [
        vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(queue_family_indicies.0)
            .queue_priorities(&priorities)
            .build(),
        vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(queue_family_indicies.1)
            .queue_priorities(&priorities)
            .build(),
    ];

    let device_create_info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_infos)
        .enabled_layer_names(&layer_name_pointers);

    let logical_device = 
        unsafe { instance.create_device(physical_device, &device_create_info, None)? };

    let graphics_queue = unsafe { logical_device.get_device_queue(queue_family_indicies.0, 0) };
    let transfer_queue = unsafe { logical_device.get_device_queue(queue_family_indicies.1, 0) };

    // clean up. 
    unsafe {
        logical_device.destroy_device(None);
        debug_utils.destroy_debug_utils_messenger(utils_messenger, None);
        instance.destroy_instance(None)
    };

    unsafe { 
        debug_utils.destroy_debug_utils_messenger(utils_messenger, None);
        instance.destroy_instance(None) 
    };


    // destroy the surface
    unsafe { surface_loader.destroy_surface(surface, None); }

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