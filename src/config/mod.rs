use serde::Serialize;
use serde::Deserialize;
use enumerations::*;

use self::filter::FilterVec;
use self::compression::CompressionVec;
use self::adjustment::AdjustmentVec;
use std::path::PathBuf;

pub mod interface;
pub mod enumerations;

pub mod filter;
pub mod compression;
pub mod adjustment;

pub type SequenceVec = Vec<Sequence>;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Config {
	input: Input,
	output: Output,
	progress_bar: Option<ProgressBarConf>,
	image: ImageSection,
	sequence: Option<SequenceVec>,
	execute: Parameter,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Input {
	receive: InputType,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Output {
	save: OutputType,
	path: PathBuf,
	naming: FileName,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ProgressBarConf {
	template: String,
	chars: String,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ImageSection {
	format: SaveFormat,
	filter: Option<FilterVec>,
	adjustment: Option<AdjustmentVec>,
	compression: Option<CompressionVec>,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Sequence {
	pub id: String,
	elements: Vec<Parameter>,
}

