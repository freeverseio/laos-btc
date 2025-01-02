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

#[derive(Boilerplate)]
pub(crate) struct PreviewAudioHtml {
	pub(crate) inscription_id: InscriptionId,
}

#[derive(Boilerplate)]
pub(crate) struct PreviewCodeHtml {
	pub(crate) inscription_id: InscriptionId,
	pub(crate) language: media::Language,
}

#[derive(Boilerplate)]
pub(crate) struct PreviewFontHtml {
	pub(crate) inscription_id: InscriptionId,
}

#[derive(Boilerplate)]
pub(crate) struct PreviewImageHtml {
	pub(crate) image_rendering: ImageRendering,
	pub(crate) inscription_id: InscriptionId,
}

#[derive(Boilerplate)]
pub(crate) struct PreviewMarkdownHtml {
	pub(crate) inscription_id: InscriptionId,
}

#[derive(Boilerplate)]
pub(crate) struct PreviewModelHtml {
	pub(crate) inscription_id: InscriptionId,
}

#[derive(Boilerplate)]
pub(crate) struct PreviewPdfHtml {
	pub(crate) inscription_id: InscriptionId,
}

#[derive(Boilerplate)]
pub(crate) struct PreviewTextHtml {
	pub(crate) inscription_id: InscriptionId,
}

#[derive(Boilerplate)]
pub(crate) struct PreviewUnknownHtml;

#[derive(Boilerplate)]
pub(crate) struct PreviewVideoHtml {
	pub(crate) inscription_id: InscriptionId,
}
