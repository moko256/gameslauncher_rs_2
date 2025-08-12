use std::{
    borrow::Cow,
    fs::File,
    sync::{Arc, LazyLock},
};

use eframe::egui;
use memmap2::{Mmap, MmapOptions};

static GLOBAL_SYSTEM_FONTS: LazyLock<SystemFonts> = LazyLock::new(|| SystemFonts::load());

pub fn set_system_fonts(ctx: &egui::Context) {
    ctx.set_fonts(GLOBAL_SYSTEM_FONTS.convert());
}

#[derive(Debug)]
struct SystemFonts {
    fonts: Vec<SystemFont>,
}

impl SystemFonts {
    fn load() -> Self {
        let keys = [SystemFontKey {
            name: "Yu Gothic UI",
            font_families: vec![egui::FontFamily::Proportional, egui::FontFamily::Monospace],
            font_properties: Some(font_kit::properties::Properties {
                style: font_kit::properties::Style::Oblique, // workaround: Almost all Yu Gothic UI variants are marked as oblique.
                ..Default::default()
            }),
        }];

        let mut fonts: Vec<SystemFont> = Vec::new();

        for key in keys {
            if let Some(font) = SystemFont::load(key) {
                fonts.push(font);
            }
        }

        Self { fonts }
    }

    fn convert(&'static self) -> egui::FontDefinitions {
        let mut definitions: egui::FontDefinitions = egui::FontDefinitions::empty();

        for font in &self.fonts {
            font.add_to(&mut definitions);
        }

        definitions
    }
}

#[derive(Debug)]
struct SystemFontKey {
    name: &'static str,
    font_families: Vec<egui::FontFamily>,
    font_properties: Option<font_kit::properties::Properties>,
}

#[derive(Debug)]
struct SystemFont {
    key: SystemFontKey,
    handle: SystemFontHandle,
}

impl SystemFont {
    fn load(key: SystemFontKey) -> Option<Self> {
        let family_name = font_kit::family_name::FamilyName::Title(key.name.to_string());
        let properties = key.font_properties.unwrap_or(Default::default());

        let font = font_kit::source::SystemSource::new()
            .select_best_match(&[family_name], &properties)
            .ok()?;

        let handle = SystemFontHandle::load(font)?;

        Some(Self { key, handle })
    }

    fn add_to(&'static self, definitions: &mut egui::FontDefinitions) {
        let font_data = self.handle.convert();

        definitions
            .font_data
            .insert(self.key.name.to_string(), Arc::new(font_data));

        for font_family in &self.key.font_families {
            if definitions.families.get(&font_family).is_none() {
                definitions.families.insert(font_family.clone(), Vec::new());
            }

            definitions
                .families
                .get_mut(&font_family)
                .unwrap()
                .push(self.key.name.to_string());
        }
    }
}

#[derive(Debug)]
enum SystemFontHandle {
    Memory {
        bytes: Arc<Vec<u8>>,
        font_index: u32,
    },
    Path {
        mmap: Mmap,
        font_index: u32,
    },
}

impl SystemFontHandle {
    #[allow(unsafe_code)]
    fn load(handle: font_kit::handle::Handle) -> Option<Self> {
        let handle = match handle {
            font_kit::handle::Handle::Memory { bytes, font_index } => {
                Self::Memory { bytes, font_index }
            }
            font_kit::handle::Handle::Path { path, font_index } => {
                let file = File::open(path).ok()?;
                let mmap = unsafe { MmapOptions::new().map(&file).ok()? };

                Self::Path { mmap, font_index }
            }
        };

        Some(handle)
    }

    fn convert(&'static self) -> egui::FontData {
        match self {
            Self::Memory { bytes, font_index } => egui::FontData {
                font: Cow::Borrowed(bytes),
                index: *font_index,
                tweak: Default::default(),
            },
            Self::Path { mmap, font_index } => egui::FontData {
                font: Cow::Borrowed(mmap.as_ref()),
                index: *font_index,
                tweak: Default::default(),
            },
        }
    }
}
