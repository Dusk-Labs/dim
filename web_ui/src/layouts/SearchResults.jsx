import React, { Component } from "react";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

import Library from "./Library.jsx";

class SearchResults extends Component {
    constructor(props) {
        super(props);

        this.state = {
            cards: <div className="spinner"></div>,
        };
    }

    async componentDidMount() {
        this.getResults();
    }

    async getResults() {
        const query = new URLSearchParams(this.props.location.search);

        const reqResults = fetch(`http://86.21.150.167:8000/api/v1/search?query=${query.get("query")}`);
        const results = await this.handle_req(reqResults);

        if (results.err) {
            return this.setState({
                cards: (
                    <div className="empty">
                        <FontAwesomeIcon icon="question-circle"/>
                        <p>FAILED TO LOAD</p>
                    </div>
                )
            });
        }

        this.setState({
            cards: <Library cards={results}/>
        });
    }

    async handle_req(promise) {
        try {
            return await (await promise).json();
        } catch (err) {
            return { err: err };
        }
    }

    render() {
        return (
            <section>
                <h1>RESULTS</h1>
                {this.state.cards}
            </section>
        );
    }
}

export default SearchResults;
