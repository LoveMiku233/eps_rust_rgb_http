use anyhow::{Result};
use esp_idf_hal::{prelude::Peripherals};
use esp_idf_svc::{eventloop::EspSystemEventLoop, wifi, nvs::EspDefaultNvsPartition, http::{server::{EspHttpServer, Configuration}}};
use rgb::RGB8;
use std::{thread::sleep, time::Duration};
use esp_idf_sys as _;
use rand::{Rng, RngCore};
use embedded_svc::{http::{Method, Headers}, io::{Write, Read}};
use std::sync::{Arc, Mutex};

mod my_wifi;
mod my_http;
mod my_rgb;
mod my_dht11;

static INDEX: &str = include_str!("test.html"); 



fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let mut rng = rand::thread_rng();
    // 获取外围设备
    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();
    // 使用Arc + mutex加锁处理
    let rgb_main = Arc::new(Mutex::new(my_rgb::my_rgb::WS2812RMT::new(peripherals.pins.gpio38, peripherals.rmt.channel0).unwrap()));
    let mut rgb1 = rgb_main.clone();
    rgb1.lock().unwrap().set_pixel(rgb::RGB8::new(255, 0, 0));
    // 温湿度传感器

    let wifi = my_wifi::my_wifi::connect_wifi(
        peripherals.modem, 
        sys_loop, 
        &nvs,
        &my_wifi::my_wifi::WifiConfig::new("WIFI NAME", "PASSWORD")).unwrap();
    log::info!("wifi!");
    log::info!("connected now!");
    rgb1.lock().unwrap().set_pixel(rgb::RGB8::new(0, 255, 0));
    // http test
    while !wifi.is_connected().unwrap(){
        
    }

    // http server
    let mut server = EspHttpServer::new(&Configuration::default()).unwrap();
    server.fn_handler("/", Method::Get, |requset| {
        let mut response = requset.into_ok_response().unwrap();
        response.write_all(INDEX.as_bytes()).unwrap();
        Ok(())
    }).unwrap();

    server.fn_handler("/about", Method::Get, |requset| {
        let mut response = requset.into_ok_response().unwrap();
        response.write_all(index_html().as_bytes()).unwrap();
        Ok(())
    }).unwrap();

    server.fn_handler("/cmd_rgb", Method::Post, |mut req| {
        let len = req.content_len().unwrap() as usize;
        let mut buf = vec![0;len];
        req.read_exact(&mut buf).unwrap();
        let s = String::from_utf8(buf).unwrap();
        if let Ok(tmp_rgb) = json::parse(&s){
            println!("r:{},g:{},b:{}", tmp_rgb["r"].as_u8().unwrap(), tmp_rgb["g"].as_u8().unwrap(), tmp_rgb["b"].as_u8().unwrap());
            rgb1.lock().unwrap().set_pixel(rgb::RGB8::new(tmp_rgb["r"].as_u8().unwrap(), tmp_rgb["g"].as_u8().unwrap(), tmp_rgb["b"].as_u8().unwrap()));
        } else {
            println!("{}", s);
        }

        Ok(())
    }).unwrap();

    println!("ip info:{:?}", wifi.sta_netif().get_ip_info().unwrap());
    loop {
        // rng.fill_bytes(&mut rgb_data);
       //  println!("R:{},G:{},B:{}", rgb_data[0], rgb_data[1], rgb_data[2]);
        // rgb.set_pixel(rgb::RGB8::new(rgb_data[0], rgb_data[1], rgb_data[2]));
        sleep(Duration::new(1, 0));
    }
    
}


fn templated(content: impl AsRef<str>) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp-rs web server</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        content.as_ref()
    )
}


fn index_html() -> String {
    templated("hello rust! - Esp32-s3")
}