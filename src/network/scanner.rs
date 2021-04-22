extern crate libxml;
use cidr_utils::cidr::IpCidr;
use libxml::parser::Parser;
use libxml::tree::*;
use serde::{Deserialize, Serialize};

pub struct Scanner {
    // ip: std::net::IpAddr,
// ports_tcp: Vec<u16>,
}

// This is "pseudo-manually" implemented because nmap produces broken XML and most automagic XML crates I tried did not care for it

#[derive(Deserialize, Serialize, Debug)]
struct XMLNmapRun {
    scanner: String,
    args: String,
    start: String,
    startstr: String,
    version: String,
    xmloutputversion: String,
    hosts: Vec<XMLNmapHost>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct XMLNmapHost {
    starttime: String,
    endtime: String,
    status: XMLNmapHostStatus,
    hostnames: Vec<XMLNmapHostname>,
    ports: Vec<XMLNmapPort>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct XMLNmapHostname {
    name: String,
    #[serde(rename = "type")]
    ty: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct XMLNmapHostStatus {
    state: String,
    reason: String,
    reason_ttl: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct XMLNmapPort {
    protocol: String,
    portid: u16,
    state: XMLNmapPortState,
    service: XMLNmapPortService,
    scripts: Vec<XMLNmapPortScript>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct XMLNmapPortScript {
    id: String,
    output: String,
    // #[serde(rename = "$value")]
    // content: String,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
struct XMLNmapPortService {
    name: String,
    product: Option<String>,
    method: String,
    conf: u8,
    version: Option<String>,
    cpe: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct XMLNmapPortState {
    state: String,
    reason: String,
    reason_ttl: String,
}

fn parse_hostnames(node: &Node) -> Vec<XMLNmapHostname> {
    let mut hostnames: Vec<XMLNmapHostname> = Vec::new();

    let mut c: Option<Node> = node.get_first_child();
    while let Some(child) = c {
        match child.get_type().unwrap() {
            NodeType::ElementNode => match child.get_name().as_str() {
                "hostname" => {
                    let properties = child.get_properties();
                    hostnames.push(XMLNmapHostname {
                        name: properties["name"].clone(),
                        ty: properties["type"].clone(),
                    });
                }
                _ => {}
            },
            _ => {}
        }
        c = child.get_next_sibling();
    }
    hostnames
}

fn parse_service(node: &Node) -> XMLNmapPortService {
    let properties = node.get_properties();

    let mut cpe: Option<String> = None;
    let mut c: Option<Node> = node.get_first_child();
    while let Some(child) = c {
        match child.get_type().unwrap() {
            NodeType::ElementNode => match child.get_name().as_str() {
                "cpe" => {
                    cpe = Some(child.get_content());
                }
                _ => {}
            },
            _ => {}
        }
        c = child.get_next_sibling();
    }

    let mut product: Option<String> = None;
    if let Some(xproduct) = properties.get("product") {
        product = Some(xproduct.clone());
    }
    let mut version: Option<String> = None;
    if let Some(xversion) = properties.get("version") {
        version = Some(xversion.clone());
    }

    XMLNmapPortService {
        name: properties["name"].clone(),
        method: properties["method"].clone(),
        conf: properties["conf"].parse().unwrap(),
        cpe: cpe,
        product: product,
        version: version,
    }
}

fn parse_port(node: &Node) -> XMLNmapPort {
    let properties = node.get_properties();

    let mut state: Option<XMLNmapPortState> = None;
    let mut service: Option<XMLNmapPortService> = None;
    let mut scripts: Vec<XMLNmapPortScript> = Vec::new();
    let mut c: Option<Node> = node.get_first_child();
    while let Some(child) = c {
        match child.get_type().unwrap() {
            NodeType::ElementNode => {
                let properties = child.get_properties();
                match child.get_name().as_str() {
                    "state" => {
                        state = Some(XMLNmapPortState {
                            state: properties["state"].clone(),
                            reason: properties["reason"].clone(),
                            reason_ttl: properties["reason_ttl"].clone(),
                        });
                    }
                    "service" => {
                        service = Some(parse_service(&child));
                    }
                    "script" => {
                        scripts.push(XMLNmapPortScript {
                            id: properties["id"].clone(),
                            output: properties["output"].clone(),
                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        c = child.get_next_sibling();
    }

    XMLNmapPort {
        protocol: properties["protocol"].clone(),
        portid: properties["portid"].clone().parse().unwrap(),
        state: state.unwrap(),
        service: service.unwrap(),
        scripts: scripts,
    }
}

fn parse_ports(node: &Node) -> Vec<XMLNmapPort> {
    let mut ports: Vec<XMLNmapPort> = Vec::new();

    let mut c: Option<Node> = node.get_first_child();
    while let Some(child) = c {
        match child.get_type().unwrap() {
            NodeType::ElementNode => match child.get_name().as_str() {
                "port" => {
                    ports.push(parse_port(&child));
                }
                _ => {}
            },
            _ => {}
        }
        c = child.get_next_sibling();
    }
    ports
}

fn parse_status(node: &Node) -> XMLNmapHostStatus {
    let properties = node.get_properties();

    XMLNmapHostStatus {
        state: properties["state"].clone(),
        reason: properties["reason"].clone(),
        reason_ttl: properties["reason_ttl"].clone(),
    }
}

fn parse_host(node: &Node) -> XMLNmapHost {
    let properties = node.get_properties();

    let mut status: Option<XMLNmapHostStatus> = None;
    let mut hostnames: Vec<XMLNmapHostname> = Vec::new();
    let mut ports: Vec<XMLNmapPort> = Vec::new();
    let mut c: Option<Node> = node.get_first_child();
    while let Some(child) = c {
        match child.get_type().unwrap() {
            NodeType::ElementNode => match child.get_name().as_str() {
                "status" => {
                    status = Some(parse_status(&child));
                }
                "hostnames" => {
                    hostnames.append(&mut parse_hostnames(&child));
                }
                "ports" => {
                    ports.append(&mut parse_ports(&child));
                }
                _ => {}
            },
            _ => {}
        }
        c = child.get_next_sibling();
    }

    XMLNmapHost {
        starttime: properties["starttime"].clone(),
        endtime: properties["endtime"].clone(),
        status: status.unwrap(),
        hostnames: hostnames,
        ports: ports,
    }
}

fn parse_run(node: &Node) -> XMLNmapRun {
    let properties = node.get_properties();

    let mut hosts: Vec<XMLNmapHost> = Vec::new();
    let mut c: Option<Node> = node.get_first_child();
    while let Some(child) = c {
        match child.get_type().unwrap() {
            NodeType::ElementNode => match child.get_name().as_str() {
                "host" => {
                    hosts.push(parse_host(&child));
                }
                _ => {
                    // println!("</{}>", node.get_name());
                }
            },
            _ => {}
        }
        c = child.get_next_sibling();
    }

    XMLNmapRun {
        scanner: properties["scanner"].clone(),
        args: properties["args"].clone(),
        start: properties["start"].clone(),
        startstr: properties["startstr"].clone(),
        version: properties["version"].clone(),
        xmloutputversion: properties["xmloutputversion"].clone(),
        hosts: hosts,
    }
}

fn parse_root(node: &Node) -> Option<XMLNmapRun> {
    match node.get_type().unwrap() {
        NodeType::ElementNode => match node.get_name().as_str() {
            "nmaprun" => Some(parse_run(&node)),
            _ => None,
        },
        NodeType::TextNode => None,
        _ => None,
    }
}

impl Scanner {
    pub fn scan_host_full(ip: &std::net::IpAddr, path: &std::path::Path) -> () {
        std::process::Command::new("nmap")
            .arg("-A")
            .arg("-p-")
            .arg(ip.to_string())
            .arg(["-oX", path.to_str().unwrap()].join(" "))
            .output()
            .expect("failed to execute process");
    }

    pub fn scan_net_ping(cidr: IpCidr, path: &std::path::Path) {
        let mut command = std::process::Command::new("nmap");
        command
            .arg("-sP")
            .arg(cidr.to_string())
            .arg("-oX")
            .arg(path.to_str().unwrap());
        println!("{:?}", command);
        let output = command.output().expect("failed to execute process");
        println!("Output: '{:?}'", output);
    }

    pub fn load_scan(path: &std::path::Path) {
        match &std::fs::read_to_string(path) {
            Ok(p) => {
                let parser = Parser::default();
                let doc = parser.parse_file(path.to_str().unwrap()).unwrap();
                let root = doc.get_root_element().unwrap();
                if let Some(run) = parse_root(&root) {
                    println!("{:?}", run);
                }
            }
            Err(e) => {
                eprintln!("Error reading Run: {}", e);
            }
        }
    }
}
