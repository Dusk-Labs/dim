import React, { PureComponent } from "react";
import CardPopup from "./Card-Popup.jsx";
import LazyImage from "./helpers/LazyImage.jsx";

import "./card.scss";
import * as Vibrant from 'node-vibrant';

class Card extends PureComponent {
    constructor(props) {
        super(props);
        this.handleMouseHover = this.handleMouseHover.bind(this);

        this.state = {
            hovering: false,
            timeout: false,
            accent: {
                background: "#f7931e",
                text: "#ffffff"
            }
        };
    }

    handleMouseHover() {
        if(this.state.hoverTimeout != null) {
            clearTimeout(this.state.hoverTimeout);
            this.setState({ hoverTimeout: null, hovering: false });
            return;
        }

        this.setState({
            hoverTimeout: setTimeout(this.renderCardPopout.bind(this), 600),
        });
    }

    renderCardPopout() {
        this.setState({
            hovering: !this.state.hovering,
        });
    }

    onLoadPoster = async (blob) => {
        const color = await Vibrant.from(URL.createObjectURL(blob)).getPalette();

        this.setState({
            accent: {
                background: color.Vibrant.getHex(),
                text: color.Vibrant.getTitleTextColor()
            }
        });
    }

    render() {
        const { accent } = this.state
        const { name, poster_path } = this.props.data;

        const cover = (
            poster_path
                ? <LazyImage alt={"cover-" + name} src={poster_path} onLoad={this.onLoadPoster}/>
                : <div className="placeholder"></div>
        );

        return (
            <div className="card-wrapper" onMouseEnter={this.handleMouseHover} onMouseLeave={this.handleMouseHover}>
                <div className="card">
                    <a href={poster_path} rel="noopener noreferrer" target="_blank">
                        { cover }
                        <p style={{opacity: + !this.state.hovering}}>{name}</p>
                    </a>
                </div>
                {this.state.hovering &&
                    <CardPopup data={this.props.data} accent={accent}/>
                }
            </div>
        );
    }
}

export default Card;
