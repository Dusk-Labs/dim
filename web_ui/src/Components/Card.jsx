import React, { Component } from "react";
import { Link } from "react-router-dom";
import Vibrant from "node-vibrant";

import CardPopup from "./CardPopup.jsx";
import LazyImage from "../Helpers/LazyImage.jsx";

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
            hoverTimeout: null
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
        if (this.state.hovering) return;

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
        try {
            const color = await Vibrant.from(this.state.blob).getPalette();

            const accent = {
                background: color.Vibrant.getHex(),
                text: color.Vibrant.getTitleTextColor()
            };

            this.setState({
                hovering: true,
                accent
            });
        } catch {
            this.setState({
                hovering: true,
                accent: {
                    background: "#f7931e",
                    text: "#fff"
                }
            });
        }
    }

    onLoadPoster = (blob) => this.setState({blob: URL.createObjectURL(blob)});
    setCardPopup = (ref) => this.cardPopup = ref;

    render() {
        const { name, poster_path, id } = this.props.data;

        const data = {
            ...this.props.data,
            accent: this.state?.accent
        };

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
                {this.state.hovering &&
                    <CardPopup setCardPopup={this.setCardPopup} data={data}/>
                }
            </div>
        );
    }
}

export default Card;