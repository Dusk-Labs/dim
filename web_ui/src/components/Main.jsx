import React, { Component } from "react";
import Card from "./Card.jsx";
import LazyImage from "./helpers/LazyImage.jsx";
import ProgressBar from "./progress-bar.jsx";
import "./main.scss";

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
                        <button>PLAY<i className="fas fa-play"></i></button>
                    </div>
                    <ProgressBar/>
                </section>
                <section className="libraries">
                    <div className="recommended">
                        <h1>RECOMMENDED</h1>
                        <div className="cards">
                            <Card id="1"/>
                            <Card id="1"/>
                            <Card id="1"/>
                            <Card id="1"/>
                            <Card id="1"/>
                            <Card id="1"/>
                            <Card id="1"/>
                            <Card id="1"/>
                            <Card id="1"/>
                            <Card id="1"/>
                        </div>
                    </div>
                </section>
            </main>
        );
    }
}

export default Main;
