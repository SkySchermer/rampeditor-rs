// The MIT License (MIT)
// 
// Copyright (c) 2016 Skylor R. Schermer
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in 
// all copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
////////////////////////////////////////////////////////////////////////////////
//!
//! Defines ramp creation operations.
//!
////////////////////////////////////////////////////////////////////////////////
use color::Color;

use super::{
	PaletteOperation, 
	HistoryEntry, 
	OperationInfo, 
	set_target, 
	get_source
};
use palette::Result;
use palette::data::PaletteOperationData;
use palette::element::{ColorElement, Mixer};
use palette::operation::Undo;
use address::Address;




////////////////////////////////////////////////////////////////////////////////
// Ramp
////////////////////////////////////////////////////////////////////////////////
/// A second-order `Mixer` which will return the linear interpoloation of its
/// source colors.
#[derive(Debug, Clone, Copy)]
pub struct Ramp(f32);


impl Mixer for Ramp {
	fn mix(&self, colors: Vec<Color>) -> Color {
		Color::rgb_lerp(colors[0], colors[1], self.0)
	}
}


////////////////////////////////////////////////////////////////////////////////
// InsertRamp
////////////////////////////////////////////////////////////////////////////////
/// Creates a linear RGB color ramp using second-order elements in the palette.
/// 
/// # Example
///
/// ```rust
/// use rampeditor::*;
/// 
/// let mut pal = Palette::new("Example", Format::Default, true);
///
/// pal.apply(Box::new(InsertColor::new(Color::new(0, 0, 0)))).unwrap();
/// pal.apply(Box::new(InsertColor::new(Color::new(150, 100, 50)))).unwrap();
/// pal.apply(Box::new(InsertRamp::new(
/// 	Address::new(0, 0, 0),
/// 	Address::new(0, 0, 1),
/// 	5
/// ))).unwrap();
///
/// assert_eq!(pal.get_color(Address::new(0, 0, 4)), Some(Color::new(75, 50, 25)));
/// assert_eq!(pal.len(), 7);
/// 
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct InsertRamp {
	/// The location to start placing the ramp elements.
	location: Option<Address>,
	/// The address of the starting color of the ramp.
	from: Address,
	/// The address of the ending color of the ramp.
	to: Address,
	/// The number of elements to create.
	count: usize,
	/// Whether to overwrite existing elements when generating new ones.
	overwrite: bool,
	/// Whether to generate placeholder slots when an invalid reference is given.
	make_sources: bool,
}


impl InsertRamp {
	/// Creates a new InsertRamp operation.
	#[inline]
	pub fn new(from: Address, to: Address, count: usize) -> InsertRamp {
		InsertRamp {
			location: None,
			from: from,
			to: to,
			count: count,
			overwrite: false,
			make_sources: false,
		}
	}

	/// Sets the location to start placing elements for the operation.
	pub fn located_at(mut self, location: Address) -> InsertRamp {
		self.location = Some(location);
		self
	}

	/// Configures the operation to overwrite existing elements as it generates
	/// new elements.
	pub fn overwrite(mut self, overwrite: bool) -> InsertRamp {
		self.overwrite = overwrite;
		self
	}

	/// Configures the operation to generate placeholder colors instead of 
	/// producing an error when empty addresses are provided. 
	pub fn make_sources(
		mut self, 
		make_sources: bool) 
		-> InsertRamp 
	{
		self.make_sources = make_sources;
		self
	}
}


impl PaletteOperation for InsertRamp {
	fn get_info(&self) -> OperationInfo {
		OperationInfo {
			name: "Insert Ramp",
			details: Some(format!("{:?}", self))
		}
	}

	fn apply(&mut self, data: &mut PaletteOperationData) -> Result<HistoryEntry> {
		
		// Get starting address.
		let starting_address = if let Some(address) = self.location {
			address
		} else {
			data.first_free_address_after(Default::default())?
		};

		// Get target addresses.
		let targets = data.find_targets(
			self.count, 
			starting_address,
			self.overwrite,
			Some(vec![self.from, self.to]) // Exclude the source locations.
		)?;

		// Get source slots.
		let mut undo = Undo::new_for(self);
		let make = self.make_sources;
		let src_from = get_source(data, self.from, make, &mut undo)?;
		let src_to = get_source(data, self.to, make, &mut undo)?;
		
		// Generate ramp.
		for (i, &address) in targets.iter().enumerate() {
			let am = (1.0 / (self.count + 1) as f32) * (i + 1) as f32;

			let new_element = ColorElement::Mixed {
				mixer: Box::new(Ramp(am)),
				sources: vec![src_from.clone(), src_to.clone()]
			};

			set_target(data, address, new_element, &mut undo)?;
		}

		Ok(HistoryEntry {
			info: self.get_info(),
			undo: Box::new(undo)
		})
	}
}

