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

        this.setCardPopup = this.setCardPopup.bind(this);
        this.handleMouseEnter = this.handleMouseEnter.bind(this);
        this.handleMouseLeave = this.handleMouseLeave.bind(this);

        this.state = {
            hovering: false,
            hoverTimeout: null,
            accentDone: false,
            accent: {
                background: "#f7931e",
                text: "#ffffff"
            }
        };
    }

    componentDidMount() {
        this.card.current.addEventListener("mouseenter", this.handleMouseEnter);
        this.cardWrapper.current.addEventListener("mouseleave", this.handleMouseLeave);
    }

    componentWillUnmount() {
        this.card.current.removeEventListener("mouseenter", this.handleMouseEnter);
        this.cardWrapper.current.removeEventListener("mouseleave", this.handleMouseLeave);
    }

    handleMouseEnter(e) {
        this.setState({
            hoverTimeout: setTimeout(this.renderCardPopout.bind(this), 600)
        });
    }

    handleMouseLeave() {
        clearTimeout(this.state.hoverTimeout);

        this.cardPopup?.classList.add("hideCardPopup");

        this.cardPopup?.addEventListener("animationend", (e) => {
            if (e.animationName !== "CardPopupHide") return;
            this.setState({hovering: false});
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
            hovering: true,
        });
    }

    onLoadPoster = async (blob) => {
        this.setState({
            posterBlob: URL.createObjectURL(blob)
        });
    }

    setCardPopup(ref) {
        this.cardPopup = ref;
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
                {this.state.hovering && <CardPopup setCardPopup={this.setCardPopup} data={this.props.data} accent={accent}/>}
            </div>
        );
    }
}

export default Card;
