use dotenv::dotenv;
use serialport;
use std::env;
use std::fmt::{self, Debug, Formatter};
use std::time::{Duration, Instant};
use vtubestudio::data::{
    EnumString, InjectParameterDataMode, InjectParameterDataRequest, ParameterValue,
    StatisticsRequest,
};
use vtubestudio::{Client, ClientEvent, Error};
struct Package {
    attention: u8,
    meditation: u8,
    // delta: u32,
    // theta: u32,
    // low_alpha: u32,
    // high_alpha: u32,
    // low_beta: u32,
    // high_beta: u32,
    // low_gamma: u32,
    // mid_gamma: u32,
}
// fn pick(buffer: &Vec<u8>, index: usize) -> u32 {
//     return (buffer[index] as u32) << 16
//         | (buffer[index + 1] as u32) << 8
//         | (buffer[index + 2] as u32);
// }
impl Package {
    fn new(buffer: Vec<u8>) -> Self {
        let attention = buffer[32];
        let meditation = buffer[34];
        // let delta = pick(&buffer, 7);
        // let theta = pick(&buffer, 10);
        // let low_alpha = pick(&buffer, 13);
        // let high_alpha = pick(&buffer, 16);
        // let low_beta = pick(&buffer, 19);
        // let high_beta = pick(&buffer, 22);
        // let low_gamma = pick(&buffer, 25);
        // let mid_gamma = pick(&buffer, 28);
        return Package {
            attention,
            meditation,
            // delta,
            // theta,
            // low_alpha,
            // high_alpha,
            // low_beta,
            // high_beta,
            // low_gamma,
            // mid_gamma,
        };
    }
    //     fn print(&self) {
    //         println!(
    //             "attention: {}, meditation: {}",
    //             self.attention, self.meditation
    //         );
    //         println!(
    //             "delta: {}, theta: {}, low_alpha: {}, high_alpha: {}",
    //             self.delta, self.theta, self.low_alpha, self.high_alpha
    //         );
    //         println!(
    //             "low_beta: {}, high_beta: {}, low_gamma: {}, mid_gamma: {}",
    //             self.low_beta, self.high_beta, self.low_gamma, self.mid_gamma
    //         );
    //     }
}
impl Debug for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "attention: {}, meditation: {}",
            self.attention, self.meditation,
        )
    }
}

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
    fn read(&mut self) -> Package {
        let buffer = Package::new(self.align());
        return buffer;
    }
    fn align(&mut self) -> Vec<u8> {
        loop {
            let mut buffer: Vec<u8> = [0; 36].to_vec();
            let mut index = 0;

            self.serial
                .read_exact(&mut buffer)
                .expect("Error reading buffer");
            while index < 33
                && !(buffer[index] == 0xAA
                    && buffer[index + 1] == 0xAA
                    && buffer[index + 2] == 0x20)
            {
                index += 1;
            }
            if index == 33 {
                continue;
            }

            let mut buffer_left: Vec<u8> = buffer.split_off(index);
            self.serial
                .read_exact(&mut buffer)
                .expect("Error reading buffer left");
            buffer_left.append(&mut buffer);
            return buffer_left;
        }
    }
}
async fn vts_init() -> Result<Client, Error> {
    dotenv().ok();
    let token = Some(env::var("TOKEN").expect("Expected a token in the environment"));
    let (mut client, mut events) = Client::builder()
        .auth_token(token)
        .authentication("Plugin name", "Developer name", None)
        .build_tungstenite();
    tokio::spawn(async move {
        while let Some(event) = events.next().await {
            match event {
                ClientEvent::NewAuthToken(new_token) => {
                    println!("Got new auth token: {new_token}");
                }
                _ => {
                    println!("Got event: {:?}", event);
                }
            }
        }
    });
    let resp = client.send(&StatisticsRequest {}).await?;
    println!("VTube Studio has been running for {}ms", resp.uptime);
    Ok(client)
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut serial = Serial::new();
    let mut client = vts_init().await?;
    loop {
        let now = Instant::now();
        let buffer = serial.read();
        client
            .send(&InjectParameterDataRequest {
                parameter_values: vec![
                    ParameterValue {
                        id: "VoiceA".to_string(),
                        value: buffer.attention as f64,
                        weight: Some(1.0),
                    },
                    ParameterValue {
                        id: "VoiceI".to_string(),
                        value: buffer.meditation as f64,
                        weight: Some(1.0),
                    },
                ],
                face_found: false,
                mode: Some(EnumString::from(InjectParameterDataMode::Add)),
            })
            .await?;
        println!("{:?}", buffer);
        println!("Time: {}ms", now.elapsed().as_millis());
    }
}
