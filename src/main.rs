
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embassy_sync::channel::Channel;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

// A command that the sensor can send to the motor.
#[derive(Debug)]
enum MotorCommand {
    Stop,
    SetSpeed(u32),
}

// A shared channel that can hold one command.
// 1 means the channel can hold one message at a time.
static CHANNEL: Channel<CriticalSectionRawMutex, MotorCommand, 1> =
    Channel::new();

#[embassy_executor::task]
async fn sensor() {
    println!("Sensor: Waiting for an object...");

    // Simulate waiting for the sensor to detect an object.
    Timer::after(Duration::from_secs(5)).await;

    println!("Sensor: Object detected!");

    // Send a command to the motor task.
    CHANNEL.send(MotorCommand::Stop).await;

    println!("Sensor: Stop command sent!");

    // Timer::after(Duration::from_secs(5)).await;
    CHANNEL.send(MotorCommand::SetSpeed(100)).await;
}    

#[embassy_executor::task]
async fn timer() {
    let mut counter = 0;
    loop {
        println!("Hello World from Embassy! {}", counter);
        counter += 1;

        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn motor() {
    let mut motor_rpm = 50;

    println!("Motor: Running at {} RPM", motor_rpm);

    loop {
        println!("Motor: Waiting for a command...");

        let command = CHANNEL.receive().await;

        match command {
            MotorCommand::Stop => {
                motor_rpm = 0;
                println!("Motor: Stopped. Speed is {} RPM", motor_rpm);
            }
            MotorCommand::SetSpeed(speed) => {
                motor_rpm = speed;
                println!("Motor: Speed is now {} RPM", motor_rpm);
            }
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {

    spawner.spawn(sensor()).unwrap();
    spawner.spawn(motor()).unwrap();

}