
use cursive::reexports::time::format_description::modifier::Period;
use image::{ImageBuffer, Rgb};

use noise::{Perlin, Seedable, NoiseFn, Fbm, utils::{PlaneMapBuilder, NoiseMapBuilder}, OpenSimplex};
use serde::{Serialize, Deserialize};
/* 
fn main(){

    let fbm = Fbm::new(); 

    let val = fbm.get([42.0, 37.0, 2.0]);
    let perlin = Perlin::new();

}
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MapMeta{
    pub height: usize,
    pub width: usize
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Map{
    pub map: Vec<String>,
    pub meta: MapMeta
}

const IMAGE_PATH: &str = "./images";

pub const NOISE_SEED: u32 = 8675309; 



lazy_static! {
 static ref SEEDED_PERLIN_NOISE: Perlin =  Perlin::set_seed(Perlin::new(), NOISE_SEED);
 static ref SEEDED_OPEN_SIMPLEX: OpenSimplex = OpenSimplex::set_seed(OpenSimplex::new(), NOISE_SEED);
}  
    #[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

pub fn test_output(){
    println!("module working")
}

pub fn generate_2d_array_from_noise() -> Vec<Vec<f64>>{
    let mut matrix = vec![vec![0.0; 640]; 640];

    let mut x = 0.0;
    let mut x_int = 0;
    while x < 64.0{
        let mut y = 0.1;
        let mut y_int = 0;
        while y < 64.0{
            
            matrix[x_int][y_int] = SEEDED_OPEN_SIMPLEX.get([x,y]);

            y = y + 1.1;
            y_int = y_int + 1;
        }
        x = x + 1.1;
        x_int = x_int + 1;
    }

    return matrix;
}

pub fn output_noise(){
    let noisevec = generate_2d_array_from_noise();

    println!("-----------------------------");
    println!("printing vectors");
    println!("");

    let mut x = 0;
    while x < 640{
        
        let vec = &noisevec[x];

        println!("{:?}", vec);
        
        x = x + 1;
    }
    println!("");
    println!("-----------------------------");
}

pub fn createPlaneMapImage(){

    
    //let fbm = Fbm::set_seed(Fbm::new(), PERLIN_SEED);

    let noiseClone = SEEDED_OPEN_SIMPLEX.clone();

    PlaneMapBuilder::new(&noiseClone)
    .set_size(1000, 1000)
    .set_x_bounds(-5.0, 5.0)
    .set_y_bounds(-5.0, 5.0)
    .build()
    .write_to_file("openSimplexMap.png");
    
}

/*
pub fn create_image_from_perlin(){
    // a default (black) image containing Rgb values
    let width = 10;
    let height = 10;
    let mut image = ImageBuffer::<Rgb<u8>>::new(width, height);

}
*/