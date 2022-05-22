use anitomy::Anitomy;
use anitomy::ElementCategory;
use torrent_name_parser::Metadata as TorrentMetadata;

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
            season: metadata
                .get(ElementCategory::AnimeSeason)
                .and_then(|x| x.parse().ok()),
            episode: metadata
                .get(ElementCategory::EpisodeNumber)
                .and_then(|x| x.parse().ok()),
        })
    }
}
