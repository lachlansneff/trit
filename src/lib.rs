//! # The trit crate
//! A crate that provides a trinary type

//! ## Usage
//! ```
//! // Creates a type that is both true and false
//! let both = Trit::Both;
//! // both.is_true() == true
//! // both.is_false() == true
//! // both.only_true() == false
//! // both.only_false() == false
//! ```

#[cfg(test)]
mod tests {
	use Trit;

    #[test]
    fn can_be_both() {
    	assert!(Trit::Both.is_true());
    	assert!(Trit::Both.is_false());
    }

    #[test]
    fn can_be_only() {
    	assert!(Trit::True.only_true());
    	assert!(Trit::False.only_false());
    }
}

/// Used to denote a trinary value (True, False, or Both)
#[derive(PartialEq)]
pub enum Trit {
	True,
	False,
	Both
}

impl Trit {
	/// Returns true if self is True or Both
	pub fn is_true(self) -> bool {
		self == Trit::True || self == Trit::Both
	}

	/// Returns true if self is False or Both
	pub fn is_false(self) -> bool {
		self == Trit::False || self == Trit::Both
	}

	/// Tests if self is True
	pub fn only_true(self) -> bool {
		self == Trit::True
	}

	/// Tests if self is False
	pub fn only_false(self) -> bool {
		self == Trit::False
	}

	/// Tests if self is both
	pub fn is_both(self) -> bool {
		self == Trit::Both
	}
}