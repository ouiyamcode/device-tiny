use core::ptr;

use esp_idf_sys::esp_tinyusb::{
    tinyusb_config_t, tinyusb_desc_config_t, tinyusb_phy_config_t, tinyusb_task_config_t,
    tinyusb_driver_install, tinyusb_port_t_TINYUSB_PORT_FULL_SPEED_0,
    tusb_desc_device_t, tusb_desc_device_qualifier_t,
};

static DEVICE_DESC: tusb_desc_device_t = tusb_desc_device_t {
    bLength: 18,
    bDescriptorType: 0x01, // DEVICE
    bcdUSB: 0x0200,        // USB 2.0
    bDeviceClass: 0x00,    // class at interface level
    bDeviceSubClass: 0x00,
    bDeviceProtocol: 0x00,
    bMaxPacketSize0: 64,
    idVendor: 0x303A,  // Espressif VID (comme dans ton header)
    idProduct: 0x4001, // PID au choix
    bcdDevice: 0x0100,
    iManufacturer: 0, // pas de strings
    iProduct: 0,
    iSerialNumber: 0,
    bNumConfigurations: 1,
};

// Full-Speed MSC config descriptor (1 interface, 2 bulk endpoints)
static FS_CONFIG_DESC: [u8; 32] = [
    // Configuration Descriptor
    9, 0x02, 0x20, 0x00, // wTotalLength = 32
    0x01, // bNumInterfaces
    0x01, // bConfigurationValue
    0x00, // iConfiguration
    0x80, // bmAttributes (bus powered)
    50,   // bMaxPower = 100mA

    // Interface Descriptor (MSC)
    9, 0x04,
    0x00, // bInterfaceNumber
    0x00, // bAlternateSetting
    0x02, // bNumEndpoints
    0x08, // bInterfaceClass = MSC
    0x06, // bInterfaceSubClass = SCSI
    0x50, // bInterfaceProtocol = BOT
    0x00, // iInterface

    // Endpoint OUT (Bulk) EP1 OUT
    7, 0x05,
    0x01,       // OUT
    0x02,       // Bulk
    0x40, 0x00, // 64 bytes
    0x00,

    // Endpoint IN (Bulk) EP1 IN
    7, 0x05,
    0x81,       // IN
    0x02,       // Bulk
    0x40, 0x00, // 64 bytes
    0x00,
];

unsafe fn install_msc_device() -> esp_idf_sys::esp_err_t {
    let desc = tinyusb_desc_config_t {
        device: &DEVICE_DESC as *const tusb_desc_device_t,
        qualifier: ptr::null::<tusb_desc_device_qualifier_t>(),
        string: ptr::null_mut(), // *mut *const c_char
        string_count: 0,
        full_speed_config: FS_CONFIG_DESC.as_ptr(),
        high_speed_config: ptr::null(),
    };

    let cfg = tinyusb_config_t {
        port: tinyusb_port_t_TINYUSB_PORT_FULL_SPEED_0,

        phy: tinyusb_phy_config_t {
            skip_setup: false,
            self_powered: false,
            vbus_monitor_io: 0,
        },

        task: tinyusb_task_config_t {
            size: 4096,
            priority: 5,
            xCoreID: 0,
        },

        descriptor: desc,
        event_cb: None,
        event_arg: ptr::null_mut(),
    };

    tinyusb_driver_install(&cfg)
}

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    unsafe {
        let err = install_msc_device();
        log::info!("tinyusb_driver_install -> {}", err);
    }

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
