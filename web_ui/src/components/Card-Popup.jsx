import React, { Component } from "react";
import "./card-popup.scss";

import RottenTomatoeLogo from "../assets/rotten_tomatoe.svg";
import IMDbLogo from "../assets/imdb.png";

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
                        <a href={trailer}><i className="fas fa-play-circle"></i>WATCH TRAILER</a>
                    </div>
                </section>

                <div class="separator"></div>

                <section className="footer">
                    <div className="length">
                        <p>{length}</p>
                        <p>HH MM SS</p>
                    </div>
                    <button href={play}>PLAY<i className="fas fa-play"></i></button>
                </section>
            </div>
        );
    }
}

export default CardPopup;