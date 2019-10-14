import React, { Component } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import TruncText from "../helpers/TruncText.jsx";
import IMDbLogo from "../assets/imdb.png";

import "./CardPopup.scss";

class CardPopup extends Component {
    constructor(props) {
        super(props);

        this.popup = React.createRef();

        this.state = {
            overflowing: false,
            class: "card-popup-right"
        };
    }

    componentDidMount() {
        const { x, width } = this.popup.current.getBoundingClientRect();
        const overflowing = (x + width > window.innerWidth - 5);

        this.setState({
            accent: this.props.accent
        });

        if (!overflowing) return;

        this.setState({
            overflowing: true,
            class: "card-popup-left"
        });

    }

    static getDerivedStateFromProps(nextProps) {
        return {
            accent: nextProps.accent,
        };
    }

    render() {
        const {
            name,
            rating,
            description,
            genres,
            year,
            duration,
            play
        } = this.props.data;

        const { accent } = this.props;
        const genre = genres[Math.floor(Math.random() * genres.length)];

        const length = {
            hh: ("0" + Math.floor(duration / 3600)).slice(-2),
            mm: ("0" + Math.floor(duration % 3600 / 60)).slice(-2),
            ss: ("0" + Math.floor(duration % 3600 % 60)).slice(-2)
        };

        const accentCSS = {
            background: accent.background,
            color: accent.text
        };

        return (
            <div className={this.state.class} ref={this.popup}>
                <div className="clipped"></div>
                <section className="header">
                    {!this.state.overflowing && <h1>{name}</h1>}
                    <div className="rating">
                        <img alt="imdb" src={IMDbLogo}></img><p>{rating}/10</p>
                    </div>
                    {this.state.overflowing && <h1>{name}</h1>}
                </section>
                <section className="content">
                    <section className="description">
                        <h4>Description</h4>
                        {description.length > 0
                            ? <TruncText content={description} max={21}/>
                            : <p>No description found.</p>
                        }
                    </section>
                    <section className="info">
                        <div className="tags">
                            <p style={accentCSS}>{year}</p>
                            <p style={accentCSS}>{genre}</p>
                        </div>
                        <div className="options">
                            <button><FontAwesomeIcon icon="pen"/></button>
                        </div>
                    </section>
                    <section className="separator"></section>
                    <section className="footer">
                        <div className="length">
                            <p>{length.hh}:{length.mm}:{length.ss}</p>
                            <p>HH MM SS</p>
                        </div>
                        <a href={play} style={accentCSS}>PLAY<FontAwesomeIcon icon="arrow-alt-circle-right"/></a>
                    </section>
                </section>

            </div>
        );
    }
}

export default CardPopup;
