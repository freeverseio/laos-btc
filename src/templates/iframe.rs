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

use super::*;

pub(crate) struct Iframe {
	inscription_id: InscriptionId,
	thumbnail: bool,
}

impl Iframe {
	pub(crate) fn thumbnail(inscription_id: InscriptionId) -> Trusted<Self> {
		Trusted(Self { inscription_id, thumbnail: true })
	}

	pub(crate) fn main(inscription_id: InscriptionId) -> Trusted<Self> {
		Trusted(Self { inscription_id, thumbnail: false })
	}
}

impl Display for Iframe {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if self.thumbnail {
			write!(
				f,
				"<a href=/inscription/{}>\
          <iframe sandbox=allow-scripts scrolling=no loading=lazy src=/preview/{}>\
          </iframe>\
        </a>",
				self.inscription_id, self.inscription_id,
			)
		} else {
			write!(
				f,
				"<iframe sandbox=allow-scripts loading=lazy src=/preview/{}></iframe>",
				self.inscription_id,
			)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn thumbnail() {
		assert_regex_match!(
      Iframe::thumbnail(inscription_id(1))
      .0.to_string(),
      "<a href=/inscription/1{64}i1><iframe sandbox=allow-scripts scrolling=no loading=lazy src=/preview/1{64}i1></iframe></a>",
    );
	}

	#[test]
	fn main() {
		assert_regex_match!(
			Iframe::main(inscription_id(1)).0.to_string(),
			"<iframe sandbox=allow-scripts loading=lazy src=/preview/1{64}i1></iframe>",
		);
	}
}
