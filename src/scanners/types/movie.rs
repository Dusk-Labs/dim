let media_id: i32;
if let Ok(media) = Media::get_by_name_and_lib(&self.conn, &self.lib, &result.get_title().unwrap()) {
    media_id = media.id;
} else {
    info!(self.log, "Inserting movie: {}", result.get_title().unwrap());
    let media = InsertableMedia {
        library_id: self.lib.id,
        name: result.get_title().unwrap(),
        description: result.overview,
        rating: match result.vote_average {
            Some(d) => Some(d as i32),
            None => None,
        },
        year,
        added: Utc::now().to_string(),
        poster_path: match result.poster_path {
            Some(path) => Some(format!(
                    "https://image.tmdb.org/t/p/w600_and_h900_bestv2{}",
                    path
                    )),
            None => None,
        },
        backdrop_path: match result.backdrop_path {
            Some(path) => Some(format!("https://image.tmdb.org/t/p/original/{}", path)),
            None => None,
        },
        media_type: self.lib.media_type.clone(),
    };

    media_id = match media.into_streamable::<InsertableMovie>(&self.conn) {
        Ok(id) => id,
        Err(err) => {
            error!(self.log, "Error inserting media: {}", err);
            return;
        }
    };

    if let Some(y) = result.genres {
        for x in y {
            let genre = InsertableGenre {
                name: x.name.clone(),
            };

            let genre_id = genre.insert(&self.conn).unwrap();

            let pair = InsertableGenreMedia { genre_id, media_id };

            pair.insert(&self.conn);
        }
    }
}

let updated_mediafile = UpdateMediaFile {
    media_id: Some(media_id),
    target_file: None,
    raw_name: None,
    raw_year: None,
    quality: None,
    codec: None,
    container: None,
    audio: None,
    original_resolution: None,
    duration: None,
    corrupt: None,
    episode: None,
    season: None,
};

updated_mediafile.update(&self.conn, orphan.id).unwrap();

let event_message = Message {
    id: media_id,
    event_type: PushEventType::EventNewCard,
};

let new_event = Event::new(&format!("/events/library/{}", self.lib.id), event_message);

let _ = self.event_tx.send(new_event);
}
