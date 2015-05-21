use std::num;

const BOLTZMANN: f64 = 0.00000005670373_f64;
const PI: f64 =  6.28318530717958647692528676655900576_f64;

fn main() {
    println!("Hello, world!");
    
    

    let mug = Mug {height: 0.095, radius: 0.04, weight: 0.282, volume: 0.364*1000.0};
    /*
    0	609.9	0.003767
    5	870	    0.005387
    10	1225	0.007612
    15	1701	0.01062
    20	2333	0.014659
    25	3130	0.019826
    30	4234	0.027125 */
    let evap_loss = evaporation_energy(mug.get_top_surface_area(), 0.019826, 0.0147);

    let temp_reduction_needed = (100.0-25.0)*4200.0*mug.get_volume(); // kJ
    println!("temp_reduction_needed: {} kJ",  temp_reduction_needed);

    println!("Energy from evaporation per second: {} kJ/s",  evap_loss);
    println!("This will take: {} seconds  ({} mins)",  temp_reduction_needed/evap_loss, (temp_reduction_needed/evap_loss)/60.0);

    let init_temp1 = final_temp(&mug, 23.0, 100.0);
    let init_temp2 = final_temp(&mug, init_temp1, 100.0);

    println!("volume: {}  init_temp1: {}  init_temp2: {}",  mug.get_volume(), init_temp1, init_temp2);

    let mut temp1 = Vec::new();
    let mut temp2 = Vec::new();

    temp1.push(init_temp1);
    temp2.push(init_temp2);

    let max_time = 3600; // s
    let room_temp = 25.0;
    let total_radiative_area = mug.get_side_surface_area() + mug.get_top_surface_area();
    println!("Area: {}m^2  Temp: {}K", total_radiative_area, init_temp1);
    println!("Area: {}m^2  Temp: {}K", total_radiative_area, init_temp2);

    let temp_reduction_needed = (init_temp1-room_temp)*4200.0*mug.get_volume()/1000.0; // kJ
    println!("temp_reduction_needed: {} kJ",  temp_reduction_needed);

    

    for time in 1..max_time {
        let prev_temp1 = temp1[time-1];
        let power1 = power_emitted(total_radiative_area, prev_temp1, room_temp, 1.0);
        temp1.push(new_temperature(mug.get_volume(), prev_temp1, power1));

        let prev_temp2 = temp2[time-1];
        let power2 = power_emitted(total_radiative_area, prev_temp2, room_temp, 1.0);
        temp2.push(new_temperature(mug.get_volume(), prev_temp2, power2));
    }

    println!("1: {}, \t 2: {}", temp1[max_time-1], temp2[max_time-1]);
}

pub struct Mug 
{
    height: f64,
    radius: f64,
    weight: f64,
    volume: f64
}

impl Mug {

    pub fn get_top_surface_area(&self) -> f64 {
        PI * self.radius*self.radius
    }

    pub fn get_side_surface_area(&self) -> f64 {
        2.0 * PI * self.radius * self.height
    }

    // volume in litres
    pub fn get_volume(&self) -> f64 {
       //self.get_top_surface_area()*self.height
       self.volume / 1000.0
    }

}

pub fn final_temp(mug: &Mug, mug_temp: f64, water_temp: f64) -> f64 {
    let water_weight = mug.get_volume(); // kg
    ((water_weight * 4200.0) * water_temp + (mug.weight * 1085.0) * mug_temp) / ((water_weight * 4200.0) + (mug.weight * 1085.0))
}

/**
 * Total power radiated in kJ/s
 */
pub fn power_emitted(area: f64, init_temp: f64, room_temp: f64, emissivity: f64) -> f64 {
    // Using stefan-boltzmann law
    let temp_4 = (init_temp+273.0).powi(4) - (room_temp+273.0).powi(4);
    temp_4*BOLTZMANN*area*emissivity
}


/// volume  litres  == 0.0001 m^3
/// start_temp Celcius
/// power_loss kJ/s == W
pub fn new_temperature(volume: f64, start_temp: f64, power_loss: f64) -> f64 {
    start_temp - power_loss / (4200.0 * volume)
}

// Mass of Mug * Cp(mug) * (x-23) = Mass of Water * Cp(wat) * (100-x)
// So that assumes mug is at 23 and the water is at 100. porcelin is 1085. I used 291 for mugg weight and 298 for water ( filling to the brim)

// tht tells u two things, the temp of the preheated mug and the temp of the water for a non preheated mug. Once u have x u stick it into the same equation but where old x is now the mug temp

/**
 * @return Heat loss in kJ/s (kW)
 */
pub fn evaporation_energy(area: f64, humidity_sat: f64, humidity_air: f64) -> f64
{
    let evaporation_coefficient: f64 = 25.0;
    let evap_heat: f64 = 2270.0; // kJ/kg
    evap_heat*evaporation_coefficient*area*(humidity_sat-humidity_air)/3600.0
}