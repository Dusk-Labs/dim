import React, { Component } from "react";
import { connect } from "react-redux";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { fetchCards } from "../actions/card.js";
import Card from "./Card.jsx";
import "./CardList.scss";

class CardList extends Component {
    constructor(props) {
        super(props);

        this.handleWS = this.handleWS.bind(this);

        this._isMounted = false;
        this.cardList = React.createRef();
    }

    componentDidMount() {
        this._isMounted = true;

        if (this.props.path) {
            this.props.fetchCards(this.props.auth.token, this.props.path);
        }

        if (window.location.protocol !== "https:") {
            this.library_ws = new WebSocket(`ws://${window.host}:3012/events/library`);
            this.library_ws.addEventListener("message", this.handleWS);
        }
    }

    componentWillUnmount() {
        this._isMounted = false;

        this.library_ws.removeEventListener("message", this.handle_ws_msg);
        this.library_ws.close();
    }

    componentDidUpdate(prevProps) {
        if (this.props.path) {
            if (this.props.path !== prevProps.path) {
                return this.props.fetchCards(this.props.auth.token, this.props.path);
            }
        }
    }

    handleWS(event) {
        const { type }= JSON.parse(event.data);

        if (type === "EventRemoveLibrary") {
            this.props.fetchCards(this.props.auth.token, this.props.path);
        }

        if (type === "EventNewLibrary") {
            this.props.fetchCards(this.props.auth.token, this.props.path);
        }

        if (type === "EventNewCard") {
            this.props.fetchCards(this.props.auth.token, this.props.path);
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
            console.log("ERR", this.props.cards.error);
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
    auth: state.auth,
    cards: state.card.cards
});

const mapActionsToProps = { fetchCards };

export default connect(mapStateToProps, mapActionsToProps)(CardList);
