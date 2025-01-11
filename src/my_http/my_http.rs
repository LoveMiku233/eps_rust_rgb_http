use embedded_svc::{http::{client::Client, Query}, io::Read, storage::DynStorage};
use esp_idf_svc::http::client::{EspHttpConnection, Configuration};

use super::error::MyHttpError;


pub fn http_get(url: impl AsRef<str>) -> Result<String, MyHttpError> {
    //  创建esp_http_connection
    let esp_client = EspHttpConnection::new(&Configuration::default()).unwrap();
    let mut client = Client::wrap(esp_client);

    let request = client.get(url.as_ref()).unwrap();
    let response = request.submit().unwrap();
    let status = response.status();
    match status {
        200..=299 => {
            let mut buf = [0_u8;1024];
            let mut reader = response;
            let size = reader.read(&mut buf).unwrap();
            let data = String::from_utf8(buf.to_vec()).expect("err");
            println!("get len: {} , datas: {}", size, data);
            Ok(data)
        },
        _ => {
            log::error!("status: {}!", status);
            Err(MyHttpError::HttpRedirection(status))
        }
    }
}

