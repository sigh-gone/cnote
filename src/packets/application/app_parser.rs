use crate::packets::shared_objs::Application;

use super::dns::DnsMessage;

pub struct AppParser;

pub fn parse_app_layer(data: &[u8]) -> Application {
    if let Ok(dns_message) = DnsMessage::new(data) {
        Application::Dns(dns_message)
    } else {
        Application::Other(data.to_vec().into_boxed_slice())
    }
}
