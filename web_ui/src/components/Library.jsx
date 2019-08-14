import React, { Component } from "react";
import Card from "./Card.jsx";

class Library extends Component {
    constructor(props) {
        super(props);

        this.state = {
            cards: [],
        };
    }

    async componentDidMount() {
        const cardReq = await fetch(this.props.url);
        const json = await cardReq.json();

        try {
            const cards = json.map(item => <Card key={item.id} data={item} src={item.poster_path}/>);
            this.setState({ cards });
        } catch (e) { }

        // * MULTIPLE SECTIONS
        // const sections = await cardReq.json();

        // for (const section in sections) {
        //     const cards = section.map(item => <Card key={item.id} data={item} src={item.poster_path}/>);

        //     this.setState({
        //         cards: {
        //             [section]: cards
        //         },
        //     });
        // }

    }

    render() {
        const { cards } = this.state;

        cards.length = 20;

        // * MULTIPLE SECTIONS
        // const sections = Object.keys(sections).map(section => {
        //     return (
        //         <section key={section}>
        //             <h1>{section}</h1>
        //             <div className="cards">
        //                 {cards[section].length !== 0
        //                     ? cards[section]
        //                     : (
        //                         <div className="empty">
        //                             <p>CURRENTLY EMPTY</p>
        //                         </div>
        //                     )
        //                 }
        //             </div>
        //         </section>
        //     );
        // });

        return (
            <div className="library">
                <div className="cards">
                    { cards }
                </div>
            </div>
        );
    }
}

export default Library;
