/*
 * Copyright (c) 2022 riyuzenn <riyuzenn@gmail.com>
 * See the license file for more info
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use anyhow::Result;
use image::imageops::FilterType;
use image::{DynamicImage, GrayImage};
use rustface::{Detector, FaceInfo, ImageData};
use std::{path::Path, process};
use crate::KuraFilter;


#[derive(Debug)]
pub struct KuraFace {
    faces: Vec<FaceInfo>,
    image_data: DynamicImage,
}
impl KuraFace {
    pub fn new(
        face_info: Vec<FaceInfo>, 
        _image_data: DynamicImage
    ) -> KuraFace {

        KuraFace {
            faces: face_info,
            image_data: _image_data,
        }
    }

    pub fn save(
        &mut self, 
        intensity: u32, 
        output_file: &str,
        filter: &KuraFilter
    ) {

        for face in self.faces.clone() {
            let bbox = face.bbox();   
            let img = self.edit_image(
                intensity,
                bbox.x().unsigned_abs(),
                bbox.y().unsigned_abs(),
                bbox.width(),
                bbox.height(),
                filter 
            );
            img.save(output_file).unwrap();
        }
    }
    
    /// Edit the image, crop the x and y coordinates of the face
    /// accesible by the face.bbox(). It works by croping the face
    /// and adding a filter on it then merging the cropped face to
    /// the original image data.
    fn edit_image(
        &mut self,
        resize_scale: u32,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        filter: &KuraFilter
    ) -> image::ImageBuffer<
        image::Rgba<u8>, 
        Vec<u8>> {

        // crop & resize image as well as apply the filter on it
        let crop_img = self.image_data.crop_imm(x, y, w, h);
        let crop_scaled_w = crop_img.width() / resize_scale;
        let crop_scaled_h = crop_img.height() / resize_scale; 

        let img = match filter {
            KuraFilter::PixelBlur => self.apply_pixel_blur(
                crop_img.clone(), 
                crop_scaled_w, 
                crop_scaled_h
            ).to_rgba8(),

            KuraFilter::GaussianBlur => self.apply_gaussian_blur(
                crop_img.clone(), 
                crop_scaled_w, 
                crop_scaled_h
            ).to_rgba8(),

            KuraFilter::Pixelated => self.apply_pixelated(
                crop_img.clone(), 
                crop_scaled_w, 
                crop_scaled_h
            ).to_rgba8(),

        };

        let mut base_image = self.image_data
            .clone().
            into_rgba8();
        
        for crop_x in 0..crop_img.width() {
            for crop_y in 0..crop_img.height() {
                let pixel = img.get_pixel(crop_x, crop_y);
                base_image.put_pixel(x + crop_x, y + crop_y, *pixel);
            }
        }
        base_image
    }

    fn apply_pixel_blur(
        &self, 
        img: DynamicImage, 
        w: u32, 
        h: u32
    ) -> DynamicImage {
    
        img.resize_exact(w, h, FilterType::Gaussian).resize_exact(
            img.width(),
            img.height(),
            FilterType::Nearest,
        )
    }

    fn apply_gaussian_blur(
        &self, 
        img: DynamicImage, 
        w: u32, 
        h: u32
    ) -> DynamicImage {
    
         img.resize_exact(w, h, FilterType::Gaussian).resize_exact(
            img.width(),
            img.height(),
            FilterType::Gaussian,
        )
    }

    fn apply_pixelated(
        &self, 
        img: DynamicImage, 
        w: u32, 
        h: u32
    ) -> DynamicImage {
    
         img.resize_exact(w, h, FilterType::Nearest).resize_exact(
            img.width(),
            img.height(),
            FilterType::Nearest,
        )    
    }
}

#[derive(Debug)]
pub struct Face {
    model_path: String,
    image_path: String,
    min_face_size: u32,
    score_thresh: f64,
    pyramid_scale_factor: f32,
    slide_window_step_x: u32,
    slide_window_step_y: u32
}

impl Face {
    /// Initialize face constructor
    pub fn new(
        _model_path: &str, 
        _image_path: &str
    ) -> Face {
    
        Face {
            model_path: _model_path.to_string(),
            image_path: _image_path.to_string(),
            min_face_size: 20,
            score_thresh: 2.0,
            pyramid_scale_factor: 0.8,
            slide_window_step_x: 4,
            slide_window_step_y: 4
        }
    }

    pub fn set_window_step_x(&mut self, v: u32) {
        self.slide_window_step_x = v;
    }

    pub fn set_window_step_y(&mut self, v: u32) {
        self.slide_window_step_y = v;
    }
    
    pub fn set_min_face_size(&mut self, v: u32) {
        self.min_face_size = v;
    }

    pub fn set_score_thresh(&mut self, v: f64) {
        self.score_thresh = v;
    }

    pub fn set_pyramid_scale_factor(&mut self, v: f32) {
        self.pyramid_scale_factor = v;
    }

    pub fn set_image(&mut self, img_path: &str) {
        self.image_path = img_path.to_string();
    }

    pub fn get_faces(&mut self) -> KuraFace {
        let mut detector = self.create_detector().unwrap();
        let image_path: DynamicImage = match image::open(
            Path::new(&self.image_path)
        ) {
            Ok(image_path) => image_path,
            Err(message) => {
                println!("Failed to read image: {}", message);
                process::exit(1);
            }
        };
        let faces = self.detect_faces(&mut *detector, &image_path.to_luma8());
        KuraFace::new(faces, image_path)
    }

    fn detect_faces(&self, detector: &mut dyn Detector, gray: &GrayImage) -> Vec<FaceInfo> {
        let (width, height) = gray.dimensions();
        let image = ImageData::new(gray, width, height);
        
        detector.detect(&image)
    }
    
    /// Create and initialize a rustface::Detector
    /// This function do not accept any arguments all paramaters
    /// to used were based of the struct field.
    fn create_detector(&mut self) -> Result<Box<dyn rustface::Detector>> {
        let mut detector = rustface::create_detector(&self.model_path).unwrap();
        detector.set_min_face_size(self.min_face_size);
        detector.set_score_thresh(self.score_thresh);
        detector.set_pyramid_scale_factor(self.pyramid_scale_factor);
        detector.set_slide_window_step(4, 4);
        Ok(detector)
    }
}
