
use clap::{
    Parser,
    ValueEnum
};

#[derive(Debug, Parser)]
#[clap(name = "Kura", version, about = "Facial anonymizer tool")]
#[clap(propagate_version = true)]
pub struct KuraParser {
    #[arg(short='f', long, value_enum, default_value_t = FilterType::PixelBlur)]
    pub filter: FilterType,
    
    #[arg(short='m', long, default_value_t = String::from("./model/seeta_fd_frontal_v1.0.bin"))]
    pub model: String,

    #[arg(short='i', long, default_value_t=50)]
    pub intensity: u32,
    
    pub image: String,
    pub output: String,

}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum FilterType {
    Gaussian,
    PixelBlur,
    Pixelated
}

#[allow(clippy::from_over_into)]
impl Into<kura::KuraFilter> for FilterType {
    fn into(self) -> kura::KuraFilter {
        match self {
            FilterType::PixelBlur => kura::KuraFilter::PixelBlur,
            FilterType::Pixelated => kura::KuraFilter::PixelBlur,
            FilterType::Gaussian => kura::KuraFilter::GaussianBlur
        }
    }
}


