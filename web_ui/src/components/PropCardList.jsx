import React, { Component } from "react";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

import Card from "./Card.jsx";

class PropCardList extends Component {
    constructor(props) {
        super(props);

        this.cardList = React.createRef();
    }

    render() {
        const cards = [];
        let card_list;
        let cardCount = 0;

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

        // START
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

        // ERR
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

        // OK
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

export default PropCardList;