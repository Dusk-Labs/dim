import React, { Component } from "react";
import CardPopup from "./Card-Popup.jsx";
import LazyImage from "./helpers/LazyImage.jsx";
import "./card.scss";

class Card extends Component {

    constructor(props) {
        super(props);
        this.handleMouseHover = this.handleMouseHover.bind(this);

        this.state = {
            data: {},
            pos: {},
            hovering: false
        };
    }

    componentDidMount() {
        // fetch(`api/${this.props.id}`)
        //     .then(res => res.json())
        //     .then(({data}) => {
        //         this.setState({
        //             data: {
        //                 name: ,
        //                 imdb: ,
        //                 rotten_tomatoes: ,
        //                 description: ,
        //                 genre: ,
        //                 year: ,
        //                 trailer: ,
        //                 length: ,
        //                 play: ,
        //             }
        //         })
        //     });
        this.setState({
            data: {
                name: "Movie Name",
                imdb: "0",
                rotten_tomatoes: "0",
                description: "Lorem ipsum",
                genre: "GENRE",
                year: this.props.id,
                trailer: "",
                length: "00:00:00",
                play: "",
            }
        })
    }

    handleMouseHover() {
        this.setState({
            hovering: !this.state.hovering
        });
    }

    render() {
        let { name, src } = this.state.data;

        if (!name) {
            name = "MISSING NAME";
        }

        return (
            <div className="card-wrapper" onMouseEnter={this.handleMouseHover} onMouseLeave={this.handleMouseHover}>
                <div className="card">
                    <a href="https://example.com/">
                        <LazyImage alt={"cover-" + name} src={src}></LazyImage>
                        <p>{name}</p>
                    </a>
                </div>
                {this.state.hovering &&
                    <CardPopup parent={<div></div>} data={this.state.data}/>
                }
            </div>
        );
    }
}

export default Card;
