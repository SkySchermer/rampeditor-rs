////////////////////////////////////////////////////////////////////////////////
//!
//! Defines structured Palette objects for storing and generating colors.
//!
//! The palette acts as a tree-like structure that acts as a collection of 
//! 'Slots' into which color elements are placed. Rgb elements will then 
//! lazily generate a color when queried. This allows for the construction of 
//! dynamic palette structures that can generate related colors based off of a 
//! small subset of 'control' colors.
//!
//! More practically, `Slot`s are identified by `Address`, and each slot 
//! contains a single `ColorElement`, which will generate a `Rgb` when either
//! the Slot's or ColorElement's `get_color` method is called. ColorElements are
//! categorized by 'order', which denotes the number of dependencies needed to
//! generate a color. For example, a second order element is dependent upon two
//! other colors, while a zeroth order color element is simply a color. These
//! dependencies are expressed through references to other slots in the palette.
//!
////////////////////////////////////////////////////////////////////////////////

#[warn(missing_docs)]
pub mod error;

#[warn(missing_docs)]
pub mod element;

#[warn(missing_docs)]
pub mod data;

#[warn(missing_docs)]
pub mod operation;

#[warn(missing_docs)]
pub mod format;

pub use palette::format::{Palette, PaletteExtensions, BasicPalette, ZplPalette};
pub use palette::error::{Error, Result};
pub use palette::operation::{
	InsertColor,
	RemoveElement,
	CopyColor,
	InsertWatcher,
	SequenceOperation,
	RepeatOperation,
	InsertRamp,
};
