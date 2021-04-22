extern crate cidr_utils;

use crate::network::{self, scanner::XMLNmapHost};
use cidr_utils::cidr::{IpCidr, Ipv4Cidr};
use std::str::FromStr;

use serde::{de, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Host {
    nmap: Option<XMLNmapHost>,
    ip: std::net::IpAddr,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Net {
    passive_arp: bool,
    // This is annoying me, I suck at understanding serde and can't seem to figure out how to deserialize IpCidr, so String it is for the time being
    cidr: String,
    hosts: Vec<Host>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    path: Option<String>,
    nets: Vec<Net>,
    client: Option<Client>,
}

impl Project {
    pub fn new() -> Project {
        Project {
            path: None,
            nets: Vec::new(),
            client: None,
        }
    }

    pub fn export(&self, path: &std::path::Path) -> std::io::Result<()> {
        std::fs::create_dir_all(path).unwrap();
        let mut file = File::create(path.join("project.json"))?;
        file.write_all(serde_json::to_string(self)?.as_bytes())?;
        Ok(())
    }

    pub fn import(path: &std::path::Path) -> std::io::Result<Project> {
        let file = File::open(path.join("project.json"))?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        let mut project: Project = serde_json::from_str(&contents)?;
        project.path = Some(path.to_str().unwrap().to_string());
        Ok(project)
    }

    pub fn scan(&self) {
        if let Some(path) = self.path.clone() {
            let ppath = std::path::Path::new(&path);
            std::fs::create_dir_all(ppath.join("scans/nmap")).unwrap();

            for net in self.nets.clone().into_iter() {
                // println!("Launching net scan: {:?}", net);
                network::scanner::Scanner::scan_net_ping(
                    IpCidr::from_str(net.cidr.clone()).unwrap(),
                    &ppath.join("scans/nmap").join(net.cidr.replace("/", "--")),
                );
            }
        }
    }

    pub fn add_net(&mut self, cidr: IpCidr) {
        // TODO: check of hosts overlap and do not add if there are
        self.nets.push(Net {
            passive_arp: false,
            cidr: cidr.to_string(),
            hosts: cidr.iter().map(|ip| Host { nmap: None, ip: ip }).collect(),
        })
        // for ip in cidr.iter() {
        //     if self.nets.iter_mut() {
        //         self.nets.push(Host { nmap: None, ip: ip });
        //     }
        // }
    }
}
