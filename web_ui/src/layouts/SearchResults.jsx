import React, { Component, Fragment } from "react";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

import CardList from "./CardList.jsx";

class SearchResults extends Component {
    constructor(props) {
        super(props);

        this.state = {
            cards: {},
            fetching: false,
            fetched: false,
            error: null
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
        this.setState({
            fetching: true
        });

        const searchURL = new URLSearchParams(this.props.location.search);

        let params = '';

        // eslint-disable-next-line
        for (const key of searchURL.keys()) {
            if (searchURL.get(key) !== undefined) {
                params += `${key}=${searchURL.get(key)}&`;
            }
        }

        if (params.length === 0) {
            return this.setState({
                fetching: false,
                fetched: true,
                error: true
            });
        };

        const res = await fetch(`http://86.21.150.167:8000/api/v1/search?${params}`);

        if (res.status !== 200) {
            return this.setState({
                fetching: false,
                fetched: true,
                error: true
            });
        }

        const results = await res.json();

        this.setState({
            fetching: false,
            fetched: true,
            cards: results
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
        let cards = <Fragment/>;

        // FETCHING
        if (this.state.fetching) {
            cards = <div className="spinner"></div>;
        }

        // ERR
        if (this.state.fetched && this.state.error) {
            cards = (
                <div className="empty">
                    <FontAwesomeIcon icon="question-circle"/>
                    <p>FAILED TO LOAD</p>
                </div>
            );
        }

        // OK
        if (this.state.fetched && !this.state.error) {
            cards = this.state.cards.length > 0
                ? <CardList cards={{"RESULTS": this.state.cards}}/>
                : (
                    <div className="empty">
                        <FontAwesomeIcon icon="question-circle"/>
                        <p>NO RESULTS FOUND</p>
                    </div>
                )
        }

        return cards;
    }
}

export default SearchResults;
