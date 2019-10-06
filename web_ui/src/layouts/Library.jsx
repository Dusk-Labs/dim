import React, { Component } from "react";
import Card from "../components/library/Card.jsx";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

class Library extends Component {
    constructor(props) {
        super(props);


        this.state = {
            cards: {},
        };
    }

    async componentDidMount() {
        this.props.cards !== undefined
            ? this.getPropCards()
            : this.getPathCards();

        if (this.props.id !== undefined)
            this.mount_websocket();
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

    mount_websocket() {
        window.library = this;
        this.websocket = new WebSocket(`ws://86.21.150.167:3012/events/library/${this.props.id}`);
        this.websocket.addEventListener('message', this.handle_ws_msg);
    }

    handle_ws_msg = async (event) => {
        let msg = JSON.parse(event.data);
        if (msg.res !== `/events/library/${this.props.id}`)
            return;
        if (msg.message.event_type.type === "EventNewCard") {
            let card_data = await this.handle_req(fetch(`http://86.21.150.167:8000/api/v1/media/${msg.message.id}`));

            if (card_data.err === undefined) {
                const key = Object.keys(this.state.cards)[0];
                this.setState({
                    cards: {key: Array.from(
                        new Set([...this.state.cards[key], card_data]
                            .map(JSON.stringify)))
                        .map(JSON.parse)
                        .sort((a, b) => {
                        let name_a = a.name.toUpperCase();
                        let name_b = b.name.toUpperCase();

                        if (name_a < name_b)
                            return -1;

                        if (name_a > name_b)
                            return 1;
                        return 0;
                    })
                }});
            }
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
        this.setState({
            cards: {"SECTION": this.props.cards}
        });
    }

    async getPathCards() {
        const req = fetch(this.props.path);
        const payload = await this.handle_req(req);

        if (payload.err) {
            return this.setState({
                error: true
            });
        }

        this.setState({
            cards: payload
        });
    };

    render() {
        const {cards, error} = this.state;
        let sections = {};

        // eslint-disable-next-line
        for (const section in cards) {
            if (cards[section].length > 0) {
                const card_list = cards[section].map((card, i) => <Card key={i} data={card}/>);
                sections[section] = card_list;
            }
        }

        const card_list = Object.keys(sections).map(section => {
            return (
                <section key={section}>
                    <h1>{section}</h1>
                    <div className="cards">
                        { sections[section] }
                    </div>
                </section>
            );
        });

        // Somehow fit this loading spinner in
        // <div className="spinner"></div>

        return (
            <div className="library">
                {error ? (
                    <div className="empty">
                        <FontAwesomeIcon icon="question-circle"/>
                        <p>FAILED TO LOAD</p>
                    </div>
                 ) : card_list.length > 0 ? (
                        card_list
                 ) : (
                    <div className="empty">
                        <FontAwesomeIcon icon="question-circle"/>
                        <p>There is no content in this library.</p>
                    </div>
                 )}
            </div>
        );
    }
}

export default Library;
