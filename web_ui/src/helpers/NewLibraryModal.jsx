import React, { Component, Fragment } from "react";
import Modal from "react-modal";
import { connect } from "react-redux";
import { Scrollbar } from "react-scrollbars-custom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { fetchDirectories } from "../actions/fileBrowserActions.js";
import { newLibrary } from "../actions/libraryActions.js";

import "./NewLibraryModal.scss";

class NewLibraryModal extends Component {
    constructor(props) {
        super(props);

        this.nameInput = React.createRef();
        this.mediaTypeInput = React.createRef();

        this.open = this.open.bind(this);
        this.close = this.close.bind(this);
        this.goBack = this.goBack.bind(this);
        this.add = this.add.bind(this);

        // ! UPDATE this.state.root ON componentDidMount WITH /api/v1/filebrowser/
        this.state = {
            visible: false,
            root: "/",
            current: "",
            previous: "",
            cache: false,
            name: "",
            media_type: ""
        };
    }

    onChange(input, e) {
        this.setState({ [input]: e.target.value });
        this.nameInput.current.style.border = "solid 2px transparent";
        this.mediaTypeInput.current.style.border = "solid 2px transparent";
    };

    open() {
        this.select(this.state.root);
        this.setState({ visible: true });
    }

    close() {
        this.setState({ visible: false });
    }

    select(path) {
        if (path === this.state.current) return;

        if (path in this.props.fileBrowser.cache) {
            return this.setState({
                current: path,
                previous: this.state.current,
                cache: this.props.fileBrowser.cache[path]
            });
        }

        this.props.fetchDirectories(path, this.props.auth.token);

        this.setState({
            current: path,
            previous: this.state.current,
            cache: false
        });
    }

    goBack() {
        if (this.state.current === this.state.root) return;

        const path = this.state.current.split("/");
        path.pop();
        this.select(path.join("/"));
    }

    add() {
        if (!this.state.name) {
            this.nameInput.current.style.border = "solid 2px #ff6961";
        }

        if (!this.state.media_type) {
            this.mediaTypeInput.current.style.border = "solid 2px #ff6961";
        }

        if (this.state.name && this.state.media_type) {
            const data = {
                name: this.state.name,
                location: this.state.current,
                media_type: this.state.media_type
            };

            this.props.newLibrary(data);
            this.close();
        }

    }

    render() {
        let dirs;

        if (!this.state.cache) {
            // FETCH_DIRECTORIES_START
            if (this.props.fileBrowser.fetching) {
                dirs = <div className="spinner"/>;
            }

            // FETCH_DIRECTORIES_ERR
            if (this.props.fileBrowser.fetched && this.props.fileBrowser.error) {
                dirs = (
                    <div className="empty">
                        <FontAwesomeIcon icon="times-circle"/>
                        <p>FAILED TO LOAD</p>
                    </div>
                );
            }

            // FETCH_DIRECTORIES_OK
            if (this.props.fileBrowser.fetched && !this.props.fileBrowser.error) {
                const { items } = this.props.fileBrowser;

                if (items.length === 0) {
                    dirs = (
                        <div className="empty">
                            <FontAwesomeIcon icon="times-circle"/>
                            <p>NO FOLDERS</p>
                        </div>
                    );
                } else {
                    dirs = items.map((dir, i) => {
                        return (
                        <div key={i} onClick={() => this.select(dir)} className="dir">
                            <FontAwesomeIcon icon="folder"/>
                            <p>{dir.replace(`${this.state.current}/`, "")}</p>
                        </div>
                    )});
                }
            }
        } else {
            const items = this.state.cache;

            if (items.length === 0) {
                dirs = (
                    <div className="empty">
                        <FontAwesomeIcon icon="times-circle"/>
                        <p>NO FOLDERS</p>
                    </div>
                );
            } else {
                dirs = items.map((dir, i) => (
                    <div key={i} onClick={() => this.select(dir)} className="dir">
                        <FontAwesomeIcon icon="folder"/>
                        <p>{dir.replace(`${this.state.current}/`, "")}</p>
                    </div>
                ));
            }
        }

        return (
            <Fragment>
                <button onClick={this.open}>+</button>
                <Modal
                    isOpen={this.state.visible}
                    contentLabel="newLibrary"
                    className="newLibraryPopup"
                    onRequestClose={this.close}
                    overlayClassName="popupOverlay"
                >
                    <h3>ADD LIBRARY</h3>
                    <h2>{this.state.current}</h2>
                    <div className="selection">
                        <div className="left">
                            <div className="dirs">
                                <Scrollbar>{dirs}</Scrollbar>
                            </div>
                        </div>
                        <div className="right">
                            <input
                                ref={this.nameInput}
                                onChange={(e) => this.onChange("name", e)}
                                placeholder="NAME"
                                type="text"
                                value={this.state.name}
                            />
                            <input
                                ref={this.mediaTypeInput}
                                onChange={(e) => this.onChange("media_type", e)}
                                placeholder="MEDIA TYPE"
                                type="text"
                                value={this.state.media_type}
                            />
                        </div>
                    </div>
                    <div className="options">
                        <div className="page-controls">
                            <button onClick={this.goBack}>
                                <FontAwesomeIcon icon="arrow-left"/>
                            </button>
                        </div>
                        <div className="select">
                            <button onClick={this.close}>CANCEL</button>
                            <button onClick={this.add}>
                                ADD
                                <FontAwesomeIcon icon="plus-circle"/>
                            </button>
                        </div>
                    </div>
                </Modal>
            </Fragment>
        )
    }
}

const mapStateToProps = (state) => ({
    auth: state.authReducer,
    fileBrowser: state.fileBrowserReducer,
    library: state.libraryReducer.new_library
});

const mapActionsToProps = {
    newLibrary,
    fetchDirectories
};

export default connect(mapStateToProps, mapActionsToProps)(NewLibraryModal);
