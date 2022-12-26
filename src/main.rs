use std::collections::{HashMap, HashSet};

trait DeviceInfoProvider {
    fn get_device_state(&self, _house: &SmartHouse) -> String {
        "No any realisations".to_string()
    }
}

trait DeviceState {
    fn get_state(&self) -> String {
        "No any realisations".to_string()
    }
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

struct SmartSocket {
    name: String,
    state: bool,
}

struct SmartThermometer {
    name: String,
    state: bool,
    temperature: i16,
}

struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}

struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
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

#[derive(Clone)]
struct Room {
    devices: HashSet<String>,
}

struct SmartHouse {
    rooms: HashMap<String, Room>,
}

impl SmartHouse {
    fn new(rooms: HashMap<String, Room>) -> Self {
        Self { rooms }
    }

    fn exists_device(&self, device_key: &str) -> bool {
        let mut has_device = false;

        let rooms = self.get_rooms();

        for room in rooms {
            for device in self.devices(&room) {
                if device == *device_key {
                    has_device = true;
                }
            }
        }

        has_device
    }

    fn get_rooms(&self) -> Vec<String> {
        self.rooms.clone().into_keys().collect()
    }

    fn devices(&self, room: &String) -> Vec<String> {
        let rooms = &self.rooms;

        rooms[room].clone().devices.into_iter().collect()
    }

    fn create_report(&self, provider: &impl DeviceInfoProvider) -> String {
        provider.get_device_state(self)
    }
}

fn main() {
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

    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
