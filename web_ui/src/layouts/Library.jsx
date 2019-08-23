import React, { Component } from "react";
import Card from "../components/library/Card.jsx";
import { NavLink } from "react-router-dom";
import FloatingBar from "../helpers/FloatingBar.jsx";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

class Library extends Component {
    constructor(props) {
        super(props);

        this.state = {
            cards: {}
        };
    }

    async componentDidMount() {
        this.fetchCards();
    }

    fetchCards = async () => {
        const req = await fetch(this.props.path);
        const payload = await req.json();
        let sections = {};

        // eslint-disable-next-line
        for (const section in payload) {
            const card_list = payload[section].map((card, i) => <Card key={i} data={card}/>);
            sections[section] = card_list;
        }

        this.setState({
            cards: sections
        });
    };

    async componentDidUpdate(prevProps) {
        if (this.props.path !== prevProps.path) {
            this.fetchCards();
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
                <FloatingBar>
                    <NavLink to="/">
                        <FontAwesomeIcon icon="home"/>
                    </NavLink>
                </FloatingBar>
                { sections }
            </div>
        );
    }
}

export default Library;
