export interface Media {
  id: number;
  name: string;
  poster_path?: string;
}

export interface SearchResult extends Media {
  library_id: number;
}
