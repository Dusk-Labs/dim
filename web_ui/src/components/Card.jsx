import React, { Component } from "react";
import LazyImage from "./helpers/LazyImage.jsx";
import "./card.css";

class Card extends Component {
    render() {
        const { name, src } = this.props;

        return (
            <div className="card">
                <a href="#">
                    <LazyImage alt={"cover-" + name} src={src}></LazyImage>
                    <p>{name}</p>
                </a>
            </div>
        );
    }
}

export default Card;
