// This file contains types shared by different parts of the API.

/**
 * A basic representation of one piece of media, such as a movie or TV series.
 */
export interface Media {
  id: number;
  name: string;
  poster_path?: string;
}

/**
 * A file belonging to one piece of media, such as a movie or an episode of a
 * TV series.
 */
export interface Version {
  display_name: string;
  file: string;
  id: number;
}
