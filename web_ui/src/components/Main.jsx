import React, { Component } from "react";
import Card from "./Card.jsx";
import Banner from "./Banner.jsx";
import BannerPages from "./BannerPagination.jsx";
import "./main.scss";

import { library } from "@fortawesome/fontawesome-svg-core";
import { faArrowAltCircleRight } from "@fortawesome/free-solid-svg-icons";

library.add(faArrowAltCircleRight);

class Main extends Component {
    constructor(props) {
        super(props);

        this.state = {
            cards: {},
            banners: [
                {
                    name: "The 100",
                    src: "/banner1.jpg",
                    desc: "Set ninety-seven years after a nuclear war has destroyed civilization, when a spaceship housing humanity's lone survivors sends one hundred juvenile delinquents back to Earth, in hopes of possibly re-populating the planet."
                },
                {
                    name: "Blade Runner 2049",
                    src: "/banner6.jpg",
                    desc: "Thirty years after the events of the first film, a new blade runner, LAPD Officer K, unearths a long-buried secret that has the potential to plunge what's left of society into chaos. K's discovery leads him on a quest to find Rick Deckard, a former LAPD blade runner who has been missing for 30 years."
                },
                {
                    name: "The Expanse",
                    src: "/banner3.jpg",
                    desc: "A thriller set two hundred years in the future following the case of a missing young woman who brings a hardened detective and a rogue ship's captain together in a race across the solar system to expose the greatest conspiracy in human history."
                }
            ]
        };
    }

    async componentDidMount() {
        const cardReq = await fetch("http://86.21.150.167:8000/api/v1/library/2/media");
        const json = await cardReq.json();
        const cards = json.map(item => <Card key={item.id} data={item} src={item.poster_path}/>);

        this.setState({
            cards: { recommended: cards },
        });
    }

    render() {
        const { cards } = this.state;

        const sections = Object.keys(cards).map((key) => {
            return <div className="recommended" key={key}>
                <h1>{key}</h1>
                <div className="cards">
                    { cards[key] }
                </div>
            </div>
        });

        return (
            <main>
                <section className="banner">
                    <BannerPages>
                        {this.state.banners.map(({name, src, desc}) => <Banner src={src} title={name} description={desc}/>)}
                    </BannerPages>
                </section>
                <section className="libraries">
                    { sections }
                </section>
            </main>
        );
    }
}

export default Main;
