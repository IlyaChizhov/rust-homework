pub mod structs;
pub mod traits;

pub mod smart_house {
    use crate::smart_house::structs::structs::Room;
    use crate::smart_house::traits::traits::DeviceInfoProvider;
    use std::collections::HashMap;

    pub struct SmartHouse {
        rooms: HashMap<String, Room>,
    }

    impl SmartHouse {
        pub fn new(rooms: HashMap<String, Room>) -> Self {
            Self { rooms }
        }

        pub(crate) fn exists_device(&self, device_key: &str) -> bool {
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

        pub fn create_report(&self, provider: &impl DeviceInfoProvider) -> String {
            provider.get_device_state(self)
        }
    }
}
