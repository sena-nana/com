use serialport;
use std::time::Duration;

struct Serial {
    serial: Box<dyn serialport::SerialPort>,
}
impl Serial {
    fn new() -> Self {
        let serial = serialport::new("COM5", 9600)
            .timeout(Duration::from_millis(2000))
            .open()
            .unwrap();
        return Serial { serial };
    }
    fn read(&mut self) -> Vec<u8> {
        let buffer = self.align();
        return buffer;
    }
    fn align(&mut self) -> Vec<u8> {
        let mut buffer: Vec<u8> = [0; 36].to_vec();
        self.serial
            .read_exact(&mut buffer)
            .expect("Error reading buffer");
        let index = buffer.iter().position(|&r| r == 0xAA).unwrap();
        buffer.drain(0..index);
        let mut buffer_left: Vec<u8> = Vec::with_capacity(index);
        self.serial
            .read_exact(&mut buffer_left)
            .expect("Error reading buffer left");
        buffer.append(&mut buffer_left);
        return buffer;
    }
}

fn main() {
    let mut serial = Serial::new();
    loop {
        let buffer = serial.read();
        println!("{:?}", buffer);
    }
}
