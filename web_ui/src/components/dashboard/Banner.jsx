import React, { PureComponent } from "react";
import LazyImage from "../../helpers/LazyImage.jsx";
import ProgressBar from "./ProgressBar.jsx";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import TruncText from "../../helpers/TruncText.jsx";
import * as Vibrant from 'node-vibrant';

import "./Banner.scss";

class Banner extends PureComponent {
    constructor(props) {
        super(props);

        this.state = {
            accent: {
                background: "#f7931e",
                text: "#ffffff"
            },
            data: {
                img: this.props.src,
                title: "TITLE",
                description: "DESCRIPTION",
                season: this.props.season,
                episode: this.props.episode,
                duration: this.props.duration,
                delta: this.props.delta,
                banner_caption: this.props.banner_caption,
                genres: [],
                year: this.props.year,
            },
        };
    }

    componentDidMount() {
        this.setState({
            data: {
                img: this.props.src,
                title: this.props.title,
                description: this.props.description,
                season: this.props.season,
                episode: this.props.episode,
                duration: this.props.duration,
                delta: this.props.delta,
                banner_caption: this.props.banner_caption,
                genres: this.props.genres,
                year: this.props.year,
            }
        });
    }

    onLoadBanner = async (blob) => {
        const color = await Vibrant.from(URL.createObjectURL(blob)).getPalette();

        this.setState({
            accent: {
                background: color.Vibrant.getHex(),
                text: color.Vibrant.getTitleTextColor()
            }
        });
    }

    render() {
        let {
            accent,
            data: {
                img,
                title,
                description,
                season,
                episode,
                duration,
                delta,
                banner_caption,
                genres,
                year
            }
        } = this.state;

        const accentCSS = {
            background: accent.background,
            color: accent.text
        };

        if (genres.length > 3) {
            genres.length = 3;
        }

        return (
            <div className="banner">
                <LazyImage alt="banner" src={img} onLoad={this.onLoadBanner}/>
                <div className="extras">
                    <p>{year}</p>
                    <FontAwesomeIcon icon="circle" style={{ color: accent.background }}/>
                    {genres.map((genre, i) => <p key={i}>{genre}</p>)}
                </div>
                <div className="info">
                    <h1>{title}</h1>
                    <div className="description">
                        <h5>{banner_caption}</h5>
                        <TruncText content={description} max={35}/>
                    </div>
                    <a
                        href={img}
                        style={accentCSS}
                        rel="noopener noreferrer"
                        target="_blank">
                        PLAY
                        <FontAwesomeIcon icon="arrow-alt-circle-right"/>
                    </a>
                </div>
                <ProgressBar id="1" accent={accent.background} season={season} episode={episode} duration={duration} delta={delta}/>
            </div>
        );
    }
}

export default Banner;
