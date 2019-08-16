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
            cards: [],
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
        const req = await fetch("http://86.21.150.167:8000/api/v1/library/1/media");
        const payload = await req.json();

        const cards = payload.map((card, i) => <Card key={i} data={card}/>);
        this.setState({ cards });
    }

    render() {
        let { cards, banners } = this.state;

        banners = banners.map(({name, src, desc}, i) => <Banner key={i} src={src} title={name} description={desc}/>);

        return (
            <main>
                <BannerPages>{banners}</BannerPages>
                <section className="libraries">
                    <div className="cards">{cards}</div>
                </section>
            </main>
        );
    }
}

export default Main;
