use ::image::{ImageBuffer, RgbImage, DynamicImage};
use std::f64::consts::{E, PI};

pub fn gaussian_blur(img: DynamicImage, filter_dim: u8){
    let rgbimg: RgbImage = img.into_rgb();
    let mut filter = vec!();
    let sigma: f64 = 5.0; 
    let mut sum: f64 = 0.0;
    for i in 0..filter_dim {
        let mut filter_row = vec!();
        for j in 0..filter_dim {
            let val = E.powf(-(i as f64 * i as f64 + j as f64 * j as f64)/(2.0 * sigma * sigma)) / (2.0 * PI * sigma * sigma);
            filter_row.push(val);
            sum += val;
        }
        filter.push(filter_row);
    }
//    println!("{:?}", sum);
    let mut norm = 0.0;
    for i in 0..filter_dim {
        for j in 0..filter_dim {
            filter[i as usize][j as usize] /= sum;
            norm += filter[i as usize][j as usize];
        }
    }
//    println!("{:?}", filter);
//    println!("NORM TOTAL: {:?}", norm);
    let mut blurred_img: RgbImage = ImageBuffer::new(rgbimg.dimensions().0, rgbimg.dimensions().1);
    println!("Dim: {} {}", rgbimg.dimensions().0, rgbimg.dimensions().1);
    
    //For each row
    for i in ((filter_dim as f64 / 2.0) as u32)..(rgbimg.dimensions().0 - (filter_dim as f64 / 2.0) as u32) {
        //For each col
        for j in ((filter_dim as f64 / 2.0) as u32)..(rgbimg.dimensions().1 - (filter_dim as f64 / 2.0) as u32) {
            let mut val = vec![0 as u32; 3];

            //For filter dimension in row
            for k in (-(filter_dim as i32) / 2)..(filter_dim as i32 / 2) {

                //For filter dimension in each col
                for l in (-(filter_dim as i32) / 2)..(filter_dim as i32 / 2) {
                    let pixel = rgbimg[((i as i32 + k) as u32, (j as i32 + l) as u32)].0;
                    //For each pixel channel (RGB)
                    for m in 0..=2 {
                        val[m as usize] += (pixel[m as usize] as f64 * filter[(k + (filter_dim as i32) / 2) as usize][(l + (filter_dim as i32) / 2) as usize]) as u32;
                    }
                }   
            }
            let mut temp_val: Vec<u8> = vec!();
            for i in val.iter() {
                temp_val.push(*i as u8);
            }
            let p = image::Pixel::from_channels(temp_val[0], temp_val[1], temp_val[2], 1);
            blurred_img.put_pixel(i, j, p);
        }
    }
    blurred_img.save("blurred.png").unwrap();
}