
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::signal::Signal;

#[derive(Debug)]
enum MotorCommand {
    Stop
}

static COMMANDS: Channel<CriticalSectionRawMutex, MotorCommand, 2> =
    Channel::new();

static TARGET_SPEED: Signal<CriticalSectionRawMutex, u32> =
    Signal::new();

#[embassy_executor::task]
async fn sensor() {
    println!("Sensor: Waiting for an object...");
    Timer::after(Duration::from_secs(5)).await;

    println!("Sensor: Object detected!");
    COMMANDS.send(MotorCommand::Stop).await;

    Timer::after(Duration::from_secs(3)).await;
    println!("Sensor: Object cleared. Requesting 100 RPM.");

    TARGET_SPEED.signal(100);
}

#[embassy_executor::task]
async fn motor() {
    let mut motor_rpm = 50;
    println!("Motor: Started at {} RPM", motor_rpm);

    loop {
        // Check for a queued command without waiting.
        if let Ok(command) = COMMANDS.try_receive() {
            match command {
                MotorCommand::Stop => {
                    motor_rpm = 0;
                    println!("Motor: STOP. Speed = {} RPM", motor_rpm);
                }
            }
        }

        // Check for a new target speed without waiting.
        if let Some(speed) = TARGET_SPEED.try_take() {
            motor_rpm = speed;
            println!("Motor: Target speed = {} RPM", motor_rpm);
        }

        // Let other tasks run before checking again.
        Timer::after(Duration::from_millis(100)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    spawner.spawn(sensor()).unwrap();
    spawner.spawn(motor()).unwrap();
}
