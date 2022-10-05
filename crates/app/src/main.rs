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

mod args;
use clap::Parser;
use kura::Face;

fn main() {

    let a: args::KuraParser = args::KuraParser::parse();
    let filter: kura::KuraFilter = a.filter.into();
    let image_path: String = a.image;
    let output_path: String = a.output;
    let intensity: u32 = a.intensity;
    let model_path: String = a.model;

    let mut f = Face::new(&model_path, &image_path);
    let mut faces = f.get_faces();
    faces.save(intensity, &output_path, &filter);
 
}
