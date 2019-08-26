import React, { Component } from "react";
import { HashLink } from 'react-router-hash-link';
import Modal from "react-modal";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import "./SidebarSearch.scss";

Modal.setAppElement("body");

class SidebarSearch extends Component {
    constructor(props) {
        super(props);

        this.searchBox = React.createRef();
        this.inputBox = React.createRef();

        this.selectOption = this.selectOption.bind(this);
        this.onChange = this.onChange.bind(this);
        this.toggleShowOptions = this.toggleShowOptions.bind(this);
        this.toggleShowSearchFor = this.toggleShowSearchFor.bind(this);

        this.state = {
            query: "",
            showSearchFor: false,
            showOptions: false,
            options: [],
            selectedOptions: [],
            results: []
        };
    }

    componentDidMount() {
        // HIDES POPUPS IF CLICKED OUTSIDE COMPONENT
        document.addEventListener("click", (e) => {
            if (!this.searchBox.current.contains(e.target)) {
                if (this.state.showOptions) {
                    this.setState({
                        showOptions: false
                    });
                }

                if (this.state.showSearchFor) {
                    this.setState({
                        showSearchFor: false
                    });
                }
            }
        });

        this.inputBox.current.addEventListener("input", this.onChange.bind(this));

        const json = [
            "name",
            "genre",
            "year",
            "rating"
        ];

        const options = json.map((option, i) => {
            return (
                <div key={i} className="option" onClick={() => this.selectOption(option)}>
                    <p>{option}:</p>
                    <FontAwesomeIcon icon="plus"/>
                </div>
            )
        });

        this.setState({
            options
        });
    }

    componentWillUnmount() {
        document.removeEventListener("click");
        this.inputBox.current.removeEventListener("input");
    }

    selectOption(option) {
        if (this.state.selectedOptions.includes(option)) return;

        this.setState(prevState => ({
            selectedOptions: [...prevState.selectedOptions, option]
        }));
    }

    onChange = async (e) => {
        this.setState({
            query: e.target.textContent.substr(0, 20),
            showSearchFor: e.target.textContent.length !== 0,
            showOptions: e.target.textContent.length === 0
        });

        if (this.state.query.length === 0) return;

        const reqResults = await fetch(`http://86.21.150.167:8000/api/v1/search?query=${this.state.query}&quick=true`);
        const results = await reqResults.json();

        this.setState({
            results
        });
    }

    toggleShowOptions() {
        if (this.state.query.length > 0) return;

        this.setState({
            showOptions: !this.state.showOptions
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
        const { options } = this.state;

        const results = (
            <div className="results">
                <p>{this.state.results.length} RESULTS</p>
                <div className="result-list">
                    {this.state.results.map((
                        { name, library_id, id }, i
                    ) => {
                        return (
                            <HashLink
                                to={`/library/${library_id}#${id}`}
                                scroll={el => {
                                    el.scrollIntoView({ behavior: "smooth", block: "center" });
                                    el.style.animation = "cardGlow 3s ease-in-out infinite";
                                }}
                                onClick={this.toggleShowSearchFor}
                                key={i}>{name}</HashLink>
                        );
                    })}
                </div>
            </div>
        );

        return (
            <div className="search-box" ref={this.searchBox}>
                <div className="search-box-wrapper">
                    <div className="input"
                        ref={this.inputBox}
                        onKeyDown={this.handleOnKeyDown}
                        contentEditable="true"
                        value={this.state.query}
                        onChange={this.onChange}
                        onFocus={this.toggleShowOptions}>
                    </div>
                    <button type="submit" onClick={this.onSubmit}>
                        <FontAwesomeIcon icon="search"/>
                    </button>
                </div>
                {this.state.showOptions &&
                    <div className="search-box-options">
                        <header>
                            <p>SEARCH OPTIONS</p>
                        </header>
                        <div className="options">
                            { options }
                        </div>
                    </div>}
                {this.state.showSearchFor &&
                    <div className="search-box-search-for">
                        <p>SEARCH FOR: <span id="query">{this.state.query}</span></p>
                        { this.state.selectedOptions.map((option, i) => <p key={i}>{option}</p>)}
                        { results }
                    </div>}
            </div>
        );
    }
}

export default SidebarSearch;
