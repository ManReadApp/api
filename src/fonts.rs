use egui::FontFamily;
use std::fmt::Display;

#[derive(Clone)]
pub enum Fonts {
    #[cfg(feature = "ccwildwords")]
    CcWildWords,
    #[cfg(feature = "kalam")]
    Kalam,
    #[cfg(feature = "komikajam")]
    KomikaJam,
    #[cfg(feature = "komikaslim")]
    KomikaSlim,
    #[cfg(feature = "vtclettererpro")]
    VtcLettererPro,
    #[cfg(feature = "bangersregular")]
    BangersRegular,
    #[cfg(feature = "animeace")]
    AnimeAce,
    #[cfg(feature = "animeace3")]
    AnimeAce3,
    #[cfg(feature = "comicshanns2")]
    ComicShanns2,
    #[cfg(feature = "jpn_font")]
    NotoSansJP,
    #[cfg(feature = "korean_font")]
    NotoSansKR,
    #[cfg(feature = "chinese_font")]
    NotoSansTC,
}

impl Display for Fonts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            #[cfg(feature = "ccwildwords")]
            Fonts::CcWildWords => "CCWildWords".to_string(),
            #[cfg(feature = "kalam")]
            Fonts::Kalam => "Kalam".to_string(),
            #[cfg(feature = "komikajam")]
            Fonts::KomikaJam => "KomikaJam".to_string(),
            #[cfg(feature = "komikaslim")]
            Fonts::KomikaSlim => "KomikaSlim".to_string(),
            #[cfg(feature = "vtclettererpro")]
            Fonts::VtcLettererPro => "VtcLettererPro".to_string(),
            #[cfg(feature = "bangersregular")]
            Fonts::BangersRegular => "BangersRegular".to_string(),
            #[cfg(feature = "animeace")]
            Fonts::AnimeAce => "AnimeAce".to_string(),
            #[cfg(feature = "animeace3")]
            Fonts::AnimeAce3 => "AnimeAce3".to_string(),
            #[cfg(feature = "comicshanns2")]
            Fonts::ComicShanns2 => "ComicShanns2".to_string(),
            #[cfg(feature = "jpn_font")]
            Fonts::NotoSansJP => "NotoSansJP".to_string(),
            #[cfg(feature = "korean_font")]
            Fonts::NotoSansKR => "NotoSansKR".to_string(),
            #[cfg(feature = "chinese_font")]
            Fonts::NotoSansTC => "NotoSansTC".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl Fonts {
    fn get_font_family(&self) -> FontFamily {
        match self {
            #[cfg(feature = "ccwildwords")]
            Fonts::CcWildWords => FontFamily::Name("CCWildWords".into()),
            #[cfg(feature = "kalam")]
            Fonts::Kalam => FontFamily::Name("Kalam".into()),
            #[cfg(feature = "komikajam")]
            Fonts::KomikaJam => FontFamily::Name("KomikaJam".into()),
            #[cfg(feature = "komikaslim")]
            Fonts::KomikaSlim => FontFamily::Name("KomikaSlim".into()),
            #[cfg(feature = "vtclettererpro")]
            Fonts::VtcLettererPro => FontFamily::Name("VtcLettererPro".into()),
            #[cfg(feature = "bangersregular")]
            Fonts::BangersRegular => FontFamily::Name("BangersRegular".into()),
            #[cfg(feature = "animeace")]
            Fonts::AnimeAce => FontFamily::Name("AnimeAce".into()),
            #[cfg(feature = "animeace3")]
            Fonts::AnimeAce3 => FontFamily::Name("AnimeAce3".into()),
            #[cfg(feature = "comicshanns2")]
            Fonts::ComicShanns2 => FontFamily::Name("ComicShanns2".into()),
            #[cfg(feature = "jpn_font")]
            Fonts::NotoSansJP => FontFamily::Proportional,
            #[cfg(feature = "korean_font")]
            Fonts::NotoSansKR => FontFamily::Proportional,
            #[cfg(feature = "chinese_font")]
            Fonts::NotoSansTC => FontFamily::Proportional,
        }
    }
    //TODO: add variant to read from os, web
    fn get_bytes(&self) -> &'static [u8] {
        match self {
            #[cfg(feature = "ccwildwords")]
            Fonts::CcWildWords => include_bytes!("assets/fonts/CC Wild Words Roman.ttf"),
            #[cfg(feature = "kalam")]
            Fonts::Kalam => include_bytes!("assets/fonts/Kalam-Regular.ttf"),
            #[cfg(feature = "komikajam")]
            Fonts::KomikaJam => include_bytes!("assets/fonts/komika-jam_[allfont.net].ttf"),
            #[cfg(feature = "komikaslim")]
            Fonts::KomikaSlim => include_bytes!("assets/fonts/KOMIKASL.ttf"),
            #[cfg(feature = "vtclettererpro")]
            Fonts::VtcLettererPro => include_bytes!("assets/fonts/VTC Letterer Pro Regular.ttf"),
            #[cfg(feature = "bangersregular")]
            Fonts::BangersRegular => include_bytes!("assets/fonts/Bangers-Regular.ttf"),
            #[cfg(feature = "animeace")]
            Fonts::AnimeAce => include_bytes!("assets/fonts/anime_ace.ttf"),
            #[cfg(feature = "animeace3")]
            Fonts::AnimeAce3 => include_bytes!("assets/fonts/anime_ace_3.ttf"),
            #[cfg(feature = "comicshanns2")]
            Fonts::ComicShanns2 => include_bytes!("assets/fonts/comic shanns 2.ttf"),
            #[cfg(feature = "jpn_font")]
            Fonts::NotoSansJP => include_bytes!("assets/fonts/NotoSansJP-Regular.ttf"),
            #[cfg(feature = "korean_font")]
            Fonts::NotoSansKR => include_bytes!("assets/fonts/NotoSansKR-Regular.otf"),
            #[cfg(feature = "chinese_font")]
            Fonts::NotoSansTC => include_bytes!("assets/fonts/NotoSansTC-Regular.otf"),
        }
    }

    fn get_all() -> Vec<Fonts> {
        [
            #[cfg(feature = "ccwildwords")]
            Fonts::CcWildWords,
            #[cfg(feature = "kalam")]
            Fonts::Kalam,
            #[cfg(feature = "komikajam")]
            Fonts::KomikaJam,
            #[cfg(feature = "komikaslim")]
            Fonts::KomikaSlim,
            #[cfg(feature = "vtclettererpro")]
            Fonts::VtcLettererPro,
            #[cfg(feature = "bangersregular")]
            Fonts::BangersRegular,
            #[cfg(feature = "animeace")]
            Fonts::AnimeAce,
            #[cfg(feature = "animeace3")]
            Fonts::AnimeAce3,
            #[cfg(feature = "comicshanns2")]
            Fonts::ComicShanns2,
            #[cfg(feature = "jpn_font")]
            Fonts::NotoSansJP,
            #[cfg(feature = "korean_font")]
            Fonts::NotoSansKR,
            #[cfg(feature = "chinese_font")]
            Fonts::NotoSansTC,
        ]
        .to_vec()
    }
}

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    for font in Fonts::get_all() {
        fonts.font_data.insert(
            font.to_string(),
            egui::FontData::from_static(font.get_bytes()),
        );
        fonts
            .families
            .entry(font.get_font_family())
            .or_default()
            .insert(0, font.to_string());
    }
    ctx.set_fonts(fonts);
}
