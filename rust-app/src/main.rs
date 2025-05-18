use std::env;

pub mod config;
pub mod celeryinterface;



use crate::config::{APP_RUN_MODE, WORKER_RUN_MODE};
use celerylib::{Task, TaskResult, TaskSuccess};
use serde::{Serialize, Deserialize};
use image::{ImageBuffer, Rgb};
use rayon::prelude::*;
use std::time::Instant;
use std::path::Path;
use std::fs;
use celeryinterface::{push_task, register_task, run_worker};

fn main() {
    // Check environment variable
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "unknown".to_string());

    register_task::<mandlebrot, MandleBrot>();
    match run_mode.as_str() {
        APP_RUN_MODE => {
            fs::create_dir_all("output").unwrap();               

            for _ in 0..100 {
                push_task::<i32, CoolTask>(42);
            }

            register_task::<mandlebrot, MandleBrot>();

            push_task::<mandlebrot, MandleBrot>(mandlebrot{width: 10000, height: 10000, max_iterations: 10000, output_path: "output/mandelbrot1.png".to_string() });    
            // push_task::<mandlebrot, MandleBrot>(mandlebrot{width: 6000, height: 5000, max_iterations: 10000, output_path: "output/mandelbrot2.png".to_string() });
            // push_task::<mandlebrot, MandleBrot>(mandlebrot{width: 500, height: 500, max_iterations: 10000, output_path: "output/mandelbrot3.png".to_string() });
        }
        WORKER_RUN_MODE => run_worker(),
        _ => {
            eprintln!("Unknown mode. Set RUN_MODE to 'app' or 'worker'");
            std::process::exit(1);
        }
    }
}

#[derive(Serialize, Deserialize)]
struct mandlebrot {
    width: u32, 
    height: u32, 
    max_iterations: u32,
    output_path: String,
}

struct MandleBrot {}

impl Task<mandlebrot> for MandleBrot {
    fn get_task_name() -> String {
        return "mandle".to_string();
    }

    fn run(targs: mandlebrot) -> TaskResult {
    let width = targs.width;
    let height = targs.height;
    let max_iterations = targs.max_iterations;
    let output_path = targs.output_path;


            // Create directory structure if it doesn't exist
    if let Some(parent) = Path::new(&output_path).parent() {
        fs::create_dir_all(parent).unwrap();
    }
    
    let start = Instant::now();
    
    // Create a new image buffer
    let mut img = ImageBuffer::new(width, height);
    
    // Calculate the Mandelbrot set in parallel
    img.enumerate_pixels_mut().par_bridge().for_each(|(x, y, pixel)| {
        // Convert pixel coordinates to complex number
        let cx = 3.5 * (x as f64) / (width as f64) - 2.5;
        let cy = 3.5 * (y as f64) / (height as f64) - 1.75;
        
        let mut z_real = 0.0;
        let mut z_imag = 0.0;
        let mut iteration = 0;
        
        // Iterate until escape or max_iterations
        while z_real * z_real + z_imag * z_imag < 4.0 && iteration < max_iterations {
            let new_real = z_real * z_real - z_imag * z_imag + cx;
            let new_imag = 2.0 * z_real * z_imag + cy;
            z_real = new_real;
            z_imag = new_imag;
            iteration += 1;
        }
        
        // Color based on iteration count
        if iteration < max_iterations {
            *pixel = Rgb([
                (iteration % 8 * 32) as u8,
                (iteration % 16 * 16) as u8,
                (iteration % 32 * 8) as u8,
            ]);
        } else {
            *pixel = Rgb([0, 0, 0]);
        }
    });
    
    let duration = start.elapsed();
    println!("Generated Mandelbrot set in {:?}", duration);
    
    // Save the image to the specified path
    img.save(&output_path).unwrap();
    println!("Image saved to {}", &output_path);

        Ok(TaskSuccess{})
    }
}



struct HellaFib {}

impl Task<Vec<i32>> for HellaFib {
    fn get_task_name() -> String {
        return "hella_fib".to_string();
    }

    fn run(numbers: Vec<i32>) -> TaskResult {
        for number in numbers {
            let res: i32 = CoolTask::fib(number);
            println!("Fib value: {}", res);
        }

        return Ok(TaskSuccess {});
    }
}

struct CoolTask {}

impl Task<i32> for CoolTask {
    fn get_task_name() -> String {
        return "cool_task".to_string();
    }

    fn run(number: i32) -> TaskResult {
        let res: i32 = Self::fib(number);
        println!("Fib value: {}", res);
        return Ok(TaskSuccess {});
        // return Err(TaskError{uuid: Uuid::new_v4(), message: "meow".to_string()});
    }
}

impl CoolTask {
    fn fib(number: i32) -> i32 {
        if number == 0 {
            return 0;
        } else if number == 1 {
            return 1;
        }
        Self::fib(number - 1) + Self::fib(number - 2)
    }
}
