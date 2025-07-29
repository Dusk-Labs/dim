pub use anitomy::Anitomy;
use anitomy::ElementCategory;
pub use torrent_name_parser::Metadata as TorrentMetadata;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Metadata {
    pub name: String,
    pub year: Option<i64>,
    pub season: Option<i64>,
    pub episode: Option<i64>,
}

pub trait FilenameMetadata {
    fn from_str(s: &str) -> Option<Metadata>;
}

impl FilenameMetadata for TorrentMetadata {
    fn from_str(s: &str) -> Option<Metadata> {
        let metadata = TorrentMetadata::from(s).ok()?;

        Some(Metadata {
            name: metadata.title().to_owned(),
            year: metadata.year().map(|x| x as i64),
            season: metadata.season().map(|x| x as i64),
            episode: metadata.episode().map(|x| x as i64),
        })
    }
}

impl FilenameMetadata for Anitomy {
    fn from_str(s: &str) -> Option<Metadata> {
        let metadata = match Anitomy::new().parse(s) {
            Ok(v) | Err(v) => v,
        };

        Some(Metadata {
            name: metadata.get(ElementCategory::AnimeTitle)?.to_string(),
            year: metadata
                .get(ElementCategory::AnimeYear)
                .and_then(|x| x.parse().ok()),
            // If season isnt specified we assume season 1 here.
            season: metadata
                .get(ElementCategory::AnimeSeason)
                .and_then(|x| x.parse().ok())
                .or(Some(1)),
            episode: metadata
                .get(ElementCategory::EpisodeNumber)
                .and_then(|x| x.parse().ok()),
        })
    }
}

/// A special filename metadata extractor that combines torrent_name_parser and anitomy which in
/// some cases is necessary. TNP is really good at extracting show titles but not season and
/// episode numbers. Anitomy excels at this. Here we combine the title extracted by TPN and the
/// season and episode number extracted by Anitomy.
pub struct CombinedExtractor;

impl FilenameMetadata for CombinedExtractor {
    fn from_str(s: &str) -> Option<Metadata> {
        let metadata_tnp = TorrentMetadata::from(s).ok()?;
        let metadata_anitomy = match Anitomy::new().parse(s) {
            Ok(v) | Err(v) => v,
        };

        Some(Metadata {
            name: metadata_tnp.title().to_owned(),
            year: metadata_tnp.year().map(|x| x as i64),
            // If season isnt specified we assume season 1 here as some releases only have a
            // episode number and no season number.
            season: metadata_anitomy
                .get(ElementCategory::AnimeSeason)
                .and_then(|x| x.parse().ok())
                .or(Some(1)),
            episode: metadata_anitomy
                .get(ElementCategory::EpisodeNumber)
                .and_then(|x| x.parse().ok()),
        })
    }
}
