import React, { Component } from "react";
import { connect } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { fetchCards } from "../actions/cardActions.js";
import Card from "../components/Card.jsx";
import "./CardList.scss";

class CardList extends Component {
    constructor(props) {
        super(props);

        this._isMounted = false;
        this.cardList = React.createRef();
    }

    componentDidMount() {
        this._isMounted = true;

        if (this.props.path) {
            return this.props.fetchCards(this.props.path);
        }

        if (this.props.id) {
            return this.mount_websocket();
        }
    }

    async componentDidUpdate(prevProps) {
        if (this.props.path) {
            if (this.props.path !== prevProps.path) {
                return this.props.fetchCards(this.props.path);
            }
        }
    }

    componentWillUnmount() {
        this._isMounted = false;
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

    render() {
        const cards = [];
        let cardCount = 0;
        let card_list;

        if (this.cardList.current) {
            cardCount = Math.floor(this.cardList.current.offsetWidth / 240) * 2;
        }

        for (let x = 0; x < cardCount; x++) {
            cards.push(
                <div key={x} className="card-wrapper" style={{overflow: "hidden"}}>
                    <div className="card">
                        <div className="placeholder"/>
                    </div>
                </div>
            );
        }

        // FETCH_CARDS_START
        if (this.props.cards.fetching) {
            card_list = (
                <section>
                    <div className="placeholder-name">
                        <div className="placeholder-text"/>
                    </div>
                    <div className="cards">{cards}</div>
                </section>
            );
        }

        // FETCH_CARDS_ERR
        if (this.props.cards.fetched && this.props.cards.error) {
            card_list = (
                <section>
                    <div className="placeholder-name">
                        <div className="placeholder-text"/>
                            <div className="horizontal-err">
                                <FontAwesomeIcon icon="times-circle"/>
                                <p>FAILED TO LOAD CARDS</p>
                            </div>
                    </div>
                    <div className="cards">{cards}</div>
                </section>
            );
        }

        // FETCH_CARDS_OK
        if (this.props.cards.fetched && !this.props.cards.error) {
            const { items } = this.props.cards;
            let sections = {};

            // eslint-disable-next-line
            for (const section in items) {
                if (items[section].length > 0) {
                    sections[section] = (
                        items[section].map((card, i) => <Card key={i} data={card}/>)
                    );
                }
            }

            if (Object.keys(sections).length === 0) {
                card_list = (
                    <section>
                        <div className="placeholder-name">
                            <div className="placeholder-text"/>
                            <div className="horizontal-err">
                                <FontAwesomeIcon icon="times-circle"/>
                                <p>NO MEDIA HAS BEEN FOUND</p>
                            </div>
                        </div>
                        <div className="cards">{cards}</div>
                    </section>
                );
            } else {
                card_list = Object.keys(sections).map(section => (
                    <section key={section}>
                        <h1>{section}</h1>
                        <div className="cards">
                            {sections[section]}
                        </div>
                    </section>
                ));
            }
        }

        return <div className="card_list" ref={this.cardList}>{card_list}</div>;
    }
}

const mapStateToProps = (state) => ({
    cards: state.cardReducer.fetch_cards
});

const mapActionsToProps = { fetchCards };

export default connect(mapStateToProps, mapActionsToProps)(CardList);
