import React, { Component } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import "./card-popup.scss";

import RottenTomatoeLogo from "../assets/rotten_tomatoe.svg";
import IMDbLogo from "../assets/imdb.png";

import { library } from "@fortawesome/fontawesome-svg-core";

import {
    faPlay,
    faPlayCircle,
} from "@fortawesome/free-solid-svg-icons";

library.add(
    faPlay,
    faPlayCircle
);

class CardPopup extends Component {
    render() {
        const {
            name,
            imdb,
            rotten_tomatoes,
            description,
            genre,
            year,
            trailer,
            length,
            play,
        } = this.props;

        return (
            <div className="card-popup">
                <section className="header">
                    <h1>{name}</h1>
                    <div className="rating">
                        <img alt="imdb" src={IMDbLogo}></img><p>{imdb}</p>
                        <img alt="rotten tomatoes" src={RottenTomatoeLogo}></img><p>{rotten_tomatoes}</p>
                    </div>
                </section>
                <section className="main">
                    <h4>Description</h4>
                    <p>{description}</p>
                    <div className="info">
                        <div className="tags">
                            <p>{genre}</p>
                            <p>{year}</p>
                        </div>
                        <a href={trailer}><FontAwesomeIcon icon="play-circle"/>WATCH TRAILER</a>
                    </div>
                </section>

                <div class="separator"></div>

                <section className="footer">
                    <div className="length">
                        <p>{length}</p>
                        <p>HH MM SS</p>
                    </div>
                    <button href={play}>PLAY<FontAwesomeIcon icon="play"/></button>
                </section>
            </div>
        );
    }
}

export default CardPopup;