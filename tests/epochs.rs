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
use ord::subcommand::epochs::Output;
use ordinals::Sat;

#[test]
fn empty() {
	assert_eq!(
		CommandBuilder::new("epochs").run_and_deserialize_output::<Output>(),
		Output {
			starting_sats: vec![
				Sat(0),
				Sat(1050000000000000),
				Sat(1575000000000000),
				Sat(1837500000000000),
				Sat(1968750000000000),
				Sat(2034375000000000),
				Sat(2067187500000000),
				Sat(2083593750000000),
				Sat(2091796875000000),
				Sat(2095898437500000),
				Sat(2097949218750000),
				Sat(2098974609270000),
				Sat(2099487304530000),
				Sat(2099743652160000),
				Sat(2099871825870000),
				Sat(2099935912620000),
				Sat(2099967955890000),
				Sat(2099983977420000),
				Sat(2099991988080000),
				Sat(2099995993410000),
				Sat(2099997995970000),
				Sat(2099998997250000),
				Sat(2099999497890000),
				Sat(2099999748210000),
				Sat(2099999873370000),
				Sat(2099999935950000),
				Sat(2099999967240000),
				Sat(2099999982780000),
				Sat(2099999990550000),
				Sat(2099999994330000),
				Sat(2099999996220000),
				Sat(2099999997060000),
				Sat(2099999997480000),
				Sat(2099999997690000)
			]
		}
	);
}
