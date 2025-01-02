// Copyright 2023-2024 Freeverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

#[macro_export]
macro_rules! define_table {
	($name:ident, $key:ty, $value:ty) => {
		const $name: TableDefinition<$key, $value> = TableDefinition::new(stringify!($name));
	};
}

#[macro_export]
macro_rules! define_multimap_table {
	($name:ident, $key:ty, $value:ty) => {
		const $name: MultimapTableDefinition<$key, $value> =
			MultimapTableDefinition::new(stringify!($name));
	};
}

#[macro_export]
macro_rules! tprintln {
  ($($arg:tt)*) => {
    if cfg!(test) {
      eprint!("==> ");
      eprintln!($($arg)*);
    }
  };
}

#[macro_export]
macro_rules! assert_regex_match {
	($value:expr, $pattern:expr $(,)?) => {
		let regex = Regex::new(&format!("^(?s){}$", $pattern)).unwrap();
		let string = $value.to_string();

		if !regex.is_match(string.as_ref()) {
			eprintln!("Regex did not match:");
			pretty_assert_eq!(regex.as_str(), string);
		}
	};
}

#[macro_export]
macro_rules! assert_matches {
  ($expression:expr, $( $pattern:pat_param )|+ $( if $guard:expr )? $(,)?) => {
    match $expression {
      $( $pattern )|+ $( if $guard )? => {}
      left => panic!(
        "assertion failed: (left ~= right)\n  left: `{:?}`\n right: `{}`",
        left,
        stringify!($($pattern)|+ $(if $guard)?)
      ),
    }
  }
}
