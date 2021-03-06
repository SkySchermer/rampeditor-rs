// The MIT License (MIT)
// 
// Copyright (c) 2017 Skylor R. Schermer
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
//! The `undo` module provides an `Undo` operation on a `Palette`.
//!
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use address::Address;
use data::Data;
use expression::Expression;
use operation::{
	HistoryEntry,
	OperationInfo,
	PaletteOperation,
};
use result::Result;

// Standard imports.
use std::collections::HashMap;
use std::mem;


////////////////////////////////////////////////////////////////////////////////
// Undo
////////////////////////////////////////////////////////////////////////////////
/// Restores a saved set of elements in the palette. 
/// 
/// The Undo operation stores `Expression`s using a `HashMap`, which means it
/// can only store one entry for each address. A create operation will have
/// priority over any other change recorded. In otherwords, if there is an
/// "address: None" entry in the `Undo`,  nothing will overwrite it. This
/// ensures  that the element at that address will be deleted if the `Undo`
/// operation is applied later.
#[derive(Debug)]
pub struct Undo {
	/// The operation being undone.
	undoing: OperationInfo,

	/// The `Expression`s to restore when applying the Undo.
	saved: HashMap<Address, Option<Expression>>,
}


impl Undo {
	/// Creates a new Undo operation.
	#[inline]
	fn new() -> Undo {
		Undo {
			undoing: OperationInfo {
				name: "Undo",
				details: None,
			},
			saved: Default::default(),
		}
	}

	/// Creates a new Undo operation for the given operation.
	#[inline]
	pub fn new_for<O>(operation: &O) -> Undo 
		where O: PaletteOperation
	{
		Undo {
			undoing: operation.info(),
			saved: Default::default(),
		}
	}

	/// Records an element change to be replayed by the Undo operation.
	#[inline]
	pub fn record(&mut self, address: Address, element: Option<Expression>) {
		if self.saved.get(&address).map_or(true, |e| !e.is_none()) {
			self.saved.insert(address, element);
		}
	}

}


impl PaletteOperation for Undo {
	fn info(&self) -> OperationInfo {
		OperationInfo {
			name: "Undo",
			details: Some(format!("{:?}", self))
		}
	}

	fn apply(&mut self, data: &mut Data) -> Result<HistoryEntry> {
		let mut redo = Undo::new();

		let saved = mem::replace(&mut self.saved, HashMap::new());

		for (address, item) in saved {
			match (item.is_some(), data.cell(address).is_some()) {

				(true, true) => { // The cell was modified.
					let elem = item.unwrap();
					let cell = data.cell(address).unwrap();
					let cur = mem::replace(&mut *cell.borrow_mut(), elem);
					redo.record(address, Some(cur));
					continue;
				},

				(true, false) => { // The cell was deleted.
					let elem = item.unwrap();
					let cell = data.create_cell(address).unwrap();
					mem::replace(&mut *cell.borrow_mut(), elem);
					redo.record(address, None);
					continue;
				},

				(false, true) => { // The cell was added.
					let cur = data.remove_cell(address)?;
					redo.record(address, Some(cur));
					continue;
				},

				_ => panic!("null entry in Undo operation")
			}
		}

		Ok(HistoryEntry {
			info: self.info(),
			undo: Box::new(redo),
		})
	}
}
