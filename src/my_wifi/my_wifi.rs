use embedded_hal::adc::Channel;
use embedded_svc::wifi::AuthMethod;
use esp_idf_hal::{prelude::Peripherals, peripheral::Peripheral};
use esp_idf_svc::{eventloop::EspSystemEventLoop, wifi::{self, EspWifi}, nvs::EspDefaultNvsPartition};

// wifi 配置信息
pub struct WifiConfig {
    wifi_ssid: String,
    wifi_pwd: String
}

impl WifiConfig {
    // 创建wifi配置文件
    pub fn new(ssid: &str, pwd: &str) -> WifiConfig {
        WifiConfig { wifi_ssid: ssid.to_owned(), wifi_pwd: pwd.to_owned() }
    }
}

pub fn connect_wifi(
    modem: impl Peripheral<P = esp_idf_hal::modem::Modem> + 'static, 
    event: EspSystemEventLoop, 
    nvs: &EspDefaultNvsPartition,
    config: &WifiConfig
    ) -> Result<Box<EspWifi<'static>>, ()>{
    let mut esp_wifi = wifi::EspWifi::new(
        modem,
        event.clone(),
        Some(nvs.to_owned())
    ).unwrap();
    let mut authmethod = AuthMethod::WPA2Personal;
    if config.wifi_pwd.is_empty() {
        authmethod = AuthMethod::None;
    }
    let mut wifi = wifi::BlockingWifi::wrap(&mut esp_wifi, event).unwrap(); 
    // 配置wifi基本信息
    wifi.set_configuration(&wifi::Configuration::Client(wifi::ClientConfiguration::default()));
    //  启动wifi 查找周围wifi信息
    wifi.start();
    log::info!("Scanning...");
    let ap_infos = wifi.scan().unwrap();
    // 打印
    println!("------Access Point Info------");
    for info in &ap_infos {
        println!("{:?}", info);
    }
    // 获取channel
    let ours = ap_infos.into_iter().find(|a| a.ssid.as_str().to_string() == config.wifi_ssid);
    let channel = if let Some(ours) =  ours{
        log::info!("find {} in channel:{}", ours.ssid, ours.channel);
        Some(ours.channel)
    }else {
        None
    };

    wifi.set_configuration(&wifi::Configuration::Client(wifi::ClientConfiguration { 
        ssid: config.wifi_ssid.as_str().into(), 
        password: config.wifi_pwd.as_str().into(),
        channel: channel,
        auth_method: authmethod,
        ..Default::default()
    })).unwrap();

    // 连接wifi
    log::info!("Connecting wifi...");
    wifi.connect();
    // 等待连接
    wifi.wait_netif_up();

    Ok(Box::new(esp_wifi))
}