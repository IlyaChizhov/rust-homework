pub mod structs {
    use crate::smart_house::smart_house::SmartHouse;
    use crate::smart_house::traits::traits::{DeviceInfoProvider, DeviceState};
    use std::collections::HashSet;

    #[derive(Clone)]
    pub struct Room {
        pub devices: HashSet<String>,
    }

    pub struct SmartThermometer {
        pub name: String,
        pub state: bool,
        pub temperature: i16,
    }

    pub struct OwningDeviceInfoProvider {
        pub socket: SmartSocket,
    }

    pub struct BorrowingDeviceInfoProvider<'a, 'b> {
        pub socket: &'a SmartSocket,
        pub thermo: &'b SmartThermometer,
    }

    pub struct SmartSocket {
        pub name: String,
        pub state: bool,
    }

    impl DeviceState for SmartSocket {
        fn get_state(&self) -> String {
            format!("Device: {}, state: {}", &self.name, &self.state)
        }
    }

    impl DeviceState for SmartThermometer {
        fn get_state(&self) -> String {
            format!(
                "Device: {}, state: {}, temperature: {}",
                &self.name, &self.state, &self.temperature
            )
        }
    }

    impl DeviceInfoProvider for OwningDeviceInfoProvider {
        fn get_device_state(&self, house: &SmartHouse) -> String {
            let default_result = "There aren't any available devices".to_string();
            let has_device = house.exists_device("socket");

            if !has_device {
                return default_result;
            }

            self.socket.get_state()
        }
    }

    impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
        fn get_device_state(&self, house: &SmartHouse) -> String {
            let default_result = "There aren't any available devices".to_string();
            let has_socket = house.exists_device("socket");
            let has_thermo = house.exists_device("thermometer");

            if !has_socket && !has_thermo {
                return default_result;
            }

            let socket_report = self.socket.get_state();
            let thermo_report = self.thermo.get_state();

            format!(
                "SOCKET - {} \r
           THERMO - {}",
                socket_report, thermo_report
            )
        }
    }
}
