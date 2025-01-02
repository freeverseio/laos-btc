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
use ord::subcommand::traits::Output;
use ordinals::Rarity;

#[test]
fn traits_command_prints_sat_traits() {
	assert_eq!(
		CommandBuilder::new("traits 0").run_and_deserialize_output::<Output>(),
		Output {
			number: 0,
			decimal: "0.0".into(),
			degree: "0°0′0″0‴".into(),
			name: "nvtdijuwxlp".into(),
			height: 0,
			cycle: 0,
			epoch: 0,
			period: 0,
			offset: 0,
			rarity: Rarity::Mythic,
		}
	);
}
#[test]
fn traits_command_for_last_sat() {
	assert_eq!(
		CommandBuilder::new("traits 2099999997689999").run_and_deserialize_output::<Output>(),
		Output {
			number: 2099999997689999,
			decimal: "6929999.0".into(),
			degree: "5°209999′1007″0‴".into(),
			name: "a".into(),
			height: 6929999,
			cycle: 5,
			epoch: 32,
			period: 3437,
			offset: 0,
			rarity: Rarity::Uncommon,
		}
	);
}
