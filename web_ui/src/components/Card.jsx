import React, { Component } from "react";
import LazyImage from "./helpers/LazyImage.jsx";
import "./card.scss";

class Card extends Component {
    render() {
        const { name, src } = this.props;

        return (
            <div className="card">
                <a href="https://example.com/">
                    <LazyImage alt={"cover-" + name} src={src}></LazyImage>
                    <p>{name}</p>
                </a>
            </div>
        );
    }
}

export default Card;
