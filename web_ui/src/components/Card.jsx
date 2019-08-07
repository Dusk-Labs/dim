import React, { Component } from "react";
import "./card.css";

class Card extends Component {
    render() {
        const { name, src } = this.props;

        return (
            <div className="card">
                <a href="#">
                    <img alt={"cover-" + name} src={src}></img>
                    <h5>{name}</h5>
                </a>
            </div>
        );
    }
}

export default Card;