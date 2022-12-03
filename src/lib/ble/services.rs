//! BLE GATT services

/// UART service
#[nrf_softdevice::gatt_service(uuid = "9e7312e1-2354-11eb-9f10-fbc30a62cf38")]
pub struct UartService {
    #[characteristic(uuid = "9e7312e1-2354-11eb-9f10-fbc30a63cf38", read, write, notify)]
    pub bytes: heapless::Vec<u8, 32>,
}

/// LED service
#[nrf_softdevice::gatt_service(uuid = "9e7312e2-2354-11eb-9f10-fbc30a62cf38")]
pub struct LedService {
    #[characteristic(uuid = "9e7312e2-2354-11eb-9f10-fbc30a63cf38", write)]
    pub state: bool,
}

/// Button service
#[nrf_softdevice::gatt_service(uuid = "9e7312e3-2354-11eb-9f10-fbc30a62cf38")]
pub struct ButtonService {
    #[characteristic(uuid = "9e7312e3-2354-11eb-9f10-fbc30a63cf38", read, notify)]
    pub state: bool,
}
