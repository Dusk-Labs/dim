import React, { Component } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Link } from "react-router-dom";
import * as Vibrant from "node-vibrant";

import ProgressBar from "./ProgressBar.jsx";
import LazyImage from "../Helpers/LazyImage.jsx";
import TruncText from "../Helpers/TruncText.jsx";

import "./Banner.scss";

class Banner extends Component {
    constructor(props) {
        super(props);

        this._isMounted = false;

        this.imageWrapper = React.createRef();
        this.getImageWrapperRef = this.getImageWrapperRef.bind(this);

        this.state = {
            backgroundColor: "#f7931e",
            textColor: "#ffffff"
        };
    }

    componentDidMount() {
        this._isMounted = true;
    }

    componentWillUnmount() {
        this._isMounted = false;
    }

    onLoadBanner = async (blob) => {
        const color = await Vibrant.from(URL.createObjectURL(blob)).getPalette();

        if (this._isMounted) {
            this.setState({
                backgroundColor: color.Vibrant.getHex(),
                textColor: color.Vibrant.getTitleTextColor()
            });
        }
    }

    getImageWrapperRef(ref) {
        this.imageWrapper = {current: ref};
    }

    render() {
        const { backgroundColor, textColor } = this.state;

        const {
            id,
            title,
            year,
            synopsis,
            backdrop,
            banner_caption,
            delta,
            duration,
            genres,
            season,
            episode
        } = this.props.banner;

        if (genres.length > 3) {
            genres.length = 3;
        }

        const progressBarData = {
            textColor: backgroundColor,
            season,
            episode,
            duration,
            delta
        };

        const accentCSS = {
            background: backgroundColor,
            color: textColor
        };

        return (
            <div className="banner">
                <LazyImage
                    alt="banner"
                    src={backdrop}
                    imageWrapperRef={this.getImageWrapperRef}
                    onLoad={this.onLoadBanner}
                />
                <div className="extras">
                    <Link to={`/search?year=${year}`}>{year}</Link>
                    <FontAwesomeIcon icon="circle" style={{ color: backgroundColor }}/>
                        {genres.map((genre, i) => <Link to={`/search?genre=${genre}`} key={i}>{genre}</Link>)}
                </div>
                <div className="info">
                    <h1>{title}</h1>
                    <div className="description">
                        <h5>{banner_caption}</h5>
                        <p><TruncText content={synopsis} max={35}/></p>
                    </div>
                    <Link to={`/play/${id}`} className="play-btn" style={accentCSS}>
                        PLAY
                        <FontAwesomeIcon icon="arrow-alt-circle-right"/>
                    </Link>
                </div>
                <ProgressBar data={progressBarData}/>
            </div>
        );
    }
}

export default Banner;
