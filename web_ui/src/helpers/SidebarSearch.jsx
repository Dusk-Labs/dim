import React, { Component } from "react";
import { HashLink } from 'react-router-hash-link';

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import "./SidebarSearch.scss";

class SidebarSearch extends Component {
    constructor(props) {
        super(props);

        this.searchBox = React.createRef();
        this.inputBox = React.createRef();

        this.onChange = this.onChange.bind(this);
        this.toggleShowSearchFor = this.toggleShowSearchFor.bind(this);
        this.handleDocumentClick = this.handleDocumentClick.bind(this);

        this.state = {
            query: "",
            showSearchFor: false,
            results: null,
            showPlaceholder: true
        };
    }

    componentDidMount() {
        document.addEventListener("click", this.handleDocumentClick);
        this.inputBox.current.addEventListener("input", this.onChange);
    }

    componentWillUnmount() {
        document.removeEventListener("click", this.handleDocumentClick);
        this.inputBox.current.removeEventListener("input", this.onChange);
    }

    async handle_req(promise) {
        try {
            return await (await promise).json();
        } catch (err) {
            return { err: err };
        }
    }

    handleDocumentClick(e) {
        // hides popups if clicked outside component
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
            showSearchFor: e.target.textContent.length !== 0,
            results: <p id="response">LOADING</p>
        });

        if (this.state.query.length >= 1) {
            this.getResults();

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

    async getResults() {
        const reqResults = fetch(`http://86.21.150.167:8000/api/v1/search?query=${this.state.query}&quick=true`);
        const results = await this.handle_req(reqResults);

        if (results.err) {
            return this.setState({
                results: <p id="response">FAILED TO LOAD</p>
            });
        }

        const list = results.map((
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

        this.setState({
            results: (
                <div className="results">
                    <p>{list.length} RESULTS</p>
                    <div className="result-list">{list}</div>
                </div>
            )
        });
    }

    toggleShowSearchFor() {
        if (this.state.query.length === 0) return;

        this.setState({
            showSearchFor: !this.state.showSearchFor
        });
    }

    // prevents multi-line (on enter)
    handleOnKeyDown = (e) => {
        if (e.which === 13) {
            e.preventDefault();
        }
    }

    render() {
        return (
            <div className="search-box" ref={this.searchBox}>
                <div className="search-box-wrapper">
                    <div className="input"
                        ref={this.inputBox}
                        onKeyDown={this.handleOnKeyDown}
                        contentEditable="true"
                        value={this.state.query}
                        onChange={this.onChange}>
                    </div>
                    {this.state.showPlaceholder &&
                        <span id="placeholder">SEARCH</span>
                    }
                    <button type="submit" onClick={this.onSubmit}>
                        <FontAwesomeIcon icon="search"/>
                    </button>
                </div>
                {this.state.showSearchFor &&
                    <div className="search-box-search-for">
                        <p>SEARCH FOR: <span id="query">{this.state.query}</span></p>
                        { this.state.results }
                    </div>}
            </div>
        );
    }
}

export default SidebarSearch;