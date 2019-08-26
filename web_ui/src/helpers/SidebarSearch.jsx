import React, { Component } from "react";
import { HashLink } from "react-router-hash-link";
import Modal from "react-modal";
import ContentEditable from "react-contenteditable";
import sanitizeHtml from "sanitize-html";

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
        // ! THIS NEEDS TO BE MOVED (CREATE REF ETC).
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
            "genre"
        ];

        this.setState({
            options: json
        });
    }

    componentWillUnmount() {
        document.removeEventListener("click");
        this.inputBox.current.removeEventListener("input");
    }

    // ! DOES NOT INSERT TAG INTO INPUT
    selectOption(option) {
        if (this.state.selectedOptions.includes(option)) return;

        let { options } = this.state;
        const index = options.indexOf(option);

        if (index !== -1) {
            options.splice(index, 1);
        }

        this.setState(prevState => ({
            selectedOptions: [...prevState.selectedOptions, option]
        }));
    }

    // ! DOES NOT REMOVE TAG INTO INPUT
    unselectOption(option) {
        if (this.state.options.includes(option)) return;

        let { selectedOptions } = this.state;
        const index = selectedOptions.indexOf(option);

        if (index !== -1) {
            selectedOptions.splice(index, 1);
        }

        this.setState(prevState => ({
            options: [...prevState.options, option]
        }));
    }

    // ! MISSING MANUAL TAG DELETION
    // ! STYLING GETS LOST ON UNFOCUS.
    onChange = async (e) => {
        try {
            let query = e.target.value

            // tag: (for styling and adding/removing from selectedOptions)
            const findTag = RegExp(/([A-z0-9]{1,}:)/gi);

            // tag: value
            const findTagValue = RegExp(/([A-z0-9]{1,}:)[ ]?([A-z0-9 ]{1,})/gi);

            if (findTag.test(query)) {
                const regex = /(<([^>]+)>)/ig;
                query = query.replace(regex, "");
            }

            if (query.match(findTag)) {
                const [ tag ] = query.match(findTag);
                const tag_name = tag.replace(":", "");
                const available = this.state.options.includes(tag_name) || this.state.selectedOptions.includes(tag_name);

                if (available) {
                    query = query.replace(findTag, `<span>${tag}</span>`);
                    this.selectOption(tag_name);
                }
            }

            this.setState({
                query,
                showSearchFor: e.target.value.length !== 0,
                showOptions: e.target.value.length === 0
            });

            if (this.state.query.length === 0) return;

            // ! UN-COMMENT WHEN FUNCTIONING
            // const reqResults = await fetch(`http://86.21.150.167:8000/api/v1/search?query=${this.state.query}&quick=true`);
            // const results = await reqResults.json();

            // this.setState({
            //     results
            // });
        } catch {}
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

    sanitize = () => {
        this.setState({ query: sanitizeHtml(this.state.query)});
    };

    render() {
        const availableOptions = this.state.options.length > 0
            ? (this.state.options.map((option, i) => {
                return (
                    <div key={i} className="option" onClick={() => this.selectOption(option)}>
                        <p>{option}:</p>
                        <FontAwesomeIcon icon="plus"/>
                    </div>)
                })
            ) : <p id="response">NO MORE OPTIONS</p>;

        const selectedOptions = this.state.selectedOptions.length > 0
            ? (this.state.selectedOptions.map((option, i) => {
                return (
                    <div key={i} className="option" onClick={() => this.unselectOption(option)}>
                        <p>{option}:</p>
                        <FontAwesomeIcon icon="minus"/>
                    </div>)
                })
            ) : <p id="response">NONE ADDED</p>;

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
                                key={i}>
                                {name}
                            </HashLink>
                        );
                    })}
                </div>
            </div>
        );

        return (
            <div className="search-box" ref={this.searchBox}>
                <div className="search-box-wrapper">
                    <ContentEditable
                        className="input"
                        innerRef={this.inputBox}
                        html={this.state.query}
                        onChange={this.onChange}
                        onBlur={this.sanitize}/>
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
                            { availableOptions }
                        </div>
                        <div className="options">
                            { selectedOptions }
                        </div>
                    </div>}
                {this.state.showSearchFor &&
                    <div className="search-box-search-for">
                        <p>SEARCH FOR: <span id="query">{this.state.query}</span></p>
                        { results }
                    </div>}
            </div>
        );
    }
}

export default SidebarSearch;
