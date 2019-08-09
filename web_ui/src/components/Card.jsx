import React, { Component } from "react";
import LazyImage from "./helpers/LazyImage.jsx";
import CardPopup from "./Card-Popup.jsx";
import "./card.scss";

class Card extends Component {

    constructor(props) {
        super(props);
        this.handleMouseHover = this.handleMouseHover.bind(this);
        this.handleMouseUnHover = this.handleMouseUnHover.bind(this);
        this.state = {
            hovering: false
        };
    }

    handleMouseHover() {
        this.setState({
            hovering: true
        });
    }

    handleMouseUnHover() {
        this.setState({
            hovering: false
        });
    }

    render() {
        const { name, src } = this.props;

        return (
            !this.state.hovering
                ? (
                    <div className="card" onMouseEnter={this.handleMouseHover} onMouseLeave={this.handleMouseUnHover}>
                        <a href="https://example.com/">
                            <LazyImage alt={"cover-" + name} src={src}></LazyImage>
                            <p>{name}</p>
                        </a>
                    </div>
                ) : (
                    <CardPopup
                        name="Spiderman: Far From Home"
                        imdb="8.7"
                        rotten_tomatoes="10"
                        description="Cba"
                        genre="Sci-Fi"
                        year="2019"
                        trailer=""
                        length="02:00:00"
                        play=""
                    />
                )
        );
    }
}

export default Card;
