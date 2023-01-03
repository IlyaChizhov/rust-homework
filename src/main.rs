mod smart_house;

fn main() {}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::smart_house::smart_house::SmartHouse;
    use crate::smart_house::structs::structs::{
        BorrowingDeviceInfoProvider, OwningDeviceInfoProvider, Room, SmartSocket, SmartThermometer,
    };

    #[test]
    fn example() {
        let socket1 = SmartSocket {
            name: "Socket one".to_string(),
            state: false,
        };
        let socket2 = SmartSocket {
            name: "Socket two".to_string(),
            state: true,
        };
        let thermo = SmartThermometer {
            name: "Smart thermometer".to_string(),
            state: true,
            temperature: -23,
        };

        let mut bedroom_devices = HashSet::new();
        let mut bathroom_devices = HashSet::new();

        bedroom_devices.insert(String::from("socket"));
        bedroom_devices.insert(String::from("thermometer"));

        bathroom_devices.insert(String::from("thermometer"));

        let bedroom = Room {
            devices: bedroom_devices,
        };

        let bathroom = Room {
            devices: bathroom_devices,
        };

        let mut rooms = HashMap::new();

        rooms.insert(String::from("Bedroom"), bedroom);
        rooms.insert(String::from("Bathroom"), bathroom);

        let house = SmartHouse::new(rooms);

        let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };

        let report1 = house.create_report(&info_provider_1);

        let info_provider_2 = BorrowingDeviceInfoProvider {
            socket: &socket2,
            thermo: &thermo,
        };

        let report2 = house.create_report(&info_provider_2);

        assert_eq!(report1, "Device: Socket one, state: false".to_string());
        assert_eq!(
            report2,
            "SOCKET - Device: Socket two, state: true \r\n           THERMO - Device: Smart thermometer, state: true, temperature: -23".to_string()
        );
    }
}
