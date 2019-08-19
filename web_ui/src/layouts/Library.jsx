import React, { Component } from "react";
import Card from "../components/library/Card.jsx";

class Library extends Component {
    constructor(props) {
        super(props);

        this.state = {
            cards: [],
        };
    }

    async componentDidMount() {
        // const { id } = this.props.match.params;
        const req = await fetch("http://86.21.150.167:8000/api/v1/library/2/media");
        const payload = await req.json();

        const cards = payload.map((card, i) => <Card key={i} data={card}/>);
        this.setState({ cards });

        // * MULTIPLE SECTIONS
        // const sections = await cardReq.json();

        // for (const section in sections) {
        //     const cards = section.map((card, i) => <Card key={i} data={card}/>);

        //     this.setState({
        //         cards: {
        //             [section]: cards
        //         },
        //     });
        // }

    }

    render() {
        const { cards } = this.state;

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
