use anyhow::Result;
use byteorder::{ByteOrder, LittleEndian};
use crazyflie_link::{Connection, LinkContext, Packet};
use std::time::Duration;
use structopt::StructOpt;

const TARGET_STM32: u8 = 0xFF;
const TARGET_NRF51: u8 = 0xFE;

#[derive(StructOpt, Debug)]
#[structopt(name = "bootloader")]
struct Opt {
    #[structopt(short, long)]
    warm: bool,

    #[structopt(name = "URI")]
    uri: Option<String>,
}

#[derive(Debug)]
struct BootloaderInfo {
    id: u8,
    protocol_version: u8,
    page_size: u16,
    buffer_pages: u16,
    flash_pages: u16,
    start_page: u16,
    cpuid: u16,
}

fn scan_for_bootloader() -> Result<String> {
    let context = crate::LinkContext::new();
    let res = context.scan_selected(vec![
        "radio://0/110/2M/E7E7E7E7E7?safelink=0",
        "radio://0/0/2M/E7E7E7E7E7?safelink=0",
    ])?;

    if res.is_empty() {
        Ok(String::from(""))
    } else {
        Ok(String::from(&res[0]))
    }
}

fn get_info(link: &Connection, target: u8) -> Result<BootloaderInfo> {
    let packet: Packet = vec![0xFF, target, 0x10].into();

    link.send_packet(packet)?;
    let packet = link.recv_packet_timeout(Duration::from_millis(100))?;
    let data = packet.get_data();

    Ok(BootloaderInfo {
        id: data[0],
        protocol_version: data[1],
        page_size: LittleEndian::read_u16(&data[2..4]),
        buffer_pages: LittleEndian::read_u16(&data[4..6]),
        flash_pages: LittleEndian::read_u16(&data[6..8]),
        start_page: LittleEndian::read_u16(&data[8..10]),
        cpuid: LittleEndian::read_u16(&data[10..12]),
    })
}

fn reset_to_bootloader(link: &Connection) -> Result<String> {
    let packet: Packet = vec![0xFF, TARGET_NRF51, 0xFF].into();
    link.send_packet(packet)?;

    let mut new_address = Vec::new();
    loop {
        let packet = link.recv_packet_timeout(Duration::from_millis(100))?;
        let data = packet.get_data();
        if data.len() > 2 && data[0..2] == [TARGET_NRF51, 0xFF] {
            new_address.push(0xb1);
            for byte in data[2..6].iter().rev() { // handle little-endian order
                new_address.push(*byte);
            }
            break;
        }
    }

    for _ in 0..10 {
        let packet: Packet = vec![0xFF, TARGET_NRF51, 0xF0, 0x00].into();
        link.send_packet(packet)?;
    }
    std::thread::sleep(Duration::from_secs(1));

    Ok(format!(
        "radio://0/0/2M/{}?safelink=0",
        hex::encode(new_address).to_uppercase()
    ))
}

fn start_bootloader(context: &LinkContext, warm: bool, uri: &str) -> Result<Connection> {
    let uri = if warm {
        let link = context.open_link(&uri)?;
        let uri = reset_to_bootloader(&link);
        link.close();
        std::thread::sleep(Duration::from_secs(1));
        uri
    } else {
        scan_for_bootloader()
    }?;

    let link = context.open_link(&uri)?;
    Ok(link)
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let context = LinkContext::new();
    let mut uri = String::new();

    if opt.warm {
        uri = match opt.uri {
            Some(uri) => uri,
            None => {
                eprintln!("no uri supplied for warm reset to bootloader");
                std::process::exit(1);
            }
        };
    }

    let link = start_bootloader(&context, opt.warm, &uri)?;

    if let Ok(info) = get_info(&link, TARGET_STM32) {
        println!("\n== stm32 ==\n{:#?}", info);
    }

    if let Ok(info) = get_info(&link, TARGET_NRF51) {
        println!("\n== nrf51 ==\n{:#?}", info);
    }

    Ok(())
}