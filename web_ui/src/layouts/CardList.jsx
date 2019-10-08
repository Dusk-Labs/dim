import React, { Component } from "react";
import Card from "../components/library/Card.jsx";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

class CardList extends Component {
    constructor(props) {
        super(props);

        this.state = {
            cards: <div className="spinner"></div>
        };
    }

    async componentDidMount() {
        this.props.cards !== undefined
            ? this.getPropCards()
            : this.getPathCards();
    }

    async componentDidUpdate(prevProps) {
        if (this.props.path !== prevProps.path) {
            return this.getPathCards();
        }

        if (this.props.cards) {
            if (this.props.cards.length !== prevProps.cards.length) {
                return this.getPropCards();
            }

            const equal = this.props.cards.every((card, i) => {
                return card.id === prevProps.cards[i].id;
            });

            if (!equal) this.getPropCards();
        }
    }

    async handle_req(promise) {
        try {
            return await (await promise).json();
        } catch (err) {
            return { err: err };
        }
    }

    getPropCards() {
        const card_list = this.props.cards.map((card, i) => <Card key={i} data={card}/>);

        this.setState({
            cards: (
                <div className="cards">
                    { card_list }
                </div>
            )
        });
    }

    async getPathCards() {
        const req = fetch(this.props.path);
        const payload = await this.handle_req(req);

        if (payload.err || payload.error) {
            return this.setState({
                cards: (
                    <div className="empty">
                        <FontAwesomeIcon icon="question-circle"/>
                        <p>FAILED TO LOAD</p>
                    </div>
                )
            });
        }

        let sections = {};

        // eslint-disable-next-line
        for (const section in payload) {
            if (payload[section].length > 0) {
                const card_list = payload[section].map((card, i) => <Card key={i} data={card}/>);
                sections[section] = card_list;
            }
        }

        const cards = Object.keys(sections).map(section => {
            return (
                <section key={section}>
                    <h1>{section}</h1>
                    <div className="cards">
                        { sections[section] }
                    </div>
                </section>
            );
        });

        this.setState({
            cards: (
                cards.length > 0
                ? cards
                : (
                    <div className="empty">
                        <FontAwesomeIcon icon="question-circle"/>
                        <p>There is no content in this library.</p>
                    </div>
                )
            )
        });
    };

    render() {
        return (
            <div className="library">
                {this.state.cards}
            </div>
        );
    }
}

export default CardList;
