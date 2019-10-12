import React, { Component } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

class LazyImage extends Component {
    constructor(props) {
        super(props);

        this.state = {
            fetching: false,
            fetched: false,
            error: false
        };
    }

    componentDidMount() {
        this.renderBlob();
    }

    componentWillUnmount() {
        if (this.state.objectUrl) {
            URL.revokeObjectURL(this.state.objectUrl);
        }
    }

    componentDidUpdate(prevProps) {
        if (prevProps.src !== this.props.src) {
            this.renderBlob();
        }
    }

    async renderBlob() {
        console.log("BEFORE", this.state);
        this.setState({
            fetching: true
        });

        const res = await fetch(this.props.src);

        if (!res.headers.get("content-type") === "image/jpeg" || !this.props.src) {
            return this.setState({
                fetching: false,
                fetched: true,
                error: true
            });
        }

        const blob = await res.blob();
        const objectUrl = URL.createObjectURL(blob);

        this.setState({
            fetching: false,
            fetched: true,
            blob,
            objectUrl
        });

        if (this.props.onLoad) {
            await this.props.onLoad(blob);
        }
    }

    render() {
        // FETCHING
        if (this.state.fetching) {
            if (!this.props.loading) {
                return (
                    <div className="placeholder">
                        <div className="spinner"></div>
                    </div>
                );
            } else return this.props.loading;
        }

        // ERR
        if (this.state.fetched && this.state.error) {
            console.log(this.state);
            if (!this.props.onFail) {
                return (
                    <div className="placeholder">
                        <div className="empty">
                            <FontAwesomeIcon icon="question-circle"/>
                            <p>FAILED TO LOAD</p>
                        </div>
                    </div>
                );
            } else return this.props.onFail();
        }

        // OK
        if (this.state.fetched && !this.state.error) {
            return (
                <div className="image-wrapper" ref={this.props.imageWrapperRef}>
                    <img src={this.state.objectUrl} alt={this.props.alt}></img>
                </div>
            );
        }

        return <div className="placeholder"></div>;
    }
}

export default LazyImage;
