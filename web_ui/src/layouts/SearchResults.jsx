import React, { Component, Fragment } from "react";
import { connect } from "react-redux";

import { search } from "../actions/searchActions.js";
import PropCardList from "./PropCardList.jsx";

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
        const searchURL = new URLSearchParams(this.props.location.search);

        let params = '';

        // eslint-disable-next-line
        for (const key of searchURL.keys()) {
            if (searchURL.get(key) !== undefined) {
                params += `${key}=${searchURL.get(key)}&`;
            }
        }

        if (params.length > 0) {
            document.title = `Dim - Results for '${searchURL.get("query")}'`;
            this.props.search(params);
        }
    }

    render() {
        return <PropCardList cards={this.props.searchList}/>
    }
}

const mapStateToProps = (state) => ({
    auth: state.authReducer,
    searchList: state.searchReducer.search
});

const mapActionsToProps = { search };

export default connect(mapStateToProps, mapActionsToProps)(SearchResults);
