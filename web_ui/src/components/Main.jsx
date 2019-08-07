import React, { Component } from "react";
import Card from "./Card.jsx";
import "./main.css";

class Main extends Component {
    render() {
        return (
            <main>
                <section className="banner">
                    <img alt="banner" src="https://pbs.twimg.com/media/DBVhqetV0AE_yEd.jpg"></img>
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
                </section>
                <section className="libraries">
                    <div className="recommended">
                        <h1>RECOMMENDED</h1>
                        <div className="cards">
                            <Card
                                name="Spider-Man: Far From Home"
                                src="https://images.mymovies.net/images/film/cin/350x522/fid19163.jpg"
                            ></Card>
                            <Card
                                name="Men in Black: International"
                                src="https://images.mymovies.net/images/film/cin/350x522/fid19167.jpg"
                            ></Card>
                            <Card
                                name="Fast & Furious Presents: Hobbs & Shaw"
                                src="https://images.mymovies.net/images/film/cin/350x522/fid19250.jpg"
                            ></Card>
                            <Card
                                name="The Lion King"
                                src="https://images.mymovies.net/images/film/cin/350x522/fid19112.jpg"
                            ></Card>
                            <Card
                                name="Toy Story 4"
                                src="https://images.mymovies.net/images/film/cin/350x522/fid19080.jpg"
                            ></Card>
                            <Card
                                name="The Angry Birds 2"
                                src="https://images.mymovies.net/images/film/cin/350x522/fid19400.jpg"
                            ></Card>
                            <Card
                                name="Annabelle Comes Home"
                                src="https://images.mymovies.net/images/film/cin/350x522/fid19409.jpg"
                            ></Card>
                            <Card
                                name="Playmobil: The Movie"
                                src="https://images.mymovies.net/images/film/cin/350x522/fid19148.jpg"
                            ></Card>
                            <Card
                                name="The Sun is Also a Star"
                                src="https://images.mymovies.net/images/film/cin/350x522/fid19267.jpg"
                            ></Card>
                            <Card
                                name="Alladin"
                                src="https://images.mymovies.net/images/film/cin/350x522/fid18993.jpg"
                            ></Card>
                        </div>
                    </div>
                </section>
            </main>
        );
    }
}

export default Main;