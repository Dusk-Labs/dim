import React, { Component } from "react";
import { connect } from "react-redux";
import { HashLink } from 'react-router-hash-link';
import { withRouter } from "react-router-dom";

import { quickSearch } from "../actions/search.js";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import "./SidebarSearch.scss";

class SidebarSearch extends Component {
    constructor(props) {
        super(props);

        this.searchBox = React.createRef();
        this.inputBox = React.createRef();

        this.onChange = this.onChange.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);
        this.toggleShowSearchFor = this.toggleShowSearchFor.bind(this);
        this.handleDocumentClick = this.handleDocumentClick.bind(this);
        this.handleKeyPress = this.handleKeyPress.bind(this);

        this.state = {
            query: "",
            showSearchFor: false,
            showPlaceholder: true
        };
    }

    componentDidMount() {
        document.addEventListener("click", this.handleDocumentClick);
        this.inputBox.current.addEventListener("input", this.onChange);
        this.inputBox.current.addEventListener("keydown", this.handleKeyPress);
    }

    componentWillUnmount() {
        document.removeEventListener("click", this.handleDocumentClick);
        this.inputBox.current.removeEventListener("input", this.onChange);
        this.inputBox.current.removeEventListener("keydown", this.handleKeyPress);
    }

    // hides popups if clicked outside component
    handleDocumentClick(e) {
        if (!this.searchBox.current.contains(e.target)) {
            if (this.state.showSearchFor) {
                this.setState({
                    showSearchFor: false
                });
            }
        }
    }

    onChange = (e) => {
        this.setState({
            query: e.target.textContent.substr(0, 20),
            showSearchFor: e.target.textContent.length !== 0
        });

        if (this.state.query.length >= 1) {
            this.props.quickSearch(this.state.query, this.props.auth.token);

            this.setState({
                showPlaceholder: false
            });
        } else {
            this.setState({
                showPlaceholder: true
            })
        }
    }

    togglePlaceholder() {
        this.setState({
            showPlaceholder: !this.state.showPlaceholder
        });
    }

    toggleShowSearchFor() {
        if (this.state.query.length === 0) return;

        this.setState({
            showSearchFor: !this.state.showSearchFor
        });
    }

    handleKeyPress(e) {
        if (e.keyCode === 13) {
            e.preventDefault();
            this.handleSubmit();
        }
    }

    handleSubmit() {
        if (this.state.query.length >= 1) {
            this.props.history.push(`/search/${this.state.query}`);
        }
    }

    render() {
        let results = <p></p>;

        // SEARCH_START
        if (this.props.quick_search.fetching) {
            results = (
                <div className="horizontal-err">
                    <p>LOADING</p>
                </div>
            );
        }

        // SEARCH_ERR
        if (this.props.quick_search.fetched && this.props.quick_search.error) {
            results = (
                <div className="horizontal-err">
                    <FontAwesomeIcon icon="times-circle"/>
                    <p>FAILED TO LOAD</p>
                </div>
            );
        }

        // SEARCH_OK
        if (this.props.quick_search.fetched && !this.props.quick_search.error) {
            const list = this.props.quick_search.items.map((
                { name, library_id, id }, i
            ) => <HashLink
                    to={`/library/${library_id}#${id}`}
                    scroll={elm => {
                        elm.scrollIntoView({ behavior: "smooth", block: "center" });
                        elm.style.animation = "cardGlow 3s ease-in-out infinite";
                    }}
                    onClick={this.toggleShowSearchFor}
                    key={i}>
                        {name}
                </HashLink>
            );

            results = (
                <div className="results">
                    <p>{list.length} {list.length === 1 ? "RESULT" : "RESULTS"}</p>
                    <div className="result-list">{list}</div>
                </div>
            );
        }

        return (
            <div className="search-box" ref={this.searchBox}>
                <div className="search-box-wrapper">
                    <div className="input"
                        ref={this.inputBox}
                        onKeyDown={this.handleKeyPress}
                        contentEditable="true"
                        value={this.state.query}
                        onChange={this.onChange}
                        autoComplete="off"
                        autoCorrect="off"
                        autoCapitalize="off"
                        spellCheck="false">
                    </div>
                    {this.state.showPlaceholder &&
                        <span id="placeholder">SEARCH</span>
                    }
                    <button onClick={this.handleSubmit}>
                        <FontAwesomeIcon icon="search"/>
                    </button>
                </div>
                {this.state.showSearchFor &&
                    <div className="search-box-search-for">
                        <p>SEARCH FOR: <span id="query">{this.state.query}</span></p>
                        {results}
                    </div>
                }
            </div>
        );
    }
}

const mapStateToProps = (state) => ({
    auth: state.authReducer,
    quick_search: state.searchReducer.quick_search
});

const mapActionsToProps = { quickSearch };

export default connect(mapStateToProps, mapActionsToProps)(withRouter(SidebarSearch));
