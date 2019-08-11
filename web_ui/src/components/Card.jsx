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
            hovering: false,
            timeout: false
        };
    }

    componentDidMount() {
        let data = this.props.data;
        this.setState({
            data: {
                name: data.name,
                imdb: `${data.rating}/10`,
                rotten_tomatoes: "90%",
                description: data.description,
                genre: "Fantasy/Sci-Fi",
                year: data.year,
                trailer: "",
                length: "00:00:00",
                play: "",
                src: data.poster_path,
            }
        })
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

    render() {
        let { name, src } = this.state.data;

        if (!name) {
            name = "MISSING NAME";
            // If we dont return a empty div then later on everything works
            // but <LazyImage> doesnt receive the correct props
            // TODO: Investigate
            return <div></div>;
        }

        return (
            <div className="card-wrapper" onMouseEnter={this.handleMouseHover} onMouseLeave={this.handleMouseHover}>
                <div className="card">
                    <a href="https://example.com/">
                        <LazyImage alt={"cover-" + name} src={src}/>
                        <p style={{opacity: + !this.state.hovering}}>{name}</p>
                    </a>
                </div>
                {this.state.hovering &&
                    <CardPopup data={this.state.data}/>
                }
            </div>
        );
    }
}

export default Card;
