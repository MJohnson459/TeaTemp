extern crate gnuplot;

use gnuplot::*;

const BOLTZMANN: f64 = 0.00000005670373_f64;
const PI: f64 =  6.28318530717958647692528676655900576_f64;

fn main() {
   
    let mug = Mug {height: 0.095, radius: 0.04, weight: 0.282, volume: 364.0};
    
    //let evap_loss = evaporation_energy(mug.get_top_surface_area(), 0.019826, 0.0147);

    //let temp_reduction_needed = (100.0-25.0)*4200.0*mug.get_volume(); // kJ
    //println!("temp_reduction_needed: {} kJ",  temp_reduction_needed);

    //println!("Energy from evaporation per second: {} kJ/s",  evap_loss);
    //println!("This will take: {} seconds  ({} mins)",  temp_reduction_needed/evap_loss, (temp_reduction_needed/evap_loss)/60.0);

    let init_temp1 = final_temp(&mug, 23.0, 100.0);
    let init_temp2 = final_temp(&mug, init_temp1, 100.0);

    // Add milk
    // (mass1: f64, temp1: f64, spec_heat1: f64, mass2: f64, temp2: f64, spec_heat2: f64)
    let init_temp3 = temp_equilibrium(mug.get_volume(), init_temp1, 4200.0,  0.1, 4.0, 4200.0);
    let init_temp4 = temp_equilibrium(mug.get_volume(), init_temp2, 4200.0,  0.1, 4.0, 4200.0);

    println!("volume: {}  init_temp1: {}  init_temp2: {}",  mug.get_volume(), init_temp1, init_temp2);

    let max_time = 3600u32; // s
    let room_temp = 25.0;

    let mut experiments = Vec::new();
    experiments.push(Experiment::new(init_temp1, room_temp, vec!(Caption("Normal Cup"), Color("red"))));
    experiments.push(Experiment::new(init_temp2, room_temp, vec!(Caption("Heated Cup"), Color("blue"))));
    experiments.push(Experiment::new(init_temp3, room_temp, vec!(Caption("Normal Cup w/ Milk"), Color("green"))));
    experiments.push(Experiment::new(init_temp4, room_temp, vec!(Caption("Heated Cup w/ Milk"), Color("purple"))));
    
    for experiment in &mut experiments {
        experiment.simulate(max_time, &mug);
    }

    plot_temps(max_time, &experiments);
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

pub struct  Experiment<'l> {
    init_temp: f64,
    room_temp: f64,
    result: Vec<f64>,
    plot_options: Vec<PlotOption<'l>>,
}

impl<'l> Experiment<'l> {
    pub fn new(init_temp: f64, room_temp: f64, plot_options: Vec<PlotOption<'l>>) -> Experiment {
        Experiment {
            init_temp: init_temp,
            room_temp: room_temp,
            result: Vec::new(),
            plot_options: plot_options,
        }
    }

    pub fn simulate(&mut self, max_time: u32, mug: &Mug) {
        self.result = simulate(max_time, self.init_temp, self.room_temp, &mug);
    }
}

pub fn final_temp(mug: &Mug, mug_temp: f64, water_temp: f64) -> f64 {
    let water_weight = mug.get_volume(); // kg
    temp_equilibrium(water_weight, water_temp, 4200.0, mug.weight, mug_temp, 1085.0)
}

pub fn temp_equilibrium(mass1: f64, temp1: f64, spec_heat1: f64, mass2: f64, temp2: f64, spec_heat2: f64) -> f64 {
    ((mass1 * spec_heat1) * temp1 + (mass2 * spec_heat2) * temp2) / ((mass1 * spec_heat1) + (mass2 * spec_heat2))
}

pub fn simulate(sim_time: u32, init_temp: f64, room_temp: f64, mug: &Mug) -> Vec<f64> {
    let mut result = Vec::new();

    result.push(init_temp);

    let total_radiative_area = mug.get_side_surface_area() + mug.get_top_surface_area();
    println!("Area: {}m^2  Temp: {}K", total_radiative_area, init_temp);

    for time in 1..sim_time as usize {
        let prev_temp = result[time-1];
        let power = power_emitted(total_radiative_area, prev_temp, room_temp, 1.0);
        result.push(new_temperature(mug.get_volume(), prev_temp, power));
    }

    result
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

pub fn plot_temps(max_time: u32, experiments: &Vec<Experiment>) {
    let mut fg = Figure::new();

    let time = (1u32..max_time).collect::<Vec<u32>>();

    {
        let mut graph = fg.axes2d();
        graph
        .set_size(0.95, 1.0)
        .set_title("Tea Temperature", &[])
        .set_x_label("Time (seconds)", &[])
        .set_y_label("Temperature (Celcius)", &[])
        .set_y_range(Fix(30.0), Fix(100.0));

        let mut index = 1;
        for experiment in experiments {
            graph.lines(time.iter(), experiment.result.iter(), &experiment.plot_options);
            index = index + 1;
        }
    }
	
	fg.show();
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
