use battery::units::ratio::percent;
use battery::Manager;
use failure::Error;

fn main() -> Result<(), Error> {
    let mut batteries = Manager::new()?.batteries()?;
    let battery = batteries.nth(0).expect("failed to get battery info");
    let battery = battery.expect("failed to get battery info");

    println!("{:.1}", battery.state_of_charge().get::<percent>());

    Ok(())
}
