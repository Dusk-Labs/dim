import React, { Component } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import Card from "../components/library/Card.jsx";
import "./CardList.scss";

class CardList extends Component {
    constructor(props) {
        super(props);

        this.state = {
            cards: {},
            fetching: false,
            fetched: false,
            error: null
        };
    }

    async componentDidMount() {
        this.getCards();

        if (this.props.id) {
            this.mount_websocket();
        }
    }

    async componentDidUpdate(prevProps) {
        if (this.props.path !== prevProps.path) {
            return this.getCards();
        }

        if (this.props.cards) {
            if (this.props.cards.length !== prevProps.cards.length) {
                return this.getCards();
            }

            // eslint-disable-next-line
            for (const section in this.props.cards) {
                const equal = this.props.cards[section].every((card, i) => {
                    return card.id === prevProps.cards[section][i].id;
                });

                if (!equal) this.getCards();
            }
        }
    }

    mount_websocket() {
        window.library = this;

        this.websocket = new WebSocket(`ws://86.21.150.167:3012/events/library/${this.props.id}`);
        this.websocket.addEventListener("message", this.handle_ws_msg);
    }

    handle_ws_msg = async (event) => {
        const msg = JSON.parse(event.data);

        if (msg.res !== `/events/library/${this.props.id}`) return;

        if (msg.message.event_type.type === "EventNewCard") {
            const new_card = await this.handle_req(fetch(`http://86.21.150.167:8000/api/v1/media/${msg.message.id}`));

            if (!new_card.err) {
                const key = Object.keys(this.state.cards)[0];

                const newCardList = Array.from(
                    new Set([...this.state.cards[key], new_card])
                        .map(JSON.stringify)
                        .map(JSON.parse)
                        .sort((a, b) => {
                            let name_a = a.name.toUpperCase();
                            let name_b = b.name.toUpperCase();

                            if (name_a < name_b) return -1;
                            if (name_a > name_b) return 1;

                            return 0;
                        })
                );

                this.setState({
                    cards: {
                        key: newCardList
                    }
                });
            }
        }
    }

    async getCards() {
        this.setState({
            fetching: true
        });

        if (this.props.path) {
            const res = await fetch(this.props.path);

            if (res.status !== 200) {
                return this.setState({
                    fetching: false,
                    fetched: true,
                    error: true
                });
            }

            const payload = await res.json();

            this.setState({
                fetching: false,
                fetched: true,
                cards: payload
            });
        } else {
            this.setState({
                fetching: false,
                fetched: true,
                cards: this.props.cards
            })
        }
    }

    render() {
        let card_list;

        // FETCHING
        if (this.state.fetching) {
            return <div className="spinner"></div>
        }

        // ERR
        if (this.state.fetched && this.state.error) {
            card_list = (
                <div className="empty">
                    <FontAwesomeIcon icon="question-circle"/>
                    <p>FAILED TO LOAD</p>
                </div>
            );
        }

        // OK
        if (this.state.fetched && !this.state.error) {
            let cards = this.state.cards;

            // eslint-disable-next-line
            for (const section in cards) {
                if (cards[section].length > 0) {
                    cards[section] = cards[section].map((card, i) => <Card key={i} data={card}/>);
                }
            }

            card_list = Object.keys(cards).map(section => {
                return (
                    <section key={section}>
                        <h1>{section}</h1>
                        <div className="cards">
                            { cards[section] }
                        </div>
                    </section>
                );
            });
        }

        return (
            <div className="card_list">
                {card_list}
            </div>
        );
    }
}

export default CardList;
