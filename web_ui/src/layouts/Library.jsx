import React, { Component } from "react";
import Card from "../components/library/Card.jsx";

class Library extends Component {
    constructor(props) {
        super(props);

        this.state = {
            cards: {},
        };
    }

    async componentDidMount() {
        this.fetchCards(this.props.path);
    }

    fetchCards = async (path) => {
        const req = await fetch(path);
        const sections = await req.json();

        for (const section in sections) {
            const cards = sections[section].map((card, i) => <Card key={i} data={card}/>);

            this.setState(prevState => ({
                cards: {[section]: cards}
            }));
        }
    }

    async componentDidUpdate(prevProps) {
        if (this.props.path !== prevProps.path) {
            this.fetchCards(this.props.path);
        }
    }

    render() {
        const { cards } = this.state;

        // * MULTIPLE SECTIONS
        const sections = Object.keys(cards).map(section => {
            return (
                <section key={section}>
                    <h1>{section}</h1>
                    <div className="cards">
                        {cards[section].length !== 0
                            ? cards[section]
                            : (
                                <div className="empty">
                                    <p>CURRENTLY EMPTY</p>
                                </div>
                            )
                        }
                    </div>
                </section>
            );
        });

        return (
            <div className="library">
                { sections }
            </div>
        );
    }
}

export default Library;
