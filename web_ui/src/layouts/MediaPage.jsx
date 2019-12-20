import React, { Component } from "react";
import { connect } from "react-redux";
import { Link } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import LazyImage from "../helpers/LazyImage.jsx";

import {
    fetchMediaInfo,
    fetchExtraMediaInfo,
    fetchMediaSeasons,
    fetchMediaSeasonEpisodes
} from "../actions/cardActions.js";

import "./MediaPage.scss";
class MediaPage extends Component {
    constructor(props) {
        super(props);

        this.showSeason = this.showSeason.bind(this);

        this.state = {
            season: undefined
        }
    }

    async componentDidMount() {
        const { id } = this.props.match.params;

        this.props.fetchMediaInfo(id);
        this.props.fetchExtraMediaInfo(id);
    }

    componentDidUpdate() {
        // FETCH_MEDIA_INFO_OK
        if (this.props.fetch_media_info.fetched && !this.props.fetch_media_info.error) {
            document.title = `Dim - ${this.props.fetch_media_info.info.name}`;
        }
    }

    showSeason(number) {
        this.setState({
            season: number
        });
    }

    render() {
        let backdrop;
        let metaContent;

        // FETCH_MEDIA_INFO_START
        if (this.props.fetch_media_info.fetching) {
            backdrop = <div className="placeholder"/>;

            metaContent = (
                <div className="meta-content">
                    <div className="cover">
                        <div className="placeholder"/>
                    </div>
                    <div className="overview">
                        <h1><div className="placeholder-text"/></h1>
                        <div className="genres">
                            <div className="placeholder-text"/>
                            <div className="placeholder-text"/>
                            <div className="placeholder-text"/>
                            <div className="placeholder-text"/>
                        </div>
                        <div className="description">
                            <div className="placeholder-text"/>
                        </div>
                        <div className="meta-info">
                            <div className="info">
                                <h4><div className="placeholder-text"/></h4>
                                <div className="placeholder-text"/>
                            </div>
                            <div className="info">
                                <h4><div className="placeholder-text"/></h4>
                                <div className="placeholder-text"/>
                            </div>
                            <div className="info">
                                <h4><div className="placeholder-text"/></h4>
                                <div className="placeholder-text"/>
                            </div>
                            <div className="info">
                                <h4><div className="placeholder-text"/></h4>
                                <div className="placeholder-text"/>
                            </div>
                        </div>
                    </div>
                </div>
            );
        }

        // FETCH_MEDIA_INFO_ERR
        if (this.props.fetch_media_info.fetched && this.props.fetch_media_info.error) {
            console.log("[FETCH MEDIA INFO] ERR", this.props.fetch_media_info);

            backdrop = <div className="placeholder"/>;

            metaContent = (
                <div className="meta-content">
                    <div className="cover">
                        <LazyImage/>
                    </div>
                    <div className="overview">
                        <div className="horizontal-err">
                            <FontAwesomeIcon icon="times-circle"/>
                            <p>FAILED TO LOAD</p>
                        </div>
                        <div className="genres">
                            <div className="placeholder-text"/>
                            <div className="placeholder-text"/>
                            <div className="placeholder-text"/>
                            <div className="placeholder-text"/>
                        </div>
                        <div className="description">
                            <div className="placeholder-text"/>
                        </div>
                        <div className="meta-info">
                            <div className="info">
                                <h4><div className="placeholder-text"/></h4>
                                <div className="placeholder-text"/>
                            </div>
                            <div className="info">
                                <h4><div className="placeholder-text"/></h4>
                                <div className="placeholder-text"/>
                            </div>
                            <div className="info">
                                <h4><div className="placeholder-text"/></h4>
                                <div className="placeholder-text"/>
                            </div>
                            <div className="info">
                                <h4><div className="placeholder-text"/></h4>
                                <div className="placeholder-text"/>
                            </div>
                        </div>
                    </div>
                </div>
            );
        }

        // FETCH_MEDIA_INFO_OK
        if (this.props.fetch_media_info.fetched && !this.props.fetch_media_info.error) {
            console.log("[FETCH MEDIA INFO] OK", this.props.fetch_media_info.info);

            const {
                backdrop_path,
                poster_path,
                description,
                genres,
                name,
                duration,
                rating,
                year,
                media_type,
                id
            } = this.props.fetch_media_info.info;

            backdrop = <LazyImage src={backdrop_path}/>;

            metaContent = (
                <div className="meta-content">
                    <div className="cover">
                        <LazyImage src={poster_path}/>
                    </div>
                    <div className="overview">
                        <h1>{name}</h1>
                        <div className="genres">
                            <Link to={`/search?year=${year}`}>{year}</Link>
                            <FontAwesomeIcon icon="circle"/>
                            {genres &&
                                genres.map((genre, i) => <Link to={`/search/genre=${genre}`} key={i}>{genre}</Link>)
                            }
                        </div>
                        <p className="description">{description}</p>
                        <div className="meta-info">
                            <div className="info">
                                <h4>ID</h4>
                                <p>{id}</p>
                            </div>
                            <div className="info">
                                <h4>MEDIA TYPE</h4>
                                <p>{media_type}</p>
                            </div>
                            <div className="info">
                                <h4>DURATION</h4>
                                <p>{Math.round(duration / 60)} min</p>
                            </div>
                            <div className="info">
                                <h4>RATING</h4>
                                <p>{rating}/10</p>
                            </div>
                        </div>
                    </div>
                </div>
            );
        }

        let mediaSeasons;
        let mediaEpisodes = {};

        // FETCH_EXTRA_MEDIA_INFO_START
        if (this.props.fetch_extra_media_info.fetching) {
            console.log("[FETCH EXTRA MEDIA INFO] FETCHING");
        }

        // FETCH_EXTRA_MEDIA_INFO_ERR
        if (this.props.fetch_extra_media_info.fetched && this.props.fetch_extra_media_info.error) {
            console.table("[FETCH EXTRA MEDIA INFO] ERR", this.props.fetch_extra_media_info);
        }

        // FETCH_EXTRA_MEDIA_INFO_OK
        if (this.props.fetch_extra_media_info.fetched && !this.props.fetch_extra_media_info.error) {
            console.log("[FETCH EXTRA MEDIA INFO] OK", this.props.fetch_extra_media_info);

            const { seasons } = this.props.fetch_extra_media_info.info;

            seasons.sort((a, b) => {
                return a.season_number - b.season_number;
            });

            console.log("SEASONS", seasons);

            mediaSeasons = seasons.map((season, si) => {
                return (
                    <div className="season" key={si} onClick={() => this.showSeason(season.season_number)}>
                        <LazyImage src={season.poster}/>
                        <p>SEASON {season.season_number}</p>
                    </div>
                );
            });

            for (let x = 0; x < seasons.length; x++) {
                seasons[x].episodes.sort((a, b) => {
                    return a.episode - b.episode;
                });

                mediaEpisodes[seasons[x].season_number] = seasons[x].episodes.map((episode, i) => {
                    return (
                        <Link to={`/play/${episode.versions[0].id}`} className="episode" key={i}>
                            <LazyImage src={episode.backdrop}/>
                            <p>EPISODE {episode.episode}</p>
                        </Link>
                    );
                });
            }
        }

        return (
            <div className="media-page">
                <div className="backdrop">
                    {backdrop}
                </div>
                {metaContent}
                <div className="content">
                    <div className="se-ep">
                        <div className="seasons">
                            <h2>SEASONS</h2>
                            <div className="list">
                                {mediaSeasons}
                            </div>
                        </div>
                        {this.state.season !== undefined &&
                            <div className="episodes">
                                <h2>SEASON {this.state.season} EPISODES</h2>
                                <div className="list">
                                    {mediaEpisodes[this.state.season]}
                                </div>
                            </div>
                        }
                    </div>
                </div>
            </div>
        );
    }
}

const mapStateToProps = (state) => ({
    fetch_media_info: state.cardReducer.fetch_media_info,
    fetch_extra_media_info: state.cardReducer.fetch_extra_media_info,
    fetch_media_seasons_info: state.cardReducer.fetch_media_seasons_info,
    fetch_media_season_episodes_info: state.cardReducer.fetch_media_season_episodes_info,
});

const mapActionsToProps = {
    fetchMediaInfo,
    fetchExtraMediaInfo,
    fetchMediaSeasons,
    fetchMediaSeasonEpisodes
};

export default connect(mapStateToProps, mapActionsToProps)(MediaPage);
