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
                name: "Spiderman: Far From Home",
                imdb: "7.9/10",
                rotten_tomatoes: "90%",
                description: "Following the events of Avengers: Endgame, Spider-Man must step up to take on new threats in a world that has changed forever.",
                genre: "Fantasy/Sci-Fi",
                year: "2019",
                trailer: "",
                length: "00:02:09",
                play: "",
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
        }

        return (
            <div className="card-wrapper" onMouseEnter={this.handleMouseHover} onMouseLeave={this.handleMouseHover}>
                <div className="card">
                    <a href="https://example.com/">
                        <LazyImage alt={"cover-" + name} src={src}></LazyImage>
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
