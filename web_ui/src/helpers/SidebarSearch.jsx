import React, { Component } from "react";
import { NavLink } from "react-router-dom";
import Modal from "react-modal";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import "./SidebarSearch.scss";

Modal.setAppElement("body");

class SidebarSearch extends Component {
    constructor(props) {
        super(props);

        this.onChange = this.onChange.bind(this);
        this.toggleFocus = this.toggleFocus.bind(this);
        this.selectOption = this.selectOption.bind(this);

        this.state = {
            query: "",
            showSearchFor: false,
            showOptions: false,
            options: [],
            selectedOptions: []
        };
    }

    componentDidMount() {
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

    selectOption(option) {
        if (this.state.selectedOptions.includes(option)) return;

        this.setState({
            selectedOptions: [this.state.selectedOptions, option]
        });
    }

    onChange(event) {
        this.setState({
            query: event.target.value.substr(0, 20),
            showSearchFor: event.target.value.length !== 0,
            showOptions: event.target.value.length === 0
        });
    }

    toggleFocus() {
        if (this.state.query.length > 0) return;

        this.setState({
            showOptions: !this.state.showOptions
        });
    }

    render() {
        const { options } = this.state;

        return (
            <div className="search-box">
                <div className="search-box-wrapper">
                    <input
                        type="text"
                        placeholder="SEARCH"
                        value={this.state.query}
                        onChange={this.onChange}
                        onFocus={this.toggleFocus}/>
                    <button type="submit">
                        <FontAwesomeIcon icon="search"/>
                    </button>
                </div>
                {this.state.showOptions &&
                    <div className="search-box-options" onBlur={this.toggleFocus}>
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
                    </div>}
            </div>
        );
    }
}

export default SidebarSearch;
