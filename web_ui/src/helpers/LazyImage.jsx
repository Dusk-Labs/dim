import React, { Component } from "react";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

class LazyImage extends Component {
    constructor(props) {
        super(props);

        this.state = {
            img: (
                <div className="placeholder">
                    <div className="empty">
                        <FontAwesomeIcon icon="question-circle"/>
                        <p>LOADING IMAGE</p>
                    </div>
                </div>
            ),
        }
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
        const resp = await fetch(this.props.src);

        if (!resp.headers.get("content-type") === "image/jpeg") {
            this.setState({
                img: (
                    <div className="placeholder">
                        <div className="empty">
                            <FontAwesomeIcon icon="question-circle"/>
                            <p>FAILED TO LOAD IMAGE</p>
                        </div>
                    </div>
                )
            });
        }

        const blob = await resp.blob();
        const objectUrl = URL.createObjectURL(blob);

        this.setState({
            blob,
            objectUrl,
            img: (
                <div className="image-wrapper">
                    <img src={objectUrl} alt={this.props.alt}></img>
                </div>
            )
        });

        if (this.props.onLoad) {
            await this.props.onLoad(blob);
        }
    }

    render() {
        return this.state.img;
    }
}

export default LazyImage;
