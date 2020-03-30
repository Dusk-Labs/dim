import React, { Component } from "react";
import { connect } from "react-redux";

import { search } from "../actions/search.js";

import PropCardList from "../Components/PropCardList.jsx";

class SearchResults extends Component {
    componentDidMount() {
        document.title = "Dim - Results";
        this.getResults();
    }

    componentDidUpdate(prevProps) {
        if (this.props.location.search !== prevProps.location.search) {
            this.getResults(this.props.auth.token);
        }
    }

    getResults() {
        const { query } = this.props.match.params;

        if (query.length > 0) {
            document.title = `Dim - Results for '${query}'`;
            this.props.search(query);
        }
    }

    render() {
        return <PropCardList cards={this.props.searchList}/>;
    }
}

const mapStateToProps = (state) => ({
    auth: state.authReducer,
    searchList: state.searchReducer.search
});

const mapActionsToProps = { search };

export default connect(mapStateToProps, mapActionsToProps)(SearchResults);
