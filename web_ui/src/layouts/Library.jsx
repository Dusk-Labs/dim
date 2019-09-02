import React, { Component } from "react";
import Card from "../components/library/Card.jsx";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

class Library extends Component {
    constructor(props) {
        super(props);

        this.state = {
            cards: <div className="spinner"></div>
        };
    }

    async componentDidMount() {
        if (this.props.cards === undefined) {
            return this.fetchCards();
        } else {
            const card_list = this.props.cards.map((card, i) => <Card key={i} data={card}/>);

            this.setState({
                cards: (
                    <div className="cards">
                        { card_list }
                    </div>
                )
            })
        }
    }

    async componentDidUpdate(prevProps) {
        if (this.props.path !== prevProps.path) {
            this.fetchCards();
        }
    }

    async handle_req(promise) {
        try {
            return await (await promise).json();
        } catch (err) {
            return { err: err };
        }
    }

    fetchCards = async () => {
        const req = fetch(this.props.path);
        const payload = await this.handle_req(req);

        if (payload.err) {
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

export default Library;
