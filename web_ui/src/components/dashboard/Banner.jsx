import React, { Component } from "react";
import LazyImage from "../../helpers/LazyImage.jsx";
import ProgressBar from "./ProgressBar.jsx";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as Vibrant from 'node-vibrant';

import "./Banner.scss";

class Banner extends Component {
    constructor(props) {
        super(props);

        this.state = {
            accent: {
                background: "#f7931e",
                text: "#ffffff"
            },
            data: {
                img: this.props.src,
                title: this.props.title,
                description: this.props.description,
            },
        };
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
        const { accent, data: {img, title, description} } = this.state;

        const accentCSS = {
            background: accent.background,
            color: accent.text
        };

        return (
            <div className="banner-wrapper">
                <LazyImage alt="banner" src={img} onLoad={this.onLoadBanner}/>
                <div className="info">
                    <h1>{title}</h1>
                    <div className="desc">
                        <h5>PICK UP WHERE YOU LEFT OFF</h5>
                        <p>{description}</p>
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
                <ProgressBar id="1" accent={accent.background}/>
            </div>
        );
    }
}

export default Banner;
