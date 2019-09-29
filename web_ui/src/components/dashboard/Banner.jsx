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

        this.imageWrapper = React.createRef();
        this.getImageWrapperRef = this.getImageWrapperRef.bind(this);
        this.handleScroll = this.handleScroll.bind(this);

        this.state = {
            accent: {
                background: "#f7931e",
                text: "#ffffff"
            },
            banner: <div className="spinner"></div>
        };
    }

    componentDidMount() {
        this.setState({
            ...this.props.banner
        }, this.renderBanner);

        window.addEventListener("scroll", this.handleScroll);
    }

    componentWillUnmount() {
        window.removeEventListener("scroll", this.handleScroll);
    }

    handleScroll() {
        const scrolled = window.pageYOffset;
        const rate = scrolled * 0.2;

        if (this.imageWrapper.current !== null) {
            this.imageWrapper.style.transform = `translate3d(0px, ${rate}px, 0px)`;
        }
    }

    onLoadBanner = async (blob) => {
        const color = await Vibrant.from(URL.createObjectURL(blob)).getPalette();

        this.setState({
            accent: {
                background: color.Vibrant.getHex(),
                text: color.Vibrant.getTitleTextColor()
            }
        }, this.renderBanner);
    }

    getImageWrapperRef(ref) {
        this.imageWrapper = ref;
    }

    renderBanner() {
        let {
            accent,
            backdrop,
            title,
            synopsis,
            season,
            episode,
            duration,
            delta,
            banner_caption,
            genres,
            year,
        } = this.state;

        const accentCSS = {
            background: accent.background,
            color: accent.text
        };

        if (genres.length > 3) {
            genres.length = 3;
        }

        this.setState({
            banner: (
                <div className="banner">
                    <LazyImage alt="banner" src={backdrop} onLoad={this.onLoadBanner} imageWrapperRef={this.getImageWrapperRef}/>
                    <div className="extras">
                        <p>{year}</p>
                        <FontAwesomeIcon icon="circle" style={{ color: accent.background }}/>
                        {genres.map((genre, i) => <p key={i}>{genre}</p>)}
                    </div>
                    <div className="info">
                        <h1>{title}</h1>
                        <div className="description">
                            <h5>{banner_caption}</h5>
                            <TruncText content={synopsis} max={35}/>
                        </div>
                        <a
                            href={backdrop}
                            style={accentCSS}
                            rel="noopener noreferrer"
                            target="_blank">
                            PLAY
                            <FontAwesomeIcon icon="arrow-alt-circle-right"/>
                        </a>
                    </div>
                    <ProgressBar id="1" accent={accent.background} season={season} episode={episode} duration={duration} delta={delta === undefined ? 0 : delta}/>
                </div>
            )
        });
    }

    render() {
        return this.state.banner;
    }
}

export default Banner;
