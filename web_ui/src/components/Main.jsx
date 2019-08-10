import React, { Component } from "react";
import Card from "./Card.jsx";
import LazyImage from "./helpers/LazyImage.jsx";
import ProgressBar from "./progress-bar.jsx";
import "./main.scss";

import { library } from "@fortawesome/fontawesome-svg-core";
import { faPlay } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

library.add(faPlay);

class Main extends Component {

    render() {
        return (
            <main>
                <section className="banner">
                    <LazyImage alt="banner" src="/banner1.jpg"></LazyImage>
                    <div className="info">
                        <h1>THE 100</h1>
                        <div className="desc">
                            <h5>PICK UP WHERE YOU LEFT OFF</h5>
                            <p>
                                Set ninety-seven years after a nuclear war
                                has destroyed civilization, when a spaceship
                                housing humanity's lone survivors sends one
                                hundred juvenile delinquents back to Earth,
                                in hopes of possibly re-populating the planet.
                            </p>
                        </div>
                        <button>PLAY<FontAwesomeIcon icon="play"/></button>
                    </div>
                    <ProgressBar id="1"/>
                </section>
                <section className="libraries">
                    <div className="recommended">
                        <h1>RECOMMENDED</h1>
                        <div className="cards">
                            <Card id="1"/>
                            <Card id="2"/>
                            <Card id="3"/>
                            <Card id="4"/>
                            <Card id="5"/>
                            <Card id="6"/>
                            <Card id="7"/>
                            <Card id="8"/>
                            <Card id="9"/>
                            <Card id="10"/>
                        </div>
                    </div>
                </section>
            </main>
        );
    }
}

export default Main;
