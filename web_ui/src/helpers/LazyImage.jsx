import React, { Component } from "react";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

class LazyImage extends Component {
    constructor(props) {
        super(props);

        this.state = {
            blob: undefined,
            objectUrl: undefined,
            blankMsg: (
                <div className="empty">
                    <FontAwesomeIcon icon="question-circle"/>
                    <p>LOADING COVER</p>
                </div>
            ),
        }
    }

    async componentDidMount() {
        if (this.props.passImageRef) {
            this.props.passImageRef(this.image);
        }

        const resp = await fetch(this.props.src);

        if (!resp.headers.get("content-type") === "image/jpeg") {
            this.setState({
                blankMsg: (
                    <div className="empty">
                        <FontAwesomeIcon icon="question-circle"/>
                        <p>FAILED TO LOAD COVER</p>
                    </div>
                )
            });

            throw new Error("Not an image");
        }

        const blob = await resp.blob();
        const objectUrl = URL.createObjectURL(blob);

        this.setState({
            blob,
            objectUrl
        });

        if (this.props.onLoad) {
            await this.props.onLoad(blob);
        }
    }

    componentWillUnmount() {
        if (this.state.objectUrl) {
            URL.revokeObjectURL(this.state.objectUrl);
        }
    }

    render() {
        const { alt } = this.props;
        const { objectUrl, blankMsg } = this.state;

        return objectUrl !== undefined
            ? <div className="image-wrapper" ref={this.props.imageWrapperRef}><img src={objectUrl} alt={alt}></img></div>
            : <div className="placeholder">{blankMsg}</div>;
    }
}

export default LazyImage;
