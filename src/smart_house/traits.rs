pub mod traits {
    use crate::smart_house::smart_house::SmartHouse;

    pub trait DeviceInfoProvider {
        fn get_device_state(&self, _house: &SmartHouse) -> String {
            "No any realisations".to_string()
        }
    }

    pub trait DeviceState {
        fn get_state(&self) -> String {
            "No any realisations".to_string()
        }
    }
}
