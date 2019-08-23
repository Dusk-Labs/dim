import React, { Component } from "react";
import Card from "../components/library/Card.jsx";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faQuestionCircle } from '@fortawesome/free-regular-svg-icons';

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
            if (payload[section].length > 0) {
                const card_list = payload[section].map((card, i) => <Card key={i} data={card}/>);
                sections[section] = card_list;
            }
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
                        { cards[section] }
                    </div>
                </section>
            );
        });

        return (
            <div className="library">
                { sections.length > 0 
                    ? sections 
                    : (
                        <div className="empty">
                            <FontAwesomeIcon icon={faQuestionCircle}/>
                            <p> There is no content in this library.</p>
                        </div>
                    )
                }
            </div>
        );
    }
}

export default Library;
