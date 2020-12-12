extern crate serial;

use serial::prelude::*;
use std::io::prelude::*;
use std::io;
use std::io::ErrorKind::TimedOut;
use std::path::Path;
// use std::sync::Mutex;
use std::time::Duration;
// use std::any::type_name;

fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud115200)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;

    port.set_timeout(Duration::from_millis(90))?;

    Ok(())
}

fn main() {
    rosrust::init("host");
    let device_path = "/tmp/tty_test1";
    let device_path = Path::new(device_path).as_os_str();
    println!("{:?}", device_path);

    // TODO(lucasw) wait and retry opening serial port in a loop
    rosrust::sleep(rosrust::Duration::from_seconds(1));

    let mut port = serial::open(&device_path).unwrap();
    interact(&mut port).unwrap();

    // let mut buf: Vec<u8> = (0..255).collect();
    let mut buf = vec![];
    for _ in 1..20 {
        buf.push('a' as u8);
        buf.push('b' as u8);
    }
    // TODO(lucasw) push a long string into buf
    port.write(&buf[..]).unwrap();
    // port.read(&mut buf[..])?;
    //
    /*
    let mutex = Mutex::new(port);
    let arc = std::sync::Arc::new(mutex);
    let arc2 = arc.clone();
    */
    /*
    let string_callback = {
        move |msg: String| {
            println!("{} {}", msg.data, msg.data.len());
            let mut port = arc2.lock().unwrap();
            // let mut buf = vec![];
            // for ch in msg.data
            port.write(&(&msg.data).as_bytes()[..]).unwrap();
        }
    };*/

    let rate = rosrust::rate(10.0);
    while rosrust::is_ok() {
        let mut buf: Vec<u8> = (0..255).collect();
        let tsec = rosrust::now().seconds();

        // try!(port.write(&buf[..]));
        let response = port.read(&mut buf[..]);
        match response {
            Ok(len) => {
                let buf = &buf[0..len];
                let text = std::str::from_utf8(buf).unwrap();
                match text {
                    "get" => {
                        let val = tsec.cos();
                        let val_str = format!("{} {}", tsec, val);
                        let val_buf = &(&val_str).as_bytes()[..];
                        port.write(val_buf).unwrap();
                    }
                    _ => {
                        println!("received unhandled serial data: {:?} {:?}", len, text);
                    }
                }
            },
            Err(e) => match e.kind() {
                TimedOut => (),
                _ => println!("error {:?}", e),
            },
        }
        rate.sleep();
    }
}
