use smart_home::devices::*;
use smart_home::smart_house_mod::*;

mod common;

#[test]
fn test_report_from_owning_device_info_provider() {
    let smart_house = SmartHouse::new("Cottage", vec![]);
    let smart_socket3 = SmartSocket {
        info: DeviceInfo {
            id: 0,
            name: "Smart Socket 3".to_string(),
            device_type: DeviceType::PowerSocket,
            description: "Smart Socket in the living room".to_string(),
        },
        is_switch_on: true,
        current_power: 220,
    };
    let info_provider_1 = OwningDeviceInfoProvider {
        device: smart_socket3,
    };
    let name = info_provider_1.get_name().clone();
    let report = smart_house.create_report(info_provider_1);
    println!("{}", report);
    assert!(report.contains(name.trim()));
}
