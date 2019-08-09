import React, { Component } from "react";
import LazyImage from "./helpers/LazyImage.jsx";
import "./card.scss";

class Card extends Component {

    constructor(props) {
        super(props);

        this.state = {
            data: {}
        };
    }

    componentDidMount() {
        fetch(`api/${this.props.id}`)
            .then(res => res.json())
            .then(({data}) => this.setState({ data: data}));
    }

    render() {
        let { name, src } = this.state.data;

        if (!name) {
            name = "MISSING NAME";
        }

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
