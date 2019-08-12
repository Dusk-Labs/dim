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
        };
        this.fetchCards();
    }

    fetchCards = async () => {
        const cardReq = await fetch(`http://86.21.150.167:8000/api/v1/library/1/media`);
        const json = await cardReq.json();
        const cards = json.map(item => <Card key={item.id} data={item} src={item.poster_path}/>);

        this.setState({
            cards: { RECOMMENDED: cards },
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

        const title1 = "THE 100";
        const desc1 = "Set ninety-seven years after a nuclear war has destroyed civilization, when a spaceship housing humanity's lone survivors sends one hundred juvenile delinquents back to Earth, in hopes of possibly re-populating the planet.";

        const title2 = "Once Upon a Time in Hollywood";
        const desc2 = "A faded television actor and his stunt double strive to achieve fame and success in the film industry during the final years of Hollywood's Golden Age in 1969 Los Angeles.";

        const title3 = "THE EXPANSE";
        const desc3 = "A thriller set two hundred years in the future following the case of a missing young woman who brings a hardened detective and a rogue ship's captain together in a race across the solar system to expose the greatest conspiracy in human history.";

        return (
            <main>
                <section className="banner">
                    <BannerPages>
                        <Banner src="/banner1.jpg" title={title1} description={desc1}/>
                        <Banner src="/result2.jpg" title={title2} description={desc2}/>
                        <Banner src="/result1.jpg" title={title3} description={desc3}/>
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
