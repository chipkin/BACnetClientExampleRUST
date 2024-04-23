pub mod cas_bacnet_stack_example_constants;
pub mod cas_bacnet_stack_adapters;

use cas_bacnet_stack_example_constants as bacnet_const;
use cas_bacnet_stack_adapters as adapter;

use std::net::UdpSocket;

use once_cell::sync::Lazy;
use std::sync::Mutex;

use std::time::SystemTime;
use std::time::Duration;

use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;

use std::thread;

use std::ptr;

use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

// Constants
const APPLICATION_VERSION: &str = "0.0.1";  // See CHANGELOG.md for a full list of changes.
const MAX_RENDER_BUFFER_LENGTH: usize = 1497;
const SETTING_CLIENT_DEVICE_INSTANCE: u32 = 389002;
const SETTING_DOWNSTREAM_DEVICE_INSTANCE: u32 = 389001;
const SETTING_DEFAULT_DOWNSTREAM_DEVICE_IP_ADDRESS: &str = "192.168.68.105";
const DOWNSTREAM_CONNECTION_STRING: [u8; 6] = [192, 168, 68, 105, 186, 192];

// Static Variables
static socket: Lazy<UdpSocket> = Lazy::new(|| {
    match UdpSocket::bind("192.168.68.109:47808") {
		Ok(udp_socket) => {
			println!("UDP Socket Setup Success");
			if let Err(err) = udp_socket.set_read_timeout(Some(Duration::from_millis(50))) {
				panic!("UDP Socket Read Timeout Setting Failed");
			}
			udp_socket
		},
		_ => {
			panic!("UDP Socket Setup Failed");
		}
	}
});
static invoke_id: Lazy<Mutex<u8>> = Lazy::new(|| {
	let mut m = 0;
    Mutex::new(m)
});

// Main function
fn main() {
	// Print versioning
	println!("CAS BACnet Stack Version: {:?}.{:?}.{:?}.{:?}", 
        adapter::get_api_major_version().unwrap(), adapter::get_api_minor_version().unwrap(), adapter::get_api_patch_version().unwrap(), adapter::get_api_build_version().unwrap());
    println!("Application Version: {:?}", APPLICATION_VERSION);

	// Loading CAS BACnet Stack functions
	if let Err(err) = load_bacnet_functions() {
		panic!("Unable to load functions from DLL");
	}

	// Add device
	if let Ok(x) = adapter::add_device(SETTING_CLIENT_DEVICE_INSTANCE) {
		if x {
			println!("Device added");
		} else {
			println!("ERROR: Device was unable to be added");
		}
	} else {
		println!("ERROR: Add Device failed");
	}

	// Set services enabled
	if let Ok(x) = adapter::set_service_enabled(SETTING_CLIENT_DEVICE_INSTANCE, bacnet_const::SERVICE_I_AM.into(), true) {
		if x {
			println!("I Am service enabled");
		} else {
			println!("ERROR: I Am service was unable to be enabled");
		}
	} else {
		println!("ERROR: Enable service failed");
	}
	if let Ok(x) = adapter::set_service_enabled(SETTING_CLIENT_DEVICE_INSTANCE, bacnet_const::SERVICE_WHO_IS.into(), true) {
		if x {
			println!("Who Is service enabled");
		} else {
			println!("ERROR: Who Is service was unable to be enabled");
		}
	} else {
		println!("ERROR: Enable service failed");
	}
	if let Ok(x) = adapter::set_service_enabled(SETTING_CLIENT_DEVICE_INSTANCE, bacnet_const::SERVICE_READ_PROPERTY_MULTIPLE.into(), true) {
		if x {
			println!("Read Property Multiple service enabled");
		} else {
			println!("ERROR: Read Property Multiple service was unable to be enabled");
		}
	} else {
		println!("ERROR: Enable service failed");
	}

	// Main Loop
	println!("Entering main loop...");
	let stdin_channel = spawn_stdin_channel();
    loop {
		adapter::bacnet_loop().unwrap();
		if let Ok(key) = stdin_channel.try_recv() {
            if check_end_loop(&key) {
				break;
			}
        }
		thread::sleep(Duration::from_millis(0));
    }
}

fn load_bacnet_functions() -> Result<bool, Box<dyn std::error::Error>> {
	adapter::register_callback_receive_message(callback_receive_message)?;
	adapter::register_callback_send_message(callback_send_message)?;
	adapter::register_callback_get_system_time(callback_get_system_time)?;
	Ok(true)
}

fn check_end_loop(key: &str) -> bool {
	if key == "q\r\n" || key == "Q\r\n" {
		return true;
	} 
	else if key == "w\r\n" || key == "W\r\n" {
		who_is();
	} 
	else if key == "r\r\n" || key == "R\r\n" {
		read_property_multiple();
	} 
	else {
		println!("Invalid input. Please enter one of the follow inputs:");
		println!("W => Who-Is message");
		println!("R => ReadPropertyMultiple message");
		println!("Q => Quit application");
	}
	false
}

fn who_is() {
	// Send the message
	println!("Sending WhoIs with no range. timeout=[3]...");
	if let Err(err) = adapter::send_who_is(DOWNSTREAM_CONNECTION_STRING.as_ptr(), 6, 0, true, 0, ptr::null(), 0) {
		println!("Who Is with no range failed.");
	}

	// Wait 2 seconds
	thread::sleep(Duration::from_millis(2));

	println!("Sending WhoIs with range, low=[389000], high=[389999] 3 second timeout...");
	if let Err(err) = adapter::send_who_is_with_limits(389000, 389999, DOWNSTREAM_CONNECTION_STRING.as_ptr(), 6, 0, true, 0, ptr::null(), 0) {
		println!("Who Is with range failed.");
	}

	thread::sleep(Duration::from_millis(2));

	println!("Sending WhoIs to specific network. network=[15], timeout=[3]");
	if let Err(err) = adapter::send_who_is(DOWNSTREAM_CONNECTION_STRING.as_ptr(), 6, 0, true, 15, ptr::null(), 0) {
		println!("Who Is to specific network failed.");
	}

	thread::sleep(Duration::from_millis(2));

	println!("Sending WhoIs to broadcast network. network=[65535], timeout=[3]");
	if let Err(err) = adapter::send_who_is(DOWNSTREAM_CONNECTION_STRING.as_ptr(), 6, 0, true, 65535, ptr::null(), 0) {
		println!("Who Is to broadcast network failed.");
	}

	thread::sleep(Duration::from_millis(2));
}

fn read_property_multiple() {
	println!("Sending Read Property. DeviceID=[{0}], property=[{1}], timeout=[3]...", SETTING_DOWNSTREAM_DEVICE_INSTANCE, bacnet_const::PROPERTY_IDENTIFIER_ALL);

	// Get the object names. All objects have object names. 
	adapter::build_read_property(bacnet_const::OBJECT_TYPE_ANALOG_INPUT, 0, bacnet_const::PROPERTY_IDENTIFIER_OBJECT_NAME, false, 0).unwrap();
	adapter::build_read_property(bacnet_const::OBJECT_TYPE_DEVICE, 389001, bacnet_const::PROPERTY_IDENTIFIER_OBJECT_NAME, false, 0).unwrap();
	adapter::build_read_property(bacnet_const::OBJECT_TYPE_CHARACTERSTRING_VALUE, 40, bacnet_const::PROPERTY_IDENTIFIER_OBJECT_NAME, false, 0).unwrap();

	// Get the present value from objects that have a present value property. 
	adapter::build_read_property(bacnet_const::OBJECT_TYPE_ANALOG_INPUT, 0, bacnet_const::PROPERTY_IDENTIFIER_PRESENT_VALUE, false, 0).unwrap();
	adapter::build_read_property(bacnet_const::OBJECT_TYPE_CHARACTERSTRING_VALUE, 40, bacnet_const::PROPERTY_IDENTIFIER_PRESENT_VALUE, false, 0).unwrap();
	
	// Send the Read property message
	let mut id = invoke_id.lock().unwrap();
	if let Err(err) = adapter::send_read_property(&mut *id, DOWNSTREAM_CONNECTION_STRING.as_ptr(), 6, 0, 0, ptr::null(), 0) {
		println!("Read Property failed.");
	}
	
	// Wait 2 seconds
	thread::sleep(Duration::from_millis(2));
}

// Referenced: https://stackoverflow.com/questions/30012995/how-can-i-read-non-blocking-from-stdin
fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}

fn callback_receive_message(message: *mut u8, max_message_length: u16, received_connection_string: *mut u8, max_connection_string_length: u8, received_connection_string_length: *mut u8, network_type: *mut u8) -> u16 {
		
	// Check parameters
	if message.is_null() || max_message_length == 0 {
		println!("Invalid input buffer");
		return 0;
	}
	if received_connection_string.is_null() || max_connection_string_length == 0 {
		println!("Invalid connection string buffer");
		return 0;
	}
	if max_connection_string_length < 6 {
		println!("Not enough space for a UDP connection string");
		return 0;
	}

	let port: u16;

	// Attempt to read bytes
	let mut buf: [u8; MAX_RENDER_BUFFER_LENGTH] = [0; MAX_RENDER_BUFFER_LENGTH];
	let (bytes_read, src_addr) = if let Ok((bytes_read, src_addr)) = socket.recv_from(&mut buf) {
		(bytes_read, src_addr)
	} else {
		(0 as usize, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080))
	};
	if bytes_read > 0 {
		port = src_addr.port();
		let ip_address = src_addr.ip().to_canonical();
		println!("FYI: Received message from [{0}:{1}], length [{2}]", ip_address, port, bytes_read);
		println!("Message: {:?}", &buf[0..usize::from(bytes_read)]);

		// Convert the IP Address to the connection string
		if !convert_ip_address_to_bytes(src_addr.ip(), received_connection_string, max_connection_string_length) {
			println!("Failed to convert the ip address into a connectionString");
			return 0;
		}
		unsafe {
			*received_connection_string.add(4) = (port / 256) as u8;
			*received_connection_string.add(5) = (port % 256) as u8;

			*received_connection_string_length = 6;
			*network_type = bacnet_const::NETWORK_TYPE_IP;
		}

		if usize::from(max_message_length) < bytes_read || usize::from(MAX_RENDER_BUFFER_LENGTH) < bytes_read {
			return 0;
		}

		let mut index = 0;
		unsafe {
			while index < bytes_read {
				*message.add(index) = buf[index];
				index += 1;
			}
		}
	}

	bytes_read.try_into().unwrap()
}

fn callback_send_message(message: *const u8, message_length: u16, connection_string: *const u8, connection_string_length: u8, network_type: u8, broadcast: bool) -> u16 {
	println!("callback_send_message");

	if message.is_null() || message_length == 0 {
		println!("Nothing to send");
		return 0;
	}
	if connection_string.is_null() || connection_string_length == 0 {
		println!("No connection string");
		return 0;
	}

	// Verify Network Type
	if network_type != bacnet_const::NETWORK_TYPE_IP {
		println!("Message for different network");
		return 0;
	}

	// Prepare the IP Address
	let ip_address: Ipv4Addr;
	unsafe {
		if broadcast {
			ip_address = Ipv4Addr::new(192, 168, 68, 255);
		}
		else {
			ip_address = Ipv4Addr::new(*connection_string.add(0), *connection_string.add(1), *connection_string.add(2), *connection_string.add(3));
		}
	}
	

	// Get the port
	let mut port: u16 = 0;
	unsafe {
		port = port + *connection_string.add(4) as u16 * 256;
		port = port + *connection_string.add(5) as u16;
	}

	println!("FYI: Sending message to [{0}:{1}], length [{2}]", ip_address, port, message_length);

	// Send the message
	let mut buf = [0; MAX_RENDER_BUFFER_LENGTH];

	if usize::from(message_length) > MAX_RENDER_BUFFER_LENGTH {
		println!("Message too large for buffer");
		return 0;
	} else {
		let mut index: usize = 0;
		unsafe {
			while index < message_length.into() {
				buf[index] = *message.add(index.into());
				index += 1;
			}
		}
	}

	if let Ok(x) = socket.send_to(&buf[0..usize::from(message_length)], (ip_address, port)) {
		return message_length;
	} else {
		println!("Failed to send message");
		return 0;
	}
}

fn convert_ip_address_to_bytes(ip_address: IpAddr, received_connection_string: *mut u8, max_connection_string_length: u8) -> bool {
	if max_connection_string_length < 4 {
		return false;
	}
	let ip_address_v4 = ip_address.to_canonical();
	match ip_address_v4 {
        IpAddr::V4(address) => {
			let ip_address_octet = address.octets();
			unsafe {
				*received_connection_string.add(0) = ip_address_octet[0];
				*received_connection_string.add(1) = ip_address_octet[1];
				*received_connection_string.add(2) = ip_address_octet[2];
				*received_connection_string.add(3) = ip_address_octet[3];
			}
			return true;
		},
        _ => {
			println!("Invalid IP Address");
			return false;
		},
     }
}

fn callback_get_system_time() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}