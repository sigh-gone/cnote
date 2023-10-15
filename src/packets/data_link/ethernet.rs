use crate::packets::{
    internet::ip::Ipv4Packet,
    shared_objs::{Description, ExtendedType, ProtocolDescriptor, ProtocolType},
    traits::{Describable, Layer, SetProtocolDescriptor},
};
use chrono::Utc;
use pnet::packet::ethernet::{EtherType, EtherTypes, EthernetPacket};
use pnet::packet::Packet;
use std::collections::HashMap;
use std::default::Default;

/*



Ethernet Header




 */
#[derive(Default, Clone, Debug)]
pub struct EthernetHeader {
    pub source_mac: String,
    pub destination_mac: String,
    pub ether_type: ProtocolDescriptor<ExtendedType<EtherType>>,
    pub payload: Vec<u8>,
    pub malformed: bool,
}
impl SetProtocolDescriptor<EtherType> for EthernetHeader {
    fn set_proto_descriptor(
        proto: ExtendedType<EtherType>,
    ) -> ProtocolDescriptor<ExtendedType<EtherType>> {
        let protocol_name = match &proto {
            ExtendedType::Known(ether_type) => set_name(ether_type),
            ExtendedType::Malformed => "malformed".to_string(),
        };

        ProtocolDescriptor {
            protocol_name,
            protocol_type: proto,
        }
    }
}
impl EthernetHeader {
    pub fn malformed(packet: &[u8]) -> EthernetHeader {
        EthernetHeader {
            source_mac: "".to_string(),
            destination_mac: "".to_string(),
            ether_type: EthernetHeader::set_proto_descriptor(ExtendedType::Malformed),
            payload: packet.to_vec(),
            malformed: true,
        }
    }
}

/*



Ethernet Frame



 */

#[derive(Default, Debug)]
pub struct EthernetFrame {
    pub id: i32,
    pub timestamp: String,
    pub header: EthernetHeader,
    pub payload: Option<Box<dyn Layer>>,
}

impl EthernetFrame {
    pub fn new(id: i32, packet: &[u8]) -> Self {
        let mut frame = EthernetFrame {
            id,
            timestamp: Utc::now().to_string(),
            ..Default::default()
        };
        frame.deserialize(packet);
        frame
    }
}

//trait impls
impl Layer for EthernetFrame {
    fn deserialize(&mut self, packet: &[u8]) {
        let packet_header: EthernetHeader = match EthernetPacket::new(packet) {
            None => EthernetHeader::malformed(packet),
            Some(header) => EthernetHeader {
                source_mac: header.get_source().to_string(),
                destination_mac: header.get_destination().to_string(),
                ether_type: EthernetHeader::set_proto_descriptor(ExtendedType::Known(
                    header.get_ethertype(),
                )),
                payload: header.payload().to_vec(),
                malformed: false,
            },
        };
        let payload: Option<Box<dyn Layer>> = match &packet_header.ether_type.protocol_type.clone()
        {
            &ExtendedType::Known(EtherTypes::Ipv4) => {
                //ipv4
                Some(Box::new(parse_ipv4(&packet_header.payload)))
            }
            _ => None,
        };
        self.header = packet_header;
        self.payload = payload;
    }

    fn get_summary(&self) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("protocol".to_string(), "ethernet".to_string());
        map.insert("Source Mac".to_string(), self.header.source_mac.to_string());
        map.insert(
            "Destination Mac".to_string(),
            self.header.destination_mac.to_string(),
        );
        map.insert(
            "EtherType".to_string(),
            self.header.ether_type.protocol_name.to_string(),
        );
        map.insert("malformed".to_string(), self.header.malformed.to_string());
        map
    }

    fn get_next(&self) -> &Option<Box<dyn Layer>> {
        &self.payload
    }
    fn protocol_type(&self) -> ProtocolType {
        ProtocolType::Ethernet
    }

    fn source(&self) -> String {
        self.header.source_mac.to_string()
    }

    fn destination(&self) -> String {
        self.header.destination_mac.to_string()
    }

    fn info(&self) -> String {
        format!("next header {}", self.header.ether_type.protocol_name)
    }
}

impl Describable for EthernetFrame {
    fn get_short(&self) -> Description {
        let (source, destination) = if self.payload.as_ref().is_none() {
            (
                self.header.source_mac.clone(),
                self.header.destination_mac.clone(),
            )
        } else {
            let payload = self.payload.as_ref().unwrap();
            (payload.source(), payload.destination())
        };

        let (protocol, info) = if let Some(payload) = self.payload.as_ref() {
            get_innermost_info(payload.as_ref())
        } else {
            (ProtocolType::Ethernet, self.info())
        };

        Description {
            id: self.id,
            timestamp: self.timestamp.clone(),
            protocol,
            source,
            destination,
            info,
        }
    }

    fn get_long(&self) -> Vec<HashMap<String, String>> {
        let mut vec_map = vec![self.get_summary()];
        let mut current_layer: Option<Box<&dyn Layer>> = Some(Box::new(self));
        while let Some(layer) = &current_layer {
            vec_map.push(layer.get_summary());
            current_layer = layer
                .get_next()
                .as_ref()
                .map(|boxed_layer| Box::new(boxed_layer.as_ref() as &dyn Layer));
        }

        vec_map
    }
}

/*



helper functions



 */

//might be in another trait
fn get_innermost_info(layer: &dyn Layer) -> (ProtocolType, String) {
    match layer.get_next() {
        Some(next) => get_innermost_info(next.as_ref()),
        None => (layer.protocol_type(), layer.info()),
    }
}

fn parse_ipv4(payload: &[u8]) -> Ipv4Packet {
    let mut packet = Ipv4Packet::default();
    packet.deserialize(payload);
    packet
}

fn set_name(proto: &EtherType) -> String {
    let name: String = match proto {
        &EtherTypes::Ipv4 => "IPv4".to_string(),
        &EtherTypes::Arp => "ARP".to_string(),
        &EtherTypes::Ipv6 => "IPv6".to_string(),
        _ => "Unknown".to_string(),
    };
    name
}
