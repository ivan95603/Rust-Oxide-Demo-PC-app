// Copyright 2022, Ivan Palijan <ivan95.603@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT> or BSD, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

//! This application is made to interface with MCU via UART.
//!
//! It displays data received from sensors MLX90614 and BMP180 connected to MCU.
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal

#![allow(dead_code)]
#![deny(missing_docs)]
#![deny(warnings)]

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, Box as Box_, Orientation};

use std::io::{self};
use std::time::Duration;

use std::str;

enum Message {
    /// Enum parameter for updating from MLX Object temperature
    UpdateObjectTemperatureLabel(String),
    /// Enum parameter for updating from MLX Ambient temperature
    UpdateAmbientTemperatureLabel(String),
    /// Enum parameter for updating from BMP180 pressure
    UpdatePressureLabel(String),
    /// Enum parameter for updating from BMP180 temperature
    UpdatePressureTemperatureLabel(String),
}

/// Function that generates all user controls 
fn build_ui(application: &gtk::Application) {

    let window = ApplicationWindow::builder()
    .application(application)
    .title("Oxide Demo PC App")
    .default_width(350)
    .default_height(70)
    .build();

    // Vertical Box that holds all controls
    let vbox = Box_::new(Orientation::Vertical, 0);


    // Horizontal box that holds Object temperature from MLX sensor
    let hbox_object_temperature = Box_::new(Orientation::Horizontal, 5);

    let label_object_temperature_label = Label::new(gtk::glib::bitflags::_core::option::Option::Some("Object temperature: ")) ;
    hbox_object_temperature.append(&label_object_temperature_label);

    let label_object_temperature_data = Label::new(gtk::glib::bitflags::_core::option::Option::Some("Object temperature DATA")) ;
    let label_object_temperature_data_clone = label_object_temperature_data.clone();
    hbox_object_temperature.append(&label_object_temperature_data);

    vbox.append(&hbox_object_temperature);


    // Horizontal box that holds Ambient temperature from MLX sensor
    let hbox_ambient_temperature = Box_::new(Orientation::Horizontal, 5);

    let label_ambient_temperature_label = Label::new(gtk::glib::bitflags::_core::option::Option::Some("Ambient temperature: ")) ;
    hbox_ambient_temperature.append(&label_ambient_temperature_label);

    let label_ambient_temperature_data = Label::new(gtk::glib::bitflags::_core::option::Option::Some("Ambient temperature DATA")) ;
    let label_ambient_temperature_data_clone = label_ambient_temperature_data.clone();
    hbox_ambient_temperature.append(&label_ambient_temperature_data);

    vbox.append(&hbox_ambient_temperature);


    // Horizontal box that holds pressure from BMP sensor
    let hbox_pressure = Box_::new(Orientation::Horizontal, 5);

    let label_pressure_label = Label::new(gtk::glib::bitflags::_core::option::Option::Some("Pressure: ")) ;
    hbox_pressure.append(&label_pressure_label);

    let label_pressure_data = Label::new(gtk::glib::bitflags::_core::option::Option::Some("Pressure DATA")) ;
    let label_pressure_data_clone = label_pressure_data.clone();
    hbox_pressure.append(&label_pressure_data);

    vbox.append(&hbox_pressure);


    // Horizontal box that holds temperature from BMP sensor
    let hbox_pressure_temperature = Box_::new(Orientation::Horizontal, 5);

    let label_pressure_temperature_label = Label::new(gtk::glib::bitflags::_core::option::Option::Some("Barometer temperature: ")) ;
    hbox_pressure_temperature.append(&label_pressure_temperature_label);

    let label_pressure_temperature_data = Label::new(gtk::glib::bitflags::_core::option::Option::Some("Pressure DATA")) ;
    let label_pressure_temperature_data_clone = label_pressure_temperature_data.clone();
    hbox_pressure_temperature.append(&label_pressure_temperature_data);

    vbox.append(&hbox_pressure_temperature);


    window.set_child(Some(&vbox));

   // Create a new sender/receiver pair with default priority
   let (sender, receiver) = gtk::glib::MainContext::channel(gtk::glib::PRIORITY_DEFAULT);

   // Spawn the thread and move the sender in there
   std::thread::spawn(move || {

   let port_name = "/dev/ttyUSB0";
   let baud_rate = 115200 as u32;

   let port = serialport::new(port_name, baud_rate)
       .timeout(Duration::from_millis(10))
       .open();

   match port {
       Ok(mut port) => {
           let mut serial_buf: Vec<u8> = vec![0; 1000];
           println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
           loop {
               match port.read(serial_buf.as_mut_slice()) {
                   Ok(t) => {
                       let s = match str::from_utf8(&serial_buf[..t]) {
                           Ok(v) => v,
                           Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                       };

                       let split = s.split(":").collect::<Vec<&str>>();
                       let size = split.len();

                       match size {
                           2 => {
                                match split[0]{
                                    // OT is start of MLX Object temperature data packet received from MCU 
                                    "OT" => {
                                        let _ = sender.send(Message::UpdateObjectTemperatureLabel(split[1].trim().to_owned()));
                                    },
                                    // AT is start of MLX Ambient temperature data packet received from MCU
                                    "AT" => {
                                        let _ = sender.send(Message::UpdateAmbientTemperatureLabel(split[1].trim().to_owned()));
                                    },
                                    // OT is start of BMP180 Pressure data packet received from MCU
                                    "P" => {
                                        let _ = sender.send(Message::UpdatePressureLabel(split[1].trim().to_owned()));
                                    },
                                    // PTC is start of BMP180 temperature data packet received from MCU
                                    "PTC" => {
                                        let _ = sender.send(Message::UpdatePressureTemperatureLabel(split[1].trim().to_owned()));
                                    },
                                    _ => {
                                        println!("Data code not valid.");
                                    }
                                }
                           },
                           _ => {
                               println!("Arriving string size problem. Size: {}", size);
                           }
                       }
                   },
                   Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                   Err(e) => eprintln!("{:?}", e),
               }
           }
       }
       Err(e) => {
           eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
           ::std::process::exit(1);
       }
   }
   });


    // Attach the receiver to the default main context (None)
    // and on every message update the label accordingly.
    receiver.attach(None, move |msg| {
        match msg {
            Message::UpdateObjectTemperatureLabel(text) => {
                // println!("UpdateObjectTemperatureLabel {}", &text);
                label_object_temperature_data_clone.set_text(text.as_str());
            },
            Message::UpdateAmbientTemperatureLabel(text) => {
                // println!("UpdateAmbientTemperatureLabel {}", &text);
                label_ambient_temperature_data_clone.set_text(text.as_str());
            },
            Message::UpdatePressureLabel(text) => {
                // println!("UpdatePressureLabel {}", &text);
                label_pressure_data_clone.set_text(text.as_str());
            },
            Message::UpdatePressureTemperatureLabel(text) => {
                // println!("UpdatePressureLabel {}", &text);
                label_pressure_temperature_data_clone.set_text(text.as_str());
            },
        }

        // Returning false here would close the receiver
        // and have senders fail
        gtk::glib::Continue(true)
    });

    window.show();
}

/// Main program funcion
fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();
}