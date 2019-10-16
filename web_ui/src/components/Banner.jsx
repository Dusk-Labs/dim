import React, { PureComponent } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Link } from "react-router-dom";
import * as Vibrant from 'node-vibrant';

import ProgressBar from "./ProgressBar.jsx";
import LazyImage from "../helpers/LazyImage.jsx";
import TruncText from "../helpers/TruncText.jsx";

import "./Banner.scss";

class Banner extends PureComponent {
    constructor(props) {
        super(props);

        this._isMounted = false;

        this.imageWrapper = React.createRef();
        this.getImageWrapperRef = this.getImageWrapperRef.bind(this);
        this.handleScroll = this.handleScroll.bind(this);

        this.state = {
            backgroundColor: "#f7931e",
            textColor: "#ffffff"
        };
    }

    componentDidMount() {
        this._isMounted = true;

        window.addEventListener("scroll", () => {
            window.requestAnimationFrame(this.handleScroll);
        });
    }

    componentWillUnmount() {
        this._isMounted = false;

        window.removeEventListener("scroll", () => {
            window.requestAnimationFrame(this.handleScroll);
        });
    }

    handleScroll() {
        const scrolled = window.pageYOffset;
        const rate = scrolled * 0.2;

        if (this.imageWrapper.current !== null) {
            this.imageWrapper.style.webkitTransform = `translate3d(0px, ${rate}px, 0px)`;
            this.imageWrapper.style.MozTransform = `translate3d(0px, ${rate}px, 0px)`;
            this.imageWrapper.style.transform = `translate3d(0px, ${rate}px, 0px)`;
        }
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
        this.imageWrapper = ref;
    }

    render() {
        const { backgroundColor, textColor } = this.state;

        const {
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
                        {genres.map((genre, i) => <Link to={`/search?genre=${genre}`}>{genre}</Link>)}
                </div>
                <div className="info">
                    <h1>{title}</h1>
                    <div className="description">
                        <h5>{banner_caption}</h5>
                        <TruncText content={synopsis} max={35}/>
                    </div>
                    <a
                        href={backdrop}
                        style={{color: textColor, background: backgroundColor}}
                        className="play-btn"
                        rel="noopener noreferrer"
                        target="_blank"
                    >
                        PLAY
                        <FontAwesomeIcon icon="arrow-alt-circle-right"/>
                    </a>
                </div>
                <ProgressBar data={progressBarData}/>
            </div>
        );
    }
}

export default Banner;
