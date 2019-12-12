import React, { Component } from "react";
import { Link } from "react-router-dom";
import * as Vibrant from 'node-vibrant';

import CardPopup from "./CardPopup.jsx";
import LazyImage from "../helpers/LazyImage.jsx";

import "./Card.scss";

class Card extends Component {
    constructor(props) {
        super(props);

        this.cardWrapper = React.createRef();
        this.card = React.createRef();
        this.handleMouseHover = this.handleMouseHover.bind(this);

        this.state = {
            hovering: false,
            timeout: false,
            accentDone: false,
            accent: {
                background: "#f7931e",
                text: "#ffffff"
            }
        };
    }

    componentDidMount() {
        this.cardWrapper.current.addEventListener("mouseenter", this.handleMouseHover);
        this.cardWrapper.current.addEventListener("mouseleave", this.handleMouseHover);
        this.cardWrapper.current.addEventListener("focusin", this.handleMouseHover);
        this.cardWrapper.current.addEventListener("focusout", this.handleMouseHover);
    }

    componentWillUnmount() {
        this.cardWrapper.current.removeEventListener("mouseenter", this.handleMouseHover);
        this.cardWrapper.current.removeEventListener("mouseleave", this.handleMouseHover);
        this.cardWrapper.current.removeEventListener("focusin", this.handleMouseHover);
        this.cardWrapper.current.removeEventListener("focusout", this.handleMouseHover);
    }

    handleMouseHover() {
        this.card.current.style.animation = "none";

        if (this.state.hoverTimeout != null) {
            clearTimeout(this.state.hoverTimeout);

            this.setState({
                hoverTimeout: null,
                hovering: false
            });

            return;
        }

        this.setState({
            hoverTimeout: setTimeout(this.renderCardPopout.bind(this), 600),
        });
    }

    async renderCardPopout() {
        if (!this.state.accentDone && this.state.posterBlob !== undefined) {
            const color = await Vibrant.from(this.state.posterBlob).getPalette();

            this.setState({
                accent: {
                    background: color.Vibrant.getHex(),
                    text: color.Vibrant.getTitleTextColor()
                }
            });
        }

        this.setState({
            hovering: !this.state.hovering,
        });
    }

    onLoadPoster = async (blob) => {
        this.setState({
            posterBlob: URL.createObjectURL(blob)
        });
    }

    render() {
        const { accent } = this.state
        const { name, poster_path, id } = this.props.data;

        return (
            <div className="card-wrapper" ref={this.cardWrapper}>
                <div id={id} className="card" ref={this.card}>
                    <Link to={`/media/${id}`}>
                        <LazyImage
                            alt={"cover-" + name}
                            src={poster_path}
                            onLoad={this.onLoadPoster}
                        />
                        <p style={{opacity: + !this.state.hovering}}>{name}</p>
                    </Link>
                </div>
                {this.state.hovering && <CardPopup data={this.props.data} accent={accent}/>}
            </div>
        );
    }
}

export default Card;
