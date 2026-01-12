use core::ptr;
use esp_idf_sys::esp_tinyusb::{
    tinyusb_config_t,
    tinyusb_desc_config_t,
    tinyusb_phy_config_t,
    tinyusb_task_config_t,
    tinyusb_port_t_TINYUSB_PORT_FULL_SPEED_0, // <- nom exact selon tes bindings
    tinyusb_driver_install,
};

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    unsafe {
        let cfg = tinyusb_config_t {
            // 1) Port
            port: tinyusb_port_t_TINYUSB_PORT_FULL_SPEED_0,

            // 2) PHY
            phy: tinyusb_phy_config_t {
                skip_setup: false,     // esp_tinyusb configure l’USB PHY interne
                self_powered: false,   // bus-powered (si self-powered => vbus_monitor_io doit être câblé)
                vbus_monitor_io: 0,    // ignoré si self_powered == false
            },

            // 3) Task TinyUSB (à adapter)
            task: tinyusb_task_config_t {
                size: 4096,        // stack task (exemple)
                priority: 5,       // priorité FreeRTOS (exemple)
                xCoreID: 0,        // affinité coeur (exemple)
            },

            // 4) Descriptors
            descriptor: tinyusb_desc_config_t {
                device: ptr::null(),            // ⚠️ dans ton header: “Must not be NULL”
                qualifier: ptr::null(),         // OK si full-speed only
                string: ptr::null_mut(),            // OK si pas de strings
                string_count: 0,
                full_speed_config: ptr::null(), // si NULL -> “default descriptor from Kconfig …” (selon ton commentaire)
                high_speed_config: ptr::null(),
            },

            // 5) Events (optionnel)
            event_cb: None,
            event_arg: ptr::null_mut(),
        };

        let err = tinyusb_driver_install(&cfg);
        // si tu veux:
        // assert_eq!(err, esp_idf_sys::ESP_OK);
        log::info!("tinyusb_driver_install -> {}", err);
    }
}
