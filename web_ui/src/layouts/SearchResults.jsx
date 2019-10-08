import React, { Component } from "react";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

import CardList from "./CardList.jsx";

class SearchResults extends Component {
    constructor(props) {
        super(props);

        this.state = {
            cards: <div className="spinner"></div>
        };
    }

    componentDidMount() {
        this.getResults();
    }

    componentDidUpdate(prevProps) {
        if (this.props.location.search !== prevProps.location.search) {
            this.getResults();
        }
    }

    async getResults() {
        const searchURL = new URLSearchParams(this.props.location.search);
        const query = searchURL.get("query");

        if (query.length === 0) {
            return this.setState({
                cards: (
                    <div className="empty">
                        <FontAwesomeIcon icon="question-circle"/>
                        <p>NO SEARCH PROVIDED</p>
                    </div>
                )
            });
        };

        const reqResults = fetch(`http://86.21.150.167:8000/api/v1/search?query=${query}`);
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
            cards: (
                results.length > 0
                    ? <CardList cards={results}/>
                    : (
                        <div className="empty">
                            <FontAwesomeIcon icon="question-circle"/>
                            <p>NO RESULTS FOUND</p>
                        </div>
                    )
            )
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
