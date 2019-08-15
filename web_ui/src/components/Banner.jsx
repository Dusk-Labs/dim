import React, { Component } from "react";
import LazyImage from "./helpers/LazyImage.jsx";
import ProgressBar from "./progress-bar.jsx";
import "./banner.scss";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as Vibrant from 'node-vibrant';
class Banner extends Component {
    constructor(props) {
        super(props);

        this.state = {
            banner_accent: "#f7931e",
            data: {
                img: this.props.src,
                title: this.props.title,
                description: this.props.description,
            },
        };
    }

    onLoadBanner = async (blob) => {
        const color = await Vibrant.from(URL.createObjectURL(blob)).getPalette();
        this.setState({ banner_accent: color.Vibrant.getHex() })
    }

    render() {
        const { banner_accent, data: {img, title, description} } = this.state;

        return (
            <div className="banner-wrapper">
                <LazyImage alt="banner" src={img} onLoad={this.onLoadBanner}/>
                <div className="info">
                    <h1>{title}</h1>
                    <div className="desc">
                        <h5>PICK UP WHERE YOU LEFT OFF</h5>
                        <p>
                            {description}
                        </p>
                    </div>
                    <a href="http://example.com/" style={{ background: banner_accent }}>PLAY<FontAwesomeIcon icon="arrow-alt-circle-right"/></a>
                </div>
                <ProgressBar id="1" accent={banner_accent}/>
            </div>
        );
    }
}

export default Banner;
