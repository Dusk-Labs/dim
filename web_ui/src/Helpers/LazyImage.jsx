import React, { Component } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

class LazyImage extends Component {
    constructor(props) {
        super(props);

        this._isMounted = false;

        this.state = {
            fetching: false,
            fetched: false,
            error: false
        };
    }

    componentDidMount() {
        this._isMounted = true;
        this.renderBlob();
    }

    componentWillUnmount() {
        this._isMounted = false;
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
        this.setState({
            fetching: true,
            fetched: false,
            error: false
        });

        const res = await fetch(this.props.src);

        if ((!res.headers.get("content-type") === "image/jpeg" || !this.props.src) && this._isMounted) {
            return this.setState({
                fetching: false,
                fetched: true,
                error: true
            });
        }

        const blob = await res.blob();
        const objectUrl = URL.createObjectURL(blob);

        if (this._isMounted) {
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
    }

    render() {
        // FETCHING
        if (this.state.fetching) {
            if (!this.props.loading) {
                return (
                    <div className="placeholder">
                        <div className="spinner"/>
                    </div>
                );
            } else return this.props.loading;
        }

        // ERR
        if (this.state.fetched && this.state.error) {
            if (!this.props.onFail) {
                return (
                    <div className="placeholder">
                        <div className="vertical-err">
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
                    <img src={this.state.objectUrl} alt={this.props.alt}/>
                </div>
            );
        }

        return <div className="placeholder"/>;
    }
}

export default LazyImage;
